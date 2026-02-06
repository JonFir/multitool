use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Ошибки при работе с API Трекера
#[derive(Debug, thiserror::Error)]
pub enum TrackerError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseFailed(#[from] serde_json::Error),

    #[error("API error: {status} - {message}")]
    ApiError { status: StatusCode, message: String },

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, TrackerError>;

/// Параметры постраничной навигации
#[derive(Debug, Clone, Serialize)]
pub struct PaginationParams {
    /// Количество объектов на странице (по умолчанию 50)
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,

    /// Номер страницы (по умолчанию 1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            per_page: Some(50),
            page: Some(1),
        }
    }
}

/// Метаданные ответа с постраничной навигацией
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationMeta {
    /// Общее количество страниц
    pub total_pages: Option<u32>,

    /// Общее количество записей
    pub total_count: Option<u32>,
}

/// Язык локализации ответов API
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Russian,
    English,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Language::Russian => "ru",
            Language::English => "en",
        }
    }
}

/// Конфигурация клиента API Трекера
#[derive(Debug, Clone)]
pub struct TrackerConfig {
    /// Базовый URL API (по умолчанию https://st-api.yandex-team.ru)
    pub base_url: String,

    /// Версия API (v2 или v3, рекомендуется v3)
    pub api_version: String,

    /// OAuth токен для аутентификации
    pub oauth_token: String,

    /// Идентификатор организации (опционально)
    pub org_id: Option<String>,

    /// Язык локализации (по умолчанию русский)
    pub language: Language,
}

impl TrackerConfig {
    /// Создать новую конфигурацию с минимальными параметрами
    pub fn new(oauth_token: impl Into<String>) -> Self {
        Self {
            base_url: "https://st-api.yandex-team.ru".to_string(),
            api_version: "v3".to_string(),
            oauth_token: oauth_token.into(),
            org_id: None,
            language: Language::Russian,
        }
    }

    /// Установить идентификатор организации
    pub fn with_org_id(mut self, org_id: impl Into<String>) -> Self {
        self.org_id = Some(org_id.into());
        self
    }

    /// Установить язык локализации
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }

    /// Установить базовый URL
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Установить версию API
    pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
        self.api_version = api_version.into();
        self
    }
}

/// Клиент для работы с API Яндекс.Трекера
#[derive(Debug, Clone)]
pub struct TrackerClient {
    config: TrackerConfig,
    client: Client,
}

impl TrackerClient {
    /// Создать новый клиент с заданной конфигурацией
    pub fn new(config: TrackerConfig) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| TrackerError::ConfigError(e.to_string()))?;

        Ok(Self { config, client })
    }

    /// Создать клиент с минимальной конфигурацией (только OAuth токен)
    pub fn with_token(oauth_token: impl Into<String>) -> Result<Self> {
        Self::new(TrackerConfig::new(oauth_token))
    }

    /// Построить полный URL для ресурса
    fn build_url(&self, resource_path: &str) -> String {
        let path = resource_path.trim_start_matches('/');
        format!(
            "{}/{}/{}",
            self.config.base_url, self.config.api_version, path
        )
    }

    /// Подготовить HTTP запрос с необходимыми заголовками
    fn prepare_request(&self, method: Method, url: &str) -> RequestBuilder {
        let mut builder = self.client.request(method, url);

        // Добавляем заголовок Authorization
        builder = builder.header(
            "Authorization",
            format!("OAuth {}", self.config.oauth_token),
        );

        // Добавляем идентификатор организации, если указан
        if let Some(org_id) = &self.config.org_id {
            builder = builder.header("X-Org-ID", org_id);
        }

        // Добавляем язык локализации
        builder = builder.header("Accept-Language", self.config.language.as_str());

        builder
    }

    /// Обработать ответ и извлечь метаданные пагинации
    async fn handle_response(&self, response: Response) -> Result<(Value, Option<PaginationMeta>)> {
        let status = response.status();

        // Извлекаем заголовки пагинации
        let total_pages = response
            .headers()
            .get("X-Total-Pages")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let total_count = response
            .headers()
            .get("X-Total-Count")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok());

        let pagination_meta = if total_pages.is_some() || total_count.is_some() {
            Some(PaginationMeta {
                total_pages,
                total_count,
            })
        } else {
            None
        };

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(TrackerError::ApiError {
                status,
                message: error_text,
            });
        }

        let json_value = response.json::<Value>().await?;
        Ok((json_value, pagination_meta))
    }

    /// Выполнить GET запрос
    pub async fn get(
        &self,
        resource_path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<(Value, Option<PaginationMeta>)> {
        let url = self.build_url(resource_path);
        let mut request = self.prepare_request(Method::GET, &url);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Выполнить GET запрос с параметрами пагинации
    pub async fn get_paginated(
        &self,
        resource_path: &str,
        pagination: &PaginationParams,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<(Value, Option<PaginationMeta>)> {
        let url = self.build_url(resource_path);
        let mut request = self.prepare_request(Method::GET, &url);

        // Добавляем параметры пагинации
        if let Some(per_page) = pagination.per_page {
            request = request.query(&[("perPage", per_page.to_string())]);
        }
        if let Some(page) = pagination.page {
            request = request.query(&[("page", page.to_string())]);
        }

        // Добавляем дополнительные параметры
        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Выполнить POST запрос
    pub async fn post<T: Serialize>(
        &self,
        resource_path: &str,
        body: &T,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<(Value, Option<PaginationMeta>)> {
        let url = self.build_url(resource_path);
        let mut request = self.prepare_request(Method::POST, &url).json(body);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Выполнить PATCH запрос
    pub async fn patch<T: Serialize>(
        &self,
        resource_path: &str,
        body: &T,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<(Value, Option<PaginationMeta>)> {
        let url = self.build_url(resource_path);
        let mut request = self.prepare_request(Method::PATCH, &url).json(body);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }

    /// Выполнить DELETE запрос
    pub async fn delete(
        &self,
        resource_path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<(Value, Option<PaginationMeta>)> {
        let url = self.build_url(resource_path);
        let mut request = self.prepare_request(Method::DELETE, &url);

        if let Some(params) = query_params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.handle_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_config_builder() {
        let config = TrackerConfig::new("test-token")
            .with_org_id("123")
            .with_language(Language::English);

        assert_eq!(config.oauth_token, "test-token");
        assert_eq!(config.org_id, Some("123".to_string()));
        assert_eq!(config.language.as_str(), "en");
    }

    #[test]
    fn test_pagination_params_default() {
        let pagination = PaginationParams::default();
        assert_eq!(pagination.per_page, Some(50));
        assert_eq!(pagination.page, Some(1));
    }

    #[test]
    fn test_build_url() {
        let config = TrackerConfig::new("test-token");
        let client = TrackerClient::new(config).unwrap();

        let url = client.build_url("/issues/TEST-1");
        assert_eq!(url, "https://st-api.yandex-team.ru/v3/issues/TEST-1");

        let url = client.build_url("issues/TEST-1");
        assert_eq!(url, "https://st-api.yandex-team.ru/v3/issues/TEST-1");
    }
}
