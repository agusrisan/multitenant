use crate::shared::types::*;
use super::value_objects::CsrfToken;

/// Session entity for web authentication
/// Represents a user's active session with CSRF protection
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Session {
    pub id: SessionId,
    pub user_id: UserId,
    pub csrf_token: CsrfToken,
    pub ip_address: Option<String>,  // Store IP as string for SQLx compatibility
    pub user_agent: Option<String>,
    pub expires_at: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Session {
    /// Create new Session for user
    ///
    /// Business Rules:
    /// - Session expires after TTL (configurable, typically 24 hours)
    /// - CSRF token generated on creation
    /// - One session per user for web (enforced in repository)
    ///
    /// # Arguments
    /// * `user_id` - ID of the user this session belongs to
    /// * `ip_address` - Optional IP address of the client as string
    /// * `user_agent` - Optional user agent string
    /// * `ttl_seconds` - Time to live in seconds
    pub fn new(
        user_id: UserId,
        ip_address: Option<String>,
        user_agent: Option<String>,
        ttl_seconds: i64,
    ) -> Self {
        let now = now();
        let expires_at = now + chrono::Duration::seconds(ttl_seconds);

        Self {
            id: new_id(),
            user_id,
            csrf_token: CsrfToken::generate(),
            ip_address,
            user_agent,
            expires_at,
            created_at: now,
            updated_at: now,
        }
    }

    /// Check if session is expired
    ///
    /// Returns true if current time is past expiration time
    pub fn is_expired(&self) -> bool {
        now() > self.expires_at
    }

    /// Refresh session expiration time
    ///
    /// Extends session lifetime by adding TTL to current time
    /// Also updates the updated_at timestamp
    pub fn refresh(&mut self, ttl_seconds: i64) {
        let now = now();
        self.expires_at = now + chrono::Duration::seconds(ttl_seconds);
        self.updated_at = now;
    }

    /// Verify CSRF token
    ///
    /// Uses constant-time comparison to prevent timing attacks
    ///
    /// # Arguments
    /// * `token` - Token to verify against session's CSRF token
    ///
    /// # Returns
    /// * `true` if tokens match, `false` otherwise
    pub fn verify_csrf(&self, token: &str) -> bool {
        self.csrf_token.verify(token)
    }

    /// Check if session is valid
    ///
    /// A session is valid if it's not expired
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

/// DTO for session cookie value
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SessionCookie {
    pub session_id: SessionId,
    pub csrf_token: String,
}

impl SessionCookie {
    pub fn new(session_id: SessionId, csrf_token: String) -> Self {
        Self {
            session_id,
            csrf_token,
        }
    }

    pub fn from_session(session: &Session) -> Self {
        Self {
            session_id: session.id,
            csrf_token: session.csrf_token.as_str().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let user_id = new_id();
        let ip = Some("127.0.0.1".to_string());
        let user_agent = Some("Mozilla/5.0".to_string());
        let ttl = 3600; // 1 hour

        let session = Session::new(user_id, ip.clone(), user_agent.clone(), ttl);

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.ip_address, ip);
        assert_eq!(session.user_agent, user_agent);
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_expiration() {
        let user_id = new_id();
        let ttl = -1; // Expired 1 second ago

        let session = Session::new(user_id, None, None, ttl);

        assert!(session.is_expired());
        assert!(!session.is_valid());
    }

    #[test]
    fn test_session_refresh() {
        let user_id = new_id();
        let ttl = -1; // Start expired

        let mut session = Session::new(user_id, None, None, ttl);
        assert!(session.is_expired());

        // Refresh with 1 hour
        session.refresh(3600);

        assert!(!session.is_expired());
        assert!(session.is_valid());
    }

    #[test]
    fn test_csrf_verification() {
        let user_id = new_id();
        let session = Session::new(user_id, None, None, 3600);

        let valid_token = session.csrf_token.as_str();
        let invalid_token = "invalid_token";

        assert!(session.verify_csrf(valid_token));
        assert!(!session.verify_csrf(invalid_token));
    }

    #[test]
    fn test_session_cookie() {
        let user_id = new_id();
        let session = Session::new(user_id, None, None, 3600);

        let cookie = SessionCookie::from_session(&session);

        assert_eq!(cookie.session_id, session.id);
        assert_eq!(cookie.csrf_token, session.csrf_token.as_str());
    }
}
