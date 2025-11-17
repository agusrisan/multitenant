/// Authentication module
///
/// This module implements user authentication following DDD and Clean Architecture:
/// - Domain: Business entities and rules (User, Session, TokenPair, value objects)
/// - Application: Use cases (register, login, logout, refresh)
/// - Infrastructure: Repositories (PostgreSQL implementations)
/// - Web: Inertia.js handlers with session-based auth
/// - API: JSON handlers with JWT-based auth

pub mod domain;
pub mod application;
pub mod infra;
pub mod web;
pub mod api;

// Re-export routes for easy mounting
pub use web::auth_web_routes;
pub use api::auth_api_routes;
