use std::collections::HashMap;
use std::convert::Infallible;
use warp::http::{Response, StatusCode};
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
    InvalidSession,
    LoginAttemptsLimit,
}

impl reject::Reject for Errors {}

#[allow(dead_code, unreachable_patterns)]
async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Errors>() {
        match e {
            Errors::PasswordNotValid => {
                tracing::error!("Password not meets security requirements.");
                code = StatusCode::UNAUTHORIZED;
                message = "Password not valid.";
            }
            Errors::InvalidSession => {
                code = StatusCode::UNAUTHORIZED;
                message = "Invalid session.";
            }
            Errors::WrongCredentials => {
                code = StatusCode::UNAUTHORIZED;
                message = "Wrong username or password.";
            }
            Errors::LoginAttemptsLimit => {
                code = StatusCode::UNAUTHORIZED;
                message = "Failed login attempt limit reached. Wait a minute and try again.";
            }
            Errors::UserNameNotValid(s) => {
                tracing::error!("{} is not a valid user name.", s);
                code = StatusCode::UNAUTHORIZED;
                message = "Username not valid.";
            }
            Errors::EmailNotValid(s) => {
                tracing::error!("{} is not a valid user email.", s);
                code = StatusCode::UNAUTHORIZED;
                message = "Email not valid.";
            }
            Errors::PasswordEncodeFailed(e) => {
                tracing::error!("Failed to verify password: {:#?}", e);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Failed to encode/decode password.";
            }
            Errors::MissingBodyFields(body) => {
                tracing::error!("Some fields are missing in request body: {:#?}", body);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Invalid Body";
            }
            Errors::DBQueryError => {
                tracing::error!("Failed to execute query.");
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
            Errors::SerializationError => {
                tracing::error!("Serialization error.");
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
            _ => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "UNHANDLED_REJECTION";
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    Ok(Response::builder()
        .status(code.as_u16())
        .body(message)
        .into_response())
}
