use crate::bootstrap::AppState;
use crate::moduls::user::application::{
    ChangePasswordCommand, ChangePasswordUseCase, GetProfileUseCase, UpdateProfileCommand,
    UpdateProfileUseCase,
};
use crate::moduls::user::domain::UserProfile;
use crate::shared::{types::UserId, AppError};
use axum::{extract::State, Json};

/// Response for successful operations with no data
#[derive(Debug, serde::Serialize)]
pub struct EmptyResponse {
    pub message: String,
}

/// GET /api/user/profile
/// Get current user's profile (JSON)
pub async fn get_profile(
    State(state): State<AppState>,
    // TODO: Extract user from JWT token
    // AuthUser(user): AuthUser,
) -> Result<Json<UserProfile>, AppError> {
    // TODO: Extract user_id from JWT claims
    // let use_case = GetProfileUseCase::new(state.profile_repo);
    // let profile = use_case.execute(user.id).await?;
    // Ok(Json(profile))

    Err(AppError::Authentication(
        "JWT authentication not yet implemented".into(),
    ))
}

/// PUT /api/user/profile
/// Update current user's profile (JSON)
pub async fn update_profile(
    State(state): State<AppState>,
    // TODO: Extract user from JWT token
    // AuthUser(user): AuthUser,
    Json(payload): Json<UpdateProfileCommand>,
) -> Result<Json<UserProfile>, AppError> {
    // TODO: Extract user_id from JWT claims
    // let use_case = UpdateProfileUseCase::new(state.profile_repo);
    // let profile = use_case.execute(user.id, payload).await?;
    // Ok(Json(profile))

    Err(AppError::Authentication(
        "JWT authentication not yet implemented".into(),
    ))
}

/// PUT /api/user/password
/// Change current user's password (JSON)
pub async fn change_password(
    State(state): State<AppState>,
    // TODO: Extract user from JWT token
    // AuthUser(user): AuthUser,
    Json(payload): Json<ChangePasswordCommand>,
) -> Result<Json<EmptyResponse>, AppError> {
    // TODO: Extract user_id from JWT claims
    // let use_case = ChangePasswordUseCase::new(state.user_repo);
    // use_case.execute(user.id, payload).await?;
    //
    // Ok(Json(EmptyResponse {
    //     message: "Password changed successfully".to_string(),
    // }))

    Err(AppError::Authentication(
        "JWT authentication not yet implemented".into(),
    ))
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
