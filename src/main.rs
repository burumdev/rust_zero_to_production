use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, startup::run};

use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let db = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;

    run(listener, db)?.await
}
