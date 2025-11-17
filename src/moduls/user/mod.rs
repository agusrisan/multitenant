// User Module
// Domain-Driven Design (DDD) structure for user profile management
//
// This module extends user functionality beyond authentication concerns.
// It follows the same layered architecture as the auth module:
// - Domain: Business entities and rules (UserProfile)
// - Application: Use cases (GetProfile, UpdateProfile, ChangePassword)
// - Infrastructure: Data persistence (PostgresUserProfileRepository)
// - Web: Inertia.js handlers for session-based auth
// - API: JSON handlers for JWT-based auth

pub mod api;
pub mod application;
pub mod domain;
pub mod infra;
pub mod web;

// Re-export commonly used items
pub use api::user_api_routes;
pub use web::user_web_routes;
