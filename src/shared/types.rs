use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Type alias for User ID
pub type UserId = Uuid;

/// Type alias for Session ID
pub type SessionId = Uuid;

/// Type alias for Token ID
pub type TokenId = Uuid;

/// Type alias for timestamps
pub type Timestamp = DateTime<Utc>;

/// Generate a new UUID v7 (time-ordered)
///
/// UUID v7 provides better database indexing performance than v4
/// because it includes a timestamp component.
pub fn new_id() -> Uuid {
    Uuid::now_v7()
}

/// Get current UTC timestamp
pub fn now() -> Timestamp {
    Utc::now()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id_generates_uuid_v7() {
        let id = new_id();
        assert_eq!(id.get_version_num(), 7);
    }

    #[test]
    fn test_new_id_is_unique() {
        let id1 = new_id();
        let id2 = new_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_now_returns_utc() {
        let timestamp = now();
        assert_eq!(timestamp.timezone(), Utc);
    }
}
