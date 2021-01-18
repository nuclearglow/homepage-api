use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ErrorType {
    NotFound,
    Internal,
    BadRequest,
}

#[derive(Debug)]
pub struct ApiError {
    pub err_type: ErrorType,
    pub message: String,
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
}

impl std::error::Error for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for ApiError {}
