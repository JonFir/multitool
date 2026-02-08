//! Модуль для работы с задачами в Яндекс.Трекере
//!
//! Содержит методы для получения информации о конкретных задачах.

use std::collections::HashMap;

use crate::models::{ExpandField, Issue};
use crate::{Result, TrackerClient};

/// Форматирует полный вывод информации о задаче (чистая функция)
///
/// # Параметры
///
/// * `issue` - Задача из Трекера
///
/// # Возвращает
///
/// Отформатированную строку для вывода в консоль
///
/// # Примеры
///
/// ```no_run
/// # use tracker_lib::{TrackerClient, task::format_issue_output};
/// # use tracker_lib::models::Issue;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = TrackerClient::from_env()?;
/// let issue = client.get_issue("TREK-123", None).await?;
/// let output = format_issue_output(&issue);
/// println!("{}", output);
/// # Ok(())
/// # }
/// ```
pub fn format_issue_output(issue: &Issue) -> String {
    let key = &issue.key;
    let title = &issue.summary;
    let status = issue
        .status
        .as_ref()
        .and_then(|s| s.display.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Неизвестен");
    let description = issue.description.as_deref().unwrap_or("Нет описания");
    let link = format!("https://st.yandex-team.ru/{}", key);

    let mut output = String::new();
    output.push('\n');
    output.push_str(&format!("📋 Задача: {}\n", key));
    output.push('\n');
    output.push_str("📌 Заголовок:\n");
    output.push_str(&format!("   {}\n", title));
    output.push('\n');
    output.push_str(&format!("🔖 Статус: {}\n", status));
    output.push('\n');
    output.push_str("📝 Описание:\n");
    // Разбиваем описание на строки для лучшей читаемости
    for line in description.lines() {
        output.push_str(&format!("   {}\n", line));
    }
    output.push('\n');
    output.push_str("🔗 Ссылка:\n");
    output.push_str(&format!("   {}\n", link));
    output.push('\n');

    output
}

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

        let expand = params
            .unwrap_or_default()
            .expand
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join(",");
        let query_params = HashMap::from([("expand".to_string(), expand)]);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Status;

    /// Создаёт минимальную задачу для тестирования
    fn create_minimal_issue(key: &str, summary: &str) -> Issue {
        Issue {
            self_link: None,
            id: None,
            key: key.to_string(),
            version: None,
            last_comment_updated_at: None,
            summary: summary.to_string(),
            parent: None,
            aliases: vec![],
            updated_by: None,
            description: None,
            sprint: vec![],
            issue_type: None,
            priority: None,
            created_at: None,
            followers: vec![],
            created_by: None,
            votes: 0,
            assignee: None,
            project: None,
            queue: None,
            updated_at: None,
            status: None,
            previous_status: None,
            favorite: false,
            tags: vec![],
        }
    }

    #[test]
    fn test_format_issue_output_minimal() {
        let issue = create_minimal_issue("TEST-1", "Test summary");

        let output = format_issue_output(&issue);

        assert!(output.contains("📋 Задача: TEST-1"));
        assert!(output.contains("📌 Заголовок:"));
        assert!(output.contains("   Test summary"));
        assert!(output.contains("🔖 Статус: Неизвестен"));
        assert!(output.contains("📝 Описание:"));
        assert!(output.contains("   Нет описания"));
        assert!(output.contains("🔗 Ссылка:"));
        assert!(output.contains("   https://st.yandex-team.ru/TEST-1"));
    }

    #[test]
    fn test_format_issue_output_full() {
        let mut issue = create_minimal_issue("TREK-9844", "Implement new feature");
        issue.status = Some(Status {
            self_link: None,
            id: None,
            key: None,
            display: Some("В работе".to_string()),
        });
        issue.description = Some("First line\nSecond line\nThird line".to_string());

        let output = format_issue_output(&issue);

        assert!(output.contains("📋 Задача: TREK-9844"));
        assert!(output.contains("   Implement new feature"));
        assert!(output.contains("🔖 Статус: В работе"));
        assert!(output.contains("   First line"));
        assert!(output.contains("   Second line"));
        assert!(output.contains("   Third line"));
        assert!(output.contains("   https://st.yandex-team.ru/TREK-9844"));
    }
}
