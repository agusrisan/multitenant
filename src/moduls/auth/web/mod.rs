/// Web layer for authentication module
///
/// This layer provides web routes with Inertia.js integration
/// and session-based authentication.

pub mod routes;
pub mod handlers;
pub mod middleware;

pub use routes::auth_web_routes;
