use rust_server_one::configuration::get_configuration;
use rust_server_one::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use warp::Filter;

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("rust_server_one".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let _connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let address: SocketAddr = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    )
    .parse()
    .expect("Unable to parse socket address");

    let hello = warp::path("hello").and(warp::get()).map(|| "Hello, World!");

    let goodbye = warp::path("goodbye")
        .and(warp::get())
        .map(|| "So long and thanks for all the fish!");

    let routes = hello.or(goodbye).with(warp::trace::request());

    warp::serve(routes).run(address).await;
}
