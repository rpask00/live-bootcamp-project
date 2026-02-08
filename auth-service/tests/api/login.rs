use crate::helpers::TestApp;
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
