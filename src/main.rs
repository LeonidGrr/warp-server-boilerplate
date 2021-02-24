use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use warp::http::{header, Method};
use warp::Filter;
use warp_server_boilerplate::configuration::get_configuration;
use warp_server_boilerplate::handlers::health_check::health_check;
use warp_server_boilerplate::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() {
    let subscriber = get_subscriber("warp-server-boilerplate".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address: SocketAddr = format!("{}:{}", configuration.application.host, configuration.application.port)
        .parse()
        .expect("Unable to parse socket address");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    let with_db = warp::any().map(move || connection_pool.clone());
    let health_check = warp::path!("health_check")
        .and(with_db)
        .and_then(health_check);
    let hello = warp::path("hello").and(warp::get()).map(|| "Hello, World!");
    let goodbye = warp::path("goodbye")
        .and(warp::get())
        .map(|| "So long and thanks for all the fish!");
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(&[header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_any_origin();
    let routes = health_check
        .or(hello)
        .or(goodbye)
        .with(cors)
        .with(warp::trace::request());

    warp::serve(routes).run(address).await;
}
