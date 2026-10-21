use std::{net::TcpListener, sync::LazyLock};

use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use zero2prod::{
    configuration::{DatabaseConfiguration, get_configuration},
    telemetry::{get_subscriber, init_subscriber},
};

pub struct TestApp {
    pub address: String,
    pub db: PgPool,
}

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to start TCP listener at random port");

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();

    let db = configure_database(&configuration.database).await;

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::startup::run(listener, db.clone()).expect("Failed to setup server");

    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db,
    }
}

pub async fn configure_database(config: &DatabaseConfiguration) -> PgPool {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");

    let query = format!(r#"CREATE DATABASE "{}";"#, config.database_name);

    sqlx::query(sqlx::AssertSqlSafe(query.as_str()))
        .execute(&mut connection)
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection)
        .await
        .expect("Failed to migrate the database");

    connection
}

#[tokio::test]
async fn it_handles_health_check_request() {
    let test_app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute health check request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

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
