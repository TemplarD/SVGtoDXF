use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
    pub context: Option<std::collections::HashMap<String, String>>,
}

pub struct Logger {
    log_dir: String,
    current_file: Option<File>,
    current_date: String,
}

impl Logger {
    pub fn new(log_dir: &str) -> Result<Self, String> {
        let mut logger = Logger {
            log_dir: log_dir.to_string(),
            current_file: None,
            current_date: String::new(),
        };
        
        // Создаем директорию для логов
        std::fs::create_dir_all(log_dir)
            .map_err(|e| format!("Не удалось создать директорию логов: {}", e))?;
        
        // Открываем файл для сегодняшнего дня
        logger.rotate_if_needed()?;
        
        Ok(logger)
    }
    
    /// Проверяет нужно ли ротировать лог файл (новый день)
    fn rotate_if_needed(&mut self) -> Result<(), String> {
        let today = Local::now().format("%Y-%m-%d").to_string();
        
        if self.current_date != today {
            self.current_date = today;
            
            // Закрываем старый файл если был открыт
            if let Some(mut file) = self.current_file.take() {
                let _ = file.flush();
            }
            
            // Открываем новый файл для сегодняшнего дня
            let filename = format!("svg-to-dxf-{}.log", self.current_date);
            let filepath = Path::new(&self.log_dir).join(&filename);
            
            self.current_file = Some(OpenOptions::new()
                .create(true)
                .append(true)
                .open(&filepath)
                .map_err(|e| format!("Не удалось открыть файл логов {}: {}", filepath.display(), e))?);
        }
        
        Ok(())
    }
    
    /// Записывает лог в файл
    pub fn log(&mut self, level: LogLevel, category: &str, message: &str, details: Option<&str>, context: Option<&std::collections::HashMap<String, String>>) -> Result<(), String> {
        self.rotate_if_needed()?;
        
        let timestamp = Local::now();
        let entry = LogEntry {
            timestamp,
            level: level.clone(),
            category: category.to_string(),
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
            context: context.cloned(),
        };
        
        // Форматируем строку для записи
        let log_line = format!(
            "[{}] [{}] [{}] {}\n",
            timestamp.format("%Y-%m-%d %H:%M:%S%.3f"),
            level.as_str(),
            category,
            message
        );
        
        // Добавляем детали если есть
        let mut full_log = log_line;
        if let Some(details) = &entry.details {
            full_log.push_str(&format!("  Details: {}\n", details));
        }
        if let Some(context) = &entry.context {
            full_log.push_str("  Context:\n");
            for (key, value) in context {
                full_log.push_str(&format!("    {}: {}\n", key, value));
            }
        }
        full_log.push('\n');
        
        // Записываем в файл
        if let Some(ref mut file) = self.current_file {
            file.write_all(full_log.as_bytes())
                .map_err(|e| format!("Не удалось записать в файл логов: {}", e))?;
            file.flush()
                .map_err(|e| format!("Не удалось сбросить буфер файла логов: {}", e))?;
        }
        
        // Также выводим в консоль для отладки
        match level {
            LogLevel::Debug => log::debug!("[{}] [{}] {}", category, message, details.unwrap_or("")),
            LogLevel::Info => log::info!("[{}] [{}] {}", category, message, details.unwrap_or("")),
            LogLevel::Warning => log::warn!("[{}] [{}] {}", category, message, details.unwrap_or("")),
            LogLevel::Error => log::error!("[{}] [{}] {}", category, message, details.unwrap_or("")),
            LogLevel::Critical => log::error!("[{}] [{}] {}", category, message, details.unwrap_or("")),
        }
        
        Ok(())
    }
    
    /// Получает список лог файлов
    pub fn get_log_files(&self) -> Result<Vec<String>, String> {
        let mut files = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(&self.log_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(filename) = entry.file_name().into_string() {
                        if filename.starts_with("svg-to-dxf-") && filename.ends_with(".log") {
                            files.push(filename);
                        }
                    }
                }
            }
        }
        
        files.sort();
        files.reverse(); // Новые файлы первыми
        
        Ok(files)
    }
    
    /// Читает содержимое лог файла
    pub fn read_log_file(&self, filename: &str) -> Result<String, String> {
        let filepath = Path::new(&self.log_dir).join(filename);
        
        std::fs::read_to_string(&filepath)
            .map_err(|e| format!("Не удалось прочитать файл логов {}: {}", filepath.display(), e))
    }
    
    /// Очищает старые логи (старше N дней)
    pub fn cleanup_old_logs(&self, days_to_keep: u32) -> Result<usize, String> {
        let mut removed_count = 0;
        let cutoff_date = Local::now() - chrono::Duration::days(days_to_keep as i64);
        
        if let Ok(entries) = std::fs::read_dir(&self.log_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            let modified_date: DateTime<Local> = modified.into();
                            if modified_date < cutoff_date {
                                if entry.file_name().to_string_lossy().starts_with("svg-to-dxf-") {
                                    let _ = std::fs::remove_file(entry.path());
                                    removed_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(removed_count)
    }
}

/// Глобальный логгер для приложения
static mut GLOBAL_LOGGER: Option<Logger> = None;
static mut LOGGER_INIT: std::sync::Once = std::sync::Once::new();

/// Инициализирует глобальный логгер
pub fn init_logger(log_dir: &str) -> Result<(), String> {
    unsafe {
        LOGGER_INIT.call_once(|| {
            match Logger::new(log_dir) {
                Ok(logger) => GLOBAL_LOGGER = Some(logger),
                Err(e) => log::error!("Не удалось инициализировать логгер: {}", e),
            }
        });
    }
    Ok(())
}

/// Записывает лог через глобальный логгер
pub fn log_message(level: LogLevel, category: &str, message: &str, details: Option<&str>, context: Option<&std::collections::HashMap<String, String>>) -> Result<(), String> {
    unsafe {
        if let Some(ref mut logger) = GLOBAL_LOGGER {
            logger.log(level, category, message, details, context)
        } else {
            // Если логгер не инициализирован, выводим в стандартный лог
            let msg = format!("[{}] [{}] {}", category, level.as_str(), message);
            match level {
                LogLevel::Debug => log::debug!("{}", msg),
                LogLevel::Info => log::info!("{}", msg),
                LogLevel::Warning => log::warn!("{}", msg),
                LogLevel::Error => log::error!("{}", msg),
                LogLevel::Critical => log::error!("{}", msg),
            }
            Ok(())
        }
    }
}

/// Удобные функции для разных уровней логирования
pub fn log_debug(category: &str, message: &str) -> Result<(), String> {
    log_message(LogLevel::Debug, category, message, None, None)
}

pub fn log_info(category: &str, message: &str) -> Result<(), String> {
    log_message(LogLevel::Info, category, message, None, None)
}

pub fn log_warning(category: &str, message: &str) -> Result<(), String> {
    log_message(LogLevel::Warning, category, message, None, None)
}

pub fn log_error(category: &str, message: &str) -> Result<(), String> {
    log_message(LogLevel::Error, category, message, None, None)
}

pub fn log_critical(category: &str, message: &str) -> Result<(), String> {
    log_message(LogLevel::Critical, category, message, None, None)
}
