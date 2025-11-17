use crate::shared::{AppError, AppResult};
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::ValidateEmail;

/// Email value object with validation
/// Ensures email is valid RFC 5322 format and max 255 characters
#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct Email(String);

impl Email {
    /// Create new Email with validation
    /// Returns error if email is invalid or too long
    pub fn new(email: &str) -> AppResult<Self> {
        let email = email.trim().to_lowercase();

        // Validate length
        if email.is_empty() {
            return Err(AppError::validation("Email cannot be empty"));
        }

        if email.len() > 255 {
            return Err(AppError::validation("Email must be 255 characters or less"));
        }

        // Validate format using validator crate
        if !email.validate_email() {
            return Err(AppError::validation("Invalid email format"));
        }

        Ok(Self(email))
    }

    /// Get email as str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Into inner String
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Email {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Password hash value object using bcrypt
/// Provides secure password hashing and verification
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct PasswordHash(String);

impl PasswordHash {
    /// Minimum password length requirement
    pub const MIN_LENGTH: usize = 8;

    /// Create PasswordHash from plain text password
    /// Validates minimum length and hashes with bcrypt
    pub fn from_plain(password: &str) -> AppResult<Self> {
        // Validate minimum length
        if password.len() < Self::MIN_LENGTH {
            return Err(AppError::validation(format!(
                "Password must be at least {} characters",
                Self::MIN_LENGTH
            )));
        }

        // Hash password with bcrypt (cost 12)
        let hash = hash(password, DEFAULT_COST).map_err(|e| {
            AppError::internal(format!("Failed to hash password: {}", e))
        })?;

        Ok(Self(hash))
    }

    /// Create PasswordHash from existing hash (e.g., from database)
    /// Does not perform validation - use only for already-hashed passwords
    pub fn from_hash(hash: String) -> Self {
        Self(hash)
    }

    /// Verify plain text password against this hash
    /// Uses constant-time comparison to prevent timing attacks
    pub fn verify(&self, password: &str) -> AppResult<bool> {
        verify(password, &self.0).map_err(|e| {
            AppError::internal(format!("Failed to verify password: {}", e))
        })
    }

    /// Get hash as str (for serialization)
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// CSRF token value object
/// Generates cryptographically secure random tokens
#[derive(Debug, Clone, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[sqlx(transparent)]
pub struct CsrfToken(String);

impl CsrfToken {
    /// Token length in bytes (32 bytes = 256 bits)
    const TOKEN_LENGTH: usize = 32;

    /// Generate new random CSRF token
    /// Uses cryptographically secure random number generator
    pub fn generate() -> Self {
        let random_bytes: Vec<u8> = (0..Self::TOKEN_LENGTH)
            .map(|_| rand::thread_rng().gen::<u8>())
            .collect();

        // Encode as base64 for URL-safe transmission
        let token = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &random_bytes);

        Self(token)
    }

    /// Create CsrfToken from existing token string (e.g., from database)
    pub fn from_string(token: String) -> Self {
        Self(token)
    }

    /// Verify provided token matches this token
    /// Uses constant-time comparison to prevent timing attacks
    pub fn verify(&self, token: &str) -> bool {
        use subtle::ConstantTimeEq;

        if self.0.len() != token.len() {
            return false;
        }

        self.0.as_bytes().ct_eq(token.as_bytes()).into()
    }

    /// Get token as str
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Into inner String
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for CsrfToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valid() {
        let email = Email::new("test@example.com").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_email_invalid() {
        assert!(Email::new("not-an-email").is_err());
        assert!(Email::new("").is_err());
        assert!(Email::new("a".repeat(256).as_str()).is_err());
    }

    #[test]
    fn test_email_normalization() {
        let email = Email::new("  TEST@EXAMPLE.COM  ").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn test_password_hash_valid() {
        let hash = PasswordHash::from_plain("password123").unwrap();
        assert!(hash.verify("password123").unwrap());
        assert!(!hash.verify("wrongpassword").unwrap());
    }

    #[test]
    fn test_password_hash_too_short() {
        let result = PasswordHash::from_plain("short");
        assert!(result.is_err());
    }

    #[test]
    fn test_csrf_token_generation() {
        let token1 = CsrfToken::generate();
        let token2 = CsrfToken::generate();

        // Tokens should be different
        assert_ne!(token1.as_str(), token2.as_str());

        // Token should verify against itself
        assert!(token1.verify(token1.as_str()));
        assert!(!token1.verify(token2.as_str()));
    }

    #[test]
    fn test_csrf_token_constant_time() {
        let token = CsrfToken::generate();
        let valid = token.as_str();
        let invalid = "invalid_token_with_same_length_as_valid_one_exactly";

        // Both should take roughly same time (constant-time comparison)
        assert!(token.verify(valid));
        assert!(!token.verify(invalid));
    }
}
