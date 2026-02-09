use anyhow::{Context, Result};
use clap::Subcommand;
use llm_lib::{CompletionOptions, LlmClient, LlmClientTrait, LlmConfig, Message};
use tracing::{info, instrument};

#[derive(Subcommand)]
pub enum LlmCommands {
    Ask {
        prompt: String,

        #[arg(short, long)]
        model: Option<String>,

        #[arg(short, long)]
        temperature: Option<f32>,

        #[arg(long)]
        max_tokens: Option<u32>,
    },
}

impl LlmCommands {
    #[instrument(skip(self))]
    pub async fn execute(self) -> Result<()> {
        match self {
            LlmCommands::Ask {
                prompt,
                model,
                temperature,
                max_tokens,
            } => {
                let model = model.unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string());
                let config = LlmConfig::new(model)?;
                let client = LlmClient::new(config)?;
                let response = ask(&client, &prompt, temperature, max_tokens).await?;
                println!("\n{}\n", response);
                Ok(())
            }
        }
    }
}

#[instrument(skip(client))]
async fn ask<T: LlmClientTrait>(
    client: &T,
    prompt: &str,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
) -> Result<String> {
    info!("Sending request to LLM");

    let response = if let (Some(temp), Some(tokens)) = (temperature, max_tokens) {
        let options = CompletionOptions::new()
            .temperature(temp)
            .max_tokens(tokens);
        let messages = vec![Message::user(prompt.to_string())];
        let completion = client.chat_completion(messages, Some(options)).await?;
        completion
            .content()
            .context("No content in response")?
            .to_string()
    } else {
        client.complete(prompt.to_string()).await?
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use llm_lib::MockLlmClientTrait;

    #[tokio::test]
    async fn test_ask_with_minimal_arguments() {
        let mut mock_client = MockLlmClientTrait::new();
        mock_client
            .expect_complete()
            .times(1)
            .returning(|_| Box::pin(async { Ok("Hello, world!".to_string()) }));

        let result = ask(&mock_client, "test prompt", None, None).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }

    #[tokio::test]
    async fn test_ask_with_maximum_arguments() {
        let mut mock_client = MockLlmClientTrait::new();
        mock_client
            .expect_chat_completion()
            .times(1)
            .returning(|_, _| {
                Box::pin(async {
                    Ok(llm_lib::ChatCompletionResponse {
                        id: "test-id".to_string(),
                        model: "test-model".to_string(),
                        choices: vec![llm_lib::Choice {
                            index: 0,
                            message: Message::assistant("Response with options"),
                            finish_reason: Some("stop".to_string()),
                        }],
                        usage: llm_lib::Usage {
                            prompt_tokens: 10,
                            completion_tokens: 20,
                            total_tokens: 30,
                        },
                        created: 1234567890,
                    })
                })
            });

        let result = ask(&mock_client, "test prompt", Some(0.7), Some(150)).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Response with options");
    }
}
