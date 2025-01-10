use emailapi::configuration::get_configuration;

use emailapi::startup::run;
use sqlx::PgPool;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    // let connection = PgConnection::connect(&configuration.database.connection_string())
    //     .await
    //     .expect("failed to connect with Postgress");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection_pool).await?.await
}
