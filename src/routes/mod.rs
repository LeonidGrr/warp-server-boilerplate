mod health_check;
mod register;
mod login;

use health_check::*;
use register::*;
use login::*;
use sqlx::PgPool;
use std::convert::Infallible;
use warp::{Filter, Rejection, Reply};

pub fn routes(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check(db_pool.clone())
        .or(register(db_pool.clone()))
        .or(login(db_pool))
}

pub fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

// let register = warp::path("register").map(|| "Hello from register");
// let login = warp::path("login").map(|| "Hello from login");
// let logout = warp::path("logout").map(|| "Hello from logout");
// let routes = register.or(login).or(logout);
