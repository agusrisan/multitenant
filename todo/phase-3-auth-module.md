# Phase 3: Auth Module (DDD + Clean Architecture)

**Status**: 🔴 Not Started
**Priority**: 🔥 CRITICAL
**Estimated Time**: 12-16 hours
**Dependencies**: Phase 1 (Setup) + Phase 2 (Backend Foundation)

## Overview
Complete authentication module following DDD and Clean Architecture:
- **Domain Layer**: Business entities and rules
- **Application Layer**: Use cases (register, login, logout, refresh)
- **Infrastructure Layer**: Repositories (database persistence)
- **Web Layer**: Inertia handlers (session + CSRF)
- **API Layer**: JSON handlers (JWT)

---

## 1. Domain Layer (Business Logic)

### 1.1 Domain Module Structure
- [ ] **Create auth domain mod.rs** 📁 `src/moduls/auth/domain/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Domain
  - **Content**:
    ```rust
    pub mod user;
    pub mod session;
    pub mod token_pair;
    pub mod value_objects;

    pub use user::User;
    pub use session::Session;
    pub use token_pair::TokenPair;
    ```

### 1.2 User Entity
- [ ] **Create user.rs** 📁 `src/moduls/auth/domain/user.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: value_objects, shared types
  - **DDD Layer**: Domain (Aggregate Root)
  - **Structure**:
    ```rust
    #[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
    pub struct User {
        pub id: UserId,
        pub email: Email,
        #[serde(skip_serializing)]
        pub password_hash: PasswordHash,
        pub name: String,
        pub email_verified: bool,
        pub is_active: bool,
        pub created_at: Timestamp,
        pub updated_at: Timestamp,
    }

    impl User {
        pub fn new(email: Email, password: &str, name: String) -> Result<Self>;
        pub fn verify_password(&self, password: &str) -> Result<bool>;
        pub fn change_password(&mut self, new_password: &str) -> Result<()>;
        pub fn verify_email(&mut self);
        pub fn deactivate(&mut self);
    }
    ```
  - **Business Rules**:
    - Email must be unique (enforced by repository)
    - Password min 8 chars, hashed with bcrypt
    - New users: email_verified=false, is_active=true
  - **Notes**: User is aggregate root for auth context

### 1.3 Session Entity
- [ ] **Create session.rs** 📁 `src/moduls/auth/domain/session.rs`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: value_objects, shared types
  - **DDD Layer**: Domain (Entity)
  - **Structure**:
    ```rust
    #[derive(Debug, Clone, sqlx::FromRow)]
    pub struct Session {
        pub id: SessionId,
        pub user_id: UserId,
        pub csrf_token: CsrfToken,
        pub ip_address: Option<IpAddr>,
        pub user_agent: Option<String>,
        pub expires_at: Timestamp,
        pub created_at: Timestamp,
        pub updated_at: Timestamp,
    }

    impl Session {
        pub fn new(
            user_id: UserId,
            ip_address: Option<IpAddr>,
            user_agent: Option<String>,
            ttl_seconds: i64,
        ) -> Self;

        pub fn is_expired(&self) -> bool;
        pub fn refresh(&mut self, ttl_seconds: i64);
        pub fn verify_csrf(&self, token: &str) -> bool;
    }
    ```
  - **Business Rules**:
    - Session expires after TTL (configurable)
    - CSRF token generated on creation
    - One session per user for web (enforce in repository)

