use crate::error::{LlmError, Result};
use crate::models::{
    ChatCompletionRequest, ChatCompletionResponse, CompletionOptions, ErrorResponse, Message,
};
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tracing::{debug, info, instrument, warn};

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub timeout_secs: u64,
    pub site_url: Option<String>,
    pub app_name: Option<String>,
}

impl LlmConfig {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            model: model.into(),
            timeout_secs: 120,
            site_url: None,
            app_name: None,
        }
    }
}

pub struct LlmClient {
    client: Client,
    config: LlmConfig,
}

impl LlmClient {
    #[instrument(skip(config), fields(model = %config.model))]
    pub fn new(config: LlmConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(LlmError::RequestFailed)?;

        info!("Created LLM client for model: {}", config.model);

        Ok(Self { client, config })
    }

    #[instrument(skip(self, messages, options), fields(message_count = messages.len()))]
    pub async fn chat_completion(
        &self,
        messages: Vec<Message>,
        options: Option<CompletionOptions>,
    ) -> Result<ChatCompletionResponse> {
        if messages.is_empty() {
            return Err(LlmError::InvalidRequest(
                "Messages cannot be empty".to_string(),
            ));
        }

        let request = ChatCompletionRequest {
            model: self.config.model.clone(),
            messages: messages.clone(),
            options: options.unwrap_or_default(),
        };

        debug!(
            "Sending chat completion request to {}",
            self.config.base_url
        );

        let url = format!("{}/chat/completions", self.config.base_url);
        let mut request_builder = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request);

        if let Some(site_url) = &self.config.site_url {
            request_builder = request_builder.header("HTTP-Referer", site_url);
        }
        if let Some(app_name) = &self.config.app_name {
            request_builder = request_builder.header("X-Title", app_name);
        }

        let response = request_builder.send().await?;
        let status = response.status();

        debug!("Received response with status: {}", status);

        match status {
            StatusCode::OK => {
                let completion: ChatCompletionResponse = response.json().await?;
                info!(
                    "Completion successful: {} tokens used",
                    completion.usage.total_tokens
                );
                Ok(completion)
            }
            StatusCode::UNAUTHORIZED => Err(LlmError::AuthError),
            StatusCode::TOO_MANY_REQUESTS => {
                let retry_after = response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse().ok());
                warn!("Rate limit exceeded, retry after: {:?}", retry_after);
                Err(LlmError::RateLimitExceeded { retry_after })
            }
            _ => {
                let error_body = response.text().await?;
                warn!("API error response: {}", error_body);

                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_body) {
                    Err(LlmError::ApiError {
                        status,
                        message: error_response.error.message,
                    })
                } else {
                    Err(LlmError::ApiError {
                        status,
                        message: error_body,
                    })
                }
            }
        }
    }

    #[instrument(skip(self, prompt))]
    pub async fn complete(&self, prompt: impl Into<String>) -> Result<String> {
        let message = Message::user(prompt);
        let response = self.chat_completion(vec![message], None).await?;

        response
            .content()
            .map(String::from)
            .ok_or_else(|| LlmError::InvalidRequest("No content in response".to_string()))
    }

    #[instrument(skip(self, system_prompt, user_prompt))]
    pub async fn complete_with_system(
        &self,
        system_prompt: impl Into<String>,
        user_prompt: impl Into<String>,
    ) -> Result<String> {
        let messages = vec![Message::system(system_prompt), Message::user(user_prompt)];
        let response = self.chat_completion(messages, None).await?;

        response
            .content()
            .map(String::from)
            .ok_or_else(|| LlmError::InvalidRequest("No content in response".to_string()))
    }

    pub fn model(&self) -> &str {
        &self.config.model
    }
}
