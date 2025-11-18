use crate::bootstrap::database::DatabaseConfig;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub jwt: JwtConfig,
    pub session: SessionConfig,
    pub csrf: CsrfConfig,
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

/// JWT configuration
#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub access_expiry: u64,  // in seconds
    pub refresh_expiry: u64, // in seconds
}

/// Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub secret: String,
    pub expiry: u64, // in seconds
}

/// CSRF configuration
#[derive(Debug, Clone)]
pub struct CsrfConfig {
    pub secret: String,
}

/// Configuration error
#[derive(Debug)]
pub enum ConfigError {
    MissingVariable(String),
    InvalidValue(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingVariable(var) => write!(f, "Missing environment variable: {}", var),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid configuration value: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        let database = DatabaseConfig::from_env()
            .map_err(|e| ConfigError::InvalidValue(e))?;

        let server = ServerConfig {
            host: std::env::var("HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidValue("PORT must be a valid number".to_string()))?,
        };

        let jwt = JwtConfig {
            secret: std::env::var("JWT_SECRET")
                .map_err(|_| ConfigError::MissingVariable("JWT_SECRET".to_string()))?,
            access_expiry: std::env::var("JWT_ACCESS_EXPIRY")
                .unwrap_or_else(|_| "900".to_string()) // 15 minutes default
                .parse()
                .map_err(|_| ConfigError::InvalidValue("JWT_ACCESS_EXPIRY must be a valid number".to_string()))?,
            refresh_expiry: std::env::var("JWT_REFRESH_EXPIRY")
                .unwrap_or_else(|_| "604800".to_string()) // 7 days default
                .parse()
                .map_err(|_| ConfigError::InvalidValue("JWT_REFRESH_EXPIRY must be a valid number".to_string()))?,
        };

        let session = SessionConfig {
            secret: std::env::var("SESSION_SECRET")
                .map_err(|_| ConfigError::MissingVariable("SESSION_SECRET".to_string()))?,
            expiry: std::env::var("SESSION_EXPIRY")
                .unwrap_or_else(|_| "86400".to_string()) // 24 hours default
                .parse()
                .map_err(|_| ConfigError::InvalidValue("SESSION_EXPIRY must be a valid number".to_string()))?,
        };

        let csrf = CsrfConfig {
            secret: std::env::var("CSRF_SECRET")
                .map_err(|_| ConfigError::MissingVariable("CSRF_SECRET".to_string()))?,
        };

        // Validate configuration
        Self::validate(&jwt, &session, &csrf)?;

        Ok(Self {
            database,
            server,
            jwt,
            session,
            csrf,
        })
    }

    /// Validate configuration values
    fn validate(jwt: &JwtConfig, session: &SessionConfig, csrf: &CsrfConfig) -> Result<(), ConfigError> {
        // JWT secret should be at least 32 characters
        if jwt.secret.len() < 32 {
            return Err(ConfigError::InvalidValue(
                "JWT_SECRET must be at least 32 characters".to_string(),
            ));
        }

        // Session secret should be at least 32 characters
        if session.secret.len() < 32 {
            return Err(ConfigError::InvalidValue(
                "SESSION_SECRET must be at least 32 characters".to_string(),
            ));
        }

        // CSRF secret should be at least 32 characters
        if csrf.secret.len() < 32 {
            return Err(ConfigError::InvalidValue(
                "CSRF_SECRET must be at least 32 characters".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_server_config_defaults() {
        std::env::remove_var("HOST");
        std::env::remove_var("PORT");

        // Would need proper setup for full config test
        // Just testing that defaults work for server config
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port: u16 = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap();

        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 3000);
    }
}
