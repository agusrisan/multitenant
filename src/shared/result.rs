use super::error::AppError;

/// Application result type alias
///
/// This is a convenience type alias for Result<T, AppError> used throughout
/// the application for consistent error handling.
pub type AppResult<T> = Result<T, AppError>;
