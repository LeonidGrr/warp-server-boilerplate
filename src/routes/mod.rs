mod health_check;
mod login;
mod logout;
mod register;
mod update_session;

use crate::domain::{SessionPool, Session};
use health_check::*;
use login::*;
use logout::*;
use register::*;
use update_session::*;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::errors::Errors;
use warp::http::Uri;
use warp::{filters, Filter, Rejection, Reply, reject, redirect};

pub fn routes(
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    health_check(db_pool.clone())
        .or(register(db_pool.clone()))
        .or(login(db_pool, Arc::clone(&session_pool)))
        .or(logout(Arc::clone(&session_pool)))
        .or(update_session(Arc::clone(&session_pool)))
}

pub fn with_db(db_pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_session_pool(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = (Arc<Mutex<SessionPool>>, Option<String>), Error = Infallible> + Clone {
    warp::any()
        .map(move || Arc::clone(&session_pool))
        .and(filters::cookie::optional("session"))
}

pub fn with_session(
    session_pool: Arc<Mutex<SessionPool>>,
) -> filters::BoxedFilter<(Session,)> {
    warp::any()
        .and(filters::cookie::optional("session"))
        .and_then(move |session_id: Option<String>| {
            let session_pool = Arc::clone(&session_pool);
            async move {
                if let Some(session_id) = session_id {
                    if session_pool.lock().await.validate_session(&session_id) {
                        session_pool.lock().await.update_session(&session_id)?;
                        return session_pool.lock().await.get_session(&session_id);
                    }
                }
                Err(redirect::redirect(Uri::from_static("/login")).into_response())
            }
        })
        .boxed()
}
