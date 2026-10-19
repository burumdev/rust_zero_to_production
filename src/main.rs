use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{configuration::get_configuration, startup::run};

use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_string = configuration.database.connection_string();
    let db = PgPool::connect(&connection_string)
        .await
        .expect("Could not connect to PG database.");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;

    run(listener, db)?.await
}
