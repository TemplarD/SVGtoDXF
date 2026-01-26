use tauri::command;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub extension: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub platform: String,
}

/// Получает список файлов и папок в директории
#[command]
pub async fn list_directory(path: String) -> Result<Vec<FileItem>, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err("Директория не существует".to_string());
    }
    
    if !path.is_dir() {
        return Err("Указанный путь не является директорией".to_string());
    }
    
    let mut items = Vec::new();
    
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let metadata = match entry.metadata() {
                            Ok(meta) => meta,
                            Err(_) => continue,
                        };
                        
                        let file_path = entry.path();
                        let name = file_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        
                        let size = if metadata.is_file() {
                            Some(metadata.len())
                        } else {
                            None
                        };
                        
                        let extension = if metadata.is_file() {
                            file_path.extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| ext.to_lowercase())
                        } else {
                            None
                        };
                        
                        items.push(FileItem {
                            name,
                            path: file_path.to_string_lossy().to_string(),
                            is_directory: metadata.is_dir(),
                            size,
                            extension,
                        });
                    }
                    Err(_) => continue,
                }
            }
            
            // Сортируем: сначала папки, потом файлы
            items.sort_by(|a, b| {
                match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
            
            Ok(items)
        }
        Err(e) => Err(format!("Ошибка чтения директории: {}", e)),
    }
}

/// Ищет SVG файлы в директории рекурсивно
#[command]
pub async fn get_svg_files_in_directory(path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&path);
    let mut svg_files = Vec::new();
    
    if !path.exists() {
        return Err("Директория не существует".to_string());
    }
    
    find_svg_files_recursive(path, &mut svg_files)?;
    
    Ok(svg_files)
}

/// Рекурсивный поиск SVG файлов
fn find_svg_files_recursive(dir: &Path, svg_files: &mut Vec<String>) -> Result<(), String> {
    if !dir.is_dir() {
        return Ok(());
    }
    
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        
                        if path.is_dir() {
                            // Рекурсивно ищем в подпапках
                            if let Err(e) = find_svg_files_recursive(&path, svg_files) {
                                log::debug!("Ошибка поиска в {}: {}", path.display(), e);
                            }
                        } else if path.is_file() {
                            // Проверяем расширение файла
                            if let Some(ext) = path.extension() {
                                if ext.to_string_lossy().to_lowercase() == "svg" {
                                    svg_files.push(path.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        Err(e) => return Err(format!("Ошибка чтения директории {}: {}", dir.display(), e)),
    }
    
    Ok(())
}

/// Получает корневые директории системы
#[command]
pub async fn get_system_roots() -> Result<Vec<String>, String> {
    let mut roots = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        // Для Windows получаем диски
        use std::path::PathBuf;
        
        for drive_letter in b'A'..=b'Z' {
            let drive_path = PathBuf::from(format!("{}:\\", drive_letter as char));
            if drive_path.exists() {
                roots.push(drive_path.to_string_lossy().to_string());
            }
        }
        
        // Добавляем сетевые пути если нужно
        roots.push("\\\\".to_string());
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Для Unix систем добавляем корень и домашнюю директорию
        roots.push("/".to_string());
        
        if let Some(home) = dirs::home_dir() {
            roots.push(home.to_string_lossy().to_string());
        }
        
        // Добавляем стандартные монтированные директории
        let mount_dirs = ["/home", "/mnt", "/media", "/Volumes"];
        for mount_dir in &mount_dirs {
            if Path::new(mount_dir).exists() {
                roots.push(mount_dir.to_string());
            }
        }
    }
    
    Ok(roots)
}

/// Получает системную информацию
#[command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();
    let platform = format!("{}-{}", os, arch);
    
    Ok(SystemInfo {
        os,
        arch,
        platform,
    })
}

/// Проверяет существует ли файл
#[command]
pub async fn file_exists(path: String) -> Result<bool, String> {
    Ok(Path::new(&path).exists())
}

/// Получает родительскую директорию
#[command]
pub async fn get_parent_directory(path: String) -> Result<Option<String>, String> {
    let path = Path::new(&path);
    
    match path.parent() {
        Some(parent) => Ok(Some(parent.to_string_lossy().to_string())),
        None => Ok(None),
    }
}

/// Проверяет доступность директории для записи
#[command]
pub async fn check_directory_writable(path: String) -> Result<bool, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        // Пробуем создать директорию
        match fs::create_dir_all(path) {
            Ok(_) => return Ok(true),
            Err(e) => return Err(format!("Не удалось создать директорию: {}", e)),
        }
    }
    
    // Пробуем создать тестовый файл
    let test_file = path.join(".write_test_12345.tmp");
    match fs::write(&test_file, "test") {
        Ok(_) => {
            // Удаляем тестовый файл
            let _ = fs::remove_file(&test_file);
            Ok(true)
        }
        Err(e) => {
            Err(format!("Нет прав записи в директорию: {}", e))
        }
    }
}

