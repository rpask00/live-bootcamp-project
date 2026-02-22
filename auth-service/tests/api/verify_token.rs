use crate::helpers::TestApp;
use auth_service::domain::email::Email;
use auth_service::utils::auth::generate_auth_cookie;
use serde_json::json;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

    let response = app
        .post_verify_token(&json!({
            "_token": "secret"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 422);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let mut app = TestApp::new().await;

    let jwt = generate_auth_cookie(&Email::parse(TestApp::get_random_email()).unwrap())
        .expect("Failed to generate auth cookie");

    let response = app
        .post_verify_token(&json!({
            "token": jwt.value()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    let response = app
        .post_verify_token(&json!({
            "token": "invalid_token"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_banned_token() {
    let mut app = TestApp::new().await;

    let jwt = generate_auth_cookie(&Email::parse(TestApp::get_random_email()).unwrap())
        .expect("Failed to generate auth cookie");

    let response = app
        .post_verify_token(&json!({
            "token": jwt.value()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);

    app.banned_token_store
        .write()
        .await
        .add_token(jwt.value().to_owned())
        .await
        .unwrap();

    assert!(app
        .banned_token_store
        .read()
        .await
        .contains_token(jwt.value().as_ref())
        .await
        .unwrap());

    let response = app
        .post_verify_token(&json!({
            "token": jwt.value()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}
