use crate::domain::{Session, SessionPool};
use crate::filters::with_session;
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

pub fn api(
    _db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("api")
        .and(with_session(session_pool))
        .and_then(health_check_handler)
}

pub async fn health_check_handler(session: Session) -> Result<impl Reply, Rejection> {
    tracing::info!("{:#?}", session);
    Ok(StatusCode::OK)
}
