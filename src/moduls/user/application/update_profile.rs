use crate::moduls::user::domain::UserProfile;
use crate::moduls::user::infra::UserProfileRepository;
use crate::shared::{types::UserId, AppError, AppResult};
use std::sync::Arc;
use validator::Validate;

/// Update Profile Command (DTO)
/// Input data for updating user profile
#[derive(Debug, Clone, serde::Deserialize, Validate)]
pub struct UpdateProfileCommand {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(length(max = 500, message = "Bio cannot exceed 500 characters"))]
    pub bio: Option<String>,

    #[validate(url(message = "Avatar URL must be a valid URL"))]
    pub avatar_url: Option<String>,
}

/// Update Profile Use Case
/// Updates user profile information with validation
pub struct UpdateProfileUseCase {
    profile_repo: Arc<dyn UserProfileRepository>,
}

impl UpdateProfileUseCase {
    pub fn new(profile_repo: Arc<dyn UserProfileRepository>) -> Self {
        Self { profile_repo }
    }

    /// Execute the use case to update a user's profile
    pub async fn execute(
        &self,
        user_id: UserId,
        cmd: UpdateProfileCommand,
    ) -> AppResult<UserProfile> {
        // 1. Validate input
        cmd.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // 2. Load current profile
        let mut profile = self
            .profile_repo
            .find_by_user_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Profile not found".into()))?;

        // 3. Update fields using domain methods (business rules applied)
        profile.update_name(cmd.name)?;
        profile.update_bio(cmd.bio)?;
        profile.update_avatar(cmd.avatar_url)?;

        // 4. Save and return updated profile
        self.profile_repo.update(&profile).await
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
    async fn test_update_profile_success() {
        let user_id = UserId::new_v7();
        let profile = UserProfile {
            user_id,
            name: "Old Name".to_string(),
            email: "test@example.com".to_string(),
            bio: None,
            avatar_url: None,
            updated_at: chrono::Utc::now(),
        };

        let repo = Arc::new(MockUserProfileRepository {
            profile: Some(profile),
        });
        let use_case = UpdateProfileUseCase::new(repo);

        let cmd = UpdateProfileCommand {
            name: "New Name".to_string(),
            bio: Some("New bio".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.name, "New Name");
        assert_eq!(updated.bio, Some("New bio".to_string()));
    }

    #[tokio::test]
    async fn test_update_profile_empty_name_fails() {
        let user_id = UserId::new_v7();
        let profile = UserProfile {
            user_id,
            name: "Old Name".to_string(),
            email: "test@example.com".to_string(),
            bio: None,
            avatar_url: None,
            updated_at: chrono::Utc::now(),
        };

        let repo = Arc::new(MockUserProfileRepository {
            profile: Some(profile),
        });
        let use_case = UpdateProfileUseCase::new(repo);

        let cmd = UpdateProfileCommand {
            name: "".to_string(),
            bio: None,
            avatar_url: None,
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_err());
    }
}
