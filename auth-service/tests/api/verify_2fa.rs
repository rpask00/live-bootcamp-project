use crate::helpers::TestApp;

#[tokio::test]
async fn test_verify_2fa() {
    let app = TestApp::new().await;

    let response = app
        .http_client
        .post(&format!("{}/verify_2fa", &app.address))
        .send()
        .await
        .expect("Failed to execute request (verify_2fa).");

    assert_eq!(response.status().as_u16(), 200);
}
