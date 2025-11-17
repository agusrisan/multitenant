use crate::shared::{types::*, AppError, AppResult};
use super::value_objects::{Email, PasswordHash};
use serde::{Deserialize, Serialize};

/// User aggregate root for authentication context
/// Represents a user in the system with authentication capabilities
#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    #[serde(skip_serializing)]
    pub password_hash: PasswordHash,
    pub name: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl User {
    /// Create new User entity
    ///
    /// Business Rules:
    /// - Email must be unique (enforced by repository)
    /// - Password must be min 8 chars (enforced by PasswordHash)
    /// - New users: email_verified=false, is_active=true
    /// - Password is hashed with bcrypt
    pub fn new(email: Email, password: &str, name: String) -> AppResult<Self> {
        // Validate name
        let name = name.trim();
        if name.is_empty() {
            return Err(AppError::validation("Name cannot be empty"));
        }

        if name.len() > 255 {
            return Err(AppError::validation("Name must be 255 characters or less"));
        }

        // Hash password (validation happens in PasswordHash::from_plain)
        let password_hash = PasswordHash::from_plain(password)?;

        let now = now();

        Ok(Self {
            id: new_id(),
            email,
            password_hash,
            name: name.to_string(),
            email_verified: false,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    /// Verify provided password against user's password hash
    ///
    /// Returns true if password matches, false otherwise
    pub fn verify_password(&self, password: &str) -> AppResult<bool> {
        self.password_hash.verify(password)
    }

    /// Change user's password
    ///
    /// Validates new password and updates password_hash
    pub fn change_password(&mut self, new_password: &str) -> AppResult<()> {
        // Validate and hash new password
        let new_hash = PasswordHash::from_plain(new_password)?;

        self.password_hash = new_hash;
        self.updated_at = now();

        Ok(())
    }

    /// Mark email as verified
    ///
    /// Called after user confirms email verification link
    pub fn verify_email(&mut self) {
        self.email_verified = true;
        self.updated_at = now();
    }

    /// Deactivate user account
    ///
    /// Deactivated users cannot login
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = now();
    }

    /// Reactivate user account
    ///
    /// Allows deactivated user to login again
    pub fn reactivate(&mut self) {
        self.is_active = true;
        self.updated_at = now();
    }

    /// Update user's name
    pub fn update_name(&mut self, name: String) -> AppResult<()> {
        let name = name.trim();
        if name.is_empty() {
            return Err(AppError::validation("Name cannot be empty"));
        }

        if name.len() > 255 {
            return Err(AppError::validation("Name must be 255 characters or less"));
        }

        self.name = name.to_string();
        self.updated_at = now();

        Ok(())
    }

    /// Check if user can login
    ///
    /// User must be active to login
    pub fn can_login(&self) -> bool {
        self.is_active
    }
}

/// DTO for creating a new user (from request)
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// DTO for user response (excludes sensitive data)
#[derive(Debug, Serialize)]
pub struct UserDto {
    pub id: UserId,
    pub email: String,
    pub name: String,
    pub email_verified: bool,
    pub is_active: bool,
    pub created_at: Timestamp,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email.into_inner(),
            name: user.name,
            email_verified: user.email_verified,
            is_active: user.is_active,
            created_at: user.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "password123", "Test User".to_string()).unwrap();

        assert_eq!(user.email.as_str(), "test@example.com");
        assert_eq!(user.name, "Test User");
        assert!(!user.email_verified);
        assert!(user.is_active);
    }

    #[test]
    fn test_create_user_empty_name() {
        let email = Email::new("test@example.com").unwrap();
        let result = User::new(email, "password123", "   ".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_verify_password() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "password123", "Test User".to_string()).unwrap();

        assert!(user.verify_password("password123").unwrap());
        assert!(!user.verify_password("wrongpassword").unwrap());
    }

    #[test]
    fn test_change_password() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "password123", "Test User".to_string()).unwrap();

        // Change password
        user.change_password("newpassword456").unwrap();

        // Old password should not work
        assert!(!user.verify_password("password123").unwrap());

        // New password should work
        assert!(user.verify_password("newpassword456").unwrap());
    }

    #[test]
    fn test_verify_email() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "password123", "Test User".to_string()).unwrap();

        assert!(!user.email_verified);

        user.verify_email();

        assert!(user.email_verified);
    }

    #[test]
    fn test_deactivate_reactivate() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "password123", "Test User".to_string()).unwrap();

        assert!(user.is_active);
        assert!(user.can_login());

        user.deactivate();

        assert!(!user.is_active);
        assert!(!user.can_login());

        user.reactivate();

        assert!(user.is_active);
        assert!(user.can_login());
    }

    #[test]
    fn test_update_name() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "password123", "Test User".to_string()).unwrap();

        user.update_name("New Name".to_string()).unwrap();

        assert_eq!(user.name, "New Name");
    }
}
