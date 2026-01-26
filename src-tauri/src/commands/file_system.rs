use tauri::command;
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

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
