//! Команды для работы с трекером задач

use anyhow::Result;
use clap::Subcommand;
use tracing::{info, instrument};
use tracker_lib::task::format_issue_output;
use tracker_lib::TrackerClient;

/// Команды для работы с трекером задач
#[derive(Subcommand)]
pub enum TrackerCommands {
    /// Получить информацию о задаче
    Issue {
        /// Идентификатор или ключ задачи (например, TREK-123)
        issue_id: String,
    },
}

impl TrackerCommands {
    /// Выполняет команду трекера
    ///
    /// # Возвращает
    ///
    /// Ok(()) при успешном выполнении команды
    pub async fn execute(&self) -> Result<()> {
        match self {
            TrackerCommands::Issue { issue_id } => execute_issue(issue_id).await,
        }
    }
}

/// Выполняет команду получения информации о задаче
///
/// # Параметры
///
/// * `issue_id` - Идентификатор или ключ задачи
///
/// # Возвращает
///
/// Ok(()) при успешном выполнении
#[instrument(fields(issue_id = %issue_id))]
async fn execute_issue(issue_id: &str) -> Result<()> {
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
