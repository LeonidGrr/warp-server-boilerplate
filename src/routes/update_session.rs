use crate::domain::SessionPool;
use crate::errors::Errors;
use crate::routes::with_session;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::{header, Response, StatusCode};
use warp::{reject, Filter, Rejection, Reply};

pub fn update_session(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::any()
        .and(with_session(session_pool))
        .and_then(update_session_handler)
}

#[tracing::instrument(name = "Session update.", skip(session_pool, session_id))]
pub async fn update_session_handler(
    session_pool: Arc<Mutex<SessionPool>>,
    session_id: Option<String>,
) -> Result<impl Reply, Rejection> {
    if let Some(session_id) = session_id {
        if session_pool.lock().await.validate_session(&session_id)? {
            let session_cookie = session_pool.lock().await.update_session(&session_id)?;

            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::SET_COOKIE, session_cookie)
                .body("Session updated!"));
        }

        session_pool.lock().await.stop_session(&session_id)?;
        return Err(reject::custom(Errors::InvalidSession));
    }
    return Err(reject::custom(Errors::InvalidSession));
}
