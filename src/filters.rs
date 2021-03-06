use crate::domain::{Session, SessionPool, LoginThrottling};
use crate::errors::Errors;

use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{filters, reject, Filter};

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

pub fn with_session(session_pool: Arc<Mutex<SessionPool>>) -> filters::BoxedFilter<(Session,)> {
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
                Err(reject::custom(Errors::InvalidSession))
            }
        })
        .boxed()
}

pub fn with_login_throttling(
    login_throttling: Arc<Mutex<LoginThrottling>>,
) -> impl Filter<Extract = (Arc<Mutex<LoginThrottling>>,), Error = Infallible> + Clone {
    warp::any()
        .map(move || Arc::clone(&login_throttling))
}