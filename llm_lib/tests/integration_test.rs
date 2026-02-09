use llm_lib::{LlmClient, LlmClientTrait, LlmConfig, LlmError, Message, Role};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_successful_chat_completion() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": "test-id-123",
        "model": "test-model",
        "created": 1234567890_u64,
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": "Hello, this is a test response"
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 10,
            "completion_tokens": 20,
            "total_tokens": 30
        }
    });

    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .and(header("Authorization", "Bearer test-api-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(response_body))
        .mount(&mock_server)
        .await;

    let config = LlmConfig {
        api_key: "test-api-key".to_string(),
        base_url: mock_server.uri(),
        model: "test-model".to_string(),
        timeout_secs: 30,
        site_url: None,
        app_name: None,
    };

    let client = LlmClient::new(config).expect("Failed to create client");
    let messages = vec![Message::user("Hello")];

    let response = client
        .chat_completion(messages, None)
        .await
        .expect("Request failed");

    assert_eq!(response.id, "test-id-123");
    assert_eq!(response.model, "test-model");
    assert_eq!(response.choices.len(), 1);
    assert_eq!(response.choices[0].message.role, Role::Assistant);
    assert_eq!(
        response.choices[0].message.content,
        "Hello, this is a test response"
    );
    assert_eq!(response.usage.total_tokens, 30);
}

#[tokio::test]
async fn test_authentication_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;

    let config = LlmConfig {
        api_key: "invalid-key".to_string(),
        base_url: mock_server.uri(),
        model: "test-model".to_string(),
        timeout_secs: 30,
        site_url: None,
        app_name: None,
    };

    let client = LlmClient::new(config).expect("Failed to create client");
    let messages = vec![Message::user("Hello")];

    let result = client.chat_completion(messages, None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        LlmError::AuthError => {}
        other => panic!("Expected AuthError, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_rate_limit_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "60"))
        .mount(&mock_server)
        .await;

    let config = LlmConfig {
        api_key: "test-api-key".to_string(),
        base_url: mock_server.uri(),
        model: "test-model".to_string(),
        timeout_secs: 30,
        site_url: None,
        app_name: None,
    };

    let client = LlmClient::new(config).expect("Failed to create client");
    let messages = vec![Message::user("Hello")];

    let result = client.chat_completion(messages, None).await;

    assert!(result.is_err());
    match result.unwrap_err() {
        LlmError::RateLimitExceeded { retry_after } => {
            assert_eq!(retry_after, Some(60));
        }
        other => panic!("Expected RateLimitExceeded, got: {:?}", other),
    }
}
