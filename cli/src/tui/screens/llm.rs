use std::{future::Future, pin::Pin};

use crossterm::event::KeyCode;
use llm_lib::{LlmClient, LlmClientTrait, LlmConfig};

use super::{Screen, ScreenEvent};

pub struct LlmScreen {
    input: String,
    output: Vec<String>,
}

impl LlmScreen {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: vec!["Режим LLM активирован".to_string()],
        }
    }

    fn limit_output(&mut self) {
        if self.output.len() > 200 {
            let drain_count = self.output.len().saturating_sub(200);
            self.output.drain(0..drain_count);
        }
    }
}

impl Screen for LlmScreen {
    fn title(&self) -> &'static str {
        "LLM"
    }

    fn input_title(&self) -> &'static str {
        "LLM: введите промпт и нажмите Enter"
    }

    fn input_text(&self) -> &str {
        &self.input
    }

    fn output_text(&self) -> String {
        self.output.join("\n\n")
    }

    fn handle_key(&mut self, key: KeyCode) -> ScreenEvent {
        match key {
            KeyCode::Backspace => {
                self.input.pop();
                ScreenEvent::None
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                ScreenEvent::None
            }
            KeyCode::Enter => {
                let input = self.input.trim().to_string();
                if input.is_empty() {
                    return ScreenEvent::None;
                }
                self.input.clear();
                ScreenEvent::Submit(input)
            }
            _ => ScreenEvent::None,
        }
    }

    fn push_output(&mut self, text: String) {
        self.output.push(text);
        self.limit_output();
    }

    fn command_preview(&self, input: &str) -> String {
        format!("> llm ask {input}")
    }

    fn execute<'a>(&'a mut self, input: String) -> Pin<Box<dyn Future<Output = String> + 'a>> {
        Box::pin(async move {
            match ask_llm(&input).await {
                Ok(output) => output,
                Err(err) => format!("Ошибка LLM: {err}"),
            }
        })
    }
}

async fn ask_llm(prompt: &str) -> anyhow::Result<String> {
    let config = LlmConfig::new("anthropic/claude-3.5-sonnet".to_string())?;
    let client = LlmClient::new(config)?;
    let response = client.complete(prompt.to_string()).await?;
    Ok(response)
}
