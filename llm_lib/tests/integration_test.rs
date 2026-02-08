use llm_lib::{CompletionOptions, LlmClient, LlmConfig, Message};

#[test]
fn test_config_creation() {
    let config = LlmConfig::new("test-api-key", "test-model");
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.model, "test-model");
    assert_eq!(config.base_url, "https://openrouter.ai/api/v1");
    assert_eq!(config.timeout_secs, 120);
}

#[test]
fn test_config_builder() {
    let config = LlmConfig::new("key", "model");
    let config = LlmConfig {
        timeout_secs: 60,
        site_url: Some("https://example.com".to_string()),
        app_name: Some("test-app".to_string()),
        ..config
    };

    assert_eq!(config.timeout_secs, 60);
    assert_eq!(config.site_url, Some("https://example.com".to_string()));
    assert_eq!(config.app_name, Some("test-app".to_string()));
}

#[test]
fn test_client_creation() {
    let config = LlmConfig::new("test-key", "test-model");
    let result = LlmClient::new(config);
    assert!(result.is_ok());

    let client = result.unwrap();
    assert_eq!(client.model(), "test-model");
}

#[test]
fn test_message_builders() {
    let system_msg = Message::system("You are helpful");
    assert_eq!(system_msg.role, llm_lib::Role::System);
    assert_eq!(system_msg.content, "You are helpful");

    let user_msg = Message::user("Hello");
    assert_eq!(user_msg.role, llm_lib::Role::User);
    assert_eq!(user_msg.content, "Hello");

    let assistant_msg = Message::assistant("Hi there");
    assert_eq!(assistant_msg.role, llm_lib::Role::Assistant);
    assert_eq!(assistant_msg.content, "Hi there");
}

#[test]
fn test_completion_options_builder() {
    let options = CompletionOptions::new()
        .temperature(0.7)
        .max_tokens(1000)
        .top_p(0.9);

    assert_eq!(options.temperature, Some(0.7));
    assert_eq!(options.max_tokens, Some(1000));
    assert_eq!(options.top_p, Some(0.9));
}

#[tokio::test]
#[ignore]
async fn test_real_api_call() {
    let api_key = std::env::var("OPEN_ROUTER_TOKEN").expect("OPEN_ROUTER_TOKEN not set");
    let config = LlmConfig::new(api_key, "anthropic/claude-3.5-sonnet");
    let client = LlmClient::new(config).expect("Failed to create client");

    let response = client
        .complete("Say 'test successful' and nothing else")
        .await
        .expect("Failed to get completion");

    assert!(!response.is_empty());
    println!("Response: {}", response);
}
