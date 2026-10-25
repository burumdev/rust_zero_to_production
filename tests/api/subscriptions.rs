use crate::helpers::spawn_app;

#[tokio::test]
async fn it_subscribes_to_newsletter_with_valid_form_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=burumdev&email=barisurum.works%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", test_app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute newsletter subscription request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "barisurum.works@gmail.com");
    assert_eq!(saved.name, "burumdev");
}

#[tokio::test]
async fn it_fails_to_subscribe_to_newsletter_with_missing_data() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "email missing"),
        ("email=ursula_le_guin%40gmail.com", "name missing"),
        ("", "both email and name missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", test_app.address))
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

#[tokio::test]
async fn it_does_not_subscribe_newsletter_when_fields_are_present_but_empty() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=&email=tester@tester.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=no-email-for-ya", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &test_app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 when the payload was {}.",
            description
        )
    }
}
