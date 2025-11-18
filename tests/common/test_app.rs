use multitenant::bootstrap::{database::DatabaseConfig, AppState};
use multitenant::config::{Config, CsrfConfig, JwtConfig, ServerConfig, SessionConfig};
use multitenant::startup::build_app;
use sqlx::PgPool;

/// Test application instance for integration testing
pub struct TestApp {
    pub address: String,
    pub db: PgPool,
    pub client: reqwest::Client,
}

impl TestApp {
    /// Spawn a new test application instance
    pub async fn spawn() -> Self {
        // Load test environment variables from .env.test
        dotenvy::from_filename(".env.test").ok();

        // Use test database URL
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/multitenant_test".to_string());

        // Create database pool
        let db = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool");

        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&db)
            .await
            .expect("Failed to run migrations");

        // Clean database before each test to ensure isolation
        sqlx::query!("TRUNCATE TABLE jwt_tokens, sessions, users RESTART IDENTITY CASCADE")
            .execute(&db)
            .await
            .expect("Failed to clean database before test");

        // Create test configuration
        let config = Config {
            database: DatabaseConfig {
                url: database_url,
                max_connections: 5,
                connect_timeout: 3,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0, // Random port
            },
            jwt: JwtConfig {
                secret: "test_jwt_secret_key_minimum_32_characters_long".to_string(),
                access_expiry: 900,
                refresh_expiry: 604800,
            },
            session: SessionConfig {
                secret: "test_session_secret_key_minimum_32_characters_long".to_string(),
                expiry: 86400,
            },
            csrf: CsrfConfig {
                secret: "test_csrf_secret_key_minimum_32_characters_long".to_string(),
            },
        };

        // Create app state
        let state = AppState::new(
            db.clone(),
            config.clone(),
            config.jwt.secret.clone(),
            config.session.secret.clone(),
            config.csrf.secret.clone(),
        );

        // Build app
        let app = build_app(state).await;

        // Spawn server on random port
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind to random port");

        let address = listener
            .local_addr()
            .expect("Failed to get local address");

        // Spawn server in background
        tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("Failed to serve app");
        });

        let address = format!("http://{}", address);

        // Create HTTP client with cookie store
        let client = reqwest::Client::builder()
            .cookie_provider(std::sync::Arc::new(reqwest::cookie::Jar::default()))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            address,
            db,
            client,
        }
    }

    /// Make a POST request with JSON body
    pub async fn post_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> reqwest::Response {
        self.client
            .post(&format!("{}{}", self.address, path))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Make a GET request
    pub async fn get(&self, path: &str) -> reqwest::Response {
        self.client
            .get(&format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Make a PUT request with JSON body
    #[allow(dead_code)]
    pub async fn put_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> reqwest::Response {
        self.client
            .put(&format!("{}{}", self.address, path))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Make a DELETE request
    #[allow(dead_code)]
    pub async fn delete(&self, path: &str) -> reqwest::Response {
        self.client
            .delete(&format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to execute request")
    }

    /// Clean up the database after tests
    pub async fn cleanup(&self) {
        // Delete all test data
        sqlx::query!("TRUNCATE TABLE jwt_tokens, sessions, users RESTART IDENTITY CASCADE")
            .execute(&self.db)
            .await
            .expect("Failed to clean up database");
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        // Cleanup happens here if needed
        tracing::debug!("TestApp dropped");
    }
}
