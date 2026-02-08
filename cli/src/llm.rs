use anyhow::{Context, Result};
use clap::Subcommand;
use llm_lib::{CompletionOptions, LlmClient, LlmConfig, Message};
use tracing::{info, instrument};

#[derive(Subcommand)]
pub enum LlmCommands {
    /// Отправить простой запрос к LLM
    Ask {
        /// Вопрос или запрос
        prompt: String,

        /// Модель для использования (по умолчанию: anthropic/claude-3.5-sonnet)
        #[arg(short, long)]
        model: Option<String>,

        /// Temperature (0.0 - 2.0)
        #[arg(short, long)]
        temperature: Option<f32>,

        /// Максимальное количество токенов
        #[arg(long)]
        max_tokens: Option<u32>,
    },

    /// Генерировать план работы на основе задач из трекера
    PlanDay {
        /// Модель для использования
        #[arg(short, long)]
        model: Option<String>,
    },

    /// Интерактивный чат с LLM
    Chat {
        /// Системный промпт
        #[arg(short, long)]
        system: Option<String>,

        /// Модель для использования
        #[arg(short, long)]
        model: Option<String>,
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
                let client = create_client(model)?;

                info!("Отправка запроса к LLM");

                let response = if let (Some(temp), Some(tokens)) = (temperature, max_tokens) {
                    let options = CompletionOptions::new()
                        .temperature(temp)
                        .max_tokens(tokens);
                    let messages = vec![Message::user(&prompt)];
                    let completion = client.chat_completion(messages, Some(options)).await?;
                    completion
                        .content()
                        .context("No content in response")?
                        .to_string()
                } else {
                    client.complete(&prompt).await?
                };

                println!("\n{}\n", response);
                Ok(())
            }

            LlmCommands::PlanDay { model } => {
                plan_day(model).await?;
                Ok(())
            }

            LlmCommands::Chat { system, model } => {
                println!("Интерактивный чат (пока не реализован)");
                println!("System prompt: {:?}", system);
                println!("Model: {:?}", model.unwrap_or_default());
                Ok(())
            }
        }
    }
}

fn create_client(model: Option<String>) -> Result<LlmClient> {
    let api_key = std::env::var("OPEN_ROUTER_TOKEN")
        .context("OPEN_ROUTER_TOKEN environment variable not set")?;

    let model = model.unwrap_or_else(|| "anthropic/claude-3.5-sonnet".to_string());

    let config = LlmConfig {
        api_key,
        model,
        base_url: "https://openrouter.ai/api/v1".to_string(),
        timeout_secs: 120,
        site_url: Some("https://github.com/yourusername/you".to_string()),
        app_name: Some("you-cli".to_string()),
    };

    Ok(LlmClient::new(config)?)
}

#[instrument]
async fn plan_day(model: Option<String>) -> Result<()> {
    use tracker_lib::search::SearchRequest;
    use tracker_lib::TrackerClient;

    info!("Получение задач из трекера");

    // Получить задачи из трекера
    let tracker = TrackerClient::from_env()
        .context("Failed to create tracker client. Make sure TRACKER_TOKEN is set")?;

    let request = SearchRequest::default();
    let issues = tracker
        .search_issues(&request, None)
        .await
        .context("Failed to fetch issues from tracker")?;

    if issues.is_empty() {
        println!("Нет задач для планирования");
        return Ok(());
    }

    // Сформировать список задач
    let task_list = issues
        .iter()
        .map(|issue| {
            let status_display = issue
                .status
                .as_ref()
                .and_then(|s| s.display.as_deref())
                .unwrap_or("Unknown");
            format!("- {} ({}): {}", issue.key, status_display, issue.summary)
        })
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"На основе следующих задач, составь структурированный план работы на день:

{}

Требования к плану:
1. Приоритизируй задачи по важности
2. Группируй похожие задачи
3. Укажи примерное время на каждую задачу
4. Добавь рекомендации по порядку выполнения
5. Учти возможные блокеры

Формат ответа: структурированный план в формате markdown."#,
        task_list
    );

    // Получить план от LLM
    info!("Генерация плана работы");

    let client = create_client(model)?;
    let plan = client
        .complete_with_system(
            "Ты - опытный менеджер проектов, помогающий планировать рабочий день",
            &prompt,
        )
        .await
        .context("Failed to generate plan")?;

    println!("\n📋 План работы на день:\n");
    println!("{}\n", plan);

    Ok(())
}