/// Проверяет права доступа к файлу или директории
#[command]
pub async fn check_file_permissions(path: String) -> Result<FilePermissions, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err("Файл или директория не существует".to_string());
    }
    
    let metadata = fs::metadata(path)
        .map_err(|e| format!("Не удалось получить метаданные: {}", e))?;
    
    let readonly = metadata.permissions().readonly();
    
    // Проверяем права записи
    let can_write = if readonly {
        false
    } else {
        // Дополнительная проверка для директорий
        if path.is_dir() {
            check_directory_writable(path.to_string_lossy().to_string()).await.unwrap_or(false)
        } else {
            !readonly
        }
    };
    
    // Проверяем права чтения
    let can_read = metadata.permissions().readonly() == false;
    
    // Проверяем права выполнения (для директорий)
    let can_execute = path.is_dir() && metadata.permissions().readonly() == false;
    
    Ok(FilePermissions {
        path: path.to_string_lossy().to_string(),
        can_read,
        can_write,
        can_execute,
        is_readonly: readonly,
        is_directory: path.is_dir(),
        file_size: if path.is_file() {
            Some(metadata.len())
        } else {
            None
        },
        modified_time: metadata.modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs()),
    })
}

/// Находит доступные директории для сохранения
#[command]
pub async fn find_writable_directories(base_path: String) -> Result<Vec<WritableDirectory>, String> {
    let base_path = Path::new(&base_path);
    let mut writable_dirs = Vec::new();
    
    // Проверяем базовую директорию
    if let Ok(writable) = check_directory_writable(base_path.to_string_lossy().to_string()).await {
        if writable {
            writable_dirs.push(WritableDirectory {
                path: base_path.to_string_lossy().to_string(),
                name: base_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("base")
                    .to_string(),
                is_default: true,
                available_space: get_available_space(&base_path).await,
            });
        }
    }
    
    // Проверяем домашнюю директорию пользователя
    if let Some(home_dir) = dirs::home_dir() {
        if let Ok(writable) = check_directory_writable(home_dir.to_string_lossy().to_string()).await {
            if writable {
                writable_dirs.push(WritableDirectory {
                    path: home_dir.to_string_lossy().to_string(),
                    name: "Домашняя директория".to_string(),
                    is_default: false,
                    available_space: get_available_space(&home_dir).await,
                });
            }
        }
    }
    
    // Проверяем временную директорию
    if let Some(temp_dir) = std::env::temp_dir().to_str() {
        let temp_path = Path::new(temp_dir).join("svg-to-dxf-converter");
        if let Ok(writable) = check_directory_writable(temp_path.to_string_lossy().to_string()).await {
            if writable {
                writable_dirs.push(WritableDirectory {
                    path: temp_path.to_string_lossy().to_string(),
                    name: "Временная директория".to_string(),
                    is_default: false,
                    available_space: get_available_space(&temp_path).await,
                });
            }
        }
    }
    
    // Проверяем директорию документов
    if let Some(doc_dir) = dirs::document_dir() {
        if let Ok(writable) = check_directory_writable(doc_dir.to_string_lossy().to_string()).await {
            if writable {
                writable_dirs.push(WritableDirectory {
                    path: doc_dir.to_string_lossy().to_string(),
                    name: "Документы".to_string(),
                    is_default: false,
                    available_space: get_available_space(&doc_dir).await,
                });
            }
        }
    }
    
    // Проверяем директорию рабочего стола
    if let Some(desktop_dir) = dirs::desktop_dir() {
        if let Ok(writable) = check_directory_writable(desktop_dir.to_string_lossy().to_string()).await {
            if writable {
                writable_dirs.push(WritableDirectory {
                    path: desktop_dir.to_string_lossy().to_string(),
                    name: "Рабочий стол".to_string(),
                    is_default: false,
                    available_space: get_available_space(&desktop_dir).await,
                });
            }
        }
    }
    
    Ok(writable_dirs)
}

