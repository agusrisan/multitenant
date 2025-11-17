# Phase 2: Backend Foundation & Infrastructure

**Status**: 🔴 Not Started
**Priority**: 🔥 CRITICAL
**Estimated Time**: 6-8 hours
**Dependencies**: Phase 1 (Project Setup) must be completed

## Overview
Core backend infrastructure following Clean Architecture principles: bootstrap layer, application state, database connectivity, migrations, shared utilities, and base error handling.

---

## 1. Bootstrap Layer (Application Initialization)

### 1.1 Bootstrap Module Structure
- [ ] **Create bootstrap mod.rs** 📁 `src/bootstrap/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **DDD Layer**: Infrastructure (Bootstrap)
  - **Purpose**: Export bootstrap components
  - **Content**:
    ```rust
    pub mod app_state;
    pub mod database;
    pub mod telemetry;

    pub use app_state::AppState;
    pub use database::init_database;
    pub use telemetry::init_telemetry;
    ```

### 1.2 Database Initialization
- [ ] **Create database.rs** 📁 `src/bootstrap/database.rs`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: Requires .env, sqlx dependency
  - **DDD Layer**: Infrastructure
  - **Tech**: SQLx 0.8.6 with PgPool
  - **Key Functions**:
    - `init_database() -> Result<PgPool>`
    - Connection pooling configuration
    - Migration runner
    - Health check function
  - **Features**:
    - Max connections: 5-10 for dev
    - Connection timeout handling
    - Auto-run migrations on startup (dev only)
    - PostgreSQL 18 optimized settings

### 1.3 Application State
- [ ] **Create app_state.rs** 📁 `src/bootstrap/app_state.rs`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: Requires database.rs, config.rs
  - **DDD Layer**: Infrastructure
  - **Purpose**: Shared application state (Axum State pattern)
  - **Structure**:
    ```rust
    #[derive(Clone)]
    pub struct AppState {
        pub db: PgPool,
        pub config: Config,
        pub jwt_secret: String,
        pub session_secret: String,
        pub csrf_secret: String,
    }
    ```
  - **Notes**: Implements Clone for Axum layer sharing

### 1.4 Telemetry & Logging
- [ ] **Create telemetry.rs** 📁 `src/bootstrap/telemetry.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (45 min)
  - **Dependencies**: tracing, tracing-subscriber
  - **DDD Layer**: Infrastructure
  - **Features**:
    - Console logging with colors
    - JSON logging for production
    - Request ID tracking
    - Performance timing
    - Log levels from RUST_LOG env
  - **Filters**: Filter out health check spam

---

## 2. Configuration Management

### 2.1 Config Module
- [ ] **Create config.rs** 📁 `src/config.rs`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: dotenvy, serde
  - **DDD Layer**: Infrastructure
  - **Structure**:
    ```rust
    #[derive(Debug, Clone)]
    pub struct Config {
        pub database: DatabaseConfig,
        pub server: ServerConfig,
        pub jwt: JwtConfig,
        pub session: SessionConfig,
        pub csrf: CsrfConfig,
    }

    impl Config {
        pub fn from_env() -> Result<Self, ConfigError>
    }
    ```
  - **Validation**: Validate on load (missing vars = panic early)
  - **Notes**: Use dotenvy for .env loading

---

## 3. Shared Utilities (Cross-Module)

### 3.1 Shared Module Structure
- [ ] **Create shared mod.rs** 📁 `src/shared/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Shared Kernel
  - **Content**:
    ```rust
    pub mod error;
    pub mod result;
    pub mod types;

    pub use error::{AppError, ErrorContext};
    pub use result::AppResult;
    ```

### 3.2 Error Handling
- [ ] **Create error.rs** 📁 `src/shared/error.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: thiserror, axum
  - **DDD Layer**: Shared Kernel
  - **Error Types**:
    ```rust
    #[derive(Debug, thiserror::Error)]
    pub enum AppError {
        #[error("Database error: {0}")]
        Database(#[from] sqlx::Error),

        #[error("Validation error: {0}")]
        Validation(String),

        #[error("Authentication error: {0}")]
        Authentication(String),

        #[error("Authorization error: {0}")]
        Authorization(String),

        #[error("Not found: {0}")]
        NotFound(String),

        #[error("Conflict: {0}")]
        Conflict(String),

        #[error("Internal error: {0}")]
        Internal(String),
    }
    ```
  - **Axum Integration**: Implement `IntoResponse` for AppError
  - **HTTP Status Mapping**:
    - Validation → 400
    - Authentication → 401
    - Authorization → 403
    - NotFound → 404
    - Conflict → 409
    - Database/Internal → 500
  - **Response Format**: JSON with error details

