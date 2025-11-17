# Phase 7: Integration, Testing & Deployment

**Status**: ✅ COMPLETE
**Priority**: 🟡 MEDIUM
**Estimated Time**: 8-12 hours
**Actual Time**: ~3 hours
**Dependencies**: All previous phases (1-6) completed ✅
**Completed**: 2025-11-17

## Overview
Final integration, testing, optimization, background jobs, security hardening, and deployment preparation. Makes the application production-ready.

---

## 1. Integration Testing

### 1.1 Test Infrastructure Setup
- [ ] **Add test dependencies to Cargo.toml** 📁 `Cargo.toml`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Dev Dependencies**:
    ```toml
    [dev-dependencies]
    tokio-test = "0.4"
    mockall = "0.12"
    fake = "2.9"
    wiremock = "0.6"
    ```
  - **Purpose**: Mocking, fake data generation, HTTP mocking

### 1.2 Test Database Setup
- [ ] **Create test database script** 📁 `scripts/test_db.sh`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: PostgreSQL
  - **Content**:
    ```bash
    #!/bin/bash
    # Create test database
    psql -U postgres -c "DROP DATABASE IF EXISTS multitenant_test;"
    psql -U postgres -c "CREATE DATABASE multitenant_test;"

    # Run migrations
    DATABASE_URL=postgres://postgres:password@localhost:5432/multitenant_test \
      sqlx migrate run
    ```
  - **Purpose**: Isolated test database

### 1.3 Integration Test Suite
- [ ] **Create integration tests directory** 📁 `tests/`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **Structure**:
    ```
    tests/
    ├── common/
    │   ├── mod.rs
    │   └── test_app.rs
    ├── auth_tests.rs
    └── user_tests.rs
    ```

- [ ] **Create test helpers** 📁 `tests/common/test_app.rs`
  - **Priority**: Medium
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: Test database
  - **Purpose**: Spawn test server, helpers
  - **Structure**:
    ```rust
    use sqlx::PgPool;
    use axum::Router;

    pub struct TestApp {
        pub address: String,
        pub db: PgPool,
    }

    impl TestApp {
        pub async fn spawn() -> Self {
            // Load test config
            // Create test database
            // Run migrations
            // Build app
            // Spawn server on random port
            // Return TestApp
        }

        pub async fn post_json<T: serde::Serialize>(
            &self,
            path: &str,
            body: &T,
        ) -> reqwest::Response {
            reqwest::Client::new()
                .post(&format!("{}{}", self.address, path))
                .json(body)
                .send()
                .await
                .expect("Failed to execute request")
        }
    }
    ```
  - **Features**: Test server spawning, HTTP client helpers

### 1.4 Auth Integration Tests
- [ ] **Create auth_tests.rs** 📁 `tests/auth_tests.rs`
  - **Priority**: Medium
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: test_app.rs
  - **Test Cases**:
    ```rust
    #[tokio::test]
    async fn test_register_success() {
        let app = TestApp::spawn().await;

        let response = app.post_json("/api/auth/register", &serde_json::json!({
            "name": "Test User",
            "email": "test@example.com",
            "password": "password123"
        })).await;

        assert_eq!(response.status(), 201);
    }

    #[tokio::test]
    async fn test_register_duplicate_email() {
        // Test conflict error
    }

    #[tokio::test]
    async fn test_login_success() {
        // Register user, then login
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        // Test authentication error
    }

    #[tokio::test]
    async fn test_refresh_token() {
        // Login, get tokens, refresh
    }

    #[tokio::test]
    async fn test_protected_route_without_auth() {
        // Test 401 response
    }
    ```
  - **Coverage**: Happy path + error cases

### 1.5 User Integration Tests
- [ ] **Create user_tests.rs** 📁 `tests/user_tests.rs`
  - **Priority**: Low
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: test_app.rs
  - **Test Cases**: Profile CRUD, password change

---

## 2. Unit Testing

