use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::fmt;

/// Application error types
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

/// Error response structure
#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
    code: String,
}

impl AppError {
    /// Get HTTP status code for this error
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) | AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Database(_) | AppError::Internal(_) | AppError::Config(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Get error code for this error
    fn error_code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Authentication(_) => "AUTHENTICATION_ERROR",
            AppError::Authorization(_) => "AUTHORIZATION_ERROR",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Conflict(_) => "CONFLICT",
            AppError::Internal(_) => "INTERNAL_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::BadRequest(_) => "BAD_REQUEST",
        }
    }

    /// Get user-facing error message
    fn user_message(&self) -> String {
        match self {
            // Don't expose internal error details to users
            AppError::Database(_) => "A database error occurred".to_string(),
            AppError::Internal(_) => "An internal error occurred".to_string(),
            AppError::Config(_) => "A configuration error occurred".to_string(),
            // Other errors can show their messages
            _ => self.to_string(),
        }
    }

    /// Get error details (for debugging)
    fn details(&self) -> Option<String> {
        match self {
            AppError::Database(e) => Some(e.to_string()),
            AppError::Internal(e) => Some(e.clone()),
            AppError::Config(e) => Some(e.clone()),
            _ => None,
        }
    }
}

/// Implement IntoResponse for Axum integration
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();

        // Log internal errors
        match &self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
            }
            AppError::Internal(e) => {
                tracing::error!("Internal error: {}", e);
            }
            AppError::Config(e) => {
                tracing::error!("Config error: {}", e);
            }
            _ => {}
        }

        let error_response = ErrorResponse {
            error: ErrorDetail {
                message: self.user_message(),
                details: if cfg!(debug_assertions) {
                    self.details()
                } else {
                    None
                },
                code: self.error_code().to_string(),
            },
        };

        (status_code, Json(error_response)).into_response()
    }
}

/// Helper trait for adding context to errors
pub trait ErrorContext<T> {
    fn context(self, msg: &str) -> Result<T, AppError>;
    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where
    E: fmt::Display,
{
    fn context(self, msg: &str) -> Result<T, AppError> {
        self.map_err(|e| AppError::Internal(format!("{}: {}", msg, e)))
    }

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| AppError::Internal(format!("{}: {}", f(), e)))
    }
}

// Implement From for common error types
impl From<crate::config::ConfigError> for AppError {
    fn from(err: crate::config::ConfigError) -> Self {
        AppError::Config(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes() {
        assert_eq!(
            AppError::Validation("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::Authentication("test".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            AppError::Authorization("test".to_string()).status_code(),
            StatusCode::FORBIDDEN
        );
        assert_eq!(
            AppError::NotFound("test".to_string()).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            AppError::Conflict("test".to_string()).status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            AppError::Internal("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(
            AppError::Validation("test".to_string()).error_code(),
            "VALIDATION_ERROR"
        );
        assert_eq!(
            AppError::Authentication("test".to_string()).error_code(),
            "AUTHENTICATION_ERROR"
        );
    }
}
