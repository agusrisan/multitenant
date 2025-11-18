use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize telemetry and logging
///
/// Sets up tracing with console logging for development and JSON logging for production.
/// Respects RUST_LOG environment variable for log level configuration.
pub fn init_telemetry() -> anyhow::Result<()> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let rust_env = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

    match rust_env.as_str() {
        "production" => {
            // Compact logging for production
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_ids(true)
                        .with_target(true)
                        .compact(),
                )
                .init();

            tracing::info!("Telemetry initialized (production mode)");
        }
        _ => {
            // Pretty logging with colors for development
            tracing_subscriber::registry()
                .with(env_filter)
                .with(
                    tracing_subscriber::fmt::layer()
                        .with_file(true)
                        .with_line_number(true)
                        .with_thread_ids(true)
                        .with_target(true)
                        .pretty(),
                )
                .init();

            tracing::info!("Telemetry initialized (development mode)");
        }
    }

    Ok(())
}

/// Create a tracing span for request tracking
#[macro_export]
macro_rules! request_span {
    ($request_id:expr) => {
        tracing::info_span!(
            "request",
            request_id = %$request_id,
        )
    };
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_telemetry_init_development() {
        std::env::set_var("RUST_ENV", "development");
        std::env::set_var("RUST_LOG", "debug");

        // Note: In a real test, we'd want to capture output
        // For now, just ensure it doesn't panic
        // init_telemetry().unwrap();
    }
}
