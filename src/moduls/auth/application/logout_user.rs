use crate::moduls::auth::infra::{SessionRepository, TokenRepository};
use crate::shared::{types::*, AppResult};
use std::sync::Arc;

/// Use case for user logout (both web and API)
///
/// Supports two logout flows:
/// 1. Web (session-based) - Deletes session
/// 2. API (JWT-based) - Revokes all user tokens
pub struct LogoutUserUseCase {
    session_repo: Arc<dyn SessionRepository>,
    token_repo: Arc<dyn TokenRepository>,
}

impl LogoutUserUseCase {
    pub fn new(
        session_repo: Arc<dyn SessionRepository>,
        token_repo: Arc<dyn TokenRepository>,
    ) -> Self {
        Self {
            session_repo,
            token_repo,
        }
    }

    /// Logout for web (session-based authentication)
    ///
    /// Business Logic:
    /// - Delete session by ID
    ///
    /// # Arguments
    /// * `session_id` - ID of session to delete
    ///
    /// # Errors
    /// - Database errors (not finding session is not an error)
    pub async fn logout_web(&self, session_id: SessionId) -> AppResult<()> {
        self.session_repo.delete(session_id).await?;
        Ok(())
    }

    /// Logout for API (JWT-based authentication)
    ///
    /// Business Logic:
    /// - Revoke all non-revoked tokens for user
    /// - This invalidates all access and refresh tokens
    ///
    /// # Arguments
    /// * `user_id` - ID of user to logout
    ///
    /// # Errors
    /// - Database errors
    ///
    /// # Notes
    /// JWT is stateless, so we need to maintain a blacklist
    /// of revoked tokens in the database. Middleware checks
    /// token revocation status before allowing access.
    pub async fn logout_api(&self, user_id: UserId) -> AppResult<()> {
        self.token_repo.revoke_all_user_tokens(user_id).await?;
        Ok(())
    }

    /// Logout from both web and API
    ///
    /// Used when user wants to logout from all devices/sessions
    ///
    /// # Arguments
    /// * `user_id` - ID of user to logout
    ///
    /// # Errors
    /// - Database errors
    pub async fn logout_all(&self, user_id: UserId) -> AppResult<()> {
        // Delete all sessions
        self.session_repo.delete_by_user_id(user_id).await?;

        // Revoke all tokens
        self.token_repo.revoke_all_user_tokens(user_id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests would require mock repositories
    // Skipping for brevity
}
