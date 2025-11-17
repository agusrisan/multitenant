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
use tower_http::{
    cors::CorsLayer,
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

    // Create the main router
    let app = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Mount authentication routes
        .nest("/web/auth", auth_web_routes())
        .nest("/api/auth", auth_api_routes())
        // Mount user module routes
        .nest("/web/user", user_web_routes())
        .nest("/api/user", user_api_routes())
        .with_state(state.clone())
        // Add CORS middleware
        .layer(CorsLayer::permissive()) // TODO: Configure properly for production
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
