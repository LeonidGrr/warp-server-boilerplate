
use std::convert::Infallible;
use std::error::Error;
use std::num::NonZeroU16;

use serde::{Deserialize, Serialize};
use warp::http::StatusCode;
use warp::{reject, Filter, Rejection, Reply};

#[derive(Debug)]
pub enum Errors {
    PasswordNotValid,
    UserNameNotValid,
    PasswordEncodeFailed,
    WrongCredentials,
    EmailNotValid,
}

impl reject::Reject for Errors {}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

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
            PasswordNotValid => {
                code = StatusCode::BAD_REQUEST;
                message = "Password not valid.";
            },
            PasswordEncodeFailed => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Failed to encode password.";
            },
            WrongCredentials => {
                code = StatusCode::UNAUTHORIZED;
                message = "Wrong username or password.";
            },
            UserNameNotValid=> {
                code = StatusCode::BAD_REQUEST;
                message = "Username not valid.";
            },
            EmailNotValid=> {
                code = StatusCode::BAD_REQUEST;
                message = "Email not valid.";
            },
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