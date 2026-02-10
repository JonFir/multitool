use clap::{Parser, Subcommand};
use tracing::info;

mod tracker;
use tracker::TrackerCommands;

mod llm;
use llm::LlmCommands;

mod tui;

#[derive(Parser)]
#[command(name = "you")]
#[command(about = "CLI утилита для управления рабочими задачами", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Работа с трекером задач
    Tracker {
        #[command(subcommand)]
        command: TrackerCommands,
    },
    /// Работа с LLM (AI ассистент)
    Llm {
        #[command(subcommand)]
        command: LlmCommands,
    },
    /// Интерактивный TUI режим
    Tui,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Инициализация трейсинга
    use tracing_subscriber::{fmt, EnvFilter};

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Запуск CLI");

    let cli = Cli::parse();

    match cli.command {
        Commands::Tracker { command } => command.execute().await?,
        Commands::Llm { command } => command.execute().await?,
        Commands::Tui => tui::run_tui().await?,
    }

    Ok(())
}
