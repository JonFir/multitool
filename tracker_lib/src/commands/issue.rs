//! Команда для получения информации о задаче

use crate::models::Issue;
use crate::TrackerClient;
use anyhow::Result;
use tracing::{info, instrument};

/// Выполняет команду получения информации о задаче
///
/// # Параметры
///
/// * `issue_id` - Идентификатор или ключ задачи
///
/// # Возвращает
///
/// Ok(()) при успешном выполнении
///
/// # Примеры
///
/// ```no_run
/// # use tracker_lib::commands::issue::execute;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// execute("TREK-123").await?;
/// # Ok(())
/// # }
/// ```
#[instrument(fields(issue_id = %issue_id))]
pub async fn execute(issue_id: &str) -> Result<()> {
    info!("Выполнение команды issue для задачи: {}", issue_id);

    // Создаём клиент из переменной окружения
    let client = TrackerClient::from_env()?;

    // Получаем информацию о задаче
    let issue = client.get_issue(issue_id, None).await?;

    // Форматируем и выводим информацию
    let output = format_issue_output(&issue);
    println!("{}", output);

    let status = issue
        .status
        .as_ref()
        .and_then(|s| s.display.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Неизвестен");
    info!(
        issue_key = %issue.key,
        issue_status = %status,
        "Информация о задаче получена и выведена успешно"
    );

    Ok(())
}

/// Форматирует полный вывод информации о задаче (чистая функция)
///
/// # Параметры
///
/// * `issue` - Задача из Трекера
///
/// # Возвращает
///
/// Отформатированную строку для вывода в консоль
fn format_issue_output(issue: &Issue) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Issue, Status};

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
