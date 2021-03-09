mod login;
mod logout;
mod register;

use crate::domain::{SessionPool, LoginThrottling};
use login::*;
use logout::*;
use register::*;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};

pub fn auth(
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
    login_throttling: Arc<Mutex<LoginThrottling>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register(db_pool.clone())
        .or(login(db_pool, Arc::clone(&session_pool), Arc::clone(&login_throttling)))
        .or(logout(Arc::clone(&session_pool)))
}