### 2.1 Domain Unit Tests
- [ ] **Add tests to domain entities** 📁 `src/moduls/auth/domain/*.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (2 hours)
  - **Dependencies**: None
  - **Examples**:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_user_verify_password() {
            let user = User::new(
                Email::new("test@example.com").unwrap(),
                "password123",
                "Test User".into(),
            ).unwrap();

            assert!(user.verify_password("password123").unwrap());
            assert!(!user.verify_password("wrong").unwrap());
        }

        #[test]
        fn test_session_is_expired() {
            let mut session = Session::new(
                uuid::Uuid::new_v4(),
                None,
                None,
                -3600, // expired 1 hour ago
            );

            assert!(session.is_expired());
        }
    }
    ```
  - **Coverage**: Value objects, entity methods, business rules

### 2.2 Use Case Unit Tests
- [ ] **Add tests to use cases** 📁 `src/moduls/auth/application/*.rs`
  - **Priority**: Medium
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: mockall for repository mocking
  - **Example**:
    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
        use mockall::predicate::*;

        #[tokio::test]
        async fn test_register_user_success() {
            let mut mock_repo = MockUserRepository::new();
            mock_repo
                .expect_find_by_email()
                .returning(|_| Ok(None)); // Email not taken
            mock_repo
                .expect_save()
                .returning(|user| Ok(user.clone()));

            let use_case = RegisterUserUseCase::new(Arc::new(mock_repo));
            let result = use_case.execute(RegisterUserCommand {
                email: "test@example.com".into(),
                password: "password123".into(),
                name: "Test".into(),
            }).await;

            assert!(result.is_ok());
        }
    }
    ```
  - **Benefits**: Fast, isolated, mockable dependencies

---

## 3. Background Jobs

### 3.1 Session Cleanup Job
- [ ] **Create cleanup job** 📁 `src/jobs/session_cleanup.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: SessionRepository
  - **Purpose**: Delete expired sessions periodically
  - **Structure**:
    ```rust
    use tokio::time::{interval, Duration};

    pub async fn session_cleanup_job(
        session_repo: Arc<dyn SessionRepository>,
    ) {
        let mut interval = interval(Duration::from_secs(3600)); // Every hour

        loop {
            interval.tick().await;

            match session_repo.delete_expired().await {
                Ok(deleted) => {
                    tracing::info!("Cleaned up {} expired sessions", deleted);
                }
                Err(e) => {
                    tracing::error!("Session cleanup failed: {}", e);
                }
            }
        }
    }
    ```
  - **Scheduling**: Run in background tokio task

### 3.2 Token Cleanup Job
- [ ] **Create token cleanup job** 📁 `src/jobs/token_cleanup.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: TokenRepository
  - **Purpose**: Delete expired JWT tokens
  - **Similar to**: session_cleanup.rs
  - **Interval**: Every 6 hours

### 3.3 Jobs Module
- [ ] **Create jobs module** 📁 `src/jobs/mod.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Content**:
    ```rust
    pub mod session_cleanup;
    pub mod token_cleanup;

    pub use session_cleanup::session_cleanup_job;
    pub use token_cleanup::token_cleanup_job;
    ```

### 3.4 Start Background Jobs
- [ ] **Update main.rs to spawn jobs** 📁 `src/main.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: jobs module
  - **Addition**:
    ```rust
    use crate::jobs::{session_cleanup_job, token_cleanup_job};

    // After building app, before serving:
    let session_repo = Arc::clone(&state.session_repo);
    tokio::spawn(async move {
        session_cleanup_job(session_repo).await;
    });

    let token_repo = Arc::clone(&state.token_repo);
    tokio::spawn(async move {
        token_cleanup_job(token_repo).await;
    });
    ```

---

## 4. Security Hardening

### 4.1 Rate Limiting
- [ ] **Add rate limiting middleware** 📁 `src/middleware/rate_limit.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: tower-governor or custom implementation
  - **Purpose**: Prevent brute force attacks
  - **Crate**: `tower-governor = "0.4"`
  - **Apply to**: Login, register, refresh endpoints
  - **Limits**:
    - Login: 5 requests/minute per IP
    - Register: 3 requests/hour per IP
    - API: 100 requests/minute per user

