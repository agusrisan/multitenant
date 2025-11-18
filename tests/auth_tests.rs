mod common;

use common::TestApp;

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_health_check() {
    let app = TestApp::spawn().await;

    let response = app.get("/health").await;

    assert_eq!(response.status(), 200);

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["database"], "connected");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_register_success() {
    let app = TestApp::spawn().await;

    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "test@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 201, "Expected 201 Created");

    let body: serde_json::Value = response.json().await.expect("Failed to parse response");
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_register_duplicate_email() {
    let app = TestApp::spawn().await;

    // First registration - should succeed
    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "duplicate@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 201);

    // Second registration with same email - should fail
    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Another User",
                "email": "duplicate@example.com",
                "password": "AnotherPassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 409, "Expected 409 Conflict");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_register_invalid_email() {
    let app = TestApp::spawn().await;

    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "invalid-email",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 400, "Expected 400 Bad Request");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_register_weak_password() {
    let app = TestApp::spawn().await;

    let response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "test@example.com",
                "password": "weak"
            }),
        )
        .await;

    assert_eq!(response.status(), 400, "Expected 400 Bad Request");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_login_success() {
    let app = TestApp::spawn().await;

    // First, register a user
    let register_response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "login@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(register_response.status(), 201);

    // Now login with the same credentials
    let login_response = app
        .post_json(
            "/api/auth/login",
            &serde_json::json!({
                "email": "login@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(login_response.status(), 200, "Expected 200 OK");

    let body: serde_json::Value = login_response.json().await.expect("Failed to parse response");
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_login_invalid_email() {
    let app = TestApp::spawn().await;

    let response = app
        .post_json(
            "/api/auth/login",
            &serde_json::json!({
                "email": "nonexistent@example.com",
                "password": "SomePassword123!"
            }),
        )
        .await;

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_login_invalid_password() {
    let app = TestApp::spawn().await;

    // Register a user
    let register_response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "wrongpass@example.com",
                "password": "CorrectPassword123!"
            }),
        )
        .await;

    assert_eq!(register_response.status(), 201);

    // Try to login with wrong password
    let login_response = app
        .post_json(
            "/api/auth/login",
            &serde_json::json!({
                "email": "wrongpass@example.com",
                "password": "WrongPassword123!"
            }),
        )
        .await;

    assert_eq!(login_response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_refresh_token() {
    let app = TestApp::spawn().await;

    // Register a user
    let register_response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "refresh@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(register_response.status(), 201);

    let register_body: serde_json::Value = register_response
        .json()
        .await
        .expect("Failed to parse register response");

    let refresh_token = register_body["refresh_token"]
        .as_str()
        .expect("refresh_token should be a string");

    // Use refresh token to get new access token
    let refresh_response = app
        .post_json(
            "/api/auth/refresh",
            &serde_json::json!({
                "refresh_token": refresh_token
            }),
        )
        .await;

    assert_eq!(refresh_response.status(), 200, "Expected 200 OK");

    let refresh_body: serde_json::Value = refresh_response
        .json()
        .await
        .expect("Failed to parse refresh response");

    assert!(refresh_body["access_token"].is_string());
    assert!(refresh_body["refresh_token"].is_string());

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_refresh_token_invalid() {
    let app = TestApp::spawn().await;

    let response = app
        .post_json(
            "/api/auth/refresh",
            &serde_json::json!({
                "refresh_token": "invalid_token_here"
            }),
        )
        .await;

    assert_eq!(response.status(), 401, "Expected 401 Unauthorized");

    app.cleanup().await;
}

#[tokio::test]
#[ignore = "integration test requires database and --test-threads=1"]
async fn test_logout_success() {
    let app = TestApp::spawn().await;

    // Register and login
    let register_response = app
        .post_json(
            "/api/auth/register",
            &serde_json::json!({
                "name": "Test User",
                "email": "logout@example.com",
                "password": "SecurePassword123!"
            }),
        )
        .await;

    assert_eq!(register_response.status(), 201);

    let register_body: serde_json::Value = register_response
        .json()
        .await
        .expect("Failed to parse response");

    let access_token = register_body["access_token"]
        .as_str()
        .expect("access_token should be a string");

    // Logout
    let logout_response = app
        .client
        .post(&format!("{}/api/auth/logout", app.address))
        .bearer_auth(access_token)
        .send()
        .await
        .expect("Failed to execute logout request");

    assert_eq!(logout_response.status(), 204, "Expected 204 No Content");

    app.cleanup().await;
}
