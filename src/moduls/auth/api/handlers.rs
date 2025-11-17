use crate::bootstrap::AppState;
use crate::moduls::auth::application::{
    RegisterUserCommand, LoginApiCommand, RefreshTokenCommand,
};
use crate::moduls::auth::domain::{TokenPair, UserDto};
use crate::shared::AppError;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

/// Request for API login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Response for API login (token pair)
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: UserDto,
}

impl From<TokenPair> for TokenResponse {
    fn from(token_pair: TokenPair) -> Self {
        Self {
            access_token: token_pair.access_token,
            refresh_token: token_pair.refresh_token,
            token_type: token_pair.token_type,
            expires_in: token_pair.expires_in,
            // User will be added separately
            user: UserDto {
                id: uuid::Uuid::nil(), // Placeholder
                email: String::new(),
                name: String::new(),
                email_verified: false,
                is_active: false,
                created_at: chrono::Utc::now(),
            },
        }
    }
}

/// Response for user info
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub user: UserDto,
}

/// POST /api/auth/register
/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserCommand>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = state.register_user_use_case.execute(payload).await?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse { user }),
    ))
}

/// POST /api/auth/login
/// Login and get JWT token pair
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    let cmd = LoginApiCommand {
        email: payload.email,
        password: payload.password,
    };

    let result = state.login_user_use_case.login_api(cmd).await?;

    let mut response = TokenResponse::from(result.token_pair);
    response.user = result.user;

    Ok(Json(response))
}

/// POST /api/auth/refresh
/// Refresh access token using refresh token
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshTokenCommand>,
) -> Result<Json<TokenResponse>, AppError> {
    let token_pair = state.refresh_token_use_case.execute(payload).await?;

    let response = TokenResponse::from(token_pair);

    Ok(Json(response))
}

/// POST /api/auth/logout
/// Logout and revoke all tokens
/// Requires authentication (JWT middleware)
pub async fn logout(
    State(state): State<AppState>,
    // TODO: Extract user from JWT middleware
    // AuthUser(user): AuthUser,
) -> Result<StatusCode, AppError> {
    // For now, this is a placeholder
    // Will need to extract user_id from JWT token in middleware
    // state.logout_user_use_case.logout_api(user.id).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/auth/me
/// Get current authenticated user
/// Requires authentication (JWT middleware)
pub async fn me(
    // TODO: Extract user from JWT middleware
    // AuthUser(user): AuthUser,
) -> Result<Json<UserResponse>, AppError> {
    // Placeholder - will be implemented with JWT middleware
    Err(AppError::authentication("Not implemented yet"))
}
