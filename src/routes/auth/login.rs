use crate::domain::{SessionPool, UserPassword, LoginThrottling};
use crate::errors::Errors;
use crate::filters::{with_db, with_session_pool, with_login_throttling};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{http::header, http::Response, http::StatusCode, reject, Filter, Rejection, Reply};

pub fn login(
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
    login_throttling: Arc<Mutex<LoginThrottling>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_db(db_pool))
        .and(with_session_pool(session_pool))
        .and(with_login_throttling(login_throttling))
        .and_then(login_handler)
}

#[tracing::instrument(name = "Log-in existing user", skip(body, _session_id, db_pool, session_pool, login_throttling))]
pub async fn login_handler(
    body: HashMap<String, String>,
    db_pool: PgPool,
    session_pool: Arc<Mutex<SessionPool>>,
    _session_id: Option<String>,
    login_throttling: Arc<Mutex<LoginThrottling>>,
) -> Result<impl Reply, Rejection> {
    tracing::info!("Verifying user credentials: {:#?}", body);
    let name = body.get(&("name".to_string()));
    let password = body.get(&("password".to_string()));

    if let (Some(name), Some(password)) = (name, password) {
        if !login_throttling.lock().await.is_login_allowed(&name) {
            return Err(reject::custom(Errors::LoginAttemptsLimit));
        }
    
        let password_hash = sqlx::query!("SELECT hash FROM users WHERE name = $1", name)
            .map(|row| UserPassword(row.hash))
            .fetch_one(&db_pool)
            .await
            .map_err(|_| reject::custom(Errors::DBQueryError))?;

        if !UserPassword::verify(&password_hash.0, password)? {
            login_throttling.lock().await.register_failed_login_attempt(&name);
            return Err(reject::custom(Errors::WrongCredentials));
        }

        login_throttling.lock().await.reset_login_attempts(&name);
        let session_cookie = session_pool.lock().await.register_session();

        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::LOCATION, "/")
            .header(header::SET_COOKIE, session_cookie)
            .body("Success!"));
    }
    Err(reject::custom(Errors::MissingBodyFields(body)))
}