### 1.4 Token Pair Entity
- [ ] **Create token_pair.rs** 📁 `src/moduls/auth/domain/token_pair.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: value_objects, jsonwebtoken
  - **DDD Layer**: Domain (Value Object + Entity)
  - **Structure**:
    ```rust
    #[derive(Debug, Clone, serde::Serialize)]
    pub struct TokenPair {
        pub access_token: String,
        pub refresh_token: String,
        pub token_type: String,  // "Bearer"
        pub expires_in: i64,     // seconds
    }

    #[derive(Debug, Clone, sqlx::FromRow)]
    pub struct JwtToken {
        pub id: TokenId,
        pub user_id: UserId,
        pub token_type: TokenType,
        pub jti: uuid::Uuid,  // JWT ID
        pub expires_at: Timestamp,
        pub revoked: bool,
        pub revoked_at: Option<Timestamp>,
        pub created_at: Timestamp,
    }

    #[derive(Debug, Clone, Copy, sqlx::Type, serde::Serialize, serde::Deserialize)]
    #[sqlx(type_name = "token_type", rename_all = "lowercase")]
    pub enum TokenType {
        Access,
        Refresh,
    }

    impl TokenPair {
        pub fn generate(
            user_id: UserId,
            jwt_secret: &str,
            access_ttl: i64,
            refresh_ttl: i64,
        ) -> Result<(Self, JwtToken, JwtToken)>;
    }
    ```
  - **Business Rules**:
    - Access token: 15 min TTL (short-lived)
    - Refresh token: 7 days TTL (long-lived)
    - JTI (JWT ID) for revocation tracking
    - Tokens can be revoked (blacklist)
  - **Notes**: Returns tuple (TokenPair, AccessJwtToken, RefreshJwtToken) for persistence

### 1.5 Value Objects
- [ ] **Create value_objects.rs** 📁 `src/moduls/auth/domain/value_objects.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: validator, bcrypt
  - **DDD Layer**: Domain (Value Objects)
  - **Value Objects**:
    ```rust
    // Email (validated)
    #[derive(Debug, Clone, sqlx::Type, serde::Serialize, serde::Deserialize)]
    #[sqlx(transparent)]
    pub struct Email(String);

    impl Email {
        pub fn new(email: &str) -> Result<Self, ValidationError>;
        // Validation: RFC 5322 format
    }

    // PasswordHash (bcrypt)
    #[derive(Debug, Clone, sqlx::Type)]
    #[sqlx(transparent)]
    pub struct PasswordHash(String);

    impl PasswordHash {
        pub fn from_plain(password: &str) -> Result<Self>;
        pub fn verify(&self, password: &str) -> Result<bool>;
        // Min 8 chars validation
    }

    // CsrfToken (random 32 bytes, base64)
    #[derive(Debug, Clone, sqlx::Type)]
    #[sqlx(transparent)]
    pub struct CsrfToken(String);

    impl CsrfToken {
        pub fn generate() -> Self;
        pub fn verify(&self, token: &str) -> bool;
    }
    ```
  - **Validation Rules**:
    - Email: Valid format, max 255 chars
    - Password: Min 8 chars, bcrypt cost 12
    - CSRF: 32-byte random, constant-time compare
  - **Notes**: Value objects enforce invariants

---

## 2. Application Layer (Use Cases)

### 2.1 Application Module Structure
- [ ] **Create auth application mod.rs** 📁 `src/moduls/auth/application/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Application
  - **Content**:
    ```rust
    pub mod register_user;
    pub mod login_user;
    pub mod logout_user;
    pub mod refresh_token;

    pub use register_user::RegisterUserUseCase;
    pub use login_user::LoginUserUseCase;
    pub use logout_user::LogoutUserUseCase;
    pub use refresh_token::RefreshTokenUseCase;
    ```

### 2.2 Register User Use Case
- [ ] **Create register_user.rs** 📁 `src/moduls/auth/application/register_user.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: domain, infra repositories
  - **DDD Layer**: Application (Use Case)
  - **Input DTO**:
    ```rust
    #[derive(Debug, serde::Deserialize, validator::Validate)]
    pub struct RegisterUserCommand {
        #[validate(email)]
        pub email: String,
        #[validate(length(min = 8))]
        pub password: String,
        #[validate(length(min = 1))]
        pub name: String,
    }
    ```
  - **Use Case**:
    ```rust
    pub struct RegisterUserUseCase {
        user_repo: Arc<dyn UserRepository>,
    }

    impl RegisterUserUseCase {
        pub async fn execute(&self, cmd: RegisterUserCommand) -> AppResult<User> {
            // 1. Validate input
            // 2. Check email uniqueness
            // 3. Create User entity
            // 4. Save to repository
            // 5. Return User
        }
    }
    ```
  - **Business Logic**:
    - Email must be unique (check repository)
    - Password hashed in User::new()
    - Email verification not implemented yet (Phase 7)
  - **Error Cases**: Email already exists → Conflict error

