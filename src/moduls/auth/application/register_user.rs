use crate::moduls::auth::domain::{User, Email, UserDto};
use crate::moduls::auth::infra::UserRepository;
use crate::shared::AppResult;
use std::sync::Arc;
use validator::Validate;

/// Command for registering a new user
#[derive(Debug, serde::Deserialize, Validate)]
pub struct RegisterUserCommand {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    #[validate(length(min = 1))]
    pub name: String,
}

/// Use case for user registration
///
/// Business Logic:
/// 1. Validate input (email format, password length, name)
/// 2. Check email uniqueness
/// 3. Create User entity (hashes password)
/// 4. Save to repository
/// 5. Return created user
///
/// Error Cases:
/// - Email already exists → Conflict error
/// - Invalid email format → Validation error
/// - Password too short → Validation error
pub struct RegisterUserUseCase {
    user_repo: Arc<dyn UserRepository>,
}

impl RegisterUserUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    /// Execute registration use case
    ///
    /// # Arguments
    /// * `cmd` - Command containing email, password, and name
    ///
    /// # Returns
    /// Created User entity
    ///
    /// # Errors
    /// - Validation error if input is invalid
    /// - Conflict error if email already exists
    /// - Database errors
    pub async fn execute(&self, cmd: RegisterUserCommand) -> AppResult<UserDto> {
        // 1. Validate input
        cmd.validate()
            .map_err(|e| crate::shared::AppError::validation(format!("Validation failed: {}", e)))?;

        // 2. Parse and validate email
        let email = Email::new(&cmd.email)?;

        // 3. Check email uniqueness
        if let Some(_existing_user) = self.user_repo.find_by_email(&email).await? {
            return Err(crate::shared::AppError::conflict("Email already exists"));
        }

        // 4. Create User entity (password is hashed in User::new)
        let user = User::new(email, &cmd.password, cmd.name)?;

        // 5. Save to repository
        let saved_user = self.user_repo.save(&user).await?;

        // 6. Return DTO (excludes password hash)
        Ok(UserDto::from(saved_user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moduls::auth::domain::User;
    use crate::shared::AppResult;
    use async_trait::async_trait;

    // Mock repository for testing
    struct MockUserRepository {
        users: std::sync::Mutex<Vec<User>>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: std::sync::Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn save(&self, user: &User) -> AppResult<User> {
            let mut users = self.users.lock().unwrap();
            users.push(user.clone());
            Ok(user.clone())
        }

        async fn find_by_id(&self, id: crate::shared::types::UserId) -> AppResult<Option<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.iter().find(|u| u.id == id).cloned())
        }

        async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>> {
            let users = self.users.lock().unwrap();
            Ok(users.iter().find(|u| u.email.as_str() == email.as_str()).cloned())
        }

        async fn update(&self, user: &User) -> AppResult<User> {
            Ok(user.clone())
        }

        async fn delete(&self, _id: crate::shared::types::UserId) -> AppResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_register_user_success() {
        let repo = Arc::new(MockUserRepository::new());
        let use_case = RegisterUserUseCase::new(repo);

        let cmd = RegisterUserCommand {
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };

        let result = use_case.execute(cmd).await;
        assert!(result.is_ok());

        let user_dto = result.unwrap();
        assert_eq!(user_dto.email, "test@example.com");
        assert_eq!(user_dto.name, "Test User");
    }

    #[tokio::test]
    async fn test_register_user_invalid_email() {
        let repo = Arc::new(MockUserRepository::new());
        let use_case = RegisterUserUseCase::new(repo);

        let cmd = RegisterUserCommand {
            email: "invalid-email".to_string(),
            password: "password123".to_string(),
            name: "Test User".to_string(),
        };

        let result = use_case.execute(cmd).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_user_password_too_short() {
        let repo = Arc::new(MockUserRepository::new());
        let use_case = RegisterUserUseCase::new(repo);

        let cmd = RegisterUserCommand {
            email: "test@example.com".to_string(),
            password: "short".to_string(),
            name: "Test User".to_string(),
        };

        let result = use_case.execute(cmd).await;
        assert!(result.is_err());
    }
}
