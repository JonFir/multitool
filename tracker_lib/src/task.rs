//! Модуль для работы с задачами в Яндекс.Трекере
//!
//! Содержит методы для получения информации о конкретных задачах.

use crate::models::{ExpandField, Issue};
use crate::{Result, TrackerClient};

/// Параметры запроса для получения задачи
#[derive(Debug, Clone, Default)]
pub struct GetIssueParams {
    /// Дополнительные поля для включения в ответ
    pub expand: Vec<ExpandField>,
}

impl TrackerClient {
    /// Получить информацию о задаче
    ///
    /// # Параметры
    ///
    /// * `issue_id` - Идентификатор или ключ задачи
    /// * `params` - Дополнительные параметры запроса (опционально)
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use tracker_lib::{TrackerClient, task::GetIssueParams};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = TrackerClient::with_token("your-oauth-token")?;
    /// let issue = client.get_issue("TREK-123", None).await?;
    /// println!("Задача: {} - {}", issue.key, issue.summary);
    /// # Ok(())
    /// # }
    /// ```
    #[tracing::instrument(skip(self), fields(issue_id = %issue_id))]
    pub async fn get_issue(&self, issue_id: &str, params: Option<GetIssueParams>) -> Result<Issue> {
        tracing::debug!("Получение задачи: {}", issue_id);

        let resource_path = format!("issues/{}", issue_id);

        // Формируем query параметры
        let mut query_params = std::collections::HashMap::new();
        if let Some(params) = params {
            if !params.expand.is_empty() {
                let expand_values: Vec<&str> =
                    params.expand.iter().map(|field| field.as_str()).collect();
                query_params.insert("expand".to_string(), expand_values.join(","));
            }
        }

        let query = if query_params.is_empty() {
            None
        } else {
            Some(&query_params)
        };

        let (json_value, _) = self.get(&resource_path, query).await?;

        let issue: Issue = serde_json::from_value(json_value)?;

        tracing::info!(
            issue_key = %issue.key,
            issue_summary = %issue.summary,
            "Задача получена успешно"
        );

        Ok(issue)
    }
}