### 2.3 Login User Use Case
- [ ] **Create login_user.rs** 📁 `src/moduls/auth/application/login_user.rs`
  - **Priority**: High
  - **Complexity**: Complex (2.5 hours)
  - **Dependencies**: domain, infra repositories
  - **DDD Layer**: Application (Use Case)
  - **Input DTOs**:
    ```rust
    // For web (session-based)
    #[derive(Debug, serde::Deserialize)]
    pub struct LoginWebCommand {
        pub email: String,
        pub password: String,
        pub ip_address: Option<IpAddr>,
        pub user_agent: Option<String>,
    }

    // For API (JWT-based)
    #[derive(Debug, serde::Deserialize)]
    pub struct LoginApiCommand {
        pub email: String,
        pub password: String,
    }
    ```
  - **Use Cases**:
    ```rust
    pub struct LoginUserUseCase {
        user_repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn SessionRepository>,
        token_repo: Arc<dyn TokenRepository>,
        jwt_secret: String,
        config: AuthConfig,
    }

    impl LoginUserUseCase {
        // Web login (returns Session)
        pub async fn login_web(&self, cmd: LoginWebCommand) -> AppResult<Session>;

        // API login (returns TokenPair)
        pub async fn login_api(&self, cmd: LoginApiCommand) -> AppResult<TokenPair>;
    }
    ```
  - **Business Logic (Web)**:
    1. Find user by email
    2. Verify password
    3. Check is_active
    4. Delete existing session (single session per user)
    5. Create new session
    6. Return session
  - **Business Logic (API)**:
    1. Find user by email
    2. Verify password
    3. Check is_active
    4. Generate TokenPair
    5. Save JwtTokens to repository
    6. Return TokenPair
  - **Error Cases**: Invalid credentials → Authentication error

### 2.4 Logout User Use Case
- [ ] **Create logout_user.rs** 📁 `src/moduls/auth/application/logout_user.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: domain, infra repositories
  - **DDD Layer**: Application (Use Case)
  - **Use Cases**:
    ```rust
    pub struct LogoutUserUseCase {
        session_repo: Arc<dyn SessionRepository>,
        token_repo: Arc<dyn TokenRepository>,
    }

    impl LogoutUserUseCase {
        // Web logout (delete session)
        pub async fn logout_web(&self, session_id: SessionId) -> AppResult<()>;

        // API logout (revoke tokens)
        pub async fn logout_api(&self, user_id: UserId) -> AppResult<()>;
    }
    ```
  - **Business Logic (Web)**: Delete session by ID
  - **Business Logic (API)**: Revoke all user's non-revoked tokens
  - **Notes**: Stateless JWT requires token blacklist

### 2.5 Refresh Token Use Case
- [ ] **Create refresh_token.rs** 📁 `src/moduls/auth/application/refresh_token.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: domain, infra repositories, jsonwebtoken
  - **DDD Layer**: Application (Use Case)
  - **Input DTO**:
    ```rust
    #[derive(Debug, serde::Deserialize)]
    pub struct RefreshTokenCommand {
        pub refresh_token: String,
    }
    ```
  - **Use Case**:
    ```rust
    pub struct RefreshTokenUseCase {
        token_repo: Arc<dyn TokenRepository>,
        jwt_secret: String,
        config: AuthConfig,
    }

    impl RefreshTokenUseCase {
        pub async fn execute(&self, cmd: RefreshTokenCommand) -> AppResult<TokenPair> {
            // 1. Decode refresh token
            // 2. Extract JTI
            // 3. Check token not revoked in DB
            // 4. Check expiration
            // 5. Revoke old refresh token
            // 6. Generate new TokenPair
            // 7. Save new tokens
            // 8. Return new TokenPair
        }
    }
    ```
  - **Business Logic**:
    - Refresh token rotation (old token revoked)
    - JTI blacklist check
    - New access + refresh tokens generated
  - **Security**: Prevents token reuse

