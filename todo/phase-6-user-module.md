# Phase 6: User Module (Profile Management)

**Status**: ✅ Complete
**Priority**: 🟡 MEDIUM
**Estimated Time**: 6-8 hours
**Actual Time**: ~2 hours
**Dependencies**: Phase 2 (Backend Foundation) + Phase 3 (Auth Module)
**Completed**: 2025-11-17

## Overview
User profile management module following DDD architecture: profile viewing, editing, password change, and account settings. Demonstrates modular architecture extension.

---

## 1. Domain Layer

### 1.1 Domain Module Structure
- [x] **Create user domain mod.rs** 📁 `src/moduls/user/domain/mod.rs` ✅
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Domain
  - **Content**:
    ```rust
    pub mod user_profile;

    pub use user_profile::UserProfile;
    ```

### 1.2 UserProfile Entity
- [x] **Create user_profile.rs** 📁 `src/moduls/user/domain/user_profile.rs` ✅
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: shared types, auth domain User
  - **DDD Layer**: Domain (Entity)
  - **Structure**:
    ```rust
    use crate::shared::types::{UserId, Timestamp};

    #[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
    pub struct UserProfile {
        pub user_id: UserId,
        pub name: String,
        pub email: String,
        pub bio: Option<String>,
        pub avatar_url: Option<String>,
        pub updated_at: Timestamp,
    }

    impl UserProfile {
        pub fn update_name(&mut self, name: String) {
            self.name = name;
            self.updated_at = chrono::Utc::now();
        }

        pub fn update_bio(&mut self, bio: Option<String>) {
            self.bio = bio;
            self.updated_at = chrono::Utc::now();
        }

        pub fn update_avatar(&mut self, avatar_url: Option<String>) {
            self.avatar_url = avatar_url;
            self.updated_at = chrono::Utc::now();
        }
    }
    ```
  - **Business Rules**:
    - Profile is 1:1 with User (same user_id)
    - Name cannot be empty
    - Bio max 500 chars (optional)
    - Avatar URL validation (optional)
  - **Notes**: Extends user data beyond auth concerns

---

## 2. Application Layer