### 4.2 Security Headers
- [ ] **Add security headers middleware** 📁 `src/middleware/security_headers.rs`
  - **Priority**: High
  - **Complexity**: Simple (30 min)
  - **Dependencies**: tower-http
  - **Headers**:
    ```rust
    use tower_http::set_header::SetResponseHeaderLayer;

    pub fn security_headers_layer() -> SetResponseHeaderLayer {
        SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        )
        .and(/* X-Frame-Options: DENY */)
        .and(/* X-XSS-Protection: 1; mode=block */)
        .and(/* Strict-Transport-Security */)
    }
    ```
  - **Protection**: XSS, clickjacking, MIME sniffing

### 4.3 CORS Configuration
- [ ] **Configure CORS properly** 📁 `src/startup.rs`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: tower-http CorsLayer
  - **Config**:
    ```rust
    use tower_http::cors::{CorsLayer, Any};

    let cors = CorsLayer::new()
        .allow_origin(config.allowed_origins.parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);
    ```
  - **Production**: Restrict origins (not Any)

### 4.4 Input Sanitization
- [ ] **Add HTML sanitization** 📁 `src/shared/sanitize.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (45 min)
  - **Dependencies**: ammonia crate
  - **Purpose**: Sanitize user input (bio, name)
  - **Function**:
    ```rust
    use ammonia::clean;

    pub fn sanitize_html(input: &str) -> String {
        clean(input)
    }
    ```
  - **Apply to**: User bio, any user-generated content

---

## 5. Performance Optimization

### 5.1 Database Indexing Review
- [ ] **Review and optimize indexes** 📁 `migrations/`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: None
  - **Checks**:
    - users.email (unique index) ✓
    - sessions.user_id ✓
    - sessions.expires_at ✓
    - jwt_tokens.jti (unique) ✓
    - jwt_tokens.user_id ✓
  - **Add if missing**: Composite indexes for common queries

### 5.2 Connection Pooling Tuning
- [ ] **Optimize database pool settings** 📁 `src/bootstrap/database.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: None
  - **Settings**:
    ```rust
    PgPoolOptions::new()
        .max_connections(20)  // Adjust based on load
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
    ```
  - **Testing**: Load testing to find optimal values

### 5.3 Query Optimization
- [ ] **Review N+1 queries** 📁 `src/moduls/*/infra/*.rs`
  - **Priority**: Low
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: None
  - **Checks**: Use EXPLAIN ANALYZE on slow queries
  - **Fix**: Add JOINs, batch queries where possible

### 5.4 Static Asset Optimization
- [ ] **Configure asset caching** 📁 `src/startup.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: tower-http ServeDir
  - **Config**:
    ```rust
    use tower_http::services::ServeDir;

    let serve_dir = ServeDir::new("resources/dist")
        .append_index_html_on_directories(false);
    ```
  - **Production**: Add cache headers (max-age=31536000 for hashed assets)

---

## 6. Logging & Monitoring

### 6.1 Structured Logging
- [ ] **Improve logging with context** 📁 `src/bootstrap/telemetry.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: tracing, tracing-subscriber
  - **Enhancements**:
    - Request ID tracking
    - User ID in logs (when authenticated)
    - Error stack traces
    - Performance timings
  - **Format**: JSON for production, pretty for dev

### 6.2 Error Tracking
- [ ] **Add error tracking** 📁 `src/shared/error.rs`
  - **Priority**: Low
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: sentry or custom
  - **Purpose**: Track errors in production
  - **Integration**: Send errors to Sentry/similar service

### 6.3 Metrics
- [ ] **Add metrics collection** 📁 `src/middleware/metrics.rs`
  - **Priority**: Low
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: prometheus or metrics crate
  - **Metrics**:
    - Request count by endpoint
    - Response times
    - Error rates
    - Database query times
  - **Endpoint**: `/metrics` (Prometheus format)

---

## 7. Documentation

### 7.1 API Documentation
- [ ] **Create API docs** 📁 `docs/api.md`
  - **Priority**: Medium
  - **Complexity**: Medium (2 hours)
  - **Dependencies**: None
  - **Content**:
    - API endpoints list
    - Request/response examples
    - Authentication flow
    - Error codes
  - **Tool**: Consider OpenAPI/Swagger spec

