/// Infrastructure layer for authentication module
///
/// This layer contains concrete implementations of repository interfaces
/// and external service integrations (database, etc).

pub mod postgres_user_repository;
pub mod postgres_session_repository;
pub mod postgres_token_repository;

// Re-export repository traits and implementations
pub use postgres_user_repository::{UserRepository, PostgresUserRepository};
pub use postgres_session_repository::{SessionRepository, PostgresSessionRepository};
pub use postgres_token_repository::{TokenRepository, PostgresTokenRepository};
