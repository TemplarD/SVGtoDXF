//! API модуль для взаимодействия с UI (Tauri v2)

use svg2dxf_core::{convert_svg_to_dxf_with_options, ConversionOptions, FileConversionResult};
use tauri::{Emitter, Window};
use tauri_plugin_dialog::DialogExt;

/// API endpoint для выбора выходной папки через системный диалог
#[tauri::command]
pub async fn api_select_output_folder(app: tauri::AppHandle) -> Result<String, String> {
    // Асинхронный выбор папки через плагин dialog
    let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();
    app.dialog()
        .file()
        .pick_folder(move |folder| {
            let _ = tx.send(folder.map(|p| p.to_string()));
        });
    match rx.recv().unwrap_or(None) {
        Some(path) => Ok(path),
        None => Err("Папка не выбрана".to_string()),
    }
}

/// API endpoint для выбора SVG файлов через системный диалог
#[tauri::command]
pub async fn api_select_files(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let (tx, rx) = std::sync::mpsc::channel::<Option<Vec<String>>>();
    app.dialog()
        .file()
        .add_filter("SVG файлы", &["svg"])
        .pick_files(move |files| {
            let paths = files.map(|list| {
                list.into_iter().map(|p| p.to_string()).collect::<Vec<_>>()
            });
            let _ = tx.send(paths);
        });
    match rx.recv().unwrap_or(None) {
        Some(paths) => Ok(paths),
        None => Ok(vec![]),
    }
}

/// API endpoint для конвертации файлов
#[tauri::command]
pub async fn api_convert_files(
    files: Vec<String>,
    output_folder: String,
    options: ConversionOptions,
    window: Window,
) -> Result<Vec<FileConversionResult>, String> {
    let mut results = Vec::new();

    // Создаем выходную папку если не существует
    std::fs::create_dir_all(&output_folder).map_err(|e| e.to_string())?;

    for (i, file_path) in files.iter().enumerate() {
        let _ = window.emit(
            "conversion_status",
            format!("Обработка файла {}: {}", i + 1, file_path),
        );

        let input_path = std::path::Path::new(file_path);
        let stem = input_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let output_path = std::path::Path::new(&output_folder).join(format!("{}.dxf", stem));

        match convert_svg_to_dxf_with_options(file_path, output_path.to_str().unwrap_or(""), options.clone()) {
            Ok(_) => {
                let result = FileConversionResult {
                    success: true,
                    input_file: file_path.clone(),
                    output_file: output_path.to_string_lossy().to_string(),
                    message: "Файл успешно конвертирован".to_string(),
                    error: None,
                };
                results.push(result);
                let _ = window.emit(
                    "conversion_complete",
                    format!("Файл {} успешно конвертирован", file_path),
                );
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
                let _ = window.emit(
                    "conversion_error",
                    format!("Ошибка конвертации файла {}: {}", file_path, e),
                );
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

/// API endpoint для получения размера файла в байтах.
#[tauri::command]
pub async fn api_get_file_size(path: String) -> Result<u64, String> {
    let meta = std::fs::metadata(&path).map_err(|e| e.to_string())?;
    Ok(meta.len())
}
