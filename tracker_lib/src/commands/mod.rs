//! CLI команды для работы с Яндекс.Трекером

use anyhow::Result;
use clap::Subcommand;

pub mod issue;

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
    ///
    /// # Примеры
    ///
    /// ```no_run
    /// # use tracker_lib::commands::TrackerCommands;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let command = TrackerCommands::Issue {
    ///     issue_id: "TREK-123".to_string(),
    /// };
    /// command.execute().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute(&self) -> Result<()> {
        match self {
            TrackerCommands::Issue { issue_id } => issue::execute(issue_id).await,
        }
    }
}
