//! Интеграционные тесты для модуля api_client
//!
//! Тестируют основные HTTP методы и обработку ответов

use std::collections::HashMap;
use tracker_lib::{Language, PaginationParams, TrackerClient, TrackerConfig, TrackerError};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Создать тестовый клиент с mock сервером
async fn create_test_client(mock_server: &MockServer) -> TrackerClient {
    let config = TrackerConfig::new("test-oauth-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3");

    TrackerClient::new(config).expect("Failed to create test client")
}

/// Создать тестовый клиент с организацией
async fn create_test_client_with_org(mock_server: &MockServer, org_id: &str) -> TrackerClient {
    let config = TrackerConfig::new("test-oauth-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3")
        .with_org_id(org_id);

    TrackerClient::new(config).expect("Failed to create test client")
}

#[tokio::test]
async fn test_get_request_success() {
    let mock_server = MockServer::start().await;

    let response_json = serde_json::json!({
        "id": "123",
        "name": "Test Resource"
    });

    Mock::given(method("GET"))
        .and(path("/v3/test/resource"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_json))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let (result, pagination) = client.get("test/resource", None).await.unwrap();

    assert_eq!(result["id"], "123");
    assert_eq!(result["name"], "Test Resource");
    assert!(pagination.is_none());
}

#[tokio::test]
async fn test_get_request_with_query_params() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/resource"))
        .and(query_param("filter", "active"))
        .and(query_param("sort", "name"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let mut query_params = HashMap::new();
    query_params.insert("filter".to_string(), "active".to_string());
    query_params.insert("sort".to_string(), "name".to_string());

    let result = client.get("test/resource", Some(&query_params)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_request_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/resources"))
        .and(query_param("perPage", "10"))
        .and(query_param("page", "2"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "results": ["item1", "item2"]
                }))
                .insert_header("X-Total-Pages", "5")
                .insert_header("X-Total-Count", "50"),
        )
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let pagination = PaginationParams {
        per_page: Some(10),
        page: Some(2),
    };

    let (result, meta) = client
        .get_paginated("test/resources", &pagination, None)
        .await
        .unwrap();

    assert_eq!(result["results"].as_array().unwrap().len(), 2);
    assert!(meta.is_some());

    let pagination_meta = meta.unwrap();
    assert_eq!(pagination_meta.total_pages, Some(5));
    assert_eq!(pagination_meta.total_count, Some(50));
}

#[tokio::test]
async fn test_post_request_success() {
    let mock_server = MockServer::start().await;

    let expected_body = serde_json::json!({
        "name": "New Resource",
        "description": "Test description"
    });

    Mock::given(method("POST"))
        .and(path("/v3/test/resources"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "new-123",
            "name": "New Resource",
            "description": "Test description"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let (result, _) = client
        .post("test/resources", &expected_body, None)
        .await
        .unwrap();

    assert_eq!(result["id"], "new-123");
    assert_eq!(result["name"], "New Resource");
}

#[tokio::test]
async fn test_patch_request_success() {
    let mock_server = MockServer::start().await;

    let update_body = serde_json::json!({
        "name": "Updated Name"
    });

    Mock::given(method("PATCH"))
        .and(path("/v3/test/resources/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": "123",
            "name": "Updated Name"
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let (result, _) = client
        .patch("test/resources/123", &update_body, None)
        .await
        .unwrap();

    assert_eq!(result["id"], "123");
    assert_eq!(result["name"], "Updated Name");
}

#[tokio::test]
async fn test_delete_request_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/v3/test/resources/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    let result = client.delete("test/resources/123", None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_authorization_header_is_set() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/auth"))
        .and(header("Authorization", "OAuth test-oauth-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = client.get("test/auth", None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_org_id_header_is_set() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/org"))
        .and(header("X-Org-ID", "test-org-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client_with_org(&mock_server, "test-org-123").await;
    let result = client.get("test/org", None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_language_header_is_set() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/lang"))
        .and(header("Accept-Language", "en"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let config = TrackerConfig::new("test-token")
        .with_base_url(mock_server.uri())
        .with_api_version("v3")
        .with_language(Language::English);

    let client = TrackerClient::new(config).unwrap();
    let result = client.get("test/lang", None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_handle_unauthorized_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/unauthorized"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = client.get("test/unauthorized", None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        TrackerError::Unauthorized => {}
        other => panic!("Expected Unauthorized error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_handle_forbidden_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/forbidden"))
        .respond_with(ResponseTemplate::new(403).set_body_string("Forbidden"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = client.get("test/forbidden", None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        TrackerError::Forbidden => {}
        other => panic!("Expected Forbidden error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_handle_not_found_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/notfound"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Resource not found"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = client.get("test/notfound", None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        TrackerError::NotFound { resource } => {
            assert_eq!(resource, "Resource not found");
        }
        other => panic!("Expected NotFound error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_handle_api_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/badrequest"))
        .respond_with(ResponseTemplate::new(400).set_body_string("Invalid request"))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let result = client.get("test/badrequest", None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        TrackerError::ApiError { status, message } => {
            assert_eq!(status.as_u16(), 400);
            assert_eq!(message, "Invalid request");
        }
        other => panic!("Expected ApiError, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_url_building_with_leading_slash() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/path"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;

    // Тестируем, что оба варианта работают
    let result1 = client.get("/test/path", None).await;
    assert!(result1.is_ok());

    let result2 = client.get("test/path", None).await;
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_pagination_without_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/no-pagination"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "results": []
        })))
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let (_, pagination) = client.get("test/no-pagination", None).await.unwrap();

    assert!(pagination.is_none());
}

#[tokio::test]
async fn test_pagination_with_partial_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v3/test/partial-pagination"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({
                    "results": []
                }))
                .insert_header("X-Total-Count", "100"),
        )
        .mount(&mock_server)
        .await;

    let client = create_test_client(&mock_server).await;
    let (_, pagination) = client.get("test/partial-pagination", None).await.unwrap();

    assert!(pagination.is_some());
    let meta = pagination.unwrap();
    assert_eq!(meta.total_count, Some(100));
    assert_eq!(meta.total_pages, None);
}
