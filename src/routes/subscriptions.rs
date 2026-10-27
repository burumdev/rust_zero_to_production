use actix_web::{HttpResponse, web};
use chrono::Utc;
use rand::{RngExt, distr::Alphanumeric};
use serde::Deserialize;
use sqlx::{Executor, PgPool, Postgres, Transaction};
use uuid::Uuid;

use crate::{
    domain::subscriber::{NewSubscriber, SubscriberName, ValidEmail},
    email_client::EmailClient,
    startup::{ApplicationBaseUrl, ApplicationPort},
};

#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = ValidEmail::parse(form.email)?;

    Ok(NewSubscriber { email, name })
}

fn generate_subscription_token() -> String {
    let mut rng = rand::rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric))
        .map(char::from)
        .take(32)
        .collect()
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip_all,
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    db: web::Data<PgPool>,
    email_client: web::Data<EmailClient>,
    base_url: web::Data<ApplicationBaseUrl>,
    port: web::Data<ApplicationPort>,
) -> HttpResponse {
    let Ok(new_subscriber) = form.0.try_into() else {
        return HttpResponse::BadRequest().finish();
    };

    let Ok(mut transaction) = db.begin().await else {
        return HttpResponse::InternalServerError().finish();
    };

    let Ok(subscriber_id) = insert_subscriber(&new_subscriber, &mut transaction).await else {
        return HttpResponse::InternalServerError().finish();
    };

    let token = generate_subscription_token();

    if insert_subscriber_confirmation(&subscriber_id, &token, &mut transaction)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    };

    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    let mut port_str = String::new();
    if port.0 != 80 && port.0 != 0 {
        port_str.push_str(&format!(":{}", port.0));
    };
    if send_confirmation_email(
        &email_client,
        new_subscriber,
        &base_url.0,
        &port_str,
        &token,
    )
    .await
    .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}

#[tracing::instrument(
    name = "Send a confirmation email to a new subscriber",
    skip(email_client, new_subscriber)
)]
pub async fn send_confirmation_email(
    email_client: &EmailClient,
    new_subscriber: NewSubscriber,
    base_url: &str,
    port: &str,
    subscription_token: &str,
) -> Result<(), reqwest::Error> {
    let confirmation_link =
        format!("{base_url}{port}/subscriptions/confirm?subscription_token={subscription_token}");

    let html_body = format!(
        "Welcome to our newsletter! Click {} to confirm your subscription.",
        confirmation_link
    );

    email_client
        .send_email(new_subscriber.email, "Welcome!", &html_body)
        .await
}

#[tracing::instrument(name = "Saving new subscriber details to the database", skip_all)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<Uuid, sqlx::Error> {
    let subscriber_id = Uuid::new_v4();
    let query = sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at, status)
            VALUES ($1, $2, $3, $4, $5)
        "#,
        subscriber_id,
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now(),
        "pending_confirmation"
    );

    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(subscriber_id)
}

#[tracing::instrument(name = "Creating new subscription confirmation record", skip_all)]
pub async fn insert_subscriber_confirmation(
    subscriber_id: &Uuid,
    token: &str,
    transaction: &mut Transaction<'_, Postgres>,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
            INSERT INTO subscription_tokens (subscription_token, subscriber_id)
            VALUES ($1, $2)
        "#,
        token,
        subscriber_id,
    );

    transaction.execute(query).await.map_err(|e| {
        tracing::error!("Failed to execute confirmation query: {:?}", e);
        e
    })?;

    Ok(())
}
