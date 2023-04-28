//! tests/health_check.rs

use std::net::TcpListener;

use zero2production::run;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Lunches the application in the background
fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").expect("Failed to bind random port");

    // Retrieve the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");

    // tokio::spawn returns a handle to the spawed future,
    // no need for it, so a non-binding let is used
    let _ = tokio::spawn(server);

    // Return the application address to the caller
    format!("http://0.0.0.0:{}", port)
}
