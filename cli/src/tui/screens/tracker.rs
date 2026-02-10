use std::{future::Future, pin::Pin};

use anyhow::Result;
use crossterm::event::KeyCode;
use tracker_lib::{task::format_issue_output, TrackerClient};

use super::{Screen, ScreenEvent};

pub struct TrackerScreen {
    input: String,
    output: Vec<String>,
}

impl TrackerScreen {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            output: vec!["Режим Tracker активирован".to_string()],
        }
    }

    fn limit_output(&mut self) {
        if self.output.len() > 200 {
            let drain_count = self.output.len().saturating_sub(200);
            self.output.drain(0..drain_count);
        }
    }
}

impl Screen for TrackerScreen {
    fn title(&self) -> &'static str {
        "Tracker"
    }

    fn input_title(&self) -> &'static str {
        "Tracker: введите ключ задачи и нажмите Enter"
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
        format!("> tracker issue {input}")
    }

    fn execute<'a>(&'a mut self, input: String) -> Pin<Box<dyn Future<Output = String> + 'a>> {
        Box::pin(async move {
            match fetch_tracker_issue(&input).await {
                Ok(output) => output,
                Err(err) => format!("Ошибка Tracker: {err}"),
            }
        })
    }
}

async fn fetch_tracker_issue(issue_id: &str) -> Result<String> {
    let client = TrackerClient::from_env()?;
    let issue = client.get_issue(issue_id, None).await?;
    Ok(format_issue_output(&issue))
}
