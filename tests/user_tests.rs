mod common;

use common::TestApp;

/// Helper function to register and get access token
async fn register_and_login(app: &TestApp) -> String {
    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "user@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 201);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    body["access_token"]
        .as_str()
        .expect("access_token should be a string")
        .to_string()
}

#[tokio::test]
async fn test_get_profile_success() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Get profile
    let response = app
        .client
        .get(&format!("{}/api/user/profile", app.address))
        .bearer_auth(&access_token)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["name"], "Test User");
    assert_eq!(body["email"], "user@example.com");
    assert!(body["id"].is_string());

    app.cleanup().await;
}

#[tokio::test]
async fn test_get_profile_unauthorized() {
    let app = TestApp::spawn().await;

    // Try to get profile without authentication
    let response = app.get("/api/user/profile").await;

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}

#[tokio::test]
async fn test_update_profile_success() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Update profile
    let response = app
        .client
        .put(&format!("{}/api/user/profile", app.address))
        .bearer_auth(&access_token)
        .json(&serde_json::json!({
            "name": "Updated Name",
            "bio": "This is my bio",
            "avatar_url": "https://example.com/avatar.jpg"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200, "Expected 200 OK");

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["name"], "Updated Name");
    assert_eq!(body["bio"], "This is my bio");
    assert_eq!(body["avatar_url"], "https://example.com/avatar.jpg");

    app.cleanup().await;
}

#[tokio::test]
async fn test_update_profile_invalid_name() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Try to update with empty name
    let response = app
        .client
        .put(&format!("{}/api/user/profile", app.address))
        .bearer_auth(&access_token)
        .json(&serde_json::json!({
            "name": "",
            "bio": "Bio"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 400, "Expected 400 Bad Request");

    app.cleanup().await;
}

#[tokio::test]
async fn test_change_password_success() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Change password
    let response = app
        .client
        .put(&format!("{}/api/user/password", app.address))
        .bearer_auth(&access_token)
        .json(&serde_json::json!({
            "current_password": "SecurePassword123!",
            "new_password": "NewSecurePassword456!"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200, "Expected 200 OK");

    // Try to login with new password
    let login_response = app
        .post_json(
            "/api/auth/login",
            &serde_json::json!({
                "email": "user@example.com",
                "password": "NewSecurePassword456!"
            }),
        )
        .await;

    assert_eq!(login_response.status(), 200, "Should be able to login with new password");

    app.cleanup().await;
}

#[tokio::test]
async fn test_change_password_wrong_current() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Try to change password with wrong current password
    let response = app
        .client
        .put(&format!("{}/api/user/password", app.address))
        .bearer_auth(&access_token)
        .json(&serde_json::json!({
            "current_password": "WrongPassword123!",
            "new_password": "NewSecurePassword456!"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}

#[tokio::test]
async fn test_change_password_weak_new_password() {
    let app = TestApp::spawn().await;
    let access_token = register_and_login(&app).await;

    // Try to change to weak password
    let response = app
        .client
        .put(&format!("{}/api/user/password", app.address))
        .bearer_auth(&access_token)
        .json(&serde_json::json!({
            "current_password": "SecurePassword123!",
            "new_password": "weak"
        }))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 400, "Expected 400 Bad Request");

    app.cleanup().await;
}

#[tokio::test]
async fn test_change_password_unauthorized() {
    let app = TestApp::spawn().await;

    // Try to change password without authentication
    let response = app
        .put_json(
            "/api/user/password",
            &serde_json::json!({
                "current_password": "SecurePassword123!",
                "new_password": "NewSecurePassword456!"
            }),
        )
        .await;

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}
