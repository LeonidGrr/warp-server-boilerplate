use serde::Serialize;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

#[derive(Debug)]
pub enum Errors {
    PasswordNotValid,
    SerializationError,
    UserNameNotValid(String),
    EmailNotValid(String),
    PasswordEncodeFailed(argon2::Error),
    WrongCredentials,
    MissingBodyFields(HashMap<String, String>),
    DBQueryError,
}

impl reject::Reject for Errors {}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

#[allow(dead_code, unreachable_patterns)]
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Errors>() {
        match e {
            Errors::PasswordNotValid => {
                tracing::error!("Password not meets security requirements.");
                code = StatusCode::BAD_REQUEST;
                message = "Password not valid.";
            }
            Errors::PasswordEncodeFailed(e) => {
                tracing::error!("Failed to verify password: {:#?}", e);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Failed to encode/decode password.";
            }
            Errors::WrongCredentials => {
                code = StatusCode::UNAUTHORIZED;
                message = "Wrong username or password.";
            }
            Errors::UserNameNotValid(s) => {
                tracing::error!("{} is not a valid user name.", s);
                code = StatusCode::BAD_REQUEST;
                message = "Username not valid.";
            }
            Errors::EmailNotValid(s) => {
                tracing::error!("{} is not a valid user email.", s);
                code = StatusCode::BAD_REQUEST;
                message = "Email not valid.";
            }
            Errors::MissingBodyFields(body) => {
                tracing::error!("Some fields are missing in request body: {:#?}", body);
                code = StatusCode::BAD_REQUEST;
                message = "Invalid Body";
            }
            Errors::DBQueryError => {
                tracing::error!("Failed to execute query.");
                code = StatusCode::BAD_REQUEST;
                message = "Internal Server Error";
            }
            Errors::SerializationError => {
                tracing::error!("Serialization error.");
                code = StatusCode::BAD_REQUEST;
                message = "Internal Server Error";
            }
            _ => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "UNHANDLED_REJECTION";
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
