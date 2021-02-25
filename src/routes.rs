use crate::handlers::*;
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn routes(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check(db_pool.clone())
}

fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

fn health_check(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("health_check")
        .and(warp::get())
        .and(with_db(db_pool))
        .and_then(health_check_handler)
}
