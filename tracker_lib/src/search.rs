//! Модуль для поиска задач в Яндекс.Трекере
//!
//! Содержит структуры и методы для выполнения поисковых запросов
//! с поддержкой различных режимов пагинации.

use crate::models::{ExpandField, Issue};
use crate::{Result, TrackerClient};
use serde::Serialize;

/// Тело запроса для поиска задач
#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchRequest {
    /// Фильтр по полям задачи (произвольный объект с парами ключ-значение)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    /// Фильтр на языке запросов
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,

    /// Список ключей задач
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,

    /// Очередь
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,

    /// Идентификатор сохраненного фильтра
    #[serde(skip_serializing_if = "Option::is_none", rename = "filterId")]
    pub filter_id: Option<u64>,

    /// Направление и поле сортировки (например, "+status" или "-createdAt")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

/// Параметры запроса для поиска задач
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    /// Дополнительные поля для включения в ответ
    pub expand: Vec<ExpandField>,

    /// Количество задач на странице (по умолчанию 50)
    pub per_page: Option<u32>,

    /// Номер страницы для постраничного отображения
    pub page: Option<u32>,

    /// Идентификатор страницы для относительной пагинации
    pub id: Option<String>,

    /// Тип прокрутки: "sorted" или "unsorted"
    pub scroll_type: Option<String>,

    /// Максимальное количество задач в ответе при прокрутке (по умолчанию 100, максимум 1000)
    pub per_scroll: Option<u32>,

    /// Время жизни контекста прокрутки в миллисекундах (по умолчанию 60000)
    pub scroll_ttl_millis: Option<u64>,

    /// Идентификатор страницы для прокрутки
    pub scroll_id: Option<String>,
}

impl TrackerClient {
    /// Найти задачи по критериям поиска
    ///
    /// # Параметры
    ///
    /// * `request` - Критерии поиска задач
    /// * `params` - Дополнительные параметры запроса (опционально)
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use tracker_lib::{TrackerClient, search::{SearchRequest, SearchParams}};
    /// # use serde_json::json;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TrackerClient::with_token("your-oauth-token")?;
    ///
    /// // Поиск с помощью фильтра
    /// let mut request = SearchRequest::default();
    /// request.filter = Some(json!({
    ///     "queue": "TREK",
    ///     "assignee": "empty()"
    /// }));
    /// request.order = Some("+status".to_string());
    ///
    /// let issues = client.search_issues(&request, None).await?;
    /// println!("Найдено задач: {}", issues.len());
    ///
    /// // Поиск с помощью языка запросов
    /// let mut request = SearchRequest::default();
    /// request.query = Some("Queue: TREK Assignee: me()".to_string());
    ///
    /// let issues = client.search_issues(&request, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument(skip(self, request))]
    pub async fn search_issues(
        &self,
        request: &SearchRequest,
        params: Option<SearchParams>,
    ) -> Result<Vec<Issue>> {
        tracing::debug!("Поиск задач с заданными критериями");

        let resource_path = "issues/_search";

        // Формируем query параметры
        let mut query_params = std::collections::HashMap::new();

        if let Some(params) = params {
            // Expand параметры
            if !params.expand.is_empty() {
                let expand_values: Vec<&str> =
                    params.expand.iter().map(|field| field.as_str()).collect();
                query_params.insert("expand".to_string(), expand_values.join(","));
            }

            // Параметры пагинации
            if let Some(per_page) = params.per_page {
                query_params.insert("perPage".to_string(), per_page.to_string());
            }
            if let Some(page) = params.page {
                query_params.insert("page".to_string(), page.to_string());
            }
            if let Some(id) = params.id {
                query_params.insert("id".to_string(), id);
            }

            // Параметры прокрутки
            if let Some(scroll_type) = params.scroll_type {
                query_params.insert("scrollType".to_string(), scroll_type);
            }
            if let Some(per_scroll) = params.per_scroll {
                query_params.insert("perScroll".to_string(), per_scroll.to_string());
            }
            if let Some(scroll_ttl) = params.scroll_ttl_millis {
                query_params.insert("scrollTTLMillis".to_string(), scroll_ttl.to_string());
            }
            if let Some(scroll_id) = params.scroll_id {
                query_params.insert("scrollId".to_string(), scroll_id);
            }
        }

        let query = if query_params.is_empty() {
            None
        } else {
            Some(&query_params)
        };

        let (json_value, _) = self.post(resource_path, request, query).await?;

        let issues: Vec<Issue> = serde_json::from_value(json_value)?;

        tracing::info!(issues_count = issues.len(), "Задачи найдены успешно");

        Ok(issues)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_request_serialization() {
        let request = SearchRequest {
            filter: Some(serde_json::json!({
                "queue": "TREK",
                "assignee": "empty()"
            })),
            order: Some("+status".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"filter\""));
        assert!(json.contains("\"queue\""));
        assert!(json.contains("\"order\""));
        assert!(!json.contains("\"query\""));
    }

    #[test]
    fn test_search_request_with_query() {
        let request = SearchRequest {
            query: Some("Queue: TREK Assignee: me()".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"query\""));
        assert!(json.contains("Queue: TREK"));
    }

    #[test]
    fn test_search_request_with_keys() {
        let request = SearchRequest {
            keys: Some(vec!["TREK-1".to_string(), "TREK-2".to_string()]),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"keys\""));
        assert!(json.contains("TREK-1"));
        assert!(json.contains("TREK-2"));
    }

    #[test]
    fn test_search_request_empty_serialization() {
        let request = SearchRequest::default();
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, "{}");
    }

    #[test]
    fn test_search_params_default() {
        let params = SearchParams::default();
        assert!(params.expand.is_empty());
        assert!(params.per_page.is_none());
        assert!(params.page.is_none());
        assert!(params.scroll_type.is_none());
    }
}
