use crate::bootstrap::AppState;
use crate::shared::AppError;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Form,
};
use serde::Deserialize;

/// Form data for profile update
#[derive(Debug, Deserialize)]
pub struct UpdateProfileForm {
    pub name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
}

/// Form data for password change
#[derive(Debug, Deserialize)]
pub struct ChangePasswordForm {
    pub current_password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

/// GET /web/user/profile
/// Show user profile page (Inertia)
pub async fn show_profile(State(_state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    // TODO: Extract user_id from authenticated session
    // For now, return placeholder
    // let use_case = GetProfileUseCase::new(state.profile_repo);
    // let profile = use_case.execute(auth_session.user_id).await?;
    // Inertia::render("User/Profile", ProfilePageProps { profile })

    Ok("Profile page (Inertia not yet implemented - TODO: extract user from session)")
}

/// GET /web/user/profile/edit
/// Show edit profile form (Inertia)
pub async fn show_edit_profile(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Extract user_id from authenticated session
    // let use_case = GetProfileUseCase::new(state.profile_repo);
    // let profile = use_case.execute(auth_session.user_id).await?;
    // Inertia::render("User/EditProfile", EditProfilePageProps { profile, errors: None })

    Ok("Edit profile page (Inertia not yet implemented - TODO: extract user from session)")
}

/// POST /web/user/profile/edit
/// Handle profile update form submission
pub async fn handle_update_profile(
    State(_state): State<AppState>,
    Form(_form): Form<UpdateProfileForm>,
) -> Result<Redirect, AppError> {
    // TODO: Extract user_id from authenticated session
    // For now, return error
    /*
    let cmd = UpdateProfileCommand {
        name: form.name,
        bio: form.bio,
        avatar_url: form.avatar_url,
    };

    let use_case = UpdateProfileUseCase::new(state.profile_repo);
    use_case.execute(auth_session.user_id, cmd).await?;

    Ok(Redirect::to("/web/user/profile"))
    */

    Err(AppError::Authentication(
        "Session authentication not yet implemented".into(),
    ))
}

/// GET /web/user/settings/password
/// Show change password form (Inertia)
pub async fn show_change_password(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // TODO: Extract user_id from authenticated session
    // Inertia::render("User/ChangePassword", ChangePasswordPageProps { errors: None })

    Ok("Change password page (Inertia not yet implemented - TODO: extract user from session)")
}

/// POST /web/user/settings/password
/// Handle password change form submission
pub async fn handle_change_password(
    State(_state): State<AppState>,
    Form(_form): Form<ChangePasswordForm>,
) -> Result<Redirect, AppError> {
    // TODO: Extract user_id from authenticated session
    // For now, return error
    /*
    let cmd = ChangePasswordCommand {
        current_password: form.current_password,
        new_password: form.new_password,
        new_password_confirmation: form.new_password_confirmation,
    };

    let use_case = ChangePasswordUseCase::new(state.change_password_use_case.clone());
    use_case.execute(auth_session.user_id, cmd).await?;

    // TODO: Show success message
    Ok(Redirect::to("/web/user/profile"))
    */

    Err(AppError::Authentication(
        "Session authentication not yet implemented".into(),
    ))
}
