use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Configuration for database connection
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connect_timeout: u64,
}

impl DatabaseConfig {
    /// Create database configuration from environment variables
    pub fn from_env() -> Result<Self, String> {
        let url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;

        let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| "DATABASE_MAX_CONNECTIONS must be a valid number".to_string())?;

        let connect_timeout = std::env::var("DATABASE_CONNECT_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|_| "DATABASE_CONNECT_TIMEOUT must be a valid number".to_string())?;

        Ok(Self {
            url,
            max_connections,
            connect_timeout,
        })
    }
}

/// Initialize database connection pool
pub async fn init_database(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Initializing database connection pool...");
    tracing::debug!(
        "Database config: max_connections={}, connect_timeout={}s",
        config.max_connections,
        config.connect_timeout
    );

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(5) // Keep minimum connections alive
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .idle_timeout(Some(Duration::from_secs(600))) // 10 minutes
        .max_lifetime(Some(Duration::from_secs(1800))) // 30 minutes
        .connect(&config.url)
        .await?;

    tracing::info!("Database connection pool initialized successfully");

    // Test the connection
    health_check(&pool).await?;

    Ok(pool)
}

/// Check database health
pub async fn health_check(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;

    tracing::debug!("Database health check passed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config_from_env() {
        std::env::set_var("DATABASE_URL", "postgres://localhost/test");
        std::env::set_var("DATABASE_MAX_CONNECTIONS", "5");
        std::env::set_var("DATABASE_CONNECT_TIMEOUT", "20");

        let config = DatabaseConfig::from_env().unwrap();
        assert_eq!(config.url, "postgres://localhost/test");
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.connect_timeout, 20);
    }
}
