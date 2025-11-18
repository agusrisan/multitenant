use crate::bootstrap::AppState;
use axum::{
    routing::get,
    Router,
};

use super::handlers;

/// User web routes (Inertia.js / session-based authentication)
/// All routes require authentication via session middleware
pub fn user_web_routes() -> Router<AppState> {
    Router::new()
        // Profile viewing
        .route("/profile", get(handlers::show_profile))
        // Profile editing
        .route(
            "/profile/edit",
            get(handlers::show_edit_profile).post(handlers::handle_update_profile),
        )
        // Password change
        .route(
            "/settings/password",
            get(handlers::show_change_password).post(handlers::handle_change_password),
        )
    // TODO: Add session authentication middleware when implemented
    // .layer(middleware::session_layer())
}
