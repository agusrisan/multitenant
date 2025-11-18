use crate::shared::{
    types::{Timestamp, UserId},
    AppError, AppResult,
};

/// UserProfile domain entity
/// Represents user profile information extending beyond authentication concerns
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct UserProfile {
    #[serde(rename = "id")]
    pub user_id: UserId,
    pub name: String,
    pub email: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub updated_at: Timestamp,
}

impl UserProfile {
    /// Update user's display name
    /// Business Rule: Name cannot be empty
    pub fn update_name(&mut self, name: String) -> AppResult<()> {
        if name.trim().is_empty() {
            return Err(AppError::Validation("Name cannot be empty".into()));
        }

        self.name = name;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Update user's bio
    /// Business Rule: Bio max 500 characters (optional)
    pub fn update_bio(&mut self, bio: Option<String>) -> AppResult<()> {
        if let Some(ref b) = bio {
            if b.len() > 500 {
                return Err(AppError::Validation(
                    "Bio cannot exceed 500 characters".into(),
                ));
            }
        }

        self.bio = bio;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Update user's avatar URL
    /// Business Rule: Avatar URL validation (optional)
    pub fn update_avatar(&mut self, avatar_url: Option<String>) -> AppResult<()> {
        if let Some(ref url) = avatar_url {
            // Basic URL validation
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(AppError::Validation(
                    "Avatar URL must be a valid HTTP/HTTPS URL".into(),
                ));
            }
        }

        self.avatar_url = avatar_url;
        self.updated_at = chrono::Utc::now();
        Ok(())
    }

    /// Validate all profile fields
    pub fn validate(&self) -> AppResult<()> {
        if self.name.trim().is_empty() {
            return Err(AppError::Validation("Name cannot be empty".into()));
        }

        if let Some(ref bio) = self.bio {
            if bio.len() > 500 {
                return Err(AppError::Validation(
                    "Bio cannot exceed 500 characters".into(),
                ));
            }
        }

        if let Some(ref url) = self.avatar_url {
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(AppError::Validation(
                    "Avatar URL must be a valid HTTP/HTTPS URL".into(),
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::types::new_id;

    fn create_test_profile() -> UserProfile {
        UserProfile {
            user_id: new_id(),
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            bio: None,
            avatar_url: None,
            updated_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_update_name_success() {
        let mut profile = create_test_profile();
        assert!(profile.update_name("New Name".to_string()).is_ok());
        assert_eq!(profile.name, "New Name");
    }

    #[test]
    fn test_update_name_empty_fails() {
        let mut profile = create_test_profile();
        assert!(profile.update_name("".to_string()).is_err());
        assert!(profile.update_name("   ".to_string()).is_err());
    }

    #[test]
    fn test_update_bio_success() {
        let mut profile = create_test_profile();
        assert!(profile.update_bio(Some("Short bio".to_string())).is_ok());
        assert_eq!(profile.bio, Some("Short bio".to_string()));
    }

    #[test]
    fn test_update_bio_too_long_fails() {
        let mut profile = create_test_profile();
        let long_bio = "a".repeat(501);
        assert!(profile.update_bio(Some(long_bio)).is_err());
    }

    #[test]
    fn test_update_avatar_valid_url() {
        let mut profile = create_test_profile();
        assert!(profile
            .update_avatar(Some("https://example.com/avatar.jpg".to_string()))
            .is_ok());
    }

    #[test]
    fn test_update_avatar_invalid_url() {
        let mut profile = create_test_profile();
        assert!(profile
            .update_avatar(Some("not-a-url".to_string()))
            .is_err());
    }
}
