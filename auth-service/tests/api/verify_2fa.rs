use crate::helpers::TestApp;
use auth_service::domain::data_stores::{LoginAttemptId, TwoFACode};
use auth_service::domain::email::Email;
use auth_service::domain::error::ErrorResponse;
use auth_service::routes::TwoFactorAuthResponse;
use axum::http::StatusCode;
use secrecy::ExposeSecret;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let body = serde_json::json!({
          "_email": "user@example.com",
          "_loginAttemptId": "invalid",
          "_two_fa_code": "invalid",
    });

    let response = app.post_verify_2fa(&body).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let mut app = TestApp::new().await;

    let random_email = TestApp::get_random_email();
    let login_attempt_id = LoginAttemptId::default();
    let two_fa_code = TwoFACode::default();

    let test_cases = vec![
        (
            "invalid_email",
            login_attempt_id.0.expose_secret(),
            two_fa_code.0.expose_secret(),
        ),
        (
            random_email.as_str(),
            "invalid_login_attempt_id",
            two_fa_code.0.expose_secret(),
        ),
        (
            random_email.as_str(),
            login_attempt_id.0.expose_secret(),
            "invalid_two_fa_code",
        ),
        ("", "", ""),
    ];

    for (email, login_attempt_id, code) in test_cases {
        let request_body = serde_json::json!({
            "email": email,
            "loginAttemptId": login_attempt_id,
            "2FACode": code
        });

        let response = app.post_verify_2fa(&request_body).await;

        assert_eq!(response.status().as_u16(), 400, "Failed for input: {:?}", request_body);

        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let mut app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "password": "password123",
        "requires2FA":true,
        "email": random_email
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status(), 201);

    let mut login_body = signup_body.clone();
    login_body.as_object_mut().unwrap().remove("requires2FA");

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status(), 206);

    let response = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    // let t = TwoFACode::default().0.expose_secret();
    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": response.login_attempt_id.as_str(),
        "2FACode": TwoFACode::default().0.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": LoginAttemptId::default().0.expose_secret(),
        "2FACode": TwoFACode::default().0.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_if_correct_code() {
    let mut app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "password": "password123",
        "requires2FA":true,
        "email": random_email
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status(), 201);

    let mut login_body = signup_body.clone();
    login_body.as_object_mut().unwrap().remove("requires2FA");

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status(), 206);

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone().into()).unwrap())
        .await
        .unwrap();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.0.expose_secret(),
        "2FACode": two_fa_code.0.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_same_code_twice() {
    let mut app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "password": "password123",
        "requires2FA":true,
        "email": random_email
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status(), 201);

    let mut login_body = signup_body.clone();
    login_body.as_object_mut().unwrap().remove("requires2FA");

    let response = app.post_login(&login_body).await;
    assert_eq!(response.status(), 206);

    let (login_attempt_id, two_fa_code) = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone().into()).unwrap())
        .await
        .unwrap();

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.0.expose_secret(),
        "2FACode": two_fa_code.0.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 200);
    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id.0.expose_secret(),
        "2FACode": two_fa_code.0.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    let mut app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    // First login call

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123"
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    let response_body = response
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Could not deserialize response body to TwoFactorAuthResponse");

    assert_eq!(response_body.message, "2FA required".to_owned());
    assert!(!response_body.login_attempt_id.is_empty());

    let login_attempt_id = response_body.login_attempt_id;

    let code_tuple = app
        .two_fa_code_store
        .read()
        .await
        .get_code(&Email::parse(random_email.clone().into()).unwrap())
        .await
        .unwrap();

    let code = code_tuple.1.as_ref();

    // Second login call

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 206);

    // 2FA attempt with old login_attempt_id and code

    let request_body = serde_json::json!({
        "email": random_email,
        "loginAttemptId": login_attempt_id,
        "2FACode": code.expose_secret()
    });

    let response = app.post_verify_2fa(&request_body).await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}
