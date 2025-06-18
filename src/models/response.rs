use serde::Serialize;
use utoipa::ToSchema;

// Using a simpler implementation to avoid generic trait bound issues
#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            status: "success".to_string(),
            message: "Operation completed successfully".to_string(),
            data: Some(data),
        }
    }
}

// Special implementation for error responses that doesn't require generic type parameter
impl ApiResponse<()> {
    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: message.to_string(),
            data: None,
        }
    }
}

// Add support for using error with JsonValue (commonly used in handlers)
impl ApiResponse<serde_json::Value> {
    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: message.to_string(),
            data: None,
        }
    }
}
