use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}

impl<T> ApiResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        ApiResponse {
            message: message.into(),
            error: None,
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>, error: Option<ErrorDetails>) -> Self {
        ApiResponse {
            message: message.into(),
            error,
            data: None,
        }
    }
}
