use crate::shared::{types::*, AppError, AppResult};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// Token pair response for API authentication
/// Contains access token (short-lived) and refresh token (long-lived)
#[derive(Debug, Clone, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,  // Always "Bearer"
    pub expires_in: i64,     // Access token expiry in seconds
}

/// JWT Token entity stored in database for revocation tracking
/// Represents a persisted JWT token (access or refresh)
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct JwtToken {
    pub id: TokenId,
    pub user_id: UserId,
    pub token_type: TokenType,
    pub jti: uuid::Uuid,  // JWT ID for revocation
    pub expires_at: Timestamp,
    pub revoked: bool,
    pub revoked_at: Option<Timestamp>,
    pub created_at: Timestamp,
}

/// Token type enum for database storage
#[derive(Debug, Clone, Copy, sqlx::Type, Serialize, Deserialize, PartialEq, Eq)]
#[sqlx(type_name = "token_type", rename_all = "lowercase")]
pub enum TokenType {
    Access,
    Refresh,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Access => write!(f, "access"),
            TokenType::Refresh => write!(f, "refresh"),
        }
    }
}

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user_id)
    pub jti: String,        // JWT ID (for revocation)
    pub exp: i64,           // Expiration time (unix timestamp)
    pub iat: i64,           // Issued at (unix timestamp)
    pub token_type: String, // "access" or "refresh"
}

impl TokenPair {
    /// Generate new token pair for user
    ///
    /// Business Rules:
    /// - Access token: Short-lived (15 min default)
    /// - Refresh token: Long-lived (7 days default)
    /// - JTI (JWT ID) for revocation tracking
    /// - Tokens can be revoked via blacklist
    ///
    /// # Arguments
    /// * `user_id` - User ID to encode in token
    /// * `jwt_secret` - Secret key for signing tokens
    /// * `access_ttl` - Access token TTL in seconds
    /// * `refresh_ttl` - Refresh token TTL in seconds
    ///
    /// # Returns
    /// Tuple of (TokenPair, AccessJwtToken, RefreshJwtToken) for persistence
    pub fn generate(
        user_id: UserId,
        jwt_secret: &str,
        access_ttl: i64,
        refresh_ttl: i64,
    ) -> AppResult<(Self, JwtToken, JwtToken)> {
        let now = now();
        let iat = now.timestamp();

        // Generate access token
        let access_jti = new_id();
        let access_exp = iat + access_ttl;
        let access_claims = Claims {
            sub: user_id.to_string(),
            jti: access_jti.to_string(),
            exp: access_exp,
            iat,
            token_type: "access".to_string(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::internal(format!("Failed to encode access token: {}", e)))?;

        // Generate refresh token
        let refresh_jti = new_id();
        let refresh_exp = iat + refresh_ttl;
        let refresh_claims = Claims {
            sub: user_id.to_string(),
            jti: refresh_jti.to_string(),
            exp: refresh_exp,
            iat,
            token_type: "refresh".to_string(),
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::internal(format!("Failed to encode refresh token: {}", e)))?;

        // Create token pair response
        let token_pair = TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: access_ttl,
        };

        // Create JwtToken entities for persistence
        let access_jwt_token = JwtToken {
            id: new_id(),
            user_id,
            token_type: TokenType::Access,
            jti: access_jti,
            expires_at: chrono::DateTime::from_timestamp(access_exp, 0)
                .ok_or_else(|| AppError::internal("Invalid access token expiration"))?,
            revoked: false,
            revoked_at: None,
            created_at: now,
        };

        let refresh_jwt_token = JwtToken {
            id: new_id(),
            user_id,
            token_type: TokenType::Refresh,
            jti: refresh_jti,
            expires_at: chrono::DateTime::from_timestamp(refresh_exp, 0)
                .ok_or_else(|| AppError::internal("Invalid refresh token expiration"))?,
            revoked: false,
            revoked_at: None,
            created_at: now,
        };

        Ok((token_pair, access_jwt_token, refresh_jwt_token))
    }

