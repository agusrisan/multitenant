use crate::config::Config;
use crate::moduls::auth::application::{
    AuthConfig, LoginUserUseCase, LogoutUserUseCase, RefreshConfig, RefreshTokenUseCase,
    RegisterUserUseCase,
};
use crate::moduls::auth::infra::{
    PostgresSessionRepository, PostgresTokenRepository, PostgresUserRepository,
};
use crate::moduls::user::application::{
    ChangePasswordUseCase, GetProfileUseCase, UpdateProfileUseCase,
};
use crate::moduls::user::infra::PostgresUserProfileRepository;
use sqlx::PgPool;
use std::sync::Arc;

/// Shared application state
///
/// This struct contains all shared resources that need to be accessible
/// across the application. It implements Clone for use with Axum's State.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub db: PgPool,

    /// Application configuration
    pub config: Config,

    /// JWT secret for signing tokens
    pub jwt_secret: String,

    /// Session secret for cookie encryption
    pub session_secret: String,

    /// CSRF secret for token generation
    pub csrf_secret: String,

    /// Repositories (exposed for direct access when needed)
    pub token_repo: Arc<PostgresTokenRepository>,

    /// Auth use cases
    pub register_user_use_case: Arc<RegisterUserUseCase>,
    pub login_user_use_case: Arc<LoginUserUseCase>,
    pub logout_user_use_case: Arc<LogoutUserUseCase>,
    pub refresh_token_use_case: Arc<RefreshTokenUseCase>,

    /// User module use cases
    pub get_profile_use_case: Arc<GetProfileUseCase>,
    pub update_profile_use_case: Arc<UpdateProfileUseCase>,
    pub change_password_use_case: Arc<ChangePasswordUseCase>,
}

impl AppState {
    /// Create a new AppState instance with repositories and use cases
    pub fn new(
        db: PgPool,
        config: Config,
        jwt_secret: String,
        session_secret: String,
        csrf_secret: String,
    ) -> Self {
        // Create repositories
        let user_repo = Arc::new(PostgresUserRepository::new(db.clone()));
        let session_repo = Arc::new(PostgresSessionRepository::new(db.clone()));
        let token_repo = Arc::new(PostgresTokenRepository::new(db.clone()));
        let profile_repo = Arc::new(PostgresUserProfileRepository::new(db.clone()));

        // Create auth config
        let auth_config = AuthConfig {
            session_ttl_seconds: config.session.expiry as i64,
            jwt_access_ttl_seconds: config.jwt.access_expiry as i64,
            jwt_refresh_ttl_seconds: config.jwt.refresh_expiry as i64,
        };

        let refresh_config = RefreshConfig {
            jwt_secret: jwt_secret.clone(),
            access_ttl_seconds: config.jwt.access_expiry as i64,
            refresh_ttl_seconds: config.jwt.refresh_expiry as i64,
        };

        // Create use cases
        let register_user_use_case = Arc::new(RegisterUserUseCase::new(user_repo.clone()));

        let login_user_use_case = Arc::new(LoginUserUseCase::new(
            user_repo.clone(),
            session_repo.clone(),
            token_repo.clone(),
            jwt_secret.clone(),
            auth_config,
        ));

        let logout_user_use_case = Arc::new(LogoutUserUseCase::new(
            session_repo.clone(),
            token_repo.clone(),
        ));

        let refresh_token_use_case = Arc::new(RefreshTokenUseCase::new(
            token_repo.clone(),
            refresh_config,
        ));

        // Create user module use cases
        let get_profile_use_case = Arc::new(GetProfileUseCase::new(profile_repo.clone()));

        let update_profile_use_case = Arc::new(UpdateProfileUseCase::new(profile_repo.clone()));

        let change_password_use_case = Arc::new(ChangePasswordUseCase::new(user_repo.clone()));

        Self {
            db,
            config,
            jwt_secret,
            session_secret,
            csrf_secret,
            token_repo,
            register_user_use_case,
            login_user_use_case,
            logout_user_use_case,
            refresh_token_use_case,
            get_profile_use_case,
            update_profile_use_case,
            change_password_use_case,
        }
    }

    /// Get database pool reference
    pub fn db(&self) -> &PgPool {
        &self.db
    }

    /// Get config reference
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_clone() {
        // AppState should implement Clone for Axum state sharing
        fn assert_clone<T: Clone>() {}
        assert_clone::<AppState>();
    }
}
