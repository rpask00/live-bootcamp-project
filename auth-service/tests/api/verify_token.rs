use crate::helpers::TestApp;

#[tokio::test]
async fn test_verify_token() {
    let app = TestApp::new().await;

    let response =     app.http_client
        .post(&format!("{}/verify_token", &app.address))
        .send()
        .await
        .expect("Failed to execute request (verify_token).");

    assert_eq!(response.status().as_u16(), 200);
}
