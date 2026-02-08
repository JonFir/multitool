use reqwest::StatusCode;

/// Errors that can occur when working with LLM API
#[derive(Debug, thiserror::Error)]
pub enum LlmError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseFailed(#[from] serde_json::Error),

    #[error("API error: {status} - {message}")]
    ApiError { status: StatusCode, message: String },

    #[error("Authentication failed: missing or invalid API key")]
    AuthError,

    #[error("Rate limit exceeded. Retry after: {retry_after:?}")]
    RateLimitExceeded { retry_after: Option<u64> },

    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    #[error("Model not found or not available: {0}")]
    ModelNotFound(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

pub type Result<T> = std::result::Result<T, LlmError>;
