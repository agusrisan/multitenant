use crate::moduls::auth::domain::TokenPair;
use crate::moduls::auth::infra::TokenRepository;
use crate::shared::{AppError, AppResult};
use std::sync::Arc;

/// Command for refreshing access token
#[derive(Debug, serde::Deserialize)]
pub struct RefreshTokenCommand {
    pub refresh_token: String,
}

/// Configuration for token refresh
pub struct RefreshConfig {
    pub jwt_secret: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

/// Use case for refreshing access tokens
///
/// Business Logic:
/// 1. Decode refresh token
/// 2. Extract JTI
/// 3. Check token not revoked in database
/// 4. Check token not expired
/// 5. Revoke old refresh token (token rotation)
/// 6. Generate new TokenPair
/// 7. Save new tokens to database
/// 8. Return new TokenPair
///
/// Security:
/// - Implements refresh token rotation (old token revoked)
/// - Prevents token reuse attacks
/// - Checks JTI blacklist
pub struct RefreshTokenUseCase {
    token_repo: Arc<dyn TokenRepository>,
    config: RefreshConfig,
}

impl RefreshTokenUseCase {
    pub fn new(
        token_repo: Arc<dyn TokenRepository>,
        config: RefreshConfig,
    ) -> Self {
        Self {
            token_repo,
            config,
        }
    }

    /// Execute refresh token use case
    ///
    /// # Arguments
    /// * `cmd` - Command containing refresh token
    ///
    /// # Returns
    /// New TokenPair with fresh access and refresh tokens
    ///
    /// # Errors
    /// - Authentication error if token invalid/expired/revoked
    /// - Database errors
    pub async fn execute(&self, cmd: RefreshTokenCommand) -> AppResult<TokenPair> {
        // 1. Decode refresh token and validate signature
        let claims = TokenPair::decode(&cmd.refresh_token, &self.config.jwt_secret)?;

        // 2. Verify this is a refresh token
        if claims.token_type != "refresh" {
            return Err(AppError::authentication("Invalid token type, expected refresh token"));
        }

        // 3. Extract JTI
        let jti = uuid::Uuid::parse_str(&claims.jti)
            .map_err(|e| AppError::internal(format!("Invalid JTI: {}", e)))?;

        // 4. Check token exists in database and not revoked
        let stored_token = self.token_repo.find_by_jti(jti)
            .await?
            .ok_or_else(|| AppError::authentication("Token not found"))?;

        if stored_token.is_revoked() {
            return Err(AppError::authentication("Token has been revoked"));
        }

        if stored_token.is_expired() {
            return Err(AppError::authentication("Token has expired"));
        }

        // 5. Revoke old refresh token (token rotation for security)
        self.token_repo.revoke(jti).await?;

        // 6. Extract user ID and generate new TokenPair
        let user_id = uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::internal(format!("Invalid user ID: {}", e)))?;

        let (token_pair, access_token, refresh_token) = TokenPair::generate(
            user_id,
            &self.config.jwt_secret,
            self.config.access_ttl_seconds,
            self.config.refresh_ttl_seconds,
        )?;

        // 7. Save new tokens to database
        self.token_repo.save(&access_token).await?;
        self.token_repo.save(&refresh_token).await?;

        // 8. Return new TokenPair
        Ok(token_pair)
    }
}

#[cfg(test)]
mod tests {
    

    // Tests would require mock repositories and token generation
    // Skipping for brevity
}