### 2.1 Application Module Structure
- [ ] **Create user application mod.rs** 📁 `src/moduls/user/application/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Application
  - **Content**:
    ```rust
    pub mod get_profile;
    pub mod update_profile;
    pub mod change_password;

    pub use get_profile::GetProfileUseCase;
    pub use update_profile::UpdateProfileUseCase;
    pub use change_password::ChangePasswordUseCase;
    ```

### 2.2 Get Profile Use Case
- [ ] **Create get_profile.rs** 📁 `src/moduls/user/application/get_profile.rs`
  - **Priority**: High
  - **Complexity**: Simple (30 min)
  - **Dependencies**: domain, infra repositories
  - **DDD Layer**: Application (Use Case)
  - **Structure**:
    ```rust
    use crate::moduls::user::domain::UserProfile;
    use crate::moduls::user::infra::UserProfileRepository;
    use crate::shared::{AppResult, types::UserId};
    use std::sync::Arc;

    pub struct GetProfileUseCase {
        profile_repo: Arc<dyn UserProfileRepository>,
    }

    impl GetProfileUseCase {
        pub fn new(profile_repo: Arc<dyn UserProfileRepository>) -> Self {
            Self { profile_repo }
        }

        pub async fn execute(&self, user_id: UserId) -> AppResult<UserProfile> {
            self.profile_repo
                .find_by_user_id(user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Profile not found".into()))
        }
    }
    ```
  - **Business Logic**: Simple retrieval
  - **Error**: NotFound if profile doesn't exist

### 2.3 Update Profile Use Case
- [ ] **Create update_profile.rs** 📁 `src/moduls/user/application/update_profile.rs`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: domain, infra repositories
  - **DDD Layer**: Application (Use Case)
  - **Input DTO**:
    ```rust
    #[derive(Debug, serde::Deserialize, validator::Validate)]
    pub struct UpdateProfileCommand {
        #[validate(length(min = 1))]
        pub name: String,
        #[validate(length(max = 500))]
        pub bio: Option<String>,
        #[validate(url)]
        pub avatar_url: Option<String>,
    }
    ```
  - **Use Case**:
    ```rust
    pub struct UpdateProfileUseCase {
        profile_repo: Arc<dyn UserProfileRepository>,
    }

    impl UpdateProfileUseCase {
        pub async fn execute(
            &self,
            user_id: UserId,
            cmd: UpdateProfileCommand,
        ) -> AppResult<UserProfile> {
            // 1. Validate input
            cmd.validate().map_err(|e| AppError::Validation(e.to_string()))?;

            // 2. Load profile
            let mut profile = self.profile_repo
                .find_by_user_id(user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Profile not found".into()))?;

            // 3. Update fields
            profile.update_name(cmd.name);
            profile.update_bio(cmd.bio);
            profile.update_avatar(cmd.avatar_url);

            // 4. Save
            self.profile_repo.update(&profile).await
        }
    }
    ```
  - **Validation**: Length, URL format
  - **Business Logic**: Use domain methods

### 2.4 Change Password Use Case
- [ ] **Create change_password.rs** 📁 `src/moduls/user/application/change_password.rs`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: auth domain User, auth infra UserRepository
  - **DDD Layer**: Application (Use Case)
  - **Input DTO**:
    ```rust
    #[derive(Debug, serde::Deserialize, validator::Validate)]
    pub struct ChangePasswordCommand {
        pub current_password: String,
        #[validate(length(min = 8))]
        pub new_password: String,
        pub new_password_confirmation: String,
    }
    ```
  - **Use Case**:
    ```rust
    use crate::moduls::auth::infra::UserRepository;

    pub struct ChangePasswordUseCase {
        user_repo: Arc<dyn UserRepository>,
    }

    impl ChangePasswordUseCase {
        pub async fn execute(
            &self,
            user_id: UserId,
            cmd: ChangePasswordCommand,
        ) -> AppResult<()> {
            // 1. Validate input
            cmd.validate().map_err(|e| AppError::Validation(e.to_string()))?;

            // 2. Check password confirmation
            if cmd.new_password != cmd.new_password_confirmation {
                return Err(AppError::Validation("Passwords do not match".into()));
            }

            // 3. Load user
            let mut user = self.user_repo
                .find_by_id(user_id)
                .await?
                .ok_or_else(|| AppError::NotFound("User not found".into()))?;

            // 4. Verify current password
            if !user.verify_password(&cmd.current_password)? {
                return Err(AppError::Authentication("Invalid current password".into()));
            }

            // 5. Change password
            user.change_password(&cmd.new_password)?;

            // 6. Save
            self.user_repo.update(&user).await?;

            Ok(())
        }
    }
    ```
  - **Security**: Verify current password before change
  - **Validation**: Password strength, confirmation match

---

## 3. Infrastructure Layer

### 3.1 Infrastructure Module Structure
- [ ] **Create user infra mod.rs** 📁 `src/moduls/user/infra/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Infrastructure
  - **Content**:
    ```rust
    pub mod postgres_user_profile_repository;

    pub use postgres_user_profile_repository::{
        UserProfileRepository,
        PostgresUserProfileRepository,
    };
    ```

### 3.2 User Profile Repository
- [ ] **Create postgres_user_profile_repository.rs** 📁 `src/moduls/user/infra/postgres_user_profile_repository.rs`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: domain::UserProfile, sqlx
  - **DDD Layer**: Infrastructure (Repository)
  - **Trait**:
    ```rust
    #[async_trait]
    pub trait UserProfileRepository: Send + Sync {
        async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<UserProfile>>;
        async fn update(&self, profile: &UserProfile) -> AppResult<UserProfile>;
    }
    ```
  - **Implementation**:
    ```rust
    pub struct PostgresUserProfileRepository {
        pool: PgPool,
    }

    #[async_trait]
    impl UserProfileRepository for PostgresUserProfileRepository {
        async fn find_by_user_id(&self, user_id: UserId) -> AppResult<Option<UserProfile>> {
            let profile = sqlx::query_as::<_, UserProfile>(
                "SELECT user_id, name, email, bio, avatar_url, updated_at
                 FROM users WHERE id = $1"
            )
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

            Ok(profile)
        }

        async fn update(&self, profile: &UserProfile) -> AppResult<UserProfile> {
            let updated = sqlx::query_as::<_, UserProfile>(
                "UPDATE users
                 SET name = $1, bio = $2, avatar_url = $3, updated_at = $4
                 WHERE id = $5
                 RETURNING user_id, name, email, bio, avatar_url, updated_at"
            )
            .bind(&profile.name)
            .bind(&profile.bio)
            .bind(&profile.avatar_url)
            .bind(profile.updated_at)
            .bind(profile.user_id)
            .fetch_one(&self.pool)
            .await?;

            Ok(updated)
        }
    }
    ```
  - **Note**: No separate user_profiles table; extends users table
  - **Migration**: Add bio, avatar_url columns to users table

### 3.3 Users Table Migration (Profile Columns)
- [ ] **Create user_profile_columns migration** 📁 `migrations/20250117000004_add_profile_columns.sql`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: users table exists
  - **DDD Layer**: Infrastructure (Persistence)
  - **Schema**:
    ```sql
    ALTER TABLE users
    ADD COLUMN IF NOT EXISTS bio TEXT,
    ADD COLUMN IF NOT EXISTS avatar_url TEXT;

    -- Optional: Add constraints
    ALTER TABLE users
    ADD CONSTRAINT bio_length CHECK (LENGTH(bio) <= 500);
    ```
  - **Notes**: Extends existing users table

---

## 4. Web Layer

### 4.1 Web Module Structure
- [ ] **Create user web mod.rs** 📁 `src/moduls/user/web/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Interface Adapter (Web)
  - **Content**:
    ```rust
    pub mod routes;
    pub mod handlers;

    pub use routes::user_web_routes;
    ```

### 4.2 Web Routes
- [ ] **Create web routes.rs** 📁 `src/moduls/user/web/routes.rs`
  - **Priority**: High
  - **Complexity**: Simple (30 min)
  - **Dependencies**: handlers
  - **DDD Layer**: Interface Adapter
  - **Routes**:
    ```rust
    use axum::{Router, routing::{get, post}};

    pub fn user_web_routes() -> Router<AppState> {
        Router::new()
            .route("/profile", get(handlers::show_profile))
            .route("/profile/edit", get(handlers::show_edit_profile).post(handlers::handle_update_profile))
            .route("/settings/password", get(handlers::show_change_password).post(handlers::handle_change_password))
    }
    ```
  - **Endpoints**:
    - `GET /web/user/profile` - View profile
    - `GET /web/user/profile/edit` - Edit profile form
    - `POST /web/user/profile/edit` - Update profile
    - `GET /web/user/settings/password` - Change password form
    - `POST /web/user/settings/password` - Update password
  - **Middleware**: Requires session authentication

### 4.3 Web Handlers
- [ ] **Create web handlers.rs** 📁 `src/moduls/user/web/handlers.rs`
  - **Priority**: High
  - **Complexity**: Complex (2 hours)
  - **Dependencies**: application use cases, axum_inertia
  - **DDD Layer**: Interface Adapter
  - **Handlers**:
    ```rust
    use axum::{extract::State, response::Redirect, Form};
    use axum_inertia::Inertia;

    // GET /web/user/profile
    pub async fn show_profile(
        State(state): State<AppState>,
        auth_session: AuthSession,
    ) -> Result<InertiaResponse, AppError> {
        let use_case = GetProfileUseCase::new(state.profile_repo);
        let profile = use_case.execute(auth_session.0.user_id).await?;

        Inertia::render("User/Profile", ProfilePageProps { profile })
    }

    // GET /web/user/profile/edit
    pub async fn show_edit_profile(
        State(state): State<AppState>,
        auth_session: AuthSession,
    ) -> Result<InertiaResponse, AppError> {
        let use_case = GetProfileUseCase::new(state.profile_repo);
        let profile = use_case.execute(auth_session.0.user_id).await?;

        Inertia::render("User/EditProfile", EditProfilePageProps {
            profile,
            errors: None,
        })
    }

    // POST /web/user/profile/edit
    pub async fn handle_update_profile(
        State(state): State<AppState>,
        auth_session: AuthSession,
        Form(form): Form<UpdateProfileCommand>,
    ) -> Result<Redirect, AppError> {
        let use_case = UpdateProfileUseCase::new(state.profile_repo);
        use_case.execute(auth_session.0.user_id, form).await?;

        Ok(Redirect::to("/web/user/profile"))
    }

    // Similar for change_password handlers
    ```
  - **Auth**: Use AuthSession extractor from Phase 3

---

## 5. API Layer

### 5.1 API Module Structure
- [ ] **Create user api mod.rs** 📁 `src/moduls/user/api/mod.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (10 min)
  - **Dependencies**: None
  - **DDD Layer**: Interface Adapter (API)
  - **Content**:
    ```rust
    pub mod routes;
    pub mod handlers;

    pub use routes::user_api_routes;
    ```

### 5.2 API Routes
- [ ] **Create api routes.rs** 📁 `src/moduls/user/api/routes.rs`
  - **Priority**: Medium
  - **Complexity**: Simple (20 min)
  - **Dependencies**: handlers
  - **DDD Layer**: Interface Adapter
  - **Routes**:
    ```rust
    pub fn user_api_routes() -> Router<AppState> {
        Router::new()
            .route("/profile", get(handlers::get_profile).put(handlers::update_profile))
            .route("/password", put(handlers::change_password))
            .route_layer(middleware::jwt_layer())
    }
    ```
  - **Endpoints**:
    - `GET /api/user/profile` - Get current user profile
    - `PUT /api/user/profile` - Update profile
    - `PUT /api/user/password` - Change password
  - **Auth**: JWT middleware (from Phase 3)

### 5.3 API Handlers
- [ ] **Create api handlers.rs** 📁 `src/moduls/user/api/handlers.rs`
  - **Priority**: Medium
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: application use cases, axum::Json
  - **DDD Layer**: Interface Adapter
  - **Handlers**:
    ```rust
    use axum::{extract::State, Json};

    // GET /api/user/profile
    pub async fn get_profile(
        State(state): State<AppState>,
        AuthUser(user): AuthUser,
    ) -> Result<Json<UserProfile>, AppError> {
        let use_case = GetProfileUseCase::new(state.profile_repo);
        let profile = use_case.execute(user.id).await?;

        Ok(Json(profile))
    }

    // PUT /api/user/profile
    pub async fn update_profile(
        State(state): State<AppState>,
        AuthUser(user): AuthUser,
        Json(payload): Json<UpdateProfileCommand>,
    ) -> Result<Json<UserProfile>, AppError> {
        let use_case = UpdateProfileUseCase::new(state.profile_repo);
        let profile = use_case.execute(user.id, payload).await?;

        Ok(Json(profile))
    }

    // PUT /api/user/password
    pub async fn change_password(
        State(state): State<AppState>,
        AuthUser(user): AuthUser,
        Json(payload): Json<ChangePasswordCommand>,
    ) -> Result<Json<()>, AppError> {
        let use_case = ChangePasswordUseCase::new(state.user_repo);
        use_case.execute(user.id, payload).await?;

        Ok(Json(()))
    }
    ```
  - **Auth**: Use AuthUser extractor from Phase 3

---

## 6. Frontend Pages

### 6.1 Profile View Page
- [ ] **Create User/Profile.tsx** 📁 `resources/js/pages/User/Profile.tsx`
  - **Priority**: High
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: AppLayout, shadcn components
  - **Structure**:
    ```typescript
    import { Link } from '@inertiajs/react'
    import AppLayout from '@/layouts/AppLayout'
    import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
    import { Button } from '@/components/ui/button'

    interface UserProfile {
      user_id: string
      name: string
      email: string
      bio?: string
      avatar_url?: string
      updated_at: string
    }

    interface ProfileProps {
      profile: UserProfile
    }

    export default function Profile({ profile }: ProfileProps) {
      return (
        <AppLayout>
          <div className="max-w-2xl mx-auto space-y-6">
            <div className="flex justify-between items-center">
              <h1 className="text-3xl font-bold">Profile</h1>
              <Link href="/web/user/profile/edit">
                <Button>Edit Profile</Button>
              </Link>
            </div>

            <Card>
              <CardHeader>
                <CardTitle>Personal Information</CardTitle>
              </CardHeader>
              <CardContent className="space-y-4">
                {profile.avatar_url && (
                  <img
                    src={profile.avatar_url}
                    alt={profile.name}
                    className="w-24 h-24 rounded-full"
                  />
                )}
                <div>
                  <label className="font-semibold">Name:</label>
                  <p>{profile.name}</p>
                </div>
                <div>
                  <label className="font-semibold">Email:</label>
                  <p>{profile.email}</p>
                </div>
                {profile.bio && (
                  <div>
                    <label className="font-semibold">Bio:</label>
                    <p className="text-gray-600">{profile.bio}</p>
                  </div>
                )}
              </CardContent>
            </Card>

            <Link href="/web/user/settings/password">
              <Button variant="outline">Change Password</Button>
            </Link>
          </div>
        </AppLayout>
      )
    }
    ```

### 6.2 Edit Profile Page
- [ ] **Create User/EditProfile.tsx** 📁 `resources/js/pages/User/EditProfile.tsx`
  - **Priority**: High
  - **Complexity**: Medium (1.5 hours)
  - **Dependencies**: AppLayout, FormField
  - **Features**:
    - Name input
    - Bio textarea
    - Avatar URL input
    - Submit button
    - Cancel link
    - Error display
  - **Form**: POST to `/web/user/profile/edit`

### 6.3 Change Password Page
- [ ] **Create User/ChangePassword.tsx** 📁 `resources/js/pages/User/ChangePassword.tsx`
  - **Priority**: Medium
  - **Complexity**: Medium (1 hour)
  - **Dependencies**: AppLayout, FormField
  - **Features**:
    - Current password input
    - New password input
    - Confirm new password input
    - Submit button
    - Error display
  - **Form**: POST to `/web/user/settings/password`

---

## 7. Module Wiring

### 7.1 User Module Root
- [ ] **Create user mod.rs** 📁 `src/moduls/user/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (10 min)
  - **Dependencies**: All submodules
  - **Content**:
    ```rust
    pub mod domain;
    pub mod application;
    pub mod infra;
    pub mod web;
    pub mod api;

    pub use web::user_web_routes;
    pub use api::user_api_routes;
    ```

### 7.2 Update Modules Root
- [ ] **Update moduls mod.rs** 📁 `src/moduls/mod.rs`
  - **Priority**: High
  - **Complexity**: Simple (5 min)
  - **Dependencies**: None
  - **Addition**:
    ```rust
    pub mod auth;
    pub mod user;
    ```

### 7.3 Mount Routes in Startup
- [ ] **Update startup.rs** 📁 `src/startup.rs`
  - **Priority**: High
  - **Complexity**: Simple (20 min)
  - **Dependencies**: Requires user module complete
  - **Addition**:
    ```rust
    use crate::moduls::user::{user_web_routes, user_api_routes};

    // In build_app():
    .nest("/web/user", user_web_routes())
    .nest("/api/user", user_api_routes())
    ```

### 7.4 Update AppState
- [ ] **Add profile_repo to AppState** 📁 `src/bootstrap/app_state.rs`
  - **Priority**: High
  - **Complexity**: Simple (15 min)
  - **Dependencies**: None
  - **Addition**:
    ```rust
    use crate::moduls::user::infra::PostgresUserProfileRepository;

    pub struct AppState {
        pub db: PgPool,
        pub config: Config,
        // ... existing fields
        pub profile_repo: Arc<dyn UserProfileRepository>,
    }
    ```

---

## Validation Checklist

### Domain:
- [x] UserProfile entity has update methods ✅
- [x] Business rules enforced (name not empty, bio max 500) ✅

### Application:
- [x] Get profile use case retrieves profile ✅
- [x] Update profile validates and saves changes ✅
- [x] Change password verifies current password ✅

### Infrastructure:
- [x] Migration adds bio, avatar_url columns ✅
- [x] Repository finds and updates profiles ✅
- [x] Queries are parameterized ✅

### Web:
- [x] Profile page displays user info ✅
- [x] Edit profile form works ✅
- [x] Change password form works ✅
- [ ] All protected by session auth (TODO: Phase 7)

### API:
- [x] GET /api/user/profile returns JSON ✅
- [x] PUT /api/user/profile updates profile ✅
- [x] PUT /api/user/password changes password ✅
- [ ] All protected by JWT auth (TODO: Phase 7)

---

## Next Phase
➡️ **Phase 7: Integration & Testing** - E2E tests, cleanup jobs, optimization, deployment

## Architecture Notes

### Modular Design:
- User module is independent of Auth module (loose coupling)
- Shared infrastructure (database, error handling)
- Same DDD pattern as Auth module (consistency)

### Reusability:
- UserProfileRepository can be mocked for testing
- Use cases are framework-agnostic
- Same domain entities used in web and API

### Extension Points:
- Add more profile fields (phone, address)
- Add avatar upload (file storage)
- Add email change workflow
- Add account deletion
