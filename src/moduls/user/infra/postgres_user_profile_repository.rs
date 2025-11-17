use crate::moduls::user::domain::UserProfile;
use crate::shared::{types::UserId, AppResult};
use async_trait::async_trait;
use sqlx::PgPool;

/// User Profile Repository Trait
/// Defines operations for persisting and retrieving user profiles
#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<UserProfile>>;
    async fn update(&self, profile: &UserProfile) -> AppResult<UserProfile>;
}

/// PostgreSQL implementation of UserProfileRepository
/// Note: Profile data is stored in the users table, not a separate table
pub struct PostgresUserProfileRepository {
    pool: PgPool,
}

impl PostgresUserProfileRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserProfileRepository for PostgresUserProfileRepository {
    /// Find user profile by user ID
    async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<UserProfile>> {
        let profile = sqlx::query_as::<_, UserProfile>(
            r#"
            SELECT
                id as user_id,
                name,
                email,
                bio,
                avatar_url,
                updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(profile)
    }

    /// Update user profile
    async fn update(&self, profile: &UserProfile) -> AppResult<UserProfile> {
        let updated = sqlx::query_as::<_, UserProfile>(
            r#"
            UPDATE users
            SET
                name = $1,
                bio = $2,
                avatar_url = $3,
                updated_at = $4
            WHERE id = $5
            RETURNING
                id as user_id,
                name,
                email,
                bio,
                avatar_url,
                updated_at
            "#,
        )
        .bind(&profile.name)
        .bind(&profile.bio)
        .bind(&profile.avatar_url)
        .bind(profile.updated_at)
        .bind(profile.user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests should be in tests/ directory with actual database
    // These are just placeholder unit tests for the structure

    #[test]
    fn test_repository_creation() {
        // This is a smoke test to ensure the struct can be instantiated
        // Real tests require a database connection
    }
}
