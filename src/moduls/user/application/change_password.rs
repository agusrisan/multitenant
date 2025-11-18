use crate::moduls::auth::infra::UserRepository;
use crate::shared::{types::UserId, AppError, AppResult};
use std::sync::Arc;
use validator::Validate;

/// Change Password Command (DTO)
/// Input data for changing user password
#[derive(Debug, Clone, serde::Deserialize, Validate)]
pub struct ChangePasswordCommand {
    pub current_password: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,

    #[serde(default)]
    pub new_password_confirmation: Option<String>,
}

/// Change Password Use Case
/// Allows users to change their password with verification
pub struct ChangePasswordUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl ChangePasswordUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Execute the use case to change a user's password
    pub async fn execute(&self, user_id: UserId, cmd: ChangePasswordCommand) -> AppResult<()> {
        // 1. Validate input
        cmd.validate()
            .map_err(|e| AppError::Validation(e.to_string()))?;

        // 2. Check password confirmation matches (if provided)
        if let Some(ref confirmation) = cmd.new_password_confirmation {
            if &cmd.new_password != confirmation {
                return Err(AppError::Validation("Passwords do not match".into()));
            }
        }

        // 3. Load user
        let mut user = self
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".into()))?;

        // 4. Verify current password
        if !user.verify_password(&cmd.current_password)? {
            return Err(AppError::Authentication(
                "Invalid current password".into(),
            ));
        }

        // 5. Change password (business rule: password hashing applied)
        user.change_password(&cmd.new_password)?;

        // 6. Save updated user
        self.user_repo.update(&user).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moduls::auth::domain::{Email, User};
    use async_trait::async_trait;

    struct MockUserRepository {
        user: Option<User>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, _id: UserId) -> AppResult<Option<User>> {
            Ok(self.user.clone())
        }

        async fn find_by_email(&self, _email: &Email) -> AppResult<Option<User>> {
            Ok(self.user.clone())
        }

        async fn save(&self, user: &User) -> AppResult<User> {
            Ok(user.clone())
        }

        async fn update(&self, user: &User) -> AppResult<User> {
            Ok(user.clone())
        }

        async fn delete(&self, _id: UserId) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_change_password_success() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "oldpassword123", "Test User".to_string()).unwrap();
        let user_id = user.id;

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "oldpassword123".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: Some("newpassword123".to_string()),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_change_password_mismatch_fails() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "oldpassword123", "Test User".to_string()).unwrap();
        let user_id = user.id;

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "oldpassword123".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: Some("differentpassword".to_string()),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_change_password_wrong_current_fails() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "oldpassword123", "Test User".to_string()).unwrap();
        let user_id = user.id;

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "wrongpassword".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: Some("newpassword123".to_string()),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_err());
    }
}
