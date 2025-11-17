use crate::bootstrap::AppState;
use axum::{
    routing::{get, put},
    Router,
};

use super::handlers;

/// User API routes (JSON / JWT-based authentication)
/// All routes require authentication via JWT middleware
pub fn user_api_routes() -> Router<AppState> {
    Router::new()
        // Profile operations
        .route(
            "/profile",
            get(handlers::get_profile).put(handlers::update_profile),
        )
        // Password change
        .route("/password", put(handlers::change_password))
    // TODO: Add JWT authentication middleware when implemented
    // .layer(middleware::jwt_layer())
}
