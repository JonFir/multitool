use serde_json::json;
use std::collections::HashMap;
use tracker_lib::{Language, PaginationParams, TrackerClient, TrackerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Пример 1: Создание клиента с минимальной конфигурацией
    println!("=== Пример 1: Создание клиента ===");
    let client = TrackerClient::with_token("your-oauth-token")?;

    // Пример 2: Создание клиента с полной конфигурацией
    println!("\n=== Пример 2: Расширенная конфигурация ===");
    let config = TrackerConfig::new("your-oauth-token")
        .with_org_id("123456")
        .with_language(Language::English);

    let client = TrackerClient::new(config)?;

    // Пример 3: Получение информации о задаче (GET)
    println!("\n=== Пример 3: Получение информации о задаче ===");
    let mut params = HashMap::new();
    params.insert("expand".to_string(), "attachments".to_string());

    match client.get("issues/JUNE-3", Some(&params)).await {
        Ok((issue, _)) => {
            println!("Issue data: {}", serde_json::to_string_pretty(&issue)?);
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 4: Создание задачи (POST)
    println!("\n=== Пример 4: Создание задачи ===");
    let create_body = json!({
        "queue": "TREK",
        "summary": "Test Issue",
        "parent": "JUNE-2",
        "type": "bug",
        "assignee": "user_login",
        "attachmentIds": [55, 56]
    });

    match client.post("issues/", &create_body, None).await {
        Ok((response, _)) => {
            println!(
                "Created issue: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 5: Изменение задачи (PATCH)
    println!("\n=== Пример 5: Изменение задачи ===");
    let update_body = json!({
        "summary": "Updated Issue Title",
        "description": "Updated description",
        "type": {
            "id": "1",
            "key": "bug"
        },
        "priority": {
            "id": "2",
            "key": "minor"
        }
    });

    match client.patch("issues/TEST-1", &update_body, None).await {
        Ok((response, _)) => {
            println!(
                "Updated issue: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 6: Поиск задач с постраничной навигацией (POST)
    println!("\n=== Пример 6: Поиск задач с пагинацией ===");
    let search_body = json!({
        "filter": {
            "queue": "TREK",
            "assignee": "user_login"
        }
    });

    let pagination = PaginationParams {
        per_page: Some(15),
        page: Some(1),
    };

    match client.post("issues/_search", &search_body, None).await {
        Ok((results, meta)) => {
            println!(
                "Search results: {}",
                serde_json::to_string_pretty(&results)?
            );
            if let Some(meta) = meta {
                println!("Total pages: {:?}", meta.total_pages);
                println!("Total count: {:?}", meta.total_count);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 7: Получение списка с пагинацией (GET)
    println!("\n=== Пример 7: Получение списка задач с пагинацией ===");
    let pagination = PaginationParams {
        per_page: Some(10),
        page: Some(1),
    };

    match client.get_paginated("issues", &pagination, None).await {
        Ok((issues, meta)) => {
            println!("Issues: {}", serde_json::to_string_pretty(&issues)?);
            if let Some(meta) = meta {
                println!(
                    "Pagination - Pages: {:?}, Total: {:?}",
                    meta.total_pages, meta.total_count
                );
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 8: Работа с массивами (добавление значений)
    println!("\n=== Пример 8: Добавление подписчиков ===");
    let add_followers_body = json!({
        "followers": {
            "add": ["user1", "user2"]
        }
    });

    match client
        .patch("issues/TEST-1", &add_followers_body, None)
        .await
    {
        Ok((response, _)) => {
            println!(
                "Updated followers: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 9: Замена значений в массиве
    println!("\n=== Пример 9: Замена подписчиков ===");
    let replace_followers_body = json!({
        "followers": {
            "replace": [
                {"target": "user1", "replacement": "user3"},
                {"target": "user2", "replacement": "user4"}
            ]
        }
    });

    match client
        .patch("issues/TEST-1", &replace_followers_body, None)
        .await
    {
        Ok((response, _)) => {
            println!(
                "Replaced followers: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 10: Обнуление поля
    println!("\n=== Пример 10: Очистка поля ===");
    let clear_field_body = json!({
        "followers": null
    });

    match client.patch("issues/TEST-1", &clear_field_body, None).await {
        Ok((response, _)) => {
            println!(
                "Cleared field: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    // Пример 11: Использование специальных символов в тексте
    println!("\n=== Пример 11: Текст с переносами строк и кавычками ===");
    let description_body = json!({
        "description": "Внесите исправления:\n1. Используйте значение \"1\" вместо значения \"2\"."
    });

    match client.patch("issues/TEST-1", &description_body, None).await {
        Ok((response, _)) => {
            println!(
                "Updated description: {}",
                serde_json::to_string_pretty(&response)?
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n=== Все примеры выполнены ===");
    Ok(())
}
