use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use warp::http::{header, Method};
use warp::Filter;
use warp_server_boilerplate::configuration::get_configuration;
use warp_server_boilerplate::routes::*;
use warp_server_boilerplate::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("warp-server-boilerplate".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
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
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(&[header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_any_origin();
    let routes = routes(connection_pool)
        .with(cors)
        .with(warp::trace::request());

    warp::serve(routes).run(address).await;
}
