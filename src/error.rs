use serde::Serialize;
use std::convert::Infallible;
use std::fmt::{Debug, Formatter};
use thiserror::Error;
use warp::{http::StatusCode, Rejection, Reply};
#[derive((Error, Debug))]
pub enum Error{
    #[error("wrong credentials")]
    WrongCredentialsError,

    #[error("jwt token not valid")]
    JWTTokenError,

    #[error("jwt token creation error")]
    JWTTokenCreationError,

    #[error("no auth header")]
    NoAuthHeaderError,

    #[error("invalid auth header")]
    InvalidAuthHeaderError,

    #[error("no permission")]
    NoPermissionError,
}

#[derive((Serialize, Debug))]
struct ErrorResponse{
    message: String,
    status: String,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "User")
    }
}

impl warp::reject::Reject for Error {}

pub async  fn handler_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found(){
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::WrongCredentialsError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::NoPermissionError => (StatusCode::UNAUTHORIZED, e.to_string()),
            Error::JWTTokenError => (StatusCode::FORBIDDEN, e.to_string()),
            Error::JWTTokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string(),
            ),
            _ => (StatusCode::BAD_REQUEST, e.to_string()),
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some(){
        (
            StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed".to_string()
            )
    } else {
        eprint!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
            )
    };

    let json = warp::reply::json(&ErrorResponse{
        status: code.to_string(),
        message
    });

    Ok(warp::reply::with_status(json, code))
}