---

## 3. Infrastructure Layer (Repositories)

### 3.1 Infrastructure Module Structure
- [ ] **Create auth infra mod.rs** 📁 `src/moduls/auth/infra/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Infrastructure
  - **Content**:
    ```rust
    pub mod postgres_user_repository;
    pub mod postgres_session_repository;
    pub mod postgres_token_repository;

    pub use postgres_user_repository::{UserRepository, PostgresUserRepository};
    pub use postgres_session_repository::{SessionRepository, PostgresSessionRepository};
    pub use postgres_token_repository::{TokenRepository, PostgresTokenRepository};
    ```

### 3.2 User Repository
- [ ] **Create postgres_user_repository.rs** 📁 `src/moduls/auth/infra/postgres_user_repository.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: domain::User, sqlx
  - **DDD Layer**: Infrastructure (Repository Interface + Implementation)
  - **Trait**:
    ```rust
    #[async_trait]
    pub trait UserRepository: Send + Sync {
        async fn save(&self, user: &User) -> AppResult<User>;
        async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
        async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
        async fn update(&self, user: &User) -> AppResult<User>;
        async fn delete(&self, id: UserId) -> AppResult<()>;
    }
    ```
  - **Implementation**:
    ```rust
    pub struct PostgresUserRepository {
        pool: PgPool,
    }

    #[async_trait]
    impl UserRepository for PostgresUserRepository {
        // SQLx queries using prepared statements
        // Error mapping: sqlx::Error → AppError
    }
    ```
  - **Queries**: Parameterized SQL (prevent injection)
  - **Tech**: SQLx 0.8.6 compile-time checked queries

### 3.3 Session Repository
- [ ] **Create postgres_session_repository.rs** 📁 `src/moduls/auth/infra/postgres_session_repository.rs`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: domain::Session, sqlx
  - **DDD Layer**: Infrastructure (Repository)
  - **Trait**:
    ```rust
    #[async_trait]
    pub trait SessionRepository: Send + Sync {
        async fn save(&self, session: &Session) -> AppResult<Session>;
        async fn find_by_id(&self, id: SessionId) -> AppResult<Option<Session>>;
        async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<Session>>;
        async fn delete(&self, id: SessionId) -> AppResult<()>;
        async fn delete_by_user_id(&self, user_id: UserId) -> AppResult<()>;
        async fn delete_expired(&self) -> AppResult<u64>;  // cleanup job
    }
    ```
  - **Special Logic**: find_by_user_id enforces single session per user
  - **Cleanup**: delete_expired() for background job (Phase 7)

### 3.4 Token Repository
- [ ] **Create postgres_token_repository.rs** 📁 `src/moduls/auth/infra/postgres_token_repository.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: domain::JwtToken, sqlx
  - **DDD Layer**: Infrastructure (Repository)
  - **Trait**:
    ```rust
    #[async_trait]
    pub trait TokenRepository: Send + Sync {
        async fn save(&self, token: &JwtToken) -> AppResult<JwtToken>;
        async fn find_by_jti(&self, jti: uuid::Uuid) -> AppResult<Option<JwtToken>>;
        async fn revoke(&self, jti: uuid::Uuid) -> AppResult<()>;
        async fn revoke_all_user_tokens(&self, user_id: UserId) -> AppResult<()>;
        async fn delete_expired(&self) -> AppResult<u64>;  // cleanup job
    }
    ```
  - **Revocation**: Update revoked=true, set revoked_at
  - **Performance**: Index on jti for fast lookups

---

## 4. Web Layer (Inertia + Session + CSRF)

### 4.1 Web Module Structure
- [ ] **Create auth web mod.rs** 📁 `src/moduls/auth/web/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Interface Adapter (Web)
  - **Content**:
    ```rust
    pub mod routes;
    pub mod handlers;
    pub mod middleware;
    pub mod view_models;

    pub use routes::auth_web_routes;
    ```

