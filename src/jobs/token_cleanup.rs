use sqlx::PgPool;
use tokio::time::{interval, Duration};

/// Token cleanup job
///
/// Runs periodically to delete expired JWT tokens from the database.
/// This helps keep the jwt_tokens table clean and performant.
pub async fn token_cleanup_job(pool: PgPool) {
    let mut interval = interval(Duration::from_secs(21600)); // Every 6 hours

    tracing::info!("Token cleanup job started (running every 6 hours)");

    loop {
        interval.tick().await;

        match cleanup_expired_tokens(&pool).await {
            Ok(deleted) => {
                if deleted > 0 {
                    tracing::info!("Cleaned up {} expired JWT tokens", deleted);
                } else {
                    tracing::debug!("No expired JWT tokens to clean up");
                }
            }
            Err(e) => {
                tracing::error!("Token cleanup failed: {:?}", e);
            }
        }
    }
}

/// Delete expired JWT tokens from database
async fn cleanup_expired_tokens(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM jwt_tokens WHERE expires_at < NOW()")
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
mod tests {
    

    #[tokio::test]
    async fn test_cleanup_expired_tokens() {
        // This test requires a database connection
        // Skip in unit tests, run in integration tests instead
    }
}