/// Проверяет доступность системных директорий
#[command]
pub async fn check_system_directories() -> Result<SystemDirectoriesCheck, String> {
    let mut results = Vec::new();
    
    // Проверяем домашнюю директорию
    if let Some(home_dir) = dirs::home_dir() {
        let home_path = home_dir.to_string_lossy().to_string();
        let writable = check_directory_writable(home_path.clone()).await.unwrap_or(false);
        
        results.push(DirectoryCheckResult {
            path: home_path,
            name: "Домашняя директория".to_string(),
            exists: true,
            is_directory: true,
            is_writable: writable,
            is_readable: true,
            error: if !writable { Some("Нет прав записи".to_string()) } else { None },
        });
    } else {
        results.push(DirectoryCheckResult {
            path: "N/A".to_string(),
            name: "Домашняя директория".to_string(),
            exists: false,
            is_directory: false,
            is_writable: false,
            is_readable: false,
            error: Some("Домашняя директория не найдена".to_string()),
        });
    }
    
    // Проверяем временную директорию
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.to_string_lossy().to_string();
    let temp_writable = check_directory_writable(temp_path.clone()).await.unwrap_or(false);
    
    results.push(DirectoryCheckResult {
        path: temp_path,
        name: "Временная директория".to_string(),
        exists: true,
        is_directory: true,
        is_writable: temp_writable,
        is_readable: true,
        error: if !temp_writable { Some("Нет прав записи".to_string()) } else { None },
    });
    
    // Проверяем директорию документов
    if let Some(doc_dir) = dirs::document_dir() {
        let doc_path = doc_dir.to_string_lossy().to_string();
        let doc_writable = check_directory_writable(doc_path.clone()).await.unwrap_or(false);
        
        results.push(DirectoryCheckResult {
            path: doc_path,
            name: "Документы".to_string(),
            exists: true,
            is_directory: true,
            is_writable: doc_writable,
            is_readable: true,
            error: if !doc_writable { Some("Нет прав записи".to_string()) } else { None },
        });
    } else {
        results.push(DirectoryCheckResult {
            path: "N/A".to_string(),
            name: "Документы".to_string(),
            exists: false,
            is_directory: false,
            is_writable: false,
            is_readable: false,
            error: Some("Директория документов не найдена".to_string()),
        });
    }
    
    // Проверяем директорию рабочего стола
    if let Some(desktop_dir) = dirs::desktop_dir() {
        let desktop_path = desktop_dir.to_string_lossy().to_string();
        let desktop_writable = check_directory_writable(desktop_path.clone()).await.unwrap_or(false);
        
        results.push(DirectoryCheckResult {
            path: desktop_path,
            name: "Рабочий стол".to_string(),
            exists: true,
            is_directory: true,
            is_writable: desktop_writable,
            is_readable: true,
            error: if !desktop_writable { Some("Нет прав записи".to_string()) } else { None },
        });
    } else {
        results.push(DirectoryCheckResult {
            path: "N/A".to_string(),
            name: "Рабочий стол".to_string(),
            exists: false,
            is_directory: false,
            is_writable: false,
            is_readable: false,
            error: Some("Рабочий стол не найден".to_string()),
        });
    }
    
    let total_dirs = results.len();
    let accessible_dirs = results.iter().filter(|r| r.exists && r.is_writable).count();
    let readonly_dirs = results.iter().filter(|r| r.exists && !r.is_writable).count();
    
    Ok(SystemDirectoriesCheck {
        results,
        summary: SystemDirectoriesSummary {
            total_directories: total_dirs,
            accessible_directories: accessible_dirs,
            readonly_directories: readonly_dirs,
            missing_directories: total_dirs - accessible_dirs - readonly_dirs,
            has_write_access: accessible_dirs > 0,
        },
    })
}

/// Получает доступное место на диске
async fn get_available_space(_path: &Path) -> Option<u64> {
    // В реальном приложении здесь можно использовать системные вызовы
    // Для примера возвращаем фиксированное значение
    Some(1024 * 1024 * 1024) // 1GB
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilePermissions {
    pub path: String,
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub is_readonly: bool,
    pub is_directory: bool,
    pub file_size: Option<u64>,
    pub modified_time: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WritableDirectory {
    pub path: String,
    pub name: String,
    pub is_default: bool,
    pub available_space: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryCheckResult {
    pub path: String,
    pub name: String,
    pub exists: bool,
    pub is_directory: bool,
    pub is_writable: bool,
    pub is_readable: bool,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDirectoriesCheck {
    pub results: Vec<DirectoryCheckResult>,
    pub summary: SystemDirectoriesSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemDirectoriesSummary {
    pub total_directories: usize,
    pub accessible_directories: usize,
    pub readonly_directories: usize,
    pub missing_directories: usize,
    pub has_write_access: bool,
}

/// Проверяет доступность директории только для чтения
#[command]
pub async fn check_directory_readonly(path: String) -> Result<bool, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err("Директория не существует".to_string());
    }
    
    // Проверяем права чтения
    match fs::read_dir(path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Ok(false)
            } else {
                Err(format!("Ошибка доступа к директории: {}", e))
            }
        }
    }
}

/// Проверяет, является ли директория только для чтения
#[command]
pub async fn is_directory_readonly(path: String) -> Result<bool, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err("Директория не существует".to_string());
    }
    
    // Проверяем права записи
    let test_file = path.join(".readonly_test_12345.tmp");
    match fs::write(&test_file, "test") {
        Ok(_) => {
            // Если можем записать, удаляем тестовый файл
            let _ = fs::remove_file(&test_file);
            Ok(false) // Не только для чтения
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                Ok(true) // Только для чтения
            } else {
                Err(format!("Ошибка проверки прав записи: {}", e))
            }
        }
    }
}

