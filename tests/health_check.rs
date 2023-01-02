#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app();
    // We need to bring in 'reqwest'
    // To perform HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//Launch application in the background somehow
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    // Launch the server as a background task
    let _ = tokio::spawn(server);
}
