mod bootstrap;
mod config;
mod shared;
mod startup;

use bootstrap::{app_state::AppState, database::init_database, telemetry::init_telemetry};
use config::Config;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 2. Initialize telemetry (logging and tracing)
    init_telemetry()?;
    tracing::info!("Starting Multitenant Auth Application...");

    // 3. Load configuration from environment
    tracing::info!("Loading configuration...");
    let config = Config::from_env()
        .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;
    tracing::info!("Configuration loaded successfully");

    // 4. Initialize database connection pool
    tracing::info!("Initializing database connection...");
    let db = init_database(&config.database)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize database: {}", e))?;
    tracing::info!("Database connection established");

    // 5. Run database migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run migrations: {}", e))?;
    tracing::info!("Database migrations completed");

    // 6. Create application state
    let state = AppState::new(
        db,
        config.clone(),
        config.jwt.secret.clone(),
        config.session.secret.clone(),
        config.csrf.secret.clone(),
    );

    // 7. Build Axum application with all routes and middleware
    tracing::info!("Building application...");
    let app = startup::build_app(state).await;

    // 8. Parse server address
    let addr = SocketAddr::from((
        config
            .server
            .host
            .parse::<std::net::IpAddr>()
            .map_err(|e| anyhow::anyhow!("Invalid HOST: {}", e))?,
        config.server.port,
    ));

    // 9. Start the server
    tracing::info!("🚀 Server listening on http://{}", addr);
    tracing::info!("📊 Health check available at http://{}/health", addr);
    tracing::info!("✅ Application started successfully!");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to {}: {}", addr, e))?;

    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}
