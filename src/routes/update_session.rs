use crate::domain::{SessionPool, Session};
use crate::routes::with_session;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::{header, Uri, StatusCode};
use warp::{redirect, Filter, Rejection, Reply, reply};

pub fn update_session(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("api")
        .and(with_session(session_pool))
        .and_then(health_check_handler)
}

// #[tracing::instrument(name = "Session update.", skip(session_pool, session_id))]
// pub async fn update_session_handler(
//     session_pool: Arc<Mutex<SessionPool>>,
//     session_id: Option<String>,
// ) -> Result<impl Reply, Rejection> {
//     if let Some(session_id) = session_id {
//         if session_pool.lock().await.validate_session(&session_id) {
//             let session_cookie = session_pool.lock().await.update_session(&session_id)?;
//             return Ok(reply::with_header(
//                 reply(),
//                 header::SET_COOKIE,
//                 session_cookie,
//             ).into_response());
//         }
//     }
//     Ok(redirect::redirect(Uri::from_static("/login")).into_response())
// }

pub async fn health_check_handler(
    session: Session,
) -> Result<impl Reply, Rejection> {
    tracing::info!("Session: {:#?}", session);
    Ok(StatusCode::OK)
}
