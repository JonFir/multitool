# TrackerLib

Библиотека для работы с API Яндекс.Трекера на Rust.

## Особенности

- Полная поддержка HTTP методов: GET, POST, PATCH, DELETE
- Встроенная обработка OAuth аутентификации
- Поддержка постраничной навигации
- Локализация на русском и английском языках
- Настраиваемый клиент с builder pattern
- Типобезопасная обработка ошибок
- Async/await на основе tokio

## Установка

Добавьте в `Cargo.toml`:

```toml
[dependencies]
trackerLib = { path = "../trackerLib" }
tokio = { version = "1", features = ["full"] }
```

## Быстрый старт

```rust
use trackerLib::TrackerClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создание клиента
    let client = TrackerClient::with_token("your-oauth-token")?;

    // Получение информации о задаче
    let (issue, _) = client.get("issues/TEST-1", None).await?;
    println!("Issue: {}", issue);

    Ok(())
}
```

## Конфигурация

### Базовая конфигурация

```rust
use trackerLib::TrackerClient;

let client = TrackerClient::with_token("your-oauth-token")?;
```

### Расширенная конфигурация

```rust
use trackerLib::{TrackerClient, TrackerConfig, Language};

let config = TrackerConfig::new("your-oauth-token")
    .with_org_id("123456")
    .with_language(Language::English)
    .with_api_version("v3");

let client = TrackerClient::new(config)?;
```

## Примеры использования

### Получение задачи (GET)

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("expand".to_string(), "attachments".to_string());

let (issue, _) = client.get("issues/TEST-1", Some(&params)).await?;
```

### Создание задачи (POST)

```rust
use serde_json::json;

let body = json!({
    "queue": "TREK",
    "summary": "Test Issue",
    "type": "bug",
    "assignee": "user_login"
});

let (created_issue, _) = client.post("issues/", &body, None).await?;
```

### Изменение задачи (PATCH)

```rust
use serde_json::json;

let body = json!({
    "summary": "Updated Title",
    "description": "Updated Description",
    "type": {
        "key": "bug"
    },
    "priority": {
        "key": "minor"
    }
});

let (updated_issue, _) = client.patch("issues/TEST-1", &body, None).await?;
```

### Удаление задачи (DELETE)

```rust
let (response, _) = client.delete("issues/TEST-1", None).await?;
```

### Постраничная навигация

```rust
use trackerLib::PaginationParams;

let pagination = PaginationParams {
    per_page: Some(15),
    page: Some(1),
};

let (issues, meta) = client.get_paginated("issues", &pagination, None).await?;

if let Some(meta) = meta {
    println!("Total pages: {:?}", meta.total_pages);
    println!("Total count: {:?}", meta.total_count);
}
```

### Поиск задач

```rust
use serde_json::json;

let search_body = json!({
    "filter": {
        "queue": "TREK",
        "assignee": "user_login"
    }
});

let (results, meta) = client.post("issues/_search", &search_body, None).await?;
```

### Работа с массивами

#### Добавление значений

```rust
use serde_json::json;

let body = json!({
    "followers": {
        "add": ["user1", "user2"]
    }
});

client.patch("issues/TEST-1", &body, None).await?;
```

#### Удаление значений

```rust
let body = json!({
    "followers": {
        "remove": ["user1"]
    }
});

client.patch("issues/TEST-1", &body, None).await?;
```

#### Полная замена массива

```rust
let body = json!({
    "followers": {
        "set": ["new_user1", "new_user2"]
    }
});

client.patch("issues/TEST-1", &body, None).await?;
```

#### Замена отдельных элементов

```rust
let body = json!({
    "followers": {
        "replace": [
            {"target": "user1", "replacement": "user3"},
            {"target": "user2", "replacement": "user4"}
        ]
    }
});

client.patch("issues/TEST-1", &body, None).await?;
```

#### Обнуление поля

```rust
let body = json!({
    "followers": null
});

client.patch("issues/TEST-1", &body, None).await?;
```

## Обработка ошибок

```rust
use trackerLib::TrackerError;

match client.get("issues/TEST-1", None).await {
    Ok((issue, _)) => {
        println!("Success: {}", issue);
    },
    Err(TrackerError::ApiError { status, message }) => {
        eprintln!("API Error {}: {}", status, message);
    },
    Err(TrackerError::AuthError(msg)) => {
        eprintln!("Authentication failed: {}", msg);
    },
    Err(e) => {
        eprintln!("Other error: {}", e);
    }
}
```

## Типы ошибок

- `TrackerError::RequestFailed` - Ошибка HTTP запроса
- `TrackerError::JsonParseFailed` - Ошибка парсинга JSON
- `TrackerError::ApiError` - Ошибка API (с кодом статуса и сообщением)
- `TrackerError::AuthError` - Ошибка аутентификации
- `TrackerError::ConfigError` - Ошибка конфигурации клиента

## Специальные символы и форматирование

При работе с текстовыми полями учитывайте:

- Двойные кавычки, обратный слеш и слеш экранируются с помощью `\`
- Перенос строки: `\n` или `\r`
- Unicode символы: `\uFFFF`

```rust
let body = json!({
    "description": "Внесите исправления:\n1. Используйте значение \"1\" вместо \"2\"."
});
```

## Локализация

По умолчанию API возвращает локализованные поля на русском языке:

```rust
use trackerLib::{TrackerConfig, Language};

// Английский язык
let config = TrackerConfig::new("token")
    .with_language(Language::English);

// Русский язык (по умолчанию)
let config = TrackerConfig::new("token")
    .with_language(Language::Russian);
```

## API версии

Библиотека поддерживает обе версии API Трекера:

- `v3` - текущая версия (рекомендуется, используется по умолчанию)
- `v2` - предыдущая версия

```rust
let config = TrackerConfig::new("token")
    .with_api_version("v3");
```

## Примеры

Полные примеры использования находятся в папке [examples/](examples/):

```bash
cargo run --example basic_usage
```

## Документация API

Официальная документация API Яндекс.Трекера:
- https://st.yandex-team.ru/docs/tracker/api/

## Лицензия

MIT OR Apache-2.0
