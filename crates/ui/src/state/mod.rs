//! Модуль управления состоянием UI приложения

use serde::{Deserialize, Serialize};

/// Статусы файла
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileStatus {
    Pending,
    Processing,
    Completed,
    Error(String),
}

/// Информация о файле
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileItem {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub status: FileStatus,
}

/// Простое состояние для теста
pub struct AppState {
    pub message: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            message: "Приложение запущено".to_string(),
        }
    }
}
