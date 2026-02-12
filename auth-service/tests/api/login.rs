use crate::helpers::TestApp;
use auth_service::domain::data_stores::LoginAttemptId;
use auth_service::domain::email::Email;
use auth_service::routes::TwoFactorAuthResponse;
use auth_service::utils::constants::env::JWT_COOKIE_NAME;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use reqwest::StatusCode;

#[tokio::test]
async fn should_return_422_if_malformed_credentials() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
          "_email": "user@example.com",
          "_password": "string"
    });

    let response = app.post_login(&body).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn should_return_400_if_bad_request() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
          "email": "user_example.com",
          "password": "string"
    });

    let response = app.post_login(&body).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    let body = serde_json::json!({
          "email": "user@example.com",
          "password": "zaq1@WSX",
          "requires2FA": false,
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let body = serde_json::json!({
          "email": "user@example.com",
          "password": "zaq2@WSX",
    });

    let response = app.post_login(&body).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED)
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
    let app = TestApp::new().await;

    let random_email: String = SafeEmail().fake();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());
}

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email: String = SafeEmail().fake();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let mut login_body = signup_body.clone();
    login_body.as_object_mut().unwrap().remove("requires2FA");

    let response = app.post_login(&login_body).await;

    let json_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(
        app.two_fa_code_store
            .read()
            .await
            .get_code(&Email::parse(random_email).unwrap())
            .await
            .unwrap()
            .0,
        LoginAttemptId::parse(json_body.login_attempt_id).unwrap()
    );
}
