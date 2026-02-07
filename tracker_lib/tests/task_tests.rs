//! Интеграционные тесты для модуля task
//!
//! Используют wiremock для мокирования HTTP запросов к API Яндекс.Трекера

use tracker_lib::task::GetIssueParams;
use tracker_lib::{TrackerClient, TrackerConfig};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_get_issue_success() {
    // Создаем mock HTTP сервер
    let mock_server = MockServer::start().await;

    // Настраиваем mock ответ
    let response_json = serde_json::json!({
        "self": "https://st-api.yandex-team.ru/v3/issues/TREK-123",
        "id": "507f1f77bcf86cd799439011",
        "key": "TREK-123",
        "version": 1,
        "summary": "Тестовая задача",
        "description": "Описание тестовой задачи",
        "type": {
            "self": "https://st-api.yandex-team.ru/v3/issuetypes/1",
            "id": "1",
            "key": "task",
            "display": "Задача"
        },
        "priority": {
            "self": "https://st-api.yandex-team.ru/v3/priorities/2",
            "id": "2",
            "key": "normal",
            "display": "Средний"
        },
        "status": {
            "self": "https://st-api.yandex-team.ru/v3/statuses/1",
            "id": "1",
            "key": "open",
            "display": "Открыта"
        },
        "createdBy": {
            "self": "https://st-api.yandex-team.ru/v3/users/1",
            "id": "user1",
            "display": "Test User"
        },
        "createdAt": "2024-01-15T10:00:00.000+0000",
        "updatedAt": "2024-01-15T10:00:00.000+0000"
    });

    Mock::given(method("GET"))
        .and(path("/v3/issues/TREK-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    // Создаем клиент, который будет обращаться к mock серверу
    let config = TrackerConfig::new("test-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    let client = TrackerClient::new(config).expect("Failed to create client");

    // Выполняем тестируемый метод
    let result = client.get_issue("TREK-123", None).await;

    // Проверяем результат
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    let issue = result.unwrap();
    assert_eq!(issue.key, "TREK-123");
    assert_eq!(issue.summary, "Тестовая задача");
    assert_eq!(
        issue.description.as_deref(),
        Some("Описание тестовой задачи")
    );
}

#[tokio::test]
async fn test_get_issue_not_found() {
    let mock_server = MockServer::start().await;

    // Настраиваем mock для ошибки 404
    Mock::given(method("GET"))
        .and(path("/v3/issues/NONEXISTENT-999"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Issue not found"))
        .mount(&mock_server)
        .await;

    let config = TrackerConfig::new("test-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    let client = TrackerClient::new(config).expect("Failed to create client");

    // Выполняем запрос
    let result = client.get_issue("NONEXISTENT-999", None).await;

    // Проверяем, что получили ошибку NotFound
    assert!(result.is_err(), "Expected Err, got Ok");
    let error = result.unwrap_err();
    assert!(
        matches!(error, tracker_lib::TrackerError::NotFound { .. }),
        "Expected NotFound error, got: {:?}",
        error
    );
}

#[tokio::test]
async fn test_get_issue_unauthorized() {
    let mock_server = MockServer::start().await;

    // Настраиваем mock для ошибки 401
    Mock::given(method("GET"))
        .and(path("/v3/issues/TREK-123"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&mock_server)
        .await;

    let config = TrackerConfig::new("invalid-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    let client = TrackerClient::new(config).expect("Failed to create client");

    let result = client.get_issue("TREK-123", None).await;

    assert!(result.is_err(), "Expected Err, got Ok");
    let error = result.unwrap_err();
    assert!(
        matches!(error, tracker_lib::TrackerError::Unauthorized),
        "Expected Unauthorized error, got: {:?}",
        error
    );
}

#[tokio::test]
async fn test_get_issue_with_expand_params() {
    let mock_server = MockServer::start().await;

    let response_json = serde_json::json!({
        "self": "https://st-api.yandex-team.ru/v3/issues/TREK-456",
        "id": "507f1f77bcf86cd799439012",
        "key": "TREK-456",
        "version": 1,
        "summary": "Задача с expand параметрами",
        "description": "Описание",
        "type": {
            "self": "https://st-api.yandex-team.ru/v3/issuetypes/1",
            "id": "1",
            "key": "task",
            "display": "Задача"
        },
        "priority": {
            "self": "https://st-api.yandex-team.ru/v3/priorities/2",
            "id": "2",
            "key": "normal",
            "display": "Средний"
        },
        "status": {
            "self": "https://st-api.yandex-team.ru/v3/statuses/1",
            "id": "1",
            "key": "open",
            "display": "Открыта"
        },
        "createdBy": {
            "self": "https://st-api.yandex-team.ru/v3/users/1",
            "id": "user1",
            "display": "Test User"
        },
        "createdAt": "2024-01-15T10:00:00.000+0000",
        "updatedAt": "2024-01-15T10:00:00.000+0000",
        "attachments": [
            {
                "id": "1",
                "name": "test.txt",
                "size": 100
            }
        ]
    });

    // Проверяем, что expand параметры передаются в query string
    Mock::given(method("GET"))
        .and(path("/v3/issues/TREK-456"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let config = TrackerConfig::new("test-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    let client = TrackerClient::new(config).expect("Failed to create client");

    // Используем expand параметры
    let params = GetIssueParams {
        expand: vec![
            tracker_lib::models::ExpandField::Attachments,
            tracker_lib::models::ExpandField::Comments,
        ],
    };

    let result = client.get_issue("TREK-456", Some(params)).await;

    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    let issue = result.unwrap();
    assert_eq!(issue.key, "TREK-456");
    assert_eq!(issue.summary, "Задача с expand параметрами");
}
