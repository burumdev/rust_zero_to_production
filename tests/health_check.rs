use std::net::TcpListener;

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

fn spawn_app() -> String {
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to start TCP listener at random port");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to setup server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
