mod api;
mod auth;
mod health_check;

use crate::domain::{SessionPool, LoginThrottling};
use api::*;
use auth::*;
use health_check::*;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

pub fn routes(
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
    login_throttling:  Arc<Mutex<LoginThrottling>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check(db_pool.clone())
        .or(auth(db_pool.clone(), Arc::clone(&session_pool), Arc::clone(&login_throttling)))
        .or(api(db_pool, Arc::clone(&session_pool)))
}
