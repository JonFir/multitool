# Примеры использования

## LLM Library

### Базовое использование

```rust
use llm_lib::{LlmClient, LlmConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Простой запрос
    let client = LlmClient::with_token(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    )?;

    let response = client.complete("Что такое Rust?").await?;
    println!("{}", response);

    Ok(())
}
```

### Расширенная конфигурация

```rust
use llm_lib::{LlmClient, LlmConfig, CompletionOptions, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создание клиента с настройками
    let config = LlmConfig::new(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    )
    .with_timeout(60)
    .with_site_url("https://yourapp.com")
    .with_app_name("Your App");

    let client = LlmClient::new(config)?;

    // Многошаговый диалог
    let messages = vec![
        Message::system("Ты - эксперт по Rust"),
        Message::user("Объясни ownership"),
    ];

    let options = CompletionOptions::new()
        .temperature(0.7)
        .max_tokens(1000);

    let response = client.chat_completion(messages, Some(options)).await?;

    if let Some(content) = response.content() {
        println!("{}", content);
        println!("Использовано токенов: {}", response.usage.total_tokens);
    }

    Ok(())
}
```

### Обработка ошибок

```rust
use llm_lib::{LlmClient, LlmError};

async fn handle_request() {
    let client = LlmClient::with_token("api-key", "model").unwrap();

    match client.complete("test").await {
        Ok(response) => println!("Ответ: {}", response),
        Err(LlmError::AuthError) => eprintln!("Неверный API ключ"),
        Err(LlmError::RateLimitExceeded { retry_after }) => {
            eprintln!("Превышен лимит, повторить через: {:?}", retry_after);
        }
        Err(LlmError::ApiError { status, message }) => {
            eprintln!("Ошибка API {}: {}", status, message);
        }
        Err(e) => eprintln!("Ошибка: {}", e),
    }
}
```

## Комбинирование tracker_lib и llm_lib

### Генерация отчета по задачам

```rust
use tracker_lib::{TrackerClient, search::SearchRequest};
use llm_lib::LlmClient;
use anyhow::Result;

async fn generate_task_report() -> Result<String> {
    // Получить задачи
    let tracker = TrackerClient::from_env()?;
    let request = SearchRequest::default();
    let issues = tracker.search_issues(&request, None).await?;

    // Сформировать описание задач
    let tasks_description = issues
        .iter()
        .map(|issue| format!("- {}: {}", issue.key, issue.summary))
        .collect::<Vec<_>>()
        .join("\n");

    // Сгенерировать отчет с помощью LLM
    let llm = LlmClient::with_token(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    )?;

    let prompt = format!(
        "Создай краткий отчет по следующим задачам:\n\n{}",
        tasks_description
    );

    let report = llm.complete_with_system(
        "Ты - менеджер проектов, создающий отчеты",
        prompt
    ).await?;

    Ok(report)
}
```

### Автоматическая приоритизация задач

```rust
use tracker_lib::{TrackerClient, search::SearchRequest};
use llm_lib::{LlmClient, Message};
use anyhow::Result;

async fn prioritize_tasks() -> Result<Vec<String>> {
    // Получить задачи
    let tracker = TrackerClient::from_env()?;
    let request = SearchRequest::default();
    let issues = tracker.search_issues(&request, None).await?;

    // Подготовить данные
    let tasks_json = serde_json::to_string_pretty(&issues)?;

    // Запросить приоритизацию у LLM
    let llm = LlmClient::with_token(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    )?;

    let messages = vec![
        Message::system("Ты - эксперт по планированию задач. Отвечай только JSON массивом ключей задач в порядке приоритета."),
        Message::user(format!("Приоритизируй эти задачи:\n\n{}", tasks_json)),
    ];

    let response = llm.chat_completion(messages, None).await?;

    let content = response.content().unwrap_or("[]");
    let priorities: Vec<String> = serde_json::from_str(content)?;

    Ok(priorities)
}
```

### Генерация описаний для задач

```rust
use tracker_lib::TrackerClient;
use llm_lib::LlmClient;
use anyhow::Result;

async fn generate_task_description(task_key: &str) -> Result<String> {
    // Получить задачу
    let tracker = TrackerClient::from_env()?;
    let issue = tracker.get_issue(task_key, None).await?;

    // Сгенерировать детальное описание
    let llm = LlmClient::with_token(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    )?;

    let prompt = format!(
        "Задача: {}\nКраткое описание: {}\n\nСоздай детальное описание задачи для команды разработки.",
        issue.key,
        issue.summary
    );

    let description = llm.complete_with_system(
        "Ты - технический писатель, создающий описания задач",
        prompt
    ).await?;

    Ok(description)
}
```

## CLI Commands

### Простые запросы

```bash
# Базовый вопрос
you llm ask "Что такое async/await в Rust?"

# С параметрами
you llm ask "Explain ownership" --model "openai/gpt-4-turbo" --temperature 0.7 --max-tokens 500

# Генерация плана работы
you llm plan-day

# С другой моделью
you llm plan-day --model "anthropic/claude-3-opus"
```

### Интеграция с трекером

```bash
# Получить задачу и спросить о ней LLM
TASK=$(you tracker get TASK-123)
you llm ask "Как лучше реализовать: $TASK"

# Генерация плана на основе текущих задач
you llm plan-day > daily-plan.md
```

## Использование с трейсингом

```bash
# Включить детальные логи
RUST_LOG=debug you llm ask "test"

# Логи только для llm_lib
RUST_LOG=llm_lib=debug you llm ask "test"

# Разные уровни для разных модулей
RUST_LOG=llm_lib=debug,tracker_lib=info you llm plan-day
```

## Советы по использованию

### Выбор модели

- **Claude 3.5 Sonnet** (по умолчанию) - лучший баланс качества и скорости
- **Claude 3 Opus** - максимальное качество для сложных задач
- **GPT-4 Turbo** - хорош для технических задач
- **GPT-3.5 Turbo** - быстрые и дешевые запросы

### Оптимизация токенов

```rust
use llm_lib::{LlmClient, CompletionOptions};

// Ограничить количество токенов для экономии
let options = CompletionOptions::new()
    .max_tokens(500)  // Максимум 500 токенов в ответе
    .temperature(0.3); // Более детерминированные ответы

let response = client.chat_completion(messages, Some(options)).await?;
```

### Повторные попытки при rate limit

```rust
use llm_lib::{LlmClient, LlmError};
use tokio::time::{sleep, Duration};

async fn request_with_retry(client: &LlmClient, prompt: &str) -> Result<String, LlmError> {
    let mut retries = 3;

    loop {
        match client.complete(prompt).await {
            Ok(response) => return Ok(response),
            Err(LlmError::RateLimitExceeded { retry_after }) => {
                if retries == 0 {
                    return Err(LlmError::RateLimitExceeded { retry_after });
                }
                retries -= 1;
                let wait_time = retry_after.unwrap_or(5);
                sleep(Duration::from_secs(wait_time)).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```
