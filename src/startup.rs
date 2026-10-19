use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

use crate::routes::{health_check::health_check, subscriptions::subscribe};

pub fn run(listener: TcpListener, db: PgPool) -> Result<Server, std::io::Error> {
    let db = web::Data::new(db);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
