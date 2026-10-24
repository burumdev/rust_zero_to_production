use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::{
    email_client::EmailClient,
    routes::{health_check::health_check, subscriptions::subscribe},
};

pub fn run(
    listener: TcpListener,
    db: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);
    let email_client = web::Data::new(email_client);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
