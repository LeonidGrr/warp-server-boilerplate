use crate::domain::SessionPool;
use crate::errors::Errors;
use crate::routes::with_session;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::{filters, reject, Filter, Rejection, Reply};

pub fn logout(
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("logout")
        .and(warp::filters::cookie::cookie("session"))
        .and(with_session(session_pool))
        .and_then(logout_handler)
}

#[tracing::instrument(name = "Logging-out user", skip(session_id, session_pool))]
pub async fn logout_handler(
    session_id: String,
    session_pool: Arc<Mutex<SessionPool>>,
) -> Result<impl Reply, Rejection> {
    // let session = serde_json::from_str(&session_id)
    //     .map_err(|_| reject::custom(Errors::SerializationError))?;
    println!("{}", session_id);
    let session = session_pool.lock().await.stop_session(session_id);
    Ok(StatusCode::OK)
}
