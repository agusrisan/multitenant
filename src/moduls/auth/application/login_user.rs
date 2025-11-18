use crate::moduls::auth::domain::{Email, Session, TokenPair, UserDto};
use crate::moduls::auth::infra::{UserRepository, SessionRepository, TokenRepository};
use crate::shared::{AppError, AppResult};
use std::sync::Arc;

/// Command for web-based login (session)
#[derive(Debug, serde::Deserialize)]
pub struct LoginWebCommand {
    pub email: String,
    pub password: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

/// Command for API-based login (JWT)
#[derive(Debug, serde::Deserialize)]
pub struct LoginApiCommand {
    pub email: String,
    pub password: String,
}

/// Login result for web authentication
pub struct WebLoginResult {
    pub user: UserDto,
    pub session: Session,
}

/// Login result for API authentication
pub struct ApiLoginResult {
    pub user: UserDto,
    pub token_pair: TokenPair,
}

/// Configuration for authentication
pub struct AuthConfig {
    pub session_ttl_seconds: i64,
    pub jwt_access_ttl_seconds: i64,
    pub jwt_refresh_ttl_seconds: i64,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            session_ttl_seconds: 86400,      // 24 hours
            jwt_access_ttl_seconds: 900,     // 15 minutes
            jwt_refresh_ttl_seconds: 604800, // 7 days
        }
    }
}

/// Use case for user login (both web and API)
///
/// Supports two authentication flows:
/// 1. Web (session-based) - Returns session
/// 2. API (JWT-based) - Returns token pair
pub struct LoginUserUseCase {
    user_repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn SessionRepository>,
    token_repo: Arc<dyn TokenRepository>,
    jwt_secret: String,
    config: AuthConfig,
}

impl LoginUserUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn SessionRepository>,
        token_repo: Arc<dyn TokenRepository>,
        jwt_secret: String,
        config: AuthConfig,
    ) -> Self {
        Self {
            user_repo,
            session_repo,
            token_repo,
            jwt_secret,
            config,
        }
    }

    /// Login for web (session-based authentication)
    ///
    /// Business Logic:
    /// 1. Find user by email
    /// 2. Verify password
    /// 3. Check user is active
    /// 4. Delete existing session (single session per user)
    /// 5. Create new session
    /// 6. Return session
    ///
    /// # Arguments
    /// * `cmd` - Command containing email, password, and client info
    ///
    /// # Returns
    /// WebLoginResult with user and session
    ///
    /// # Errors
    /// - Authentication error if credentials invalid
    /// - Authentication error if user inactive
    pub async fn login_web(&self, cmd: LoginWebCommand) -> AppResult<WebLoginResult> {
        // 1. Find user by email
        let email = Email::new(&cmd.email)?;
        let user = self.user_repo.find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::authentication("Invalid email or password"))?;

        // 2. Verify password
        let password_valid = user.verify_password(&cmd.password)?;
        if !password_valid {
            return Err(AppError::authentication("Invalid email or password"));
        }

        // 3. Check user is active
        if !user.can_login() {
            return Err(AppError::authentication("Account is not active"));
        }

        // 4. Delete existing sessions (single session per user)
        self.session_repo.delete_by_user_id(user.id).await?;

        // 5. Create new session
        let session = Session::new(
            user.id,
            cmd.ip_address,
            cmd.user_agent,
            self.config.session_ttl_seconds,
        );

        let saved_session = self.session_repo.save(&session).await?;

        // 6. Return result
        Ok(WebLoginResult {
            user: UserDto::from(user),
            session: saved_session,
        })
    }

    /// Login for API (JWT-based authentication)
    ///
    /// Business Logic:
    /// 1. Find user by email
    /// 2. Verify password
    /// 3. Check user is active
    /// 4. Generate TokenPair (access + refresh)
    /// 5. Save JwtTokens to repository (for revocation tracking)
    /// 6. Return TokenPair
    ///
    /// # Arguments
    /// * `cmd` - Command containing email and password
    ///
    /// # Returns
    /// ApiLoginResult with user and token pair
    ///
    /// # Errors
    /// - Authentication error if credentials invalid
    /// - Authentication error if user inactive
    pub async fn login_api(&self, cmd: LoginApiCommand) -> AppResult<ApiLoginResult> {
        // 1. Find user by email
        let email = Email::new(&cmd.email)?;
        let user = self.user_repo.find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::authentication("Invalid email or password"))?;

        // 2. Verify password
        let password_valid = user.verify_password(&cmd.password)?;
        if !password_valid {
            return Err(AppError::authentication("Invalid email or password"));
        }

        // 3. Check user is active
        if !user.can_login() {
            return Err(AppError::authentication("Account is not active"));
        }

        // 4. Generate TokenPair
        let (token_pair, access_token, refresh_token) = TokenPair::generate(
            user.id,
            &self.jwt_secret,
            self.config.jwt_access_ttl_seconds,
            self.config.jwt_refresh_ttl_seconds,
        )?;

        // 5. Save tokens to repository (for revocation tracking)
        self.token_repo.save(&access_token).await?;
        self.token_repo.save(&refresh_token).await?;

        // 6. Return result
        Ok(ApiLoginResult {
            user: UserDto::from(user),
            token_pair,
        })
    }
}

#[cfg(test)]
mod tests {
    

    // Tests would require mock repositories
    // Skipping for brevity - similar to RegisterUser tests
}
