use std::net::TcpListener;

//spin up an instance of our application & returns the base url of the application
async fn spawn_app() -> String {
    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port();
    let server = emailapi::run(listner)
        .await
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{port}")
}

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

// test subscribe route with a valid form data
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let address = spawn_app().await;
    println!("App is spawned at : {address}");
    let client = reqwest::Client::new();
    let subscription_route = format!("{}/subscriptions", &address);
    println!("Calling subscription route: {subscription_route}");
    //Act
    let body = "name=viraj%20patva&email=viraj%40gmail.com";
    let response = client
        .post(&subscription_route)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");
    //Assert
    assert_eq!(200, response.status().as_u16());
}

// test subscribe route with an invalid form data
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app_address = spawn_app().await;
    let client = reqwest::Client::new();
    let subscription_route = format!("{}/subscriptions", &app_address);
    let test_cases = vec![
        ("name=viraj%20patva", "missing the email"),
        ("email=viraj%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&subscription_route)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
