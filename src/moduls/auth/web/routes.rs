use crate::bootstrap::AppState;
use super::handlers;
use axum::{
    routing::{get, post},
    Router,
};

/// Create web authentication routes
///
/// Routes:
/// - GET /web/auth/login - Show login page
/// - POST /web/auth/login - Process login
/// - GET /web/auth/register - Show registration page
/// - POST /web/auth/register - Process registration
/// - POST /web/auth/logout - Logout user
pub fn auth_web_routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(handlers::show_login).post(handlers::handle_login))
        .route("/register", get(handlers::show_register).post(handlers::handle_register))
        .route("/logout", post(handlers::handle_logout))
    // TODO: Add CSRF middleware
    // TODO: Add session middleware for protected routes
}
