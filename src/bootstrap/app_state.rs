use crate::config::Config;
use sqlx::PgPool;

/// Shared application state
///
/// This struct contains all shared resources that need to be accessible
/// across the application. It implements Clone for use with Axum's State.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: PgPool,

    /// Application configuration
    pub config: Config,

    /// JWT secret for signing tokens
    pub jwt_secret: String,

    /// Session secret for cookie encryption
    pub session_secret: String,

    /// CSRF secret for token generation
    pub csrf_secret: String,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(
        db: PgPool,
        config: Config,
        jwt_secret: String,
        session_secret: String,
        csrf_secret: String,
    ) -> Self {
        Self {
            db,
            config,
            jwt_secret,
            session_secret,
            csrf_secret,
        }
    }

    /// Get database pool reference
    pub fn db(&self) -> &PgPool {
        &self.db
    }

    /// Get config reference
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_clone() {
        // AppState should implement Clone for Axum state sharing
        fn assert_clone<T: Clone>() {}
        assert_clone::<AppState>();
    }
}
