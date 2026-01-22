use crate::helpers::TestApp;

#[tokio::test]
async fn test_login() {
    let app = TestApp::new().await;

    let response =     app.http_client
        .post(&format!("{}/login", &app.address))
        .send()
        .await
        .expect("Failed to execute request (login).");

    assert_eq!(response.status().as_u16(), 200);
}




