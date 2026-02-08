# LLM Library

Библиотека для работы с LLM API через OpenRouter.

## Возможности

- ✅ Поддержка OpenRouter API
- ✅ Chat completion с историей диалога
- ✅ Конфигурируемые модели и параметры
- ✅ Полная поддержка трейсинга (tracing)
- ✅ Удобная обработка ошибок
- ✅ Типизированные запросы и ответы

## Установка

Добавьте в ваш CLI или другую часть workspace:

```toml
[dependencies]
llm_lib = { path = "../llm_lib" }
```

## Быстрый старт

### Простой запрос

```rust
use llm_lib::{LlmClient, LlmConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = LlmConfig::new(
        "your-openrouter-api-key",
        "anthropic/claude-3.5-sonnet"
    );
    let client = LlmClient::new(config)?;

    let response = client.complete("Что такое Rust?").await?;
    println!("Ответ: {}", response);

    Ok(())
}
```

### С системным промптом

```rust
let response = client.complete_with_system(
    "Ты - эксперт по Rust",
    "Объясни async/await"
).await?;
```

### Многошаговый диалог

```rust
use llm_lib::{Message, CompletionOptions};

let messages = vec![
    Message::system("Ты - помощник программиста"),
    Message::user("Привет!"),
    Message::assistant("Привет! Чем могу помочь?"),
    Message::user("Расскажи про трейты"),
];

let options = CompletionOptions::new()
    .temperature(0.7)
    .max_tokens(1000);

let response = client.chat_completion(messages, Some(options)).await?;
if let Some(content) = response.content() {
    println!("Ответ: {}", content);
}
```

## Конфигурация

### Базовая конфигурация

```rust
use llm_lib::{LlmClient, LlmConfig};

let config = LlmConfig::new(
    "your-api-key",
    "anthropic/claude-3.5-sonnet"
);

let client = LlmClient::new(config)?;
```

### Расширенная конфигурация

```rust
let config = LlmConfig {
    api_key: "api-key".to_string(),
    model: "model".to_string(),
    base_url: "https://openrouter.ai/api/v1".to_string(),
    timeout_secs: 60,
    site_url: Some("https://yourapp.com".to_string()),
    app_name: Some("Your App".to_string()),
};

let client = LlmClient::new(config)?;
```

## Доступные модели

OpenRouter поддерживает множество моделей:

- `anthropic/claude-3.5-sonnet` - Claude 3.5 Sonnet
- `anthropic/claude-3-opus` - Claude 3 Opus
- `openai/gpt-4-turbo` - GPT-4 Turbo
- `openai/gpt-3.5-turbo` - GPT-3.5 Turbo
- И многие другие...

Полный список: https://openrouter.ai/models

## Обработка ошибок

```rust
use llm_lib::{LlmClient, LlmError};

match client.complete("test").await {
    Ok(response) => println!("Success: {}", response),
    Err(LlmError::AuthError) => eprintln!("Invalid API key"),
    Err(LlmError::RateLimitExceeded { retry_after }) => {
        eprintln!("Rate limited, retry after: {:?}", retry_after);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Трейсинг

Библиотека использует `tracing` для логирования:

```bash
# Включить логи при запуске
RUST_LOG=debug cargo run

# Только логи llm_lib
RUST_LOG=llm_lib=debug cargo run
```

## Тестирование

```bash
# Запустить юнит-тесты
cargo test -p llm_lib

# Запустить интеграционные тесты (требуется API ключ)
OPEN_ROUTER_TOKEN=your-key cargo test -p llm_lib -- --ignored

# Проверить линтером
cargo clippy -p llm_lib -- -D warnings

# Форматировать код
cargo fmt -p llm_lib
```

## Пример: Интеграция с tracker_lib

```rust
use tracker_lib::TrackerClient;
use llm_lib::LlmClient;

async fn generate_daily_plan() -> Result<String, Box<dyn std::error::Error>> {
    // Получить задачи из трекера
    let tracker = TrackerClient::from_env()?;
    let issues = tracker.search_issues(None, None).await?;

    // Сформировать промпт
    let task_list = issues.iter()
        .map(|i| format!("- {} ({})", i.summary, i.key))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        "На основе этих задач составь план работы на день:\n\n{}",
        task_list
    );

    // Получить план от LLM
    let config = LlmConfig::new(
        std::env::var("OPEN_ROUTER_TOKEN")?,
        "anthropic/claude-3.5-sonnet"
    );
    let llm = LlmClient::new(config)?;

    let plan = llm.complete_with_system(
        "Ты - ассистент для планирования работы",
        prompt
    ).await?;

    Ok(plan)
}
```

## Лицензия

MIT OR Apache-2.0
