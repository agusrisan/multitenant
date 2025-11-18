use crate::bootstrap::AppState;
use crate::moduls::auth::api::middleware::jwt_auth_middleware;
use axum::{
    middleware,
    routing::{get, put},
    Router,
};

use super::handlers;

/// User API routes (JSON / JWT-based authentication)
/// All routes require authentication via JWT middleware
pub fn user_api_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Profile operations
        .route(
            "/profile",
            get(handlers::get_profile).put(handlers::update_profile),
        )
        // Password change
        .route("/password", put(handlers::change_password))
        // Add JWT authentication middleware to all routes
        .route_layer(middleware::from_fn_with_state(state, jwt_auth_middleware))
}