### 4.2 Web Routes
- [ ] **Create web routes.rs** 📁 `src/moduls/auth/web/routes.rs`
  - **Priority**: High
  - **Complexity**: Medium (45 min)
  - **Dependencies**: handlers, middleware
  - **DDD Layer**: Interface Adapter
  - **Routes**:
    ```rust
    pub fn auth_web_routes() -> Router<AppState> {
        Router::new()
            .route("/login", get(handlers::show_login).post(handlers::handle_login))
            .route("/register", get(handlers::show_register).post(handlers::handle_register))
            .route("/logout", post(handlers::handle_logout))
            .layer(middleware::csrf_layer())
    }
    ```
  - **Endpoints**:
    - `GET /web/login` - Show login form
    - `POST /web/login` - Process login
    - `GET /web/register` - Show register form
    - `POST /web/register` - Process registration
    - `POST /web/logout` - Logout user
  - **Middleware**: CSRF protection on all POST routes

### 4.3 Web Handlers
- [ ] **Create web handlers.rs** 📁 `src/moduls/auth/web/handlers.rs`
  - **Priority**: High
  - **Complexity**: Complex (3 hours)
  - **Dependencies**: application use cases, view_models, axum_inertia
  - **DDD Layer**: Interface Adapter
  - **Handlers**:
    ```rust
    // GET /web/login
    pub async fn show_login() -> Result<InertiaResponse, AppError> {
        Inertia::render("Auth/Login", LoginPageProps { errors: None })
    }

    // POST /web/login
    pub async fn handle_login(
        State(state): State<AppState>,
        headers: HeaderMap,
        Form(form): Form<LoginForm>,
    ) -> Result<Redirect, AppError> {
        // 1. Extract IP + User-Agent
        // 2. Call LoginUserUseCase::login_web()
        // 3. Set session cookie
        // 4. Redirect to /web/dashboard
    }

    // Similar for register, logout
    ```
  - **Session Management**:
    - Cookie: `session_id` (HttpOnly, Secure, SameSite=Strict)
    - CSRF token in form hidden field
  - **Error Handling**: Validation errors → Inertia with errors prop
  - **Tech**: axum_inertia 0.9.0 for Inertia::render()

### 4.4 View Models (Inertia Props)
- [ ] **Create view_models.rs** 📁 `src/moduls/auth/web/view_models.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (30 min)
  - **Dependencies**: serde
  - **DDD Layer**: Interface Adapter
  - **Props**:
    ```rust
    #[derive(Debug, serde::Serialize)]
    pub struct LoginPageProps {
        pub errors: Option<HashMap<String, Vec<String>>>,
    }

    #[derive(Debug, serde::Serialize)]
    pub struct RegisterPageProps {
        pub errors: Option<HashMap<String, Vec<String>>>,
    }
    ```
  - **Purpose**: Type-safe props for React components

### 4.5 CSRF Middleware
- [ ] **Create csrf_layer.rs** 📁 `src/moduls/auth/web/middleware/csrf_layer.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: tower, axum
  - **DDD Layer**: Infrastructure (Middleware)
  - **Responsibility**:
    - Generate CSRF token on GET requests
    - Validate CSRF token on POST requests
    - Compare with session's csrf_token
  - **Cookie**: `csrf_token` (read by frontend for forms)
  - **Header**: `X-CSRF-Token` or form field `_csrf`

