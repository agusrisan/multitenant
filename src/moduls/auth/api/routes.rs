use crate::bootstrap::AppState;
use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

/// Create API authentication routes
///
/// Routes:
/// - POST /api/auth/register - Register new user
/// - POST /api/auth/login - Login and get JWT tokens
/// - POST /api/auth/refresh - Refresh access token
/// - POST /api/auth/logout - Logout (revoke tokens) [requires auth]
/// - GET /api/auth/me - Get current user [requires auth]
pub fn auth_api_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/refresh", post(handlers::refresh))
        .route("/logout", post(handlers::logout))
        .route("/me", get(handlers::me))
    // TODO: Add JWT middleware for protected routes (logout, me)
    // .route_layer(middleware::from_fn_with_state(state.clone(), jwt_auth_middleware))
}
