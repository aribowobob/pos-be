use std::fmt;

// Add #[allow(dead_code)] to suppress the warning
#[allow(dead_code)]
#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    Unauthorized,
    Forbidden,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            ServiceError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
            ServiceError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
            ServiceError::Forbidden => write!(f, "Forbidden"),
        }
    }
}
