use serde_derive::Serialize;
use std::convert::Infallible;
use std::fmt;
use warp::reject::Reject;
use warp::{Rejection, Reply};

#[derive(Debug)]
pub enum ErrorType {
    NotFound,
    Internal,
    BadRequest,
    Webauthn,
}

#[derive(Debug)]
pub struct ApiError {
    pub err_type: ErrorType,
    pub message: String,
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

impl ApiError {
    pub fn new(message: &str, err_type: ErrorType) -> ApiError {
        ApiError {
            message: message.to_string(),
            err_type,
        }
    }

    pub fn to_http_status(&self) -> warp::http::StatusCode {
        match self.err_type {
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
            ErrorType::Webauthn => warp::http::StatusCode::UNAUTHORIZED,
        }
    }

    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> ApiError {
        ApiError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => match db_err {
                    diesel::result::DatabaseErrorKind::UniqueViolation => ErrorType::BadRequest,
                    _ => ErrorType::Internal,
                },
                diesel::result::Error::NotFound => ErrorType::NotFound,
                // Here we can handle other cases if needed
                _ => ErrorType::Internal,
            },
        )
    }

    pub fn from_webauthn_error(err: webauthn_rs::error::WebauthnError, context: &str) -> ApiError {
        ApiError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                // TODO: handle some specific error here like InvalidUsername
                _ => ErrorType::Webauthn,
            },
        )
    }
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for ApiError {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(app_err) = err.find::<ApiError>() {
        code = app_err.to_http_status();
        message = app_err.message.as_str();
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Unhandled rejection";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
