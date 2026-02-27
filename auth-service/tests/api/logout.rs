use crate::helpers::TestApp;
use auth_service::domain::email::Email;
use auth_service::utils::auth::generate_auth_cookie;
use auth_service::utils::constants::env::JWT_COOKIE_NAME;
use reqwest::Url;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let mut app = TestApp::new().await;

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let mut app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 401);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let mut app = TestApp::new().await;

    let fake_email = Email::parse(TestApp::get_random_email().into()).unwrap();

    let jwt = generate_auth_cookie(&fake_email).expect("Failed to generate auth cookie");

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{}={}; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME, jwt.value()),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let banned_token_store = app.banned_token_store.read().await;

    assert!(banned_token_store.contains_token(jwt.value().as_ref()).await.unwrap());
    drop(banned_token_store);

    app.clean_up().await;
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let mut app = TestApp::new().await;
    let fake_email = Email::parse(TestApp::get_random_email().into()).unwrap();

    let jwt = generate_auth_cookie(&fake_email).expect("Failed to generate auth cookie");

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!("{}={}; HttpOnly; SameSite=Lax; Secure; Path=/", JWT_COOKIE_NAME, jwt.value()),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);

    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 400);
    app.clean_up().await;
}
