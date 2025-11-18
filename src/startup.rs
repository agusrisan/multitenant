use crate::bootstrap::AppState;
use crate::moduls::auth::{auth_api_routes, auth_web_routes};
use crate::moduls::user::{user_api_routes, user_web_routes};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Serialize;
use axum::http::{header, HeaderValue, Method};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    set_header::SetResponseHeaderLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

/// Health check response
#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    database: String,
    timestamp: String,
}

/// Build the Axum application with all routes and middleware
pub async fn build_app(state: AppState) -> Router {
    tracing::info!("Building application router...");

    // Configure CORS - restrict origins in production
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string());

    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .filter_map(|origin| origin.trim().parse().ok())
        .collect();

    let cors = if origins.is_empty() {
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_credentials(true)
    };

    // Create the main router
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Mount authentication routes
        .nest("/web/auth", auth_web_routes())
        .nest("/api/auth", auth_api_routes())
        // Mount user module routes
        .nest("/web/user", user_web_routes())
        .nest("/api/user", user_api_routes(state.clone()))
        .with_state(state.clone())
        // Add security headers
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        // Add CORS middleware
        .layer(cors)
        // Add compression middleware
        .layer(CompressionLayer::new())
        // Add tracing middleware
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(LatencyUnit::Micros),
                ),
        );

    tracing::info!("Application router built successfully");
    app
}

/// Health check handler
async fn health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, StatusCode> {
    // Check database connectivity
    let db_status = match crate::bootstrap::database::health_check(&state.db).await {
        Ok(_) => "connected",
        Err(e) => {
            tracing::error!("Database health check failed: {:?}", e);
            "disconnected"
        }
    };

    let response = HealthResponse {
        status: "healthy".to_string(),
        database: db_status.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    // If database is down, return 503 Service Unavailable
    if db_status == "disconnected" {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            database: "connected".to_string(),
            timestamp: "2025-01-17T10:30:00Z".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("healthy"));
        assert!(json.contains("connected"));
    }
}