/// Находит альтернативные директории для сохранения
#[command]
pub async fn find_alternative_save_directories(original_path: String) -> Result<Vec<AlternativeSaveDirectory>, String> {
    let original_path = Path::new(&original_path);
    let mut alternatives = Vec::new();
    
    // Если оригинальная директория доступна для записи, возвращаем ее
    if let Ok(writable) = check_directory_writable(original_path.to_string_lossy().to_string()).await {
        alternatives.push(AlternativeSaveDirectory {
            path: original_path.to_string_lossy().to_string(),
            name: "Оригинальная директория".to_string(),
            is_available: true,
            is_preferred: true,
            reason: "Директория доступна для записи".to_string(),
            available_space: get_available_space(&original_path).await,
        });
    }
    
    // Проверяем родительскую директорию
    if let Some(parent) = original_path.parent() {
        let parent_path = parent.to_string_lossy().to_string();
        if let Ok(_writable) = check_directory_writable(parent_path.clone()).await {
            alternatives.push(AlternativeSaveDirectory {
                path: parent_path.clone(),
                name: "Родительская директория".to_string(),
                is_available: true,
                is_preferred: false,
                reason: "Родительская директория доступна для записи".to_string(),
                available_space: get_available_space(Path::new(&parent_path)).await,
            });
        }
    }
    
    // Проверяем домашнюю директорию
    if let Some(home_dir) = dirs::home_dir() {
        let home_path = home_dir.to_string_lossy().to_string();
        if let Ok(_writable) = check_directory_writable(home_path.clone()).await {
            alternatives.push(AlternativeSaveDirectory {
                path: home_path.clone(),
                name: "Домашняя директория".to_string(),
                is_available: true,
                is_preferred: false,
                reason: "Домашняя директория пользователя".to_string(),
                available_space: get_available_space(Path::new(&home_path)).await,
            });
        }
    }
    
    // Проверяем директорию документов
    if let Some(doc_dir) = dirs::document_dir() {
        let doc_path = doc_dir.to_string_lossy().to_string();
        if let Ok(_writable) = check_directory_writable(doc_path.clone()).await {
            alternatives.push(AlternativeSaveDirectory {
                path: doc_path.clone(),
                name: "Документы".to_string(),
                is_available: true,
                is_preferred: false,
                reason: "Директория документов".to_string(),
                available_space: get_available_space(Path::new(&doc_path)).await,
            });
        }
    }
    
    // Проверяем директорию рабочего стола
    if let Some(desktop_dir) = dirs::desktop_dir() {
        let desktop_path = desktop_dir.to_string_lossy().to_string();
        if let Ok(_writable) = check_directory_writable(desktop_path.clone()).await {
            alternatives.push(AlternativeSaveDirectory {
                path: desktop_path.clone(),
                name: "Рабочий стол".to_string(),
                is_available: true,
                is_preferred: false,
                reason: "Рабочий стол".to_string(),
                available_space: get_available_space(Path::new(&desktop_path)).await,
            });
        }
    }
    
    // Проверяем временную директорию
    let temp_dir = std::env::temp_dir();
    let temp_path = temp_dir.join("svg-to-dxf-converter");
    if let Ok(writable) = check_directory_writable(temp_path.to_string_lossy().to_string()).await {
        alternatives.push(AlternativeSaveDirectory {
            path: temp_path.to_string_lossy().to_string(),
            name: "Временная директория".to_string(),
            is_available: true,
            is_preferred: false,
            reason: "Временная директория системы".to_string(),
            available_space: get_available_space(&temp_path).await,
        });
    }
    
    // Проверяем директорию загрузок
    if let Some(download_dir) = dirs::download_dir() {
        let download_path = download_dir.join("SVG-to-DXF");
        if let Ok(writable) = check_directory_writable(download_path.to_string_lossy().to_string()).await {
            alternatives.push(AlternativeSaveDirectory {
                path: download_path.to_string_lossy().to_string(),
                name: "Загрузки".to_string(),
                is_available: true,
                is_preferred: false,
                reason: "Директория загрузок".to_string(),
                available_space: get_available_space(&download_path).await,
            });
        }
    }
    
    Ok(alternatives)
}

