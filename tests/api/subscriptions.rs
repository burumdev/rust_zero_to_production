use crate::helpers::spawn_app;

#[tokio::test]
async fn it_subscribes_to_newsletter_with_valid_form_data() {
    let test_app = spawn_app().await;

    let body = "name=burumdev&email=barisurum.works%40gmail.com";
    let response = test_app.post_subscriptions(body).await;

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

    let test_cases = vec![
        ("name=le%20guin", "email missing"),
        ("email=ursula_le_guin%40gmail.com", "name missing"),
        ("", "both email and name missing"),
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
        ("name=&email=tester@tester.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=no-email-for-ya", "invalid email"),
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

#[tokio::test]
async fn it_sends_a_confirmation_email_with_valid_subscription() {
    let test_app = spawn_app().await;
    let body = "name=burumdev&email=barisurum.works@gmail.com";

    let response = test_app.post_subscriptions(body.into()).await;

    assert_eq!(
        200,
        response.status().as_u16(),
        "The API did not return a 200 when payload was valid"
    )
}
