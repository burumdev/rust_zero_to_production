use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Params {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip_all)]
pub async fn confirm(params: web::Query<Params>, db: web::Data<PgPool>) -> HttpResponse {
    let Ok(id) = get_subscriber_id_from_token(&db, &params.subscription_token).await else {
        return HttpResponse::InternalServerError().finish();
    };

    match id {
        None => HttpResponse::Unauthorized().finish(),
        Some(subscriber_id) => {
            if update_subscriber_confirmed(&db, subscriber_id)
                .await
                .is_err()
            {
                return HttpResponse::InternalServerError().finish();
            }

            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip_all)]
pub async fn update_subscriber_confirmed(
    db: &PgPool,
    subscriber_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip_all)]
pub async fn get_subscriber_id_from_token(
    db: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
        WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscriber_id))
}
