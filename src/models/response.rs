use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: String,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            data: Some(data),
            message: "success".to_string(),
            error: None,
        }
    }

    pub fn error(error_message: &str) -> Self {
        Self {
            data: None,
            message: "error".to_string(),
            error: Some(error_message.to_string()),
        }
    }
}
