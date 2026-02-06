//! Модуль для работы с задачами в Яндекс.Трекере
//!
//! Содержит DTO (Data Transfer Objects) для представления задач
//! и методы для работы с API задач.

use crate::{Result, TrackerClient};
use serde::{Deserialize, Serialize};

/// Информация о пользователе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор пользователя
    pub id: Option<String>,

    /// Отображаемое имя пользователя
    pub display: Option<String>,

    /// Уникальный идентификатор аккаунта пользователя в Яндекс.Паспорте
    #[serde(rename = "passportUid")]
    pub passport_uid: Option<u64>,

    /// Уникальный идентификатор пользователя в облаке
    #[serde(rename = "cloudUid")]
    pub cloud_uid: Option<String>,
}

/// Информация о статусе задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор статуса
    pub id: Option<String>,

    /// Ключ статуса
    pub key: Option<String>,

    /// Отображаемое название статуса
    pub display: Option<String>,
}

/// Информация о приоритете
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Priority {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор приоритета
    pub id: Option<String>,

    /// Ключ приоритета
    pub key: Option<String>,

    /// Отображаемое название приоритета
    pub display: Option<String>,
}

/// Информация о типе задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueType {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор типа задачи
    pub id: Option<String>,

    /// Ключ типа задачи
    pub key: Option<String>,

    /// Отображаемое название типа задачи
    pub display: Option<String>,
}

/// Информация об очереди
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор очереди
    pub id: Option<String>,

    /// Ключ очереди
    pub key: Option<String>,

    /// Отображаемое название очереди
    pub display: Option<String>,
}

/// Информация о спринте
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор спринта
    pub id: Option<String>,

    /// Отображаемое название спринта
    pub display: Option<String>,
}

/// Информация о проекте
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор проекта
    pub id: Option<String>,

    /// Отображаемое название проекта
    pub display: Option<String>,
}

/// Информация о проектах задачи
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Основной проект задачи
    pub primary: Option<ProjectInfo>,

    /// Список дополнительных проектов задачи
    #[serde(default)]
    pub secondary: Vec<ProjectInfo>,
}

/// Информация о родительской задаче
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentIssue {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор задачи
    pub id: Option<String>,

    /// Ключ задачи
    pub key: Option<String>,

    /// Отображаемое название задачи
    pub display: Option<String>,
}

/// Задача в Яндекс.Трекере
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    /// Адрес ресурса API
    #[serde(rename = "self")]
    pub self_link: Option<String>,

    /// Идентификатор задачи
    pub id: Option<String>,

    /// Ключ задачи
    pub key: String,

    /// Версия задачи
    pub version: Option<u32>,

    /// Дата и время последнего добавленного комментария
    #[serde(rename = "lastCommentUpdatedAt")]
    pub last_comment_updated_at: Option<String>,

    /// Название задачи
    pub summary: String,

    /// Родительская задача
    pub parent: Option<ParentIssue>,

    /// Массив с информацией об альтернативных ключах задачи
    #[serde(default)]
    pub aliases: Vec<String>,

    /// Последний сотрудник, изменявший задачу
    #[serde(rename = "updatedBy")]
    pub updated_by: Option<User>,

    /// Описание задачи
    pub description: Option<String>,

    /// Спринты
    #[serde(default)]
    pub sprint: Vec<Sprint>,

    /// Тип задачи
    #[serde(rename = "type")]
    pub issue_type: Option<IssueType>,

    /// Приоритет
    pub priority: Option<Priority>,

    /// Дата и время создания задачи
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,

    /// Наблюдатели задачи
    #[serde(default)]
    pub followers: Vec<User>,

    /// Создатель задачи
    #[serde(rename = "createdBy")]
    pub created_by: Option<User>,

    /// Количество голосов за задачу
    #[serde(default)]
    pub votes: u32,

    /// Исполнитель задачи
    pub assignee: Option<User>,

    /// Проекты задачи
    pub project: Option<Project>,

    /// Очередь задачи
    pub queue: Option<Queue>,

    /// Дата и время последнего обновления задачи
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,

    /// Статус задачи
    pub status: Option<Status>,

    /// Предыдущий статус задачи
    #[serde(rename = "previousStatus")]
    pub previous_status: Option<Status>,

    /// Признак избранной задачи
    #[serde(default)]
    pub favorite: bool,

    /// Теги задачи
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Параметры запроса для получения задачи
#[derive(Debug, Clone, Default)]
pub struct GetIssueParams {
    /// Дополнительные поля для включения в ответ
    pub expand: Vec<ExpandField>,
}

/// Дополнительные поля для включения в ответ
#[derive(Debug, Clone, Copy)]
pub enum ExpandField {
    /// Переходы по жизненному циклу
    Transitions,
    /// Вложения
    Attachments,
}

impl ExpandField {
    fn as_str(&self) -> &str {
        match self {
            ExpandField::Transitions => "transitions",
            ExpandField::Attachments => "attachments",
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_deserialization() {
        let json = r#"{
            "self": "https://st-api.yandex-team.ru/v3/issues/TREK-9844",
            "id": "593cd211ef7e8a33",
            "key": "TREK-9844",
            "version": 7,
            "summary": "Test task",
            "aliases": ["JUNE-3"],
            "votes": 5,
            "favorite": false,
            "tags": ["bug", "urgent"]
        }"#;

        let issue: Issue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.key, "TREK-9844");
        assert_eq!(issue.summary, "Test task");
        assert_eq!(issue.version, Some(7));
        assert_eq!(issue.votes, 5);
        assert!(!issue.favorite);
        assert_eq!(issue.tags.len(), 2);
        assert_eq!(issue.aliases.len(), 1);
    }

    #[test]
    fn test_issue_minimal_deserialization() {
        let json = r#"{
            "key": "TEST-1",
            "summary": "Minimal task"
        }"#;

        let issue: Issue = serde_json::from_str(json).unwrap();
        assert_eq!(issue.key, "TEST-1");
        assert_eq!(issue.summary, "Minimal task");
        assert_eq!(issue.votes, 0);
        assert!(!issue.favorite);
        assert!(issue.tags.is_empty());
    }

    #[test]
    fn test_expand_field_as_str() {
        assert_eq!(ExpandField::Transitions.as_str(), "transitions");
        assert_eq!(ExpandField::Attachments.as_str(), "attachments");
    }
}
