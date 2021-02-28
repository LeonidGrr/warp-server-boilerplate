use crate::domain::{User, UserEmail, UserName, UserPassword};
use crate::routes::with_db;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use std::collections::HashMap;
use std::convert::TryInto;
use uuid::Uuid;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

// #[derive(Deserialize)]
// pub struct FormData {
//     pub email: String,
//     pub name: String,
//     pub password: String,
// }

// impl TryInto<User> for FormData {
//     type Error = String;

//     fn try_into(self) -> Result<User, Self::Error> {
//         let name = UserName::parse(self.name)?;
//         let email = UserEmail::parse(self.email)?;
//         let password = UserPassword {};
//         Ok(User {
//             email,
//             name,
//             password,
//         })
//     }
// }

pub fn register(db_pool: PgPool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::form())
        .and(with_db(db_pool))
        .and_then(register_handler)
}

pub async fn register_handler(
    body: HashMap<String, String>,
    db_pool: PgPool,
) -> Result<impl Reply, Rejection> {
    tracing::info!("Creating new user from data {:?}", body);
    let name = body.get(&("name".to_string()));
    let email = body.get(&("email".to_string()));
    let password = body.get(&("password".to_string()));

    if let (Some(name), Some(email), Some(password)) = (name, email, password) {

    } else {
        // return reject();
    }
    // let result = sqlx::query!("SELECT * FROM blank")
    //     .fetch_one(&db_pool)
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("Failed to execute query: {:?}", e);
    //         reject()
    //     })?;
    // tracing::info!("{:?}", result);

    // Ok(StatusCode::CREATED)
}
