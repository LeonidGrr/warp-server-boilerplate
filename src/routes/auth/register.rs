use crate::domain::User;
use crate::errors::Errors;
use crate::filters::with_db;
use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

pub fn register(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::form())
        .and(with_db(db_pool))
        .and_then(register_handler)
}

#[tracing::instrument(name = "Saving new user in the database", skip(body, db_pool))]
pub async fn register_handler(
    body: HashMap<String, String>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    tracing::info!("Creating new user from data {:#?}", body);
    let name = body.get(&("name".to_string()));
    let email = body.get(&("email".to_string()));
    let password = body.get(&("password".to_string()));

    if let (Some(name), Some(email), Some(password)) = (name, email, password) {
        let user = User::new(name, email, password)?;

        sqlx::query!(
            r#"
        INSERT INTO users (id, email, name, created_at, hash)
        VALUES ($1, $2, $3, $4, $5)
            "#,
            Uuid::new_v4(),
            user.email.as_ref(),
            user.name.as_ref(),
            user.created_at,
            user.password.as_ref(),
        )
        .execute(&db_pool)
        .await
        .map_err(|_| reject::custom(Errors::DBQueryError))?;
        return Ok(StatusCode::OK);
    }
    Err(reject::custom(Errors::MissingBodyFields(body)))
}
