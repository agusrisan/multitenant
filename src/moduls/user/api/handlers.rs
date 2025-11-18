use crate::bootstrap::AppState;
use crate::moduls::auth::api::middleware::AuthenticatedUser;
use crate::moduls::user::application::{ChangePasswordCommand, UpdateProfileCommand};
use crate::moduls::user::domain::UserProfile;
use crate::shared::AppError;
use axum::{extract::State, Json};

/// Response for successful operations with no data
#[derive(Debug, serde::Serialize)]
pub struct EmptyResponse {
    pub message: String,
}

/// GET /api/user/profile
/// Get current user's profile (JSON)
/// Requires JWT authentication
pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserProfile>, AppError> {
    // Use the authenticated user ID from JWT claims
    let profile = state
        .get_profile_use_case
        .execute(auth_user.user_id)
        .await?;

    Ok(Json(profile))
}

/// PUT /api/user/profile
/// Update current user's profile (JSON)
/// Requires JWT authentication
pub async fn update_profile(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<UpdateProfileCommand>,
) -> Result<Json<UserProfile>, AppError> {
    // Use the authenticated user ID from JWT claims
    let profile = state
        .update_profile_use_case
        .execute(auth_user.user_id, payload)
        .await?;

    Ok(Json(profile))
}

/// PUT /api/user/password
/// Change current user's password (JSON)
/// Requires JWT authentication
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<ChangePasswordCommand>,
) -> Result<Json<EmptyResponse>, AppError> {
    // Use the authenticated user ID from JWT claims
    state
        .change_password_use_case
        .execute(auth_user.user_id, payload)
        .await?;

    Ok(Json(EmptyResponse {
        message: "Password changed successfully".to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_response_serialization() {
        let response = EmptyResponse {
            message: "Success".to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Success"));
    }
}
