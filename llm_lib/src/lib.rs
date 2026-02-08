//! # LLM API Client Library
//!
//! A library for interacting with LLM APIs through OpenRouter.
//!
//! ## Features
//!
//! - Support for OpenRouter API
//! - Chat completion with conversation history
//! - Configurable models and parameters
//! - Full tracing and observability support
//! - Easy error handling with `anyhow`
//!
//! ## Example Usage
//!
//! ```no_run
//! use llm_lib::{LlmClient, LlmConfig, Message, CompletionOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = LlmConfig::new("your-api-key", "anthropic/claude-3.5-sonnet");
//!     let client = LlmClient::new(config)?;
//!
//!     let response = client.complete("What is Rust?").await?;
//!     println!("Response: {}", response);
//!
//!     let response = client.complete_with_system(
//!         "You are a helpful coding assistant",
//!         "Explain async/await in Rust"
//!     ).await?;
//!     println!("Response: {}", response);
//!
//!     let messages = vec![
//!         Message::system("You are a helpful assistant"),
//!         Message::user("Hello!"),
//!         Message::assistant("Hi! How can I help you?"),
//!         Message::user("Tell me about Rust"),
//!     ];
//!
//!     let options = CompletionOptions::new()
//!         .temperature(0.7)
//!         .max_tokens(1000);
//!
//!     let response = client.chat_completion(messages, Some(options)).await?;
//!     if let Some(content) = response.content() {
//!         println!("Response: {}", content);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! ```no_run
//! use llm_lib::{LlmClient, LlmConfig};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let config = LlmConfig {
//!     api_key: "your-api-key".to_string(),
//!     model: "anthropic/claude-3.5-sonnet".to_string(),
//!     base_url: "https://openrouter.ai/api/v1".to_string(),
//!     timeout_secs: 60,
//!     site_url: Some("https://yourapp.com".to_string()),
//!     app_name: Some("Your App Name".to_string()),
//! };
//!
//! let client = LlmClient::new(config)?;
//! # Ok(())
//! # }
//! ```

mod client;
mod error;
pub mod models;

pub use client::{LlmClient, LlmConfig};
pub use error::{LlmError, Result};
pub use models::{ChatCompletionResponse, Choice, CompletionOptions, Message, Role, Usage};
