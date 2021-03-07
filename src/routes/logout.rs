use crate::domain::SessionPool;
use crate::routes::with_session;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub fn logout(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("logout")
        .and(with_session(session_pool))
        .and_then(logout_handler)
}

#[tracing::instrument(name = "Logging-out user", skip(session_id, session_pool))]
pub async fn logout_handler(
    session_pool: Arc<Mutex<SessionPool>>,
    session_id: Option<String>,
) -> Result<impl Reply, Rejection> {
    if let Some(session_id) = session_id {
        session_pool.lock().await.stop_session(&session_id)?;
    }
    Ok(StatusCode::OK)
}