    /// Decode and validate JWT token
    ///
    /// Validates signature, expiration, and token structure
    /// Does NOT check revocation - caller must check against database
    ///
    /// # Arguments
    /// * `token` - JWT token string to decode
    /// * `jwt_secret` - Secret key for validation
    ///
    /// # Returns
    /// Decoded Claims if valid
    pub fn decode(token: &str, jwt_secret: &str) -> AppResult<Claims> {
        let validation = Validation::default();

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::authentication("Token has expired")
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                AppError::authentication("Invalid token")
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                AppError::authentication("Invalid token signature")
            }
            _ => AppError::authentication(format!("Token validation failed: {}", e)),
        })?;

        Ok(token_data.claims)
    }

    /// Extract JTI from token without full validation
    ///
    /// Used for quick JTI lookup before full validation
    /// Still validates signature and basic structure
    pub fn extract_jti(token: &str, jwt_secret: &str) -> AppResult<uuid::Uuid> {
        let claims = Self::decode(token, jwt_secret)?;

        uuid::Uuid::parse_str(&claims.jti)
            .map_err(|e| AppError::internal(format!("Invalid JTI in token: {}", e)))
    }

    /// Extract user ID from token
    pub fn extract_user_id(token: &str, jwt_secret: &str) -> AppResult<UserId> {
        let claims = Self::decode(token, jwt_secret)?;

        uuid::Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::internal(format!("Invalid user ID in token: {}", e)))
    }
}

impl JwtToken {
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        now() > self.expires_at
    }

    /// Check if token is revoked
    pub fn is_revoked(&self) -> bool {
        self.revoked
    }

    /// Check if token is valid (not expired and not revoked)
    pub fn is_valid(&self) -> bool {
        !self.is_expired() && !self.is_revoked()
    }

    /// Revoke token
    pub fn revoke(&mut self) {
        self.revoked = true;
        self.revoked_at = Some(now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &str = "test_secret_key_for_jwt_signing_minimum_32_chars";

    #[test]
    fn test_generate_token_pair() {
        let user_id = new_id();
        let access_ttl = 900; // 15 min
        let refresh_ttl = 604800; // 7 days

        let result = TokenPair::generate(user_id, TEST_SECRET, access_ttl, refresh_ttl);
        assert!(result.is_ok());

        let (token_pair, access_token, refresh_token) = result.unwrap();

        assert_eq!(token_pair.token_type, "Bearer");
        assert_eq!(token_pair.expires_in, access_ttl);
        assert_eq!(access_token.token_type, TokenType::Access);
        assert_eq!(refresh_token.token_type, TokenType::Refresh);
        assert_eq!(access_token.user_id, user_id);
        assert_eq!(refresh_token.user_id, user_id);
    }

    #[test]
    fn test_decode_valid_token() {
        let user_id = new_id();
        let (token_pair, _, _) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        let claims = TokenPair::decode(&token_pair.access_token, TEST_SECRET);
        assert!(claims.is_ok());

        let claims = claims.unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_decode_invalid_signature() {
        let user_id = new_id();
        let (token_pair, _, _) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        let result = TokenPair::decode(&token_pair.access_token, "wrong_secret");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_jti() {
        let user_id = new_id();
        let (token_pair, access_token, _) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        let jti = TokenPair::extract_jti(&token_pair.access_token, TEST_SECRET).unwrap();
        assert_eq!(jti, access_token.jti);
    }

    #[test]
    fn test_extract_user_id() {
        let user_id = new_id();
        let (token_pair, _, _) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        let extracted_user_id = TokenPair::extract_user_id(&token_pair.access_token, TEST_SECRET).unwrap();
        assert_eq!(extracted_user_id, user_id);
    }

    #[test]
    fn test_jwt_token_expiration() {
        let user_id = new_id();
        let (_, mut access_token, _) = TokenPair::generate(user_id, TEST_SECRET, -1, 604800).unwrap();

        // Token should be expired (TTL = -1 second)
        assert!(access_token.is_expired());
        assert!(!access_token.is_valid());
    }

    #[test]
    fn test_jwt_token_revocation() {
        let user_id = new_id();
        let (_, mut access_token, _) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        assert!(!access_token.is_revoked());
        assert!(access_token.is_valid());

        access_token.revoke();

        assert!(access_token.is_revoked());
        assert!(!access_token.is_valid());
        assert!(access_token.revoked_at.is_some());
    }

    #[test]
    fn test_token_types() {
        let user_id = new_id();
        let (_, access_token, refresh_token) = TokenPair::generate(user_id, TEST_SECRET, 900, 604800).unwrap();

        assert_eq!(access_token.token_type, TokenType::Access);
        assert_eq!(refresh_token.token_type, TokenType::Refresh);
        assert_ne!(access_token.jti, refresh_token.jti);
    }
}
