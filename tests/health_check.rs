use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app().await;
    println!("App is spawned at : {address}");
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let health_check_route = format!("{}/health_check", &address);
    // Act
    println!("Calling health check route: {health_check_route}");
    let response = client
        .get(health_check_route)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~
async fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port();
    let server = emailapi::run(listner)
        .await
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}
