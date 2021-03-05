mod health_check;
mod login;
mod register;

use crate::domain::SessionPool;
use health_check::*;
use login::*;
use register::*;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check(db_pool.clone())
        .or(register(db_pool.clone()))
        .or(login(db_pool, Arc::clone(&session_pool)))
}

pub fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_session(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = (Arc<Mutex<SessionPool>>,), Error = Infallible> + Clone {
    warp::any().map(move || Arc::clone(&session_pool))
}
