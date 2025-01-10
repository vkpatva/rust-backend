use emailapi::configuration::get_configuration;
use sqlx::PgPool ;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,    
}
//spin up an instance of our application & returns the base url of the application
async fn spawn_app() -> TestApp {
    
    // creating tcp listener to randomly allocate port 

    let listner = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listner.local_addr().unwrap().port();
    
    //creating postgress connection using PGPOOL
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(
        &configuration.database.connection_string()
    ).await.expect(
        "Failed to connect to Postgres. Please ensure the database server is running and the configuration is correct."
    );

    //starting server with connection and listner
    let server = emailapi::startup::run(listner, connection_pool.clone()).await.expect("Failed to bind address");

    let _ = tokio::spawn(server);
   let address =  format!("http://127.0.0.1:{port}");
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let test_app= spawn_app().await;
    println!("App is spawned at : {}",test_app.address);
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let health_check_route = format!("{}/health_check", &test_app.address);
    // Act
    println!("Calling health check route: {health_check_route}");
    let response = client.get(health_check_route).send().await.expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// test subscribe route with a valid form data
#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    //Arrange
    let test_app = spawn_app().await;
    println!("App is spawned at : {}", test_app.address);
    let client = reqwest::Client::new();
    // act
    let subscription_route = format!("{}/subscriptions", &test_app.address);
    println!("Calling subscription route: {subscription_route}");
    let body = "name=viraj%20patva&email=viraj%40gmail.com";
    let response = client
        .post(&subscription_route)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send().await
        .expect("Failed to execute request.");
    //Assert
    let saved: (String, String) = sqlx
        ::query_as("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db_pool).await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.0, "viraj@gmail.com");
    assert_eq!(saved.1, "viraj patva");
    assert_eq!(200, response.status().as_u16());
}

// test subscribe route with an invalid form data
#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let subscription_route = format!("{}/subscriptions", &test_app.address);
    let test_cases = vec![
        ("name=viraj%20patva", "missing the email"),
        ("email=viraj%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&subscription_route)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send().await
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
