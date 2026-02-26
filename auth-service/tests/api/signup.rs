use crate::helpers::TestApp;
use auth_service::domain::error::ErrorResponse;
use auth_service::routes::SignupResponse;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let mut app = TestApp::new().await;

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

        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_201_if_valid_input() {
    let mut app = TestApp::new().await;

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

    assert_eq!(response_code, 201, "Succeed for input: {:?}", test_case);
    app.clean_up().await;
}

// #[tokio::test]
// async fn should_return_400_if_invalid_input() {
//     let mut app = TestApp::new().await;
//
//     let test_case = serde_json::json!({
//         "password":"password123",
//         "requires2FA":true,
//         "email": "invalid_email"
//     });
//
//     let response = app.post_signup(&test_case).await;
//     assert_eq!(response.status().as_u16(), 201);
//
//     let response = app.post_signup(&test_case).await;
//     assert_eq!(
//         response
//             .json::<ErrorResponse>()
//             .await
//             .expect("Could not deserialize response body to ErrorResponse")
//             .error,
//         "Invalid credentials.".to_owned()
//     );
// }

#[tokio::test]
async fn should_return_400_if_invalid_properties() {
    let mut app = TestApp::new().await;

    let test_case = serde_json::json!({
        "password":"password123",
        "requires2FA": true,
        "email": "invalid_email"
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 400);
    let test_case = serde_json::json!({
        "password":"abc",
        "requires2FA": true,
        "email": "valid_email@example.com"
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 400);
    app.clean_up().await;
}

#[tokio::test]
async fn should_return_409_if_email_is_used() {
    let mut app = TestApp::new().await;

    let test_case = serde_json::json!({
        "password":"password123",
        "requires2FA":true,
        "email": TestApp::get_random_email()
    });

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&test_case).await;
    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    );
    app.clean_up().await;
}
