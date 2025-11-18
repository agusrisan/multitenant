// JWT authentication middleware

use crate::bootstrap::AppState;
use crate::moduls::auth::domain::token_pair::TokenPair;
use crate::moduls::auth::infra::postgres_token_repository::TokenRepository;
use crate::shared::error::AppError;
use crate::shared::types::UserId;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

/// Authenticated user extension
/// Add to request extensions after successful JWT validation
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: UserId,
}

/// JWT authentication middleware
///
/// Validates JWT tokens from Authorization header
/// Checks token signature, expiration, and revocation status
/// Adds AuthenticatedUser to request extensions on success
///
/// # Flow
/// 1. Extract Authorization: Bearer <token> header
/// 2. Decode and validate JWT signature
/// 3. Check token not revoked in database
/// 4. Add user_id to request extensions
/// 5. Return 401 if any step fails
pub async fn jwt_auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::authentication("Missing Authorization header"))?;

    // Extract Bearer token
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError::authentication("Invalid Authorization header format"))?;

    // Decode and validate JWT
    let claims = TokenPair::decode(token, &state.jwt_secret)?;

    // Extract JTI and check revocation status
    let jti = uuid::Uuid::parse_str(&claims.jti)
        .map_err(|_| AppError::authentication("Invalid token ID"))?;

    // Check if token is revoked by finding it in database
    if let Some(jwt_token) = state.token_repo.find_by_jti(jti).await? {
        if jwt_token.is_revoked() {
            return Err(AppError::authentication("Token has been revoked"));
        }
    } else {
        // Token not found in database - invalid token
        return Err(AppError::authentication("Token not found"));
    }

    // Extract user ID
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::authentication("Invalid user ID in token"))?;

    // Add authenticated user to request extensions
    let authenticated_user = AuthenticatedUser { user_id };
    request.extensions_mut().insert(authenticated_user);

    // Continue to next middleware/handler
    Ok(next.run(request).await)
}

/// Axum extractor for authenticated user
///
/// Use this in handler parameters to get the authenticated user
/// Will return 401 if user not found in extensions
impl axum::extract::FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Unauthorized - no valid authentication".to_string(),
            ))
    }
}