### 3.3 Result Type Alias
- [ ] **Create result.rs** 📁 `src/shared/result.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: Requires error.rs
  - **DDD Layer**: Shared Kernel
  - **Content**:
    ```rust
    use super::error::AppError;

    pub type AppResult<T> = Result<T, AppError>;
    ```
  - **Purpose**: Convenience type alias for consistent error handling

### 3.4 Common Types
- [ ] **Create types.rs** 📁 `src/shared/types.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: uuid, chrono, serde
  - **DDD Layer**: Shared Kernel
  - **Common Types**:
    ```rust
    pub type UserId = uuid::Uuid;
    pub type SessionId = uuid::Uuid;
    pub type TokenId = uuid::Uuid;
    pub type Timestamp = chrono::DateTime<chrono::Utc>;

    // Helper for UUID v7 generation
    pub fn new_id() -> uuid::Uuid {
        uuid::Uuid::now_v7()
    }
    ```
  - **Notes**: UUID v7 provides time-ordering

---

## 4. Database Migrations

### 4.1 Users Table Migration
- [ ] **Create users migration** 📁 `migrations/20250117000001_create_users.sql`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: None
  - **DDD Layer**: Infrastructure (Persistence)
  - **Schema**:
    ```sql
    CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

    CREATE TABLE users (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
        email VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        email_verified BOOLEAN NOT NULL DEFAULT FALSE,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

    CREATE INDEX idx_users_email ON users(email);
    CREATE INDEX idx_users_created_at ON users(created_at);
    ```
  - **Tech**: PostgreSQL 18 with UUID v7 support
  - **Indexes**: Email lookup, time-based queries

### 4.2 Sessions Table Migration
- [ ] **Create sessions migration** 📁 `migrations/20250117000002_create_sessions.sql`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: Requires users table
  - **DDD Layer**: Infrastructure (Persistence)
  - **Schema**:
    ```sql
    CREATE TABLE sessions (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        csrf_token VARCHAR(255) NOT NULL,
        ip_address INET,
        user_agent TEXT,
        expires_at TIMESTAMPTZ NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

    CREATE INDEX idx_sessions_user_id ON sessions(user_id);
    CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
    CREATE INDEX idx_sessions_csrf_token ON sessions(csrf_token);
    ```
  - **Features**: CSRF protection, IP/UA tracking, expiration
  - **Cleanup**: Index on expires_at for efficient purging

### 4.3 JWT Tokens Table Migration
- [ ] **Create jwt_tokens migration** 📁 `migrations/20250117000003_create_jwt_tokens.sql`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: Requires users table
  - **DDD Layer**: Infrastructure (Persistence)
  - **Schema**:
    ```sql
    CREATE TYPE token_type AS ENUM ('access', 'refresh');

    CREATE TABLE jwt_tokens (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v7(),
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        token_type token_type NOT NULL,
        jti UUID NOT NULL UNIQUE,  -- JWT ID for revocation
        expires_at TIMESTAMPTZ NOT NULL,
        revoked BOOLEAN NOT NULL DEFAULT FALSE,
        revoked_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    );

    CREATE INDEX idx_jwt_tokens_user_id ON jwt_tokens(user_id);
    CREATE INDEX idx_jwt_tokens_jti ON jwt_tokens(jti);
    CREATE INDEX idx_jwt_tokens_expires_at ON jwt_tokens(expires_at);
    CREATE INDEX idx_jwt_tokens_revoked ON jwt_tokens(revoked) WHERE NOT revoked;
    ```
  - **Features**: Token revocation, refresh token rotation
  - **Indexes**: Partial index on non-revoked tokens

