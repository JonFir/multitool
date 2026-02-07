//! Интеграционные тесты для модуля search
//!
//! Тестируют функциональность поиска задач с различными параметрами

use tracker_lib::models::ExpandField;
use tracker_lib::search::{SearchParams, SearchRequest};
use tracker_lib::{TrackerClient, TrackerConfig};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Создать тестовый клиент с mock сервером
async fn create_test_client(mock_server: &MockServer) -> TrackerClient {
    let config = TrackerConfig::new("test-oauth-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    TrackerClient::new(config).expect("Failed to create test client")
}

#[tokio::test]
async fn test_search_issues_with_filter() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "filter": {
            "queue": "TREK",
            "assignee": "empty()"
        },
        "order": "+status"
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-1",
            "id": "507f1f77bcf86cd799439011",
            "key": "TREK-1",
            "version": 1,
            "summary": "Первая задача",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000"
        },
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-2",
            "id": "507f1f77bcf86cd799439012",
            "key": "TREK-2",
            "version": 1,
            "summary": "Вторая задача",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user2",
                "display": "User 2"
            },
            "createdAt": "2024-01-15T11:00:00.000+0000",
            "updatedAt": "2024-01-15T11:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.filter = Some(serde_json::json!({
        "queue": "TREK",
        "assignee": "empty()"
    }));
    request.order = Some("+status".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 2);
    assert_eq!(issues[0].key, "TREK-1");
    assert_eq!(issues[0].summary, "Первая задача");
    assert_eq!(issues[1].key, "TREK-2");
    assert_eq!(issues[1].summary, "Вторая задача");
}

#[tokio::test]
async fn test_search_issues_with_query() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: TREK Assignee: me()"
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-100",
            "id": "507f1f77bcf86cd799439020",
            "key": "TREK-100",
            "version": 1,
            "summary": "Моя задача",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "2",
                "key": "inProgress",
                "display": "В работе"
            },
            "createdBy": {
                "id": "current-user",
                "display": "Current User"
            },
            "createdAt": "2024-01-15T12:00:00.000+0000",
            "updatedAt": "2024-01-15T12:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: TREK Assignee: me()".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-100");
    assert_eq!(issues[0].summary, "Моя задача");
}

#[tokio::test]
async fn test_search_issues_with_keys() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "keys": ["TREK-1", "TREK-2", "TREK-3"]
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-1",
            "id": "507f1f77bcf86cd799439011",
            "key": "TREK-1",
            "version": 1,
            "summary": "Задача 1",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.keys = Some(vec![
        "TREK-1".to_string(),
        "TREK-2".to_string(),
        "TREK-3".to_string(),
    ]);

    let result = client.search_issues(&request, None).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-1");
}

#[tokio::test]
async fn test_search_issues_with_pagination() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "queue": "TREK"
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-10",
            "id": "507f1f77bcf86cd799439030",
            "key": "TREK-10",
            "version": 1,
            "summary": "Задача на второй странице",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(query_param("perPage", "10"))
        .and(query_param("page", "2"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.queue = Some("TREK".to_string());

    let params = SearchParams {
        per_page: Some(10),
        page: Some(2),
        ..Default::default()
    };

    let result = client.search_issues(&request, Some(params)).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-10");
}

#[tokio::test]
async fn test_search_issues_with_expand() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: TREK"
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-200",
            "id": "507f1f77bcf86cd799439040",
            "key": "TREK-200",
            "version": 1,
            "summary": "Задача с расширенными полями",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000",
            "attachments": [
                {
                    "id": "1",
                    "name": "file.txt",
                    "size": 1024
                }
            ]
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(query_param("expand", "attachments,comments"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: TREK".to_string());

    let params = SearchParams {
        expand: vec![ExpandField::Attachments, ExpandField::Comments],
        ..Default::default()
    };

    let result = client.search_issues(&request, Some(params)).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-200");
}

#[tokio::test]
async fn test_search_issues_with_scroll() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: TREK"
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-300",
            "id": "507f1f77bcf86cd799439050",
            "key": "TREK-300",
            "version": 1,
            "summary": "Задача из scroll запроса",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(query_param("scrollType", "sorted"))
        .and(query_param("perScroll", "100"))
        .and(query_param("scrollTTLMillis", "60000"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: TREK".to_string());

    let params = SearchParams {
        scroll_type: Some("sorted".to_string()),
        per_scroll: Some(100),
        scroll_ttl_millis: Some(60000),
        ..Default::default()
    };

    let result = client.search_issues(&request, Some(params)).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-300");
}

#[tokio::test]
async fn test_search_issues_empty_result() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: NONEXISTENT"
    });

    let response_json = serde_json::json!([]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: NONEXISTENT".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 0);
}

#[tokio::test]
async fn test_search_issues_unauthorized() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: TREK"
    });

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: TREK".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        matches!(error, tracker_lib::TrackerError::Unauthorized),
        "Expected Unauthorized error, got: {:?}",
        error
    );
}

#[tokio::test]
async fn test_search_issues_forbidden() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Queue: RESTRICTED"
    });

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Queue: RESTRICTED".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        matches!(error, tracker_lib::TrackerError::Forbidden),
        "Expected Forbidden error, got: {:?}",
        error
    );
}

#[tokio::test]
async fn test_search_issues_bad_request() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "query": "Invalid query syntax!!!"
    });

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(400).set_body_string("Invalid query syntax"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.query = Some("Invalid query syntax!!!".to_string());

    let result = client.search_issues(&request, None).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(
        matches!(error, tracker_lib::TrackerError::ApiError { .. }),
        "Expected ApiError, got: {:?}",
        error
    );
}

#[tokio::test]
async fn test_search_issues_with_filter_id() {
    let mock_server = MockServer::start().await;

    let request_body = serde_json::json!({
        "filterId": 12345
    });

    let response_json = serde_json::json!([
        {
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-500",
            "id": "507f1f77bcf86cd799439060",
            "key": "TREK-500",
            "version": 1,
            "summary": "Задача из сохраненного фильтра",
            "type": {
                "id": "1",
                "key": "task",
                "display": "Задача"
            },
            "priority": {
                "id": "2",
                "key": "normal",
                "display": "Средний"
            },
            "status": {
                "id": "1",
                "key": "open",
                "display": "Открыта"
            },
            "createdBy": {
                "id": "user1",
                "display": "User 1"
            },
            "createdAt": "2024-01-15T10:00:00.000+0000",
            "updatedAt": "2024-01-15T10:00:00.000+0000"
        }
    ]);

    Mock::given(method("POST"))
        .and(path("/v3/issues/_search"))
        .and(body_json(&request_body))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut request = SearchRequest::default();
    request.filter_id = Some(12345);

    let result = client.search_issues(&request, None).await;

    assert!(result.is_ok());
    let issues = result.unwrap();
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].key, "TREK-500");
    assert_eq!(issues[0].summary, "Задача из сохраненного фильтра");
}