### 7.2 Development Guide
- [ ] **Create dev guide** 📁 `docs/development.md`
  - **Priority**: Low
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: None
  - **Content**:
    - Setup instructions
    - Running tests
    - Database migrations
    - Contributing guidelines

### 7.3 Deployment Guide
- [ ] **Create deployment guide** 📁 `docs/deployment.md`
  - **Priority**: Medium
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: None
  - **Content**:
    - Environment variables
    - Database setup
    - Production build
    - Docker deployment
    - Reverse proxy config (nginx)

### 7.4 Architecture Documentation
- [ ] **Create architecture doc** 📁 `docs/architecture.md`
  - **Priority**: Low
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: None
  - **Content**:
    - DDD layers explanation
    - Module structure
    - Data flow diagrams
    - Design decisions

---

## 8. Production Build

### 8.1 Frontend Build
- [ ] **Create production build script** 📁 `scripts/build_frontend.sh`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Content**:
    ```bash
    #!/bin/bash
    cd resources
    npm run build
    ```
  - **Output**: `resources/dist/` with optimized assets

### 8.2 Backend Build
- [ ] **Create release build script** 📁 `scripts/build_backend.sh`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Content**:
    ```bash
    #!/bin/bash
    cargo build --release
    ```
  - **Output**: `target/release/auth-app`

### 8.3 Docker Setup
- [ ] **Create Dockerfile** 📁 `Dockerfile`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: None
  - **Multi-stage Build**:
    ```dockerfile
    # Frontend build stage
    FROM node:20 AS frontend
    WORKDIR /app/resources
    COPY resources/package*.json ./
    RUN npm install
    COPY resources/ ./
    RUN npm run build

    # Backend build stage
    FROM rust:1.75 AS backend
    WORKDIR /app
    COPY Cargo.toml Cargo.lock ./
    COPY src/ ./src/
    COPY migrations/ ./migrations/
    RUN cargo build --release

    # Runtime stage
    FROM debian:bookworm-slim
    RUN apt-get update && apt-get install -y libpq5 ca-certificates
    COPY --from=backend /app/target/release/auth-app /usr/local/bin/
    COPY --from=frontend /app/resources/dist /app/resources/dist
    COPY migrations/ /app/migrations/
    WORKDIR /app
    EXPOSE 3000
    CMD ["auth-app"]
    ```

- [ ] **Create docker-compose.yml** 📁 `docker-compose.yml`
  - **Priority**: Medium
  - **Complexity**: Medium (45 min)
  - **Dependencies**: None
  - **Services**: App, PostgreSQL, optional Redis (future)

### 8.4 Environment Configuration
- [ ] **Create production .env.example** 📁 `.env.production.example`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: None
  - **Variables**: All required env vars with production values

---

## 9. CI/CD Setup

### 9.1 GitHub Actions Workflow
- [ ] **Create CI workflow** 📁 `.github/workflows/ci.yml`
  - **Priority**: Low
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: None
  - **Jobs**:
    - Lint (cargo clippy)
    - Test (cargo test)
    - Build (cargo build)
    - Frontend lint (npm run lint)
    - Frontend build (npm run build)
  - **Triggers**: Push, pull request

### 9.2 Pre-commit Hooks
- [ ] **Create pre-commit config** 📁 `.pre-commit-config.yaml`
  - **Priority**: Low
  - **Complexity**: Simple (30 min)
  - **Dependencies**: pre-commit tool
  - **Hooks**: Format check, lint, test

---

## 10. Final Checklist

### Security:
- [ ] All passwords hashed with bcrypt
- [ ] CSRF protection on all state-changing endpoints
- [ ] JWT tokens have expiration
- [ ] SQL injection prevented (parameterized queries)
- [ ] XSS prevented (input sanitization)
- [ ] Rate limiting on auth endpoints
- [ ] HTTPS enforced in production
- [ ] Security headers configured
- [ ] Secrets not in repository

### Performance:
- [ ] Database indexes on foreign keys
- [ ] Connection pooling configured
- [ ] Static assets cached
- [ ] Gzip compression enabled
- [ ] No N+1 queries

