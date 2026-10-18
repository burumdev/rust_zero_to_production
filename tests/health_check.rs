use std::net::TcpListener;

fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to start TCP listener at random port");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to setup server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn it_handles_health_check_request() {
    let base_url = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{base_url}/health_check"))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn it_subscribes_to_newsletter_with_valid_form_data() {
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=burumdev&email=barisurum.works%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", base_url))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute newsletter subscription request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn it_fails_to_subscribe_to_newsletter_with_missing_data() {
    let base_url = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "email missing"),
        ("email=ursula_le_guin%40gmail.com", "name missing"),
        ("", "both email and name missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", base_url))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute newsletter subscription request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API should have failed with 400 Bad Request, but the response was different when payload had {}.",
            error_message
        );
    }
}
