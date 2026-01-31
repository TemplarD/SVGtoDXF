//! API модуль для взаимодействия с UI

use tauri::{Emitter, Window};
use svg2dxf_core::{SvgConverter, FileConversionResult};
use std::path::Path;

/// API endpoint для выбора выходной папки через системный диалог
#[tauri::command]
pub async fn api_select_output_folder() -> Result<String, String> {
    use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
    
    // Используем системный диалог выбора папки
    let folder_path = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    
    // В реальном коде здесь будет вызов системного диалога
    // Но пока используем текущую директорию как запасной вариант
    Ok(folder_path)
}

/// API endpoint для выбора SVG файлов через системный диалог
#[tauri::command]
pub async fn api_select_files() -> Result<Vec<String>, String> {
    // Временная реализация - возвращаем тестовый файл
    let test_file = "/media/templard/4EAE65D6AE65B6DD2/projects/svgtodxf/test.svg";
    
    if Path::new(test_file).exists() {
        Ok(vec![test_file.to_string()])
    } else {
        Ok(vec![])
    }
}

/// API endpoint для конвертации файлов
#[tauri::command]
pub async fn api_convert_files(
    files: Vec<String>,
    output_folder: String,
    window: Window,
) -> Result<Vec<FileConversionResult>, String> {
    let mut results = Vec::new();
    let mut converter = SvgConverter::new();
    
    // Создаем выходную папку если не существует
    std::fs::create_dir_all(&output_folder).map_err(|e| e.to_string())?;
    
    for (i, file_path) in files.iter().enumerate() {
        // Отправляем статус в UI
        let _ = window.emit("conversion_status", format!("Обработка файла {}: {}", i + 1, file_path));
        
        // Реальная конвертация через core модуль
        let input_path = Path::new(file_path);
        let output_path = Path::new(&output_folder)
            .join(format!("{}.dxf", input_path.file_stem().unwrap().to_string_lossy()));
        
        match converter.convert_file(input_path, &output_path) {
            Ok(_) => {
                let result = FileConversionResult {
                    success: true,
                    input_file: file_path.clone(),
                    output_file: output_path.to_string_lossy().to_string(),
                    message: format!("Файл успешно конвертирован"),
                    error: None,
                };
                results.push(result);
                
                // Отправляем статус об успешной конвертации
                let _ = window.emit("conversion_complete", format!("Файл {} успешно конвертирован", file_path));
            }
            Err(e) => {
                let result = FileConversionResult {
                    success: false,
                    input_file: file_path.clone(),
                    output_file: output_path.to_string_lossy().to_string(),
                    message: "Ошибка конвертации".to_string(),
                    error: Some(e.to_string()),
                };
                results.push(result);
                
                // Отправляем статус об ошибке
                let _ = window.emit("conversion_error", format!("Ошибка конвертации файла {}: {}", file_path, e));
            }
        }
    }
    
    Ok(results)
}

/// API endpoint для получения статуса
#[tauri::command]
pub async fn api_get_status() -> Result<String, String> {
    Ok("Готов к работе".to_string())
}
