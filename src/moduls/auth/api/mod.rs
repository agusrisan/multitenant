/// API layer for authentication module
///
/// This layer provides JSON-based REST API endpoints
/// with JWT authentication.

pub mod routes;
pub mod handlers;
pub mod middleware;

pub use routes::auth_api_routes;
