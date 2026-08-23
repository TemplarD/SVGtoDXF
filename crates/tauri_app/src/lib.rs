//! SVG to DXF Tauri App Module

use svg2dxf_core::SvgConverter;
use tauri::State;

mod api;

#[cfg(test)]
mod tests;

/// Состояние приложения
pub struct AppState {
    pub converter: std::sync::Arc<std::sync::Mutex<SvgConverter>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            converter: std::sync::Arc::new(std::sync::Mutex::new(SvgConverter::new())),
        }
    }
}

/// Запускает Tauri приложение
#[tauri::command]
async fn convert_svg_to_dxf(
    input_path: String,
    output_path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    match state.converter.lock().unwrap().convert_file(
        std::path::Path::new(&input_path),
        std::path::Path::new(&output_path)
    ) {
        Ok(_) => Ok(format!("Файл успешно конвертирован: {}", output_path)),
        Err(e) => Err(format!("Ошибка конвертации: {}", e)),
    }
}

/// Выбор выходной папки
#[tauri::command]
async fn select_output_folder() -> Result<String, String> {
    let output_dir = std::env::current_dir()
        .unwrap()
        .join("output");
    
    std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    
    Ok(output_dir.to_string_lossy().to_string())
}

/// Конвертация всех SVG файлов в папке
#[tauri::command]
async fn convert_folder(
    input_folder: String,
    output_folder: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let mut results = Vec::new();
    let input_path = std::path::Path::new(&input_folder);
    let output_path = std::path::Path::new(&output_folder);
    
    if !input_path.exists() {
        return Err("Входная папка не существует".to_string());
    }
    
    std::fs::create_dir_all(output_path).map_err(|e| e.to_string())?;
    
    if let Ok(entries) = std::fs::read_dir(input_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if let Some(extension) = file_path.extension() {
                if extension == "svg" {
                    let file_name = file_path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy();
                    let dxf_name = format!("{}.dxf", file_name);
                    let dxf_path = output_path.join(&dxf_name);
                    
                    match state.converter.lock().unwrap().convert_file(&file_path, &dxf_path) {
                        Ok(_) => {
                            results.push(format!("✓ {} -> {}", 
                                file_path.file_name().unwrap().to_string_lossy(), 
                                dxf_name));
                        }
                        Err(e) => {
                            results.push(format!("✗ {} ошибка: {}", 
                                file_path.file_name().unwrap().to_string_lossy(), 
                                e));
                        }
                    }
                }
            }
        }
    }
    
    Ok(results)
}

/// Основная функция запуска приложения
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            convert_svg_to_dxf,
            select_output_folder,
            convert_folder,
            api::api_select_output_folder,
            api::api_select_files,
            api::api_convert_files,
            api::api_get_status
        ])
        .setup(|_app| {
            println!("🚀 SVG to DXF Converter запущен");
            println!("🔧 Нажмите F12 в браузере для включения дебаг режима");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Ошибка при запуске Tauri приложения");
}