### 4.6 Session Middleware
- [ ] **Create session_layer.rs** 📁 `src/moduls/auth/web/middleware/session_layer.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: tower, axum, SessionRepository
  - **DDD Layer**: Infrastructure (Middleware)
  - **Responsibility**:
    - Extract `session_id` cookie
    - Load Session from repository
    - Check not expired
    - Add Session to request extensions
  - **Extractor**:
    ```rust
    pub struct AuthSession(pub Session);

    #[async_trait]
    impl<S> FromRequestParts<S> for AuthSession {
        // Extract from extensions (added by middleware)
    }
    ```
  - **Protected Routes**: Use AuthSession extractor

---

## 5. API Layer (JWT)

### 5.1 API Module Structure
- [ ] **Create auth api mod.rs** 📁 `src/moduls/auth/api/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Interface Adapter (API)
  - **Content**:
    ```rust
    pub mod routes;
    pub mod handlers;
    pub mod middleware;

    pub use routes::auth_api_routes;
    ```

### 5.2 API Routes
- [ ] **Create api routes.rs** 📁 `src/moduls/auth/api/routes.rs`
  - **Priority**: High
  - **Complexity**: Medium (30 min)
  - **Dependencies**: handlers, middleware
  - **DDD Layer**: Interface Adapter
  - **Routes**:
    ```rust
    pub fn auth_api_routes() -> Router<AppState> {
        Router::new()
            .route("/login", post(handlers::login))
            .route("/register", post(handlers::register))
            .route("/refresh", post(handlers::refresh))
            .route("/logout", post(handlers::logout))
            .route("/me", get(handlers::me))
            .route_layer(middleware::jwt_layer())  // except login, register
    }
    ```
  - **Endpoints**:
    - `POST /api/auth/login` - Login (returns tokens)
    - `POST /api/auth/register` - Register
    - `POST /api/auth/refresh` - Refresh access token
    - `POST /api/auth/logout` - Revoke tokens
    - `GET /api/auth/me` - Get current user (protected)
  - **Content-Type**: application/json

### 5.3 API Handlers
- [ ] **Create api handlers.rs** 📁 `src/moduls/auth/api/handlers.rs`
  - **Priority**: High
  - **Complexity**: Complex (2.5 hours)
  - **Dependencies**: application use cases, axum::Json
  - **DDD Layer**: Interface Adapter
  - **Handlers**:
    ```rust
    // POST /api/auth/login
    pub async fn login(
        State(state): State<AppState>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<TokenResponse>, AppError> {
        // Call LoginUserUseCase::login_api()
        // Return TokenPair as JSON
    }

    // GET /api/auth/me (protected)
    pub async fn me(
        AuthUser(user): AuthUser,
    ) -> Result<Json<UserResponse>, AppError> {
        // Return current user
    }
    ```
  - **DTOs**:
    ```rust
    #[derive(serde::Deserialize)]
    pub struct LoginRequest { email: String, password: String }

    #[derive(serde::Serialize)]
    pub struct TokenResponse {
        access_token: String,
        refresh_token: String,
        token_type: String,
        expires_in: i64,
    }
    ```
  - **Error Format**: JSON with error message

### 5.4 JWT Middleware
- [ ] **Create jwt_layer.rs** 📁 `src/moduls/auth/api/middleware/jwt_layer.rs`
  - **Priority**: High
  - **Complexity**: Complex (2.5 hours)
  - **Dependencies**: jsonwebtoken, tower, axum
  - **DDD Layer**: Infrastructure (Middleware)
  - **Responsibility**:
    - Extract `Authorization: Bearer <token>` header
    - Decode JWT
    - Validate signature, expiration
    - Check JTI not revoked (TokenRepository)
    - Load User from repository
    - Add User to request extensions
  - **Extractor**:
    ```rust
    pub struct AuthUser(pub User);

    #[async_trait]
    impl<S> FromRequestParts<S> for AuthUser {
        // Extract from extensions (added by middleware)
        // Return 401 if missing/invalid
    }
    ```
  - **Performance**: Cache user lookup (optional, Phase 7)

---

