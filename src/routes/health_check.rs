use crate::filters::with_db;
use sqlx::PgPool;
use warp::{http::StatusCode, Filter, Rejection, Reply};

pub fn health_check(
    db_pool: PgPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health_check")
        .and(with_db(db_pool))
        .and_then(health_check_handler)
}

pub async fn health_check_handler(_: PgPool) -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}