---

## 5. Main Application Setup

### 5.1 Startup Module
- [ ] **Create startup.rs** 📁 `src/startup.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: Requires bootstrap, config
  - **DDD Layer**: Infrastructure (Application Entry)
  - **Functions**:
    ```rust
    pub async fn build_app(config: Config, db: PgPool) -> Router {
        // Initialize AppState
        // Setup middleware (tracing, cors, static files)
        // Mount module routes
        // Inertia middleware
        // Return Router
    }
    ```
  - **Middleware Stack**:
    1. Tracing layer (request logging)
    2. CORS (for API)
    3. Static file serving (resources/dist)
    4. Inertia middleware (axum_inertia)
  - **Route Structure**:
    - `/` - Root (will redirect to /web/dashboard or /web/login)
    - `/web/*` - Inertia routes (returns HTML with React)
    - `/api/*` - JSON API routes
    - `/health` - Health check endpoint

### 5.2 Main Entry Point
- [ ] **Create main.rs** 📁 `src/main.rs`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: Requires startup.rs, bootstrap
  - **DDD Layer**: Infrastructure (Application Entry)
  - **Responsibilities**:
    ```rust
    #[tokio::main]
    async fn main() -> anyhow::Result<()> {
        // 1. Load .env
        dotenvy::dotenv().ok();

        // 2. Initialize telemetry
        init_telemetry()?;

        // 3. Load config
        let config = Config::from_env()?;

        // 4. Initialize database
        let db = init_database(&config.database).await?;

        // 5. Run migrations
        sqlx::migrate!("./migrations").run(&db).await?;

        // 6. Build app
        let app = build_app(config.clone(), db).await;

        // 7. Start server
        let listener = TcpListener::bind(
            format!("{}:{}", config.server.host, config.server.port)
        ).await?;

        tracing::info!("Server running on {}", listener.local_addr()?);

        axum::serve(listener, app).await?;

        Ok(())
    }
    ```
  - **Error Handling**: Graceful panic with context
  - **Logging**: Log startup steps

### 5.3 Module Declaration
- [ ] **Update main.rs with modules** 📁 `src/main.rs`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Content**:
    ```rust
    mod bootstrap;
    mod config;
    mod startup;
    mod shared;
    mod moduls;  // Will be populated in Phase 3+
    ```

---

## 6. Health Check Endpoint

### 6.1 Health Check Handler
- [ ] **Add health check to startup.rs** 📁 `src/startup.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: None
  - **DDD Layer**: Infrastructure
  - **Endpoint**: `GET /health`
  - **Response**:
    ```json
    {
      "status": "healthy",
      "database": "connected",
      "timestamp": "2025-01-17T10:30:00Z"
    }
    ```
  - **Checks**: Database connectivity, timestamp

---

## Validation Checklist

- [ ] `cargo build` compiles successfully
- [ ] `cargo sqlx prepare` generates sqlx-data.json
- [ ] Migrations run without errors
- [ ] Server starts on http://localhost:3000
- [ ] Health check endpoint returns 200 OK
- [ ] Database connection pool initializes
- [ ] Telemetry logs appear in console
- [ ] Error types convert to HTTP responses correctly

---

## Architecture Notes

### Clean Architecture Layers Applied:
1. **Infrastructure Layer** (Bootstrap):
   - Database initialization
   - Telemetry setup
   - Application state management

2. **Shared Kernel**:
   - Common types (UserId, Timestamp)
   - Error handling
   - Result type aliases

3. **Interface Adapters** (Startup):
   - HTTP routing
   - Middleware configuration
   - Request/response handling

### Dependency Flow:
```
main.rs → bootstrap → database ✓
main.rs → config ✓
main.rs → startup → router ✓
shared (errors) → used by all layers ✓
```

---

## Next Phase
➡️ **Phase 3: Auth Module** - Implement authentication with DDD layers (domain → application → infrastructure → web/api)

## Technical Notes
- SQLx compile-time checking requires database to be running during `cargo build`
- Use `cargo sqlx prepare` for offline builds
- UUID v7 in PostgreSQL 18 provides better indexing performance than v4
- Session cleanup job should be added later (Phase 7)