### Reliability:
- [ ] Error handling on all endpoints
- [ ] Database transactions where needed
- [ ] Background jobs have error recovery
- [ ] Logging configured for debugging
- [ ] Health check endpoint works

### Code Quality:
- [ ] No compiler warnings
- [ ] All tests passing
- [ ] Code follows DDD principles
- [ ] Documentation complete
- [ ] Git history clean

---

## Production Deployment Steps

1. **Provision Server**:
   - VPS (DigitalOcean, AWS EC2, etc.)
   - PostgreSQL 18 database
   - Domain name + SSL certificate

2. **Setup Environment**:
   - Copy .env.production
   - Set strong secrets (JWT, session, CSRF)
   - Configure DATABASE_URL
   - Set RUST_ENV=production

3. **Database Setup**:
   - Create production database
   - Run migrations: `sqlx migrate run`
   - Verify schema

4. **Build Application**:
   - Frontend: `npm run build`
   - Backend: `cargo build --release`
   - Or use Docker: `docker-compose up -d`

5. **Configure Reverse Proxy** (nginx):
   - Proxy `/` to app
   - Serve static files
   - SSL termination
   - Gzip compression

6. **Start Application**:
   - Run binary or Docker container
   - Verify health check: `curl http://localhost:3000/health`

7. **Monitoring**:
   - Setup logs aggregation
   - Configure alerts
   - Monitor metrics

---

## Post-Deployment

### Immediate (Day 1):
- [ ] Monitor error logs
- [ ] Test all critical flows
- [ ] Check performance metrics
- [ ] Verify backup strategy

### Short-term (Week 1):
- [ ] Analyze usage patterns
- [ ] Optimize slow queries
- [ ] Fix any bugs discovered
- [ ] Collect user feedback

### Long-term (Month 1):
- [ ] Review security logs
- [ ] Plan new features
- [ ] Optimize resource usage
- [ ] Update dependencies

---

## Future Enhancements

### Authentication:
- [ ] Email verification workflow
- [ ] Password reset via email
- [ ] Two-factor authentication (2FA)
- [ ] OAuth providers (Google, GitHub)
- [ ] Remember me functionality

### Features:
- [ ] User roles and permissions
- [ ] Multi-tenancy (organization support)
- [ ] Activity logs (audit trail)
- [ ] API keys for programmatic access
- [ ] Webhooks

### Infrastructure:
- [ ] Redis for caching
- [ ] Message queue (background jobs)
- [ ] File upload (S3)
- [ ] Email service integration
- [ ] CDN for static assets

### Monitoring:
- [ ] APM (Application Performance Monitoring)
- [ ] User analytics
- [ ] A/B testing framework
- [ ] Feature flags

---

## Completion Criteria

✅ **Phase 7 is complete when**:
- All integration tests pass
- Background jobs are running
- Security hardening applied
- Documentation complete
- Production build works
- Application deployed and accessible
- Monitoring configured
- No critical bugs

---

## Resources

### Testing:
- Rust testing: https://doc.rust-lang.org/book/ch11-00-testing.html
- SQLx testing: https://github.com/launchbadge/sqlx/tree/main/examples

### Security:
- OWASP Top 10: https://owasp.org/www-project-top-ten/
- Rust security: https://rustsec.org/

### Deployment:
- Docker best practices: https://docs.docker.com/develop/dev-best-practices/
- PostgreSQL tuning: https://wiki.postgresql.org/wiki/Tuning_Your_PostgreSQL_Server

### Monitoring:
- Prometheus: https://prometheus.io/docs/introduction/overview/
- Grafana: https://grafana.com/docs/

---

## Success! 🎉

After completing Phase 7, you have a **production-ready** Rust web application with:
- ✅ Complete authentication system (session + JWT)
- ✅ User profile management
- ✅ DDD + Clean Architecture
- ✅ React 19 + Inertia.js frontend
- ✅ Tailwind CSS 4 + shadcn/ui
- ✅ Comprehensive testing
- ✅ Security hardening
- ✅ Performance optimization
- ✅ Production deployment

**This is a solid foundation for building multi-tenant SaaS applications!**
