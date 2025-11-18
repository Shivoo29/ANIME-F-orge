use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
    NotFound(String),
    Conflict(String),
    ValidationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let (status, error_type) = match self {
            ApiError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
            ApiError::BadRequest(_) => (StatusCode::BAD_REQUEST, "BAD_REQUEST"),
            ApiError::Unauthorized(_) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            ApiError::NotFound(_) => (StatusCode::NOT_FOUND, "NOT_FOUND"),
            ApiError::Conflict(_) => (StatusCode::CONFLICT, "CONFLICT"),
            ApiError::ValidationError(_) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR"),
        };

        let message = match self {
            ApiError::InternalServerError(msg)
            | ApiError::BadRequest(msg)
            | ApiError::Unauthorized(msg)
            | ApiError::NotFound(msg)
            | ApiError::Conflict(msg)
            | ApiError::ValidationError(msg) => msg.clone(),
        };

        HttpResponse::build(status).json(ErrorResponse {
            error: error_type.to_string(),
            message,
        })
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => ApiError::NotFound("Resource not found".to_string()),
            sqlx::Error::Database(db_err) => {
                if let Some(constraint) = db_err.constraint() {
                    ApiError::Conflict(format!("Database constraint violation: {}", constraint))
                } else {
                    ApiError::InternalServerError(format!("Database error: {}", db_err))
                }
            }
            _ => ApiError::InternalServerError(format!("Database error: {}", error)),
        }
    }
}

impl From<bcrypt::BcryptError> for ApiError {
    fn from(error: bcrypt::BcryptError) -> Self {
        ApiError::InternalServerError(format!("Bcrypt error: {}", error))
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        ApiError::Unauthorized(format!("JWT error: {}", error))
    }
}
