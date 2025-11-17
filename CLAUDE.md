# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Language Rules

**IMPORTANT**: Claude harus selalu respond dalam **Indonesian (Bahasa Indonesia)** untuk semua komunikasi, penjelasan, dan diskusi. Namun, untuk semua coding-related activities, gunakan **English**:
- Function names, variable names, class names
- Code comments dan documentation
- Commit messages
- Error messages dalam code
- API endpoint names
- Database column names

## Project Overview

**Multitenant Authentication Application** built with Rust backend and React frontend, following Domain-Driven Design (DDD) and Clean Architecture principles.

**Tech Stack:**
- **Backend**: Rust, Axum 0.8.7, SQLx 0.8.6, PostgreSQL 18
- **Frontend**: React 19, TypeScript 5.6+, Inertia.js 2.0, Vite 6, Tailwind CSS 4, shadcn/ui
- **Authentication**: Dual auth system - Session-based (web) + JWT (API)
- **Architecture**: DDD + Clean Architecture with modular structure

**Current Status**: Project is in planning phase with detailed roadmap (see `todo/` directory).

## Development Commands

### Database
```bash
# Start PostgreSQL (Docker)
docker-compose up -d postgres

# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add <migration_name>

# Prepare SQLx for offline compilation
cargo sqlx prepare
```

### Backend
```bash
# Run development server
cargo run

# Build for production
cargo build --release

# Run tests
cargo test

# Run specific test module
cargo test auth_tests

# Lint and check
cargo clippy
cargo check
```

### Frontend
```bash
# Install dependencies
cd resources && npm install

# Run development server (separate terminal)
cd resources && npm run dev

# Build for production
cd resources && npm run build

# Type check
cd resources && npm run type-check
```

## Architecture & Code Structure

### Clean Architecture Layers

The codebase follows strict DDD layering:

1. **Domain Layer** (`src/moduls/*/domain/`)
   - Business entities (User, Session, TokenPair)
   - Value objects (Email, PasswordHash, CsrfToken)
   - Business rules and invariants
   - No dependencies on infrastructure or frameworks

2. **Application Layer** (`src/moduls/*/application/`)
   - Use cases (RegisterUser, LoginUser, LogoutUser, RefreshToken)
   - Orchestrates domain objects
   - Defines repository interfaces (traits)
   - Framework-agnostic

3. **Infrastructure Layer** (`src/moduls/*/infra/`)
   - Repository implementations (PostgreSQL)
   - External service integrations
   - Depends on application layer interfaces

4. **Interface Adapters** (`src/moduls/*/web/` and `src/moduls/*/api/`)
   - **Web**: Inertia.js handlers for session-based auth
   - **API**: JSON handlers for JWT-based auth
   - Route definitions and middleware
   - Request/response DTOs

5. **Bootstrap Layer** (`src/bootstrap/`)
   - Application initialization
   - Database connection pooling
   - Telemetry and logging setup

6. **Shared Kernel** (`src/shared/`)
   - Common types (UserId, SessionId, Timestamp)
   - Error handling (AppError, AppResult)
   - Cross-module utilities

### Module Structure

Each feature module follows this structure:
```
src/moduls/<module>/
├── domain/          # Entities, value objects, business rules
├── application/     # Use cases, repository traits
├── infra/          # Repository implementations, DB access
├── web/            # Inertia handlers (session auth)
└── api/            # JSON handlers (JWT auth)
```

### Dual Authentication System

**Session-based (Web routes `/web/*`):**
- Uses Inertia.js for hybrid SPA experience
- Session cookies + CSRF protection
- Returns HTML with React hydration
- Middleware: SessionLayer, CsrfLayer

**JWT-based (API routes `/api/*`):**
- Access tokens (15 min) + Refresh tokens (7 days)
- Stored in `jwt_tokens` table for revocation
- Bearer token authentication
- Middleware: JwtLayer

### Key Design Decisions

1. **UUID v7**: Time-ordered UUIDs for better database indexing performance (native PostgreSQL 18 support)
2. **Monorepo**: Frontend and backend in same repository for simplicity
3. **Compile-time SQL checking**: SQLx verifies queries at compile time
4. **Type-safe routing**: Axum's type-safe extractors and state management
5. **Inertia.js**: Eliminates need for separate REST API for web routes

## Important Patterns

