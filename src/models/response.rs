use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T>
where
    T: Serialize,
{
    pub status: String,
    pub message: String,
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
    
    pub fn error(message: &str) -> ApiResponse<serde_json::Value> {
        ApiResponse {
            status: "error".to_string(),
            message: message.to_string(),
            data: None,
        }
    }
}
