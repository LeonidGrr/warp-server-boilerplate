use crate::domain::UserPassword;
use crate::errors::Errors;
use crate::routes::with_db;
use sqlx::PgPool;
use std::collections::HashMap;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

pub fn login(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_db(db_pool))
        .and_then(login_handler)
}

#[tracing::instrument(name = "Log-in existing user", skip(body, db_pool))]
pub async fn login_handler(
    body: HashMap<String, String>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    tracing::info!("Verify user credentials from data {:?}", body);
    let name = body.get(&("name".to_string()));
    let password = body.get(&("password".to_string()));

    if let (Some(name), Some(password)) = (name, password) {
        let password_hash = sqlx::query!("SELECT hash FROM users WHERE name = $1", name)
            .map(|row| UserPassword(row.hash))
            .fetch_one(&db_pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                return reject::custom(Errors::DBQueryError);
            })?;

        if !UserPassword::verify(&password_hash.0, password)? {
            return Err(reject::custom(Errors::WrongCredentials));
        }
        Ok(StatusCode::OK)
    } else {
        tracing::error!("Some fields are missing in request body: {:?}", body);
        return Err(reject::custom(Errors::MissingBodyFields));
    }
}
