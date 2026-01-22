use crate::helpers::TestApp;

#[tokio::test]
async fn test_logout() {
    let app = TestApp::new().await;

    let response =     app.http_client
        .post(&format!("{}/logout", &app.address))
        .send()
        .await
        .expect("Failed to execute request (logout).");

    assert_eq!(response.status().as_u16(), 200);
}

