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

    pub new_password_confirmation: String,
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

        // 2. Check password confirmation matches
        if cmd.new_password != cmd.new_password_confirmation {
            return Err(AppError::Validation("Passwords do not match".into()));
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

        async fn save(&self, _user: &User) -> AppResult<()> {
            Ok(())
        }

        async fn update(&self, _user: &User) -> AppResult<()> {
            Ok(())
        }

        async fn delete(&self, _id: UserId) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_change_password_success() {
        let user_id = UserId::new_v7();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let mut user = User::new(user_id, "Test User".to_string(), email);
        user.change_password("oldpassword123").unwrap();

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "oldpassword123".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: "newpassword123".to_string(),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_change_password_mismatch_fails() {
        let user_id = UserId::new_v7();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let mut user = User::new(user_id, "Test User".to_string(), email);
        user.change_password("oldpassword123").unwrap();

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "oldpassword123".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: "differentpassword".to_string(),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_change_password_wrong_current_fails() {
        let user_id = UserId::new_v7();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let mut user = User::new(user_id, "Test User".to_string(), email);
        user.change_password("oldpassword123").unwrap();

        let repo = Arc::new(MockUserRepository { user: Some(user) });
        let use_case = ChangePasswordUseCase::new(repo);

        let cmd = ChangePasswordCommand {
            current_password: "wrongpassword".to_string(),
            new_password: "newpassword123".to_string(),
            new_password_confirmation: "newpassword123".to_string(),
        };

        let result = use_case.execute(user_id, cmd).await;
        assert!(result.is_err());
    }
}
