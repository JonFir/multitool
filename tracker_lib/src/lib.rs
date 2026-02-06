//! # Yandex Tracker API Client Library
//!
//! Библиотека для работы с API Яндекс.Трекера.
//!
//! ## Пример использования
//!
//! ```no_run
//! use tracker_lib::{TrackerClient, TrackerConfig, Language};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Создание клиента с минимальной конфигурацией
//!     let client = TrackerClient::with_token("your-oauth-token")?;
//!
//!     // Получение информации о задаче
//!     let issue = client.get_issue("TEST-1", None).await?;
//!     println!("Задача: {} - {}", issue.key, issue.summary);
//!
//!     Ok(())
//! }
//! ```

mod api_client;
pub mod task;

pub use api_client::{
    Language, PaginationMeta, PaginationParams, Result, TrackerClient, TrackerConfig, TrackerError,
};
