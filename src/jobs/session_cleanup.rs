use sqlx::PgPool;
use tokio::time::{interval, Duration};

/// Session cleanup job
///
/// Runs periodically to delete expired sessions from the database.
/// This helps keep the sessions table clean and performant.
pub async fn session_cleanup_job(pool: PgPool) {
    let mut interval = interval(Duration::from_secs(3600)); // Every hour

    tracing::info!("Session cleanup job started (running every 1 hour)");

    loop {
        interval.tick().await;

        match cleanup_expired_sessions(&pool).await {
            Ok(deleted) => {
                if deleted > 0 {
                    tracing::info!("Cleaned up {} expired sessions", deleted);
                } else {
                    tracing::debug!("No expired sessions to clean up");
                }
            }
            Err(e) => {
                tracing::error!("Session cleanup failed: {:?}", e);
            }
        }
    }
}

/// Delete expired sessions from database
async fn cleanup_expired_sessions(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM sessions WHERE expires_at < NOW()")
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cleanup_expired_sessions() {
        // This test requires a database connection
        // Skip in unit tests, run in integration tests instead
    }
}
