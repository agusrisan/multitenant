use crate::moduls::user::domain::UserProfile;
use crate::moduls::user::infra::UserProfileRepository;
use crate::shared::{types::UserId, AppError, AppResult};
use std::sync::Arc;

/// Get Profile Use Case
/// Retrieves user profile information
pub struct GetProfileUseCase {
    profile_repo: Arc<dyn UserProfileRepository>,
}

impl GetProfileUseCase {
    pub fn new(profile_repo: Arc<dyn UserProfileRepository>) -> Self {
        Self { profile_repo }
    }

    /// Execute the use case to get a user's profile
    pub async fn execute(&self, user_id: UserId) -> AppResult<UserProfile> {
        self.profile_repo
            .find_by_user_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Profile not found".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moduls::user::domain::UserProfile;
    use async_trait::async_trait;

    struct MockUserProfileRepository {
        profile: Option<UserProfile>,
    }

    #[async_trait]
    impl UserProfileRepository for MockUserProfileRepository {
        async fn find_by_user_id(&self, _user_id: UserId) -> AppResult<Option<UserProfile>> {
            Ok(self.profile.clone())
        }

        async fn update(&self, profile: &UserProfile) -> AppResult<UserProfile> {
            Ok(profile.clone())
        }
    }

    #[tokio::test]
    async fn test_get_profile_success() {
        let user_id = UserId::new_v7();
        let profile = UserProfile {
            user_id,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: None,
            avatar_url: None,
            updated_at: chrono::Utc::now(),
        };

        let repo = Arc::new(MockUserProfileRepository {
            profile: Some(profile.clone()),
        });
        let use_case = GetProfileUseCase::new(repo);

        let result = use_case.execute(user_id).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "Test User");
    }

    #[tokio::test]
    async fn test_get_profile_not_found() {
        let repo = Arc::new(MockUserProfileRepository { profile: None });
        let use_case = GetProfileUseCase::new(repo);

        let result = use_case.execute(UserId::new_v7()).await;
        assert!(result.is_err());
    }
}