## 6. Module Wiring

### 6.1 Auth Module Root
- [ ] **Create auth mod.rs** 📁 `src/moduls/auth/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: All submodules
  - **DDD Layer**: Module Root
  - **Content**:
    ```rust
    pub mod domain;
    pub mod application;
    pub mod infra;
    pub mod web;
    pub mod api;

    pub use web::auth_web_routes;
    pub use api::auth_api_routes;
    ```

### 6.2 Modules Root
- [ ] **Create moduls mod.rs** 📁 `src/moduls/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Content**:
    ```rust
    pub mod auth;
    // pub mod user;  // Phase 6
    ```

### 6.3 Mount Routes in Startup
- [ ] **Update startup.rs to mount auth routes** 📁 `src/startup.rs`
  - **Priority**: High
  - **Complexity**: Medium (30 min)
  - **Dependencies**: Requires auth module complete
  - **DDD Layer**: Infrastructure (Application Entry)
  - **Changes**:
    ```rust
    use crate::moduls::auth::{auth_web_routes, auth_api_routes};

    pub async fn build_app(config: Config, db: PgPool) -> Router {
        let state = AppState { /* ... */ };

        Router::new()
            .route("/health", get(health_check))
            .nest("/web/auth", auth_web_routes())
            .nest("/api/auth", auth_api_routes())
            .with_state(state)
            .layer(/* middleware stack */)
    }
    ```

---

## Validation Checklist

### Domain Layer:
- [ ] User entity enforces business rules
- [ ] Session generates CSRF tokens
- [ ] TokenPair generates valid JWTs
- [ ] Value objects validate on construction

### Application Layer:
- [ ] Register use case checks email uniqueness
- [ ] Login use case verifies password
- [ ] Refresh use case rotates tokens
- [ ] Logout use case revokes tokens

### Infrastructure Layer:
- [ ] UserRepository saves/finds users
- [ ] SessionRepository manages sessions
- [ ] TokenRepository tracks JWT revocation
- [ ] All queries parameterized (no SQL injection)

### Web Layer:
- [ ] Inertia renders React components
- [ ] CSRF middleware validates tokens
- [ ] Session middleware loads user
- [ ] Forms submit with CSRF tokens

### API Layer:
- [ ] JWT middleware validates tokens
- [ ] Protected routes require auth
- [ ] Refresh endpoint rotates tokens
- [ ] Logout revokes all user tokens

### Integration:
- [ ] Can register new user via web
- [ ] Can login via web (sets session cookie)
- [ ] Can logout via web
- [ ] Can register via API
- [ ] Can login via API (returns JWT)
- [ ] Can access protected endpoint with JWT
- [ ] Can refresh access token

---

## Architecture Notes

### DDD Layers Applied:
1. **Domain**: Pure business logic (User, Session, TokenPair, value objects)
2. **Application**: Use cases orchestrating domain + infrastructure
3. **Infrastructure**: Repositories (PostgreSQL), external services
4. **Interface Adapters**: Web (Inertia) + API (JSON) handlers

### Dependency Flow:
```
web/api → application → domain ✓
application → infra (repositories) ✓
infra → domain ✓
domain → NO dependencies ✓
```

### Clean Architecture Principles:
- Domain layer has zero external dependencies
- Application layer depends only on domain abstractions
- Infrastructure implements domain interfaces
- Web/API layers are thin adapters

---

## Next Phase
➡️ **Phase 4: Frontend Foundation** - React 19, Vite, Tailwind 4, shadcn setup

## Security Notes
- **Password Security**: bcrypt cost 12 (2^12 iterations)
- **CSRF Protection**: Synchronizer token pattern
- **Session Security**: HttpOnly, Secure, SameSite=Strict cookies
- **JWT Security**: Short-lived access tokens, refresh rotation
- **Token Revocation**: JTI blacklist in database
- **SQL Injection**: Parameterized queries via SQLx
- **Timing Attacks**: Constant-time comparisons for tokens
