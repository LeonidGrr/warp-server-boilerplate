use crate::errors::Errors;
use crate::routes::with_db;
use sqlx::PgPool;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

pub fn health_check(
    db_pool: PgPool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health_check")
        .and(warp::get())
        .and(with_db(db_pool))
        .and_then(health_check_handler)
}

pub async fn health_check_handler(db_pool: PgPool) -> Result<impl Reply, Rejection> {
    let result = sqlx::query!("SELECT * FROM blank")
        .fetch_one(&db_pool)
        .await
        .map_err(|e| reject::custom(Errors::DBQueryError(e)))?;
    tracing::info!("{:?}", result);

    Ok(StatusCode::OK)
}
