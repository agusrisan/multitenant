use crate::moduls::auth::domain::{User, Email};
use crate::shared::{types::*, AppError, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;

/// UserRepository trait defining user persistence operations
///
/// This trait defines the contract for user storage.
/// Implementations must handle all database-specific logic.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Save new user to database
    ///
    /// # Errors
    /// - Conflict if email already exists (unique constraint violation)
    /// - Database errors
    async fn save(&self, user: &User) -> AppResult<User>;

    /// Find user by ID
    ///
    /// Returns None if user not found
    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;

    /// Find user by email
    ///
    /// Returns None if user not found
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;

    /// Update existing user
    ///
    /// # Errors
    /// - NotFound if user doesn't exist
    /// - Database errors
    async fn update(&self, user: &User) -> AppResult<User>;

    /// Delete user by ID
    ///
    /// # Errors
    /// - NotFound if user doesn't exist
    /// - Database errors
    async fn delete(&self, id: UserId) -> AppResult<()>;
}

/// PostgreSQL implementation of UserRepository
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, email, password_hash, name, email_verified, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, email, password_hash, name, email_verified, is_active, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(user.email.as_str())
        .bind(user.password_hash.as_str())
        .bind(&user.name)
        .bind(user.email_verified)
        .bind(user.is_active)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            // Check for unique constraint violation (email already exists)
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    return AppError::conflict("Email already exists");
                }
            }
            AppError::internal(format!("Failed to save user: {}", e))
        })?;

        Ok(result)
    }

    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, email_verified, is_active, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find user: {}", e)))?;

        Ok(result)
    }

    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, User>(
            r#"
            SELECT id, email, password_hash, name, email_verified, is_active, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email.as_str())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to find user: {}", e)))?;

        Ok(result)
    }

    async fn update(&self, user: &User) -> AppResult<User> {
        let result = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET email = $2, password_hash = $3, name = $4, email_verified = $5, is_active = $6, updated_at = $7
            WHERE id = $1
            RETURNING id, email, password_hash, name, email_verified, is_active, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(user.email.as_str())
        .bind(user.password_hash.as_str())
        .bind(&user.name)
        .bind(user.email_verified)
        .bind(user.is_active)
        .bind(user.updated_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to update user: {}", e)))?
        .ok_or_else(|| AppError::not_found("User not found"))?;

        Ok(result)
    }

    async fn delete(&self, id: UserId) -> AppResult<()> {
        let rows_affected = sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("Failed to delete user: {}", e)))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(AppError::not_found("User not found"));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would go here
    // Requires test database setup
}
