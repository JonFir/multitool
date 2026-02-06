//! Команда для получения информации о задаче

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

    // Форматируем данные для вывода
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

    // Выводим информацию в красивом формате
    println!();
    println!("📋 Задача: {}", key);
    println!();
    println!("📌 Заголовок:");
    println!("   {}", title);
    println!();
    println!("🔖 Статус: {}", status);
    println!();
    println!("📝 Описание:");
    // Разбиваем описание на строки для лучшей читаемости
    for line in description.lines() {
        println!("   {}", line);
    }
    println!();
    println!("🔗 Ссылка:");
    println!("   {}", link);
    println!();

    info!(
        issue_key = %key,
        issue_status = %status,
        "Информация о задаче получена и выведена успешно"
    );

    Ok(())
}
