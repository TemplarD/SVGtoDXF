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
    pub path: String,
    pub size: u64,
    pub status: FileStatus,
}

/// Настройки конвертации (совпадают с backend ConversionOptions)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversionOptions {
    /// Сохранять цвета (fill/stroke) в DXF
    pub preserve_colors: bool,
    /// Точные цвета (true-color) вместо приближённых ACI
    pub true_color: bool,
    /// Заливку рисовать параллельными линиями
    pub fill_as_lines: bool,
    /// Шаг заливки линиями (в DXF-единицах)
    pub fill_step: f64,
    /// Трассировать растровые изображения
    pub trace_raster: bool,
    /// Порог яркости для трассировки растра (0..255)
    pub raster_threshold: u8,
    /// Заменять существующие DXF. Если false — к имени добавляется
    /// уникальный индекс (_1, _2, …), чтобы не перезаписать результат.
    pub overwrite: bool,
    /// Добавлять к имени суффикс `_color` (активно при preserve_colors).
    pub add_color_suffix: bool,
    /// Добавлять к имени суффикс `_hatch` (активно при fill_as_lines).
    pub add_hatch_suffix: bool,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            preserve_colors: true,
            true_color: false,
            fill_as_lines: false,
            fill_step: 2.0,
            trace_raster: true,
            raster_threshold: 128,
            overwrite: false,
            add_color_suffix: false,
            add_hatch_suffix: false,
        }
    }
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
