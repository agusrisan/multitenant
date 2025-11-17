use crate::moduls::auth::domain::JwtToken;
use crate::shared::{types::*, AppError, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

/// TokenRepository trait defining JWT token persistence operations
///
/// This trait defines the contract for token storage and revocation.
/// Tokens are stored in database to enable revocation (blacklist).
#[async_trait]
pub trait TokenRepository: Send + Sync {
    /// Save new token to database
    ///
    /// Used when generating new access/refresh tokens
    async fn save(&self, token: &JwtToken) -> AppResult<JwtToken>;

    /// Find token by JTI (JWT ID)
    ///
    /// Returns None if token not found
    /// Used for revocation checking
    async fn find_by_jti(&self, jti: Uuid) -> AppResult<Option<JwtToken>>;

    /// Revoke token by JTI
    ///
    /// Sets revoked=true and revoked_at=NOW()
    /// Used for logout and token rotation
    async fn revoke(&self, jti: Uuid) -> AppResult<()>;

    /// Revoke all tokens for a user
    ///
    /// Used for logout (revokes all access and refresh tokens)
    /// Sets revoked=true for all non-revoked tokens
    async fn revoke_all_user_tokens(&self, user_id: UserId) -> AppResult<()>;

    /// Delete all expired tokens
    ///
    /// Cleanup job to remove old tokens from database
    /// Returns number of tokens deleted
    async fn delete_expired(&self) -> AppResult<u64>;
}

/// PostgreSQL implementation of TokenRepository
pub struct PostgresTokenRepository {
    pool: PgPool,
}

impl PostgresTokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TokenRepository for PostgresTokenRepository {
    async fn save(&self, token: &JwtToken) -> AppResult<JwtToken> {
        let result = sqlx::query_as::<_, JwtToken>(
            r#"
            INSERT INTO jwt_tokens (id, user_id, token_type, jti, expires_at, revoked, revoked_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, token_type, jti, expires_at, revoked, revoked_at, created_at
            "#,
        )
        .bind(token.id)
        .bind(token.user_id)
        .bind(token.token_type)
        .bind(token.jti)
        .bind(token.expires_at)
        .bind(token.revoked)
        .bind(token.revoked_at)
        .bind(token.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to save token: {}", e)))?;

        Ok(result)
    }

    async fn find_by_jti(&self, jti: Uuid) -> AppResult<Option<JwtToken>> {
        let result = sqlx::query_as::<_, JwtToken>(
            r#"
            SELECT id, user_id, token_type, jti, expires_at, revoked, revoked_at, created_at
            FROM jwt_tokens
            WHERE jti = $1
            "#,
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find token: {}", e)))?;

        Ok(result)
    }

    async fn revoke(&self, jti: Uuid) -> AppResult<()> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE jwt_tokens
            SET revoked = true, revoked_at = NOW()
            WHERE jti = $1 AND revoked = false
            "#,
        )
        .bind(jti)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to revoke token: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            tracing::warn!("Attempted to revoke non-existent or already revoked token: {}", jti);
        }

        Ok(())
    }

    async fn revoke_all_user_tokens(&self, user_id: UserId) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE jwt_tokens
            SET revoked = true, revoked_at = NOW()
            WHERE user_id = $1 AND revoked = false
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to revoke user tokens: {}", e)))?;

        Ok(())
    }

    async fn delete_expired(&self) -> AppResult<u64> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM jwt_tokens
            WHERE expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete expired tokens: {}", e)))?
        .rows_affected();

        Ok(rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would go here
    // Requires test database setup
}
