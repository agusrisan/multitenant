use crate::moduls::auth::domain::Session;
use crate::shared::{types::*, AppError, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;

/// SessionRepository trait defining session persistence operations
///
/// This trait defines the contract for session storage.
/// Sessions are used for web-based authentication with cookies.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Save new session to database
    ///
    /// # Business Rules
    /// - Enforces single session per user (deletes existing sessions)
    async fn save(&self, session: &Session) -> AppResult<Session>;

    /// Find session by ID
    ///
    /// Returns None if session not found
    async fn find_by_id(&self, id: SessionId) -> AppResult<Option<Session>>;

    /// Find session by user ID
    ///
    /// Returns most recent session for user
    /// Used to enforce single session per user
    async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<Session>>;

    /// Delete session by ID
    ///
    /// Used for logout
    async fn delete(&self, id: SessionId) -> AppResult<()>;

    /// Delete all sessions for a user
    ///
    /// Used when enforcing single session per user
    async fn delete_by_user_id(&self, user_id: UserId) -> AppResult<()>;

    /// Delete all expired sessions
    ///
    /// Cleanup job to remove old sessions
    /// Returns number of sessions deleted
    async fn delete_expired(&self) -> AppResult<u64>;
}

/// PostgreSQL implementation of SessionRepository
pub struct PostgresSessionRepository {
    pool: PgPool,
}

impl PostgresSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PostgresSessionRepository {
    async fn save(&self, session: &Session) -> AppResult<Session> {
        // First, delete any existing sessions for this user (single session per user)
        self.delete_by_user_id(session.user_id).await?;

        // Insert new session
        let result = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (id, user_id, csrf_token, ip_address, user_agent, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, csrf_token, ip_address, user_agent, expires_at, created_at, updated_at
            "#,
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.csrf_token.as_str())
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(session.expires_at)
        .bind(session.created_at)
        .bind(session.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to save session: {}", e)))?;

        Ok(result)
    }

    async fn find_by_id(&self, id: SessionId) -> AppResult<Option<Session>> {
        let result = sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, csrf_token, ip_address, user_agent, expires_at, created_at, updated_at
            FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find session: {}", e)))?;

        Ok(result)
    }

    async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<Session>> {
        let result = sqlx::query_as::<_, Session>(
            r#"
            SELECT id, user_id, csrf_token, ip_address, user_agent, expires_at, created_at, updated_at
            FROM sessions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find session: {}", e)))?;

        Ok(result)
    }

    async fn delete(&self, id: SessionId) -> AppResult<()> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete session: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            // Not necessarily an error - session might already be deleted
            tracing::warn!("Attempted to delete non-existent session: {}", id);
        }

        Ok(())
    }

    async fn delete_by_user_id(&self, user_id: UserId) -> AppResult<()> {
        sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete sessions: {}", e)))?;

        Ok(())
    }

    async fn delete_expired(&self) -> AppResult<u64> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM sessions
            WHERE expires_at < NOW()
            "#,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete expired sessions: {}", e)))?
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
