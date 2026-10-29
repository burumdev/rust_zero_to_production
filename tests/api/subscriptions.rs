use crate::helpers::spawn_app;

#[tokio::test]
async fn it_subscribes_to_newsletter_with_valid_form_data() {
    let test_app = spawn_app().await;

    let body = "{\"name\":\"user\",\"email\":\"user@gmail.com\"}";
    let response = test_app.post_subscriptions(body).await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn it_persists_new_subscriber() {
    let test_app = spawn_app().await;

    let body = "{\"name\":\"user\",\"email\":\"user@gmail.com\"}";
    let response = test_app.post_subscriptions(body).await;

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name, status FROM subscriptions")
        .fetch_one(&test_app.db)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "user@gmail.com");
    assert_eq!(saved.name, "user");
    assert_eq!(saved.status, "pending_confirmation");
}

#[tokio::test]
async fn it_fails_to_subscribe_to_newsletter_with_missing_data() {
    let test_app = spawn_app().await;

    let test_cases = vec![
        ("{\"name\":\"le guin\"}", "email missing"),
        ("{\"email\":\"ursula_le_guin@gmail.com\"}", "name missing"),
        ("{}", "both email and name missing"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = test_app.post_subscriptions(invalid_body).await;

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
    let test_cases = vec![
        (
            "{\"name\":\"\",\"email\":\"tester@tester.com\"}",
            "empty name",
        ),
        ("{\"name\":\"Ursula\",\"email\":\"\"}", "empty email"),
        (
            "{\"name\":\"Ursula\",\"email\":\"no-email-for-ya\"}",
            "invalid email",
        ),
    ];

    for (body, description) in test_cases {
        let response = test_app.post_subscriptions(body).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not return a 400 when the payload was {}.",
            description
        )
    }
}
