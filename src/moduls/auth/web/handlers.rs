use crate::bootstrap::AppState;
use crate::moduls::auth::application::{RegisterUserCommand, LoginWebCommand};
use crate::shared::AppError;
use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;

/// Form data for web login
#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

/// Form data for web registration
#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// GET /web/auth/login
/// Show login page (Inertia)
pub async fn show_login() -> Result<String, AppError> {
    // TODO: Implement Inertia rendering
    // Inertia::render("Auth/Login", LoginPageProps { errors: None })
    Ok("Login page (Inertia not yet implemented)".to_string())
}

/// POST /web/auth/login
/// Process login form
pub async fn handle_login(
    State(state): State<AppState>,
    Json(form): Json<LoginForm>,
) -> Result<StatusCode, AppError> {
    let cmd = LoginWebCommand {
        email: form.email,
        password: form.password,
        ip_address: None, // TODO: Extract from request
        user_agent: None,  // TODO: Extract from headers
    };

    let _result = state.login_user_use_case.login_web(cmd).await?;

    // TODO: Set session cookie
    // TODO: Redirect to dashboard

    Ok(StatusCode::OK)
}

/// GET /web/auth/register
/// Show registration page (Inertia)
pub async fn show_register() -> Result<String, AppError> {
    // TODO: Implement Inertia rendering
    Ok("Register page (Inertia not yet implemented)".to_string())
}

/// POST /web/auth/register
/// Process registration form
pub async fn handle_register(
    State(state): State<AppState>,
    Json(form): Json<RegisterForm>,
) -> Result<StatusCode, AppError> {
    let cmd = RegisterUserCommand {
        email: form.email,
        password: form.password,
        name: form.name,
    };

    let _user = state.register_user_use_case.execute(cmd).await?;

    // TODO: Auto-login after registration
    // TODO: Redirect to dashboard

    Ok(StatusCode::CREATED)
}

/// POST /web/auth/logout
/// Logout user (delete session)
pub async fn handle_logout(
    State(state): State<AppState>,
    // TODO: Extract session from middleware
) -> Result<StatusCode, AppError> {
    // TODO: Get session_id from authenticated session
    // state.logout_user_use_case.logout_web(session_id).await?;

    // TODO: Clear session cookie
    // TODO: Redirect to login

    Ok(StatusCode::OK)
}