### Repository Pattern
Repositories are defined as traits in the application layer and implemented in infrastructure:
```rust
// Application layer
pub trait UserRepository {
    async fn find_by_id(&self, id: UserId) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &Email) -> AppResult<Option<User>>;
    async fn save(&self, user: &User) -> AppResult<()>;
}

// Infrastructure layer
pub struct PostgresUserRepository { /* ... */ }
impl UserRepository for PostgresUserRepository { /* ... */ }
```

### Error Handling
Use `AppError` enum for all errors with proper HTTP status mapping:
- Validation → 400
- Authentication → 401
- Authorization → 403
- NotFound → 404
- Conflict → 409
- Database/Internal → 500

All handlers return `AppResult<T>` which auto-converts to JSON responses.

### State Management
`AppState` is shared across all handlers via Axum's state system:
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

## Database Conventions

- **Primary keys**: UUID v7 (time-ordered)
- **Timestamps**: Always `TIMESTAMPTZ` (UTC)
- **Foreign keys**: CASCADE on delete for user-related data
- **Indexes**: On foreign keys, frequently queried columns, and expiration times
- **Naming**: snake_case for tables and columns
- **Migrations**: Timestamp prefix format `YYYYMMDDHHMMSS_description.sql`

## Frontend Architecture

### Inertia.js Pages
React pages are served by backend routes at `resources/js/pages/`:
```
pages/
├── Auth/
│   ├── Login.tsx
│   └── Register.tsx
└── Dashboard/
    └── Index.tsx
```

### Layouts
Two main layouts in `resources/js/layouts/`:
- `AuthLayout.tsx`: For login/register pages (unauthenticated)
- `AppLayout.tsx`: For dashboard and authenticated pages

### Tailwind CSS 4
Uses new `@import "tailwindcss"` syntax. shadcn/ui components go in `resources/js/components/`.

### Type Safety
Backend passes type-safe props to frontend via Inertia. Define TypeScript interfaces matching backend DTOs.

## Environment Configuration

Required variables in `.env`:
```env
DATABASE_URL=postgres://postgres:password@localhost:5432/multitenant_db
HOST=127.0.0.1
PORT=3000
JWT_SECRET=<secret>
JWT_ACCESS_EXPIRY=900
JWT_REFRESH_EXPIRY=604800
SESSION_SECRET=<secret>
CSRF_SECRET=<secret>
RUST_LOG=debug
RUST_ENV=development
```

## Security Considerations

1. **Password Hashing**: Use bcrypt with appropriate work factor
2. **CSRF Protection**: All web form submissions must include CSRF token
3. **SQL Injection**: SQLx's compile-time checking prevents injection
4. **Token Revocation**: JWT tokens stored in database for revocation capability
5. **Session Expiry**: Automatic cleanup job for expired sessions (planned in Phase 7)
6. **Rate Limiting**: To be implemented in Phase 7

## Development Workflow

### Starting a New Feature Module

1. Create module structure: `src/moduls/<module>/`
2. Start with domain layer (entities, value objects)
3. Define application layer (use cases, repository traits)
4. Implement infrastructure layer (repository implementations)
5. Add web/api handlers and routes
6. Write integration tests
7. Update `src/main.rs` to include new module

### Making Changes to Auth Flow

Auth is the core module. Changes require:
1. Update domain entities if business rules change
2. Modify use cases in application layer
3. Update repository if persistence changes
4. Adjust handlers and middleware
5. Update frontend forms and validation

### Adding New Database Tables

1. Create migration: `sqlx migrate add <name>`
2. Write SQL in `migrations/` directory
3. Run migration: `sqlx migrate run`
4. Run `cargo sqlx prepare` for offline compilation
5. Update domain entities and repositories

## Project Phases

Refer to `todo/` directory for detailed phase breakdowns:
- Phase 1: Project Setup (4-6h)
- Phase 2: Backend Foundation (6-8h)
- Phase 3: Auth Module (12-16h)
- Phase 4: Frontend Foundation (6-8h)
- Phase 5: Auth Frontend (4-6h)
- Phase 6: User Module (6-8h)
- Phase 7: Integration & Deploy (8-12h)

Total estimated: 46-64 hours

## Testing Strategy

### Unit Tests
- Test domain entities and business rules in isolation
- Test use cases with mocked repositories
- Located alongside source files

### Integration Tests
- Test full request/response cycles
- Use test database (separate from development)
- Located in `tests/` directory

### Running Tests
```bash
# All tests
cargo test

# Specific module
cargo test moduls::auth

# With output
cargo test -- --nocapture
```
