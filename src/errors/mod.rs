use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)] // Allow unused variants as they may be used in the future
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
    DatabaseConnectionError,
    DatabaseQueryError(String),
    NotFound,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    status: String,
    error_code: Option<String>,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::InternalServerError => write!(f, "Internal server error"),
            ServiceError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
            ServiceError::DatabaseConnectionError => write!(f, "Could not connect to database"),
            ServiceError::DatabaseQueryError(msg) => write!(f, "Database error: {}", msg),
            ServiceError::NotFound => write!(f, "Resource not found"),
        }
    }
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: self.to_string(),
                    status: "error".to_string(),
                    error_code: Some("internal_server_error".to_string()),
                })
            }
            ServiceError::BadRequest(msg) => {
                HttpResponse::BadRequest().json(ErrorResponse {
                    message: msg.clone(),
                    status: "error".to_string(),
                    error_code: Some("bad_request".to_string()),
                })
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json(ErrorResponse {
                    message: self.to_string(),
                    status: "error".to_string(),
                    error_code: Some("unauthorized".to_string()),
                })
            }
            ServiceError::DatabaseConnectionError => {
                HttpResponse::ServiceUnavailable().json(ErrorResponse {
                    message: "Database service is currently unavailable".to_string(),
                    status: "error".to_string(),
                    error_code: Some("database_unavailable".to_string()),
                })
            }
            ServiceError::DatabaseQueryError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse {
                    message: "An error occurred while processing your request".to_string(),
                    status: "error".to_string(),
                    error_code: Some("database_error".to_string()),
                })
            }
            ServiceError::NotFound => {
                HttpResponse::NotFound().json(ErrorResponse {
                    message: self.to_string(),
                    status: "error".to_string(),
                    error_code: Some("not_found".to_string()),
                })
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceError::DatabaseConnectionError => StatusCode::SERVICE_UNAVAILABLE,
            ServiceError::DatabaseQueryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}
