use crate::helpers::TestApp;
use auth_service::routes::{SignupRequest, SignupResponse};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "_password":"password123",
            "requires2FA":true,
            "email": random_email
        }),
        serde_json::json!({
            "password":"password123",
            "_requires2FA":true,
            "email": random_email
        }),
        serde_json::json!({
            "password":"password123",
            "requires2FA":true,
            "_+email": random_email
        }),
    ];

    for test_case in test_cases {
        let response = app.post_signup(&test_case).await;

        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let test_case = serde_json::json!({
        "password":"password123",
        "requires2FA":true,
        "email": random_email
    });

    let expected_response = SignupResponse {
        message: "User signed up successfully".into(),
    };

    let response = app.post_signup(&test_case).await;
    let response_code = response.status().as_u16();

    assert_eq!(
        response
            .json::<SignupResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response,
        "Succeed for input: {:?}",
        test_case
    );

    assert_eq!(
        response_code,
        201,
        "Succeed for input: {:?}",
        test_case
    )
}