/// Проверяет и предлагает альтернативные пути сохранения
#[command]
pub async fn suggest_save_path(original_path: String) -> Result<SavePathSuggestion, String> {
    let alternatives = find_alternative_save_directories(original_path.clone()).await?;
    
    if alternatives.is_empty() {
        return Err("Не найдено доступных директорий для сохранения".to_string());
    }
    
    let preferred = alternatives.iter().find(|d| d.is_preferred && d.is_available);
    let available: Vec<_> = alternatives.iter().filter(|d| d.is_available).collect();
    
    if available.is_empty() {
        return Err("Нет доступных директорий для сохранения".to_string());
    }
    
    Ok(SavePathSuggestion {
        original_path,
        suggested_path: preferred.unwrap_or(&available[0]).path.clone(),
        alternatives: available.iter().map(|d| (*d).clone()).collect(),
        reason: if preferred.is_some() {
            "Оригинальная директория доступна".to_string()
        } else {
            "Оригинальная директория недоступна, предложены альтернативы".to_string()
        },
    })
}

/// Проверяет права доступа к директории
#[command]
pub async fn check_directory_access_detailed(path: String) -> Result<DirectoryAccessInfo, String> {
    let path = Path::new(&path);
    
    if !path.exists() {
        return Err("Директория не существует".to_string());
    }
    
    let mut access_info = DirectoryAccessInfo {
        path: path.to_string_lossy().to_string(),
        exists: true,
        is_directory: path.is_dir(),
        can_read: false,
        can_write: false,
        can_execute: false,
        is_readonly: false,
        file_count: 0,
        directory_count: 0,
        total_size: 0,
        permissions: Vec::new(),
        error: None,
    };
    
    // Проверяем права чтения
    match fs::read_dir(path) {
        Ok(entries) => {
            access_info.can_read = true;
            
            for entry in entries {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                
                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(_) => continue,
                };
                
                match entry.file_type() {
                    Ok(file_type) => {
                        if file_type.is_dir() {
                            access_info.directory_count += 1;
                        } else {
                            access_info.file_count += 1;
                            access_info.total_size += metadata.len();
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                access_info.error = Some("Отказано в доступе".to_string());
            } else {
                access_info.error = Some(format!("Ошибка чтения: {}", e));
            }
        }
    }
    
    // Проверяем права записи
    let test_file = path.join(".write_test_12345.tmp");
    match fs::write(&test_file, "test") {
        Ok(_) => {
            access_info.can_write = true;
            let _ = fs::remove_file(&test_file);
        }
        Err(_) => {
            access_info.can_write = false;
            access_info.is_readonly = true;
        }
    }
    
    // Проверяем права выполнения (для директорий)
    if path.is_dir() {
        access_info.can_execute = access_info.can_read;
    }
    
    // Получаем информацию о правах доступа
    let metadata = match fs::metadata(path) {
        Ok(meta) => meta,
        Err(_) => {
            return Err("Не удалось получить метаданные".to_string());
        }
    };
    
    let permissions = metadata.permissions();
    access_info.is_readonly = permissions.readonly();
    
    // Добавляем информацию о правах
    access_info.permissions.push(format!("Read: {}", !permissions.readonly()));
    access_info.permissions.push(format!("Write: {}", !permissions.readonly()));
    access_info.permissions.push(format!("Execute: {}", path.is_dir()));
    
    Ok(access_info)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryAccessInfo {
    pub path: String,
    pub exists: bool,
    pub is_directory: bool,
    pub can_read: bool,
    pub can_write: bool,
    pub can_execute: bool,
    pub is_readonly: bool,
    pub file_count: usize,
    pub directory_count: usize,
    pub total_size: u64,
    pub permissions: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AlternativeSaveDirectory {
    pub path: String,
    pub name: String,
    pub is_available: bool,
    pub is_preferred: bool,
    pub reason: String,
    pub available_space: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SavePathSuggestion {
    pub original_path: String,
    pub suggested_path: String,
    pub alternatives: Vec<AlternativeSaveDirectory>,
    pub reason: String,
}
