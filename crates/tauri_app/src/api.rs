//! API модуль для взаимодействия с UI (Tauri v2)

use std::path::{Path, PathBuf};
use svg2dxf_core::{convert_svg_to_dxf_with_options, ConversionOptions, FileConversionResult};
use tauri::{Emitter, Window};
use tauri_plugin_dialog::DialogExt;

/// API endpoint для выбора выходной папки через системный диалог
#[tauri::command]
pub async fn api_select_output_folder(app: tauri::AppHandle) -> Result<String, String> {
    // Асинхронный выбор папки через плагин dialog
    let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();
    app.dialog().file().pick_folder(move |folder| {
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
            let paths =
                files.map(|list| list.into_iter().map(|p| p.to_string()).collect::<Vec<_>>());
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
    // Имена, уже занятые в рамках этого прогона (защита от коллизий
    // между файлами с одинаковым базовым именем при overwrite=false).
    let mut used_names: std::collections::HashSet<String> = std::collections::HashSet::new();

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

        // Суффиксы к имени: цвет и штриховка (только когда включены
        // соответствующие опции и разрешён их суффикс).
        let mut name_suffix = String::new();
        if options.preserve_colors && options.add_color_suffix {
            name_suffix.push_str("_color");
        }
        if options.fill_as_lines && options.add_hatch_suffix {
            name_suffix.push_str("_hatch");
        }

        // Базовое имя выходного файла: stem + суффиксы + .dxf
        let base_name = format!("{}{}.dxf", stem, name_suffix);
        let output_path = resolve_output_path(
            &std::path::Path::new(&output_folder),
            &base_name,
            options.overwrite,
        );

        // Учитываем уже выбранные в этом прогоне имена (чтобы два файла
        // с одинаковым базовым именем не писались в один путь).
        let mut final_path = output_path;
        if !options.overwrite {
            let mut candidate = final_path.clone();
            let mut guard = 1u32;
            while used_names.contains(
                &candidate
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
                    .unwrap_or_default(),
            ) {
                let stem2 = candidate
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                let ext2 = candidate
                    .extension()
                    .map(|e| e.to_string_lossy().to_string())
                    .unwrap_or_else(|| "dxf".into());
                candidate = std::path::Path::new(&output_folder)
                    .join(format!("{}_{}.{}", stem2, guard, ext2));
                guard += 1;
                if guard > 9999 {
                    break;
                }
            }
            final_path = candidate;
        }
        used_names.insert(
            final_path
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default(),
        );

        match convert_svg_to_dxf_with_options(
            file_path,
            final_path.to_str().unwrap_or(""),
            options.clone(),
        ) {
            Ok(_) => {
                let result = FileConversionResult {
                    success: true,
                    input_file: file_path.clone(),
                    output_file: final_path.to_string_lossy().to_string(),
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
                    output_file: final_path.to_string_lossy().to_string(),
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

/// Определяет итоговый путь выходного DXF-файла.
///
/// * `overwrite = true` — возвращает `base_name` как есть (файл будет
///   перезаписан при конвертации).
/// * `overwrite = false` — если файл уже существует (или уже был выбран
///   для другого файла в этом же прогоне), добавляет индекс `_1`, `_2`, …
///   пока не найдёт свободное имя. Так гарантируется, что результаты не
///   перезапишут друг друга.
fn resolve_output_path(dir: &Path, base_name: &str, overwrite: bool) -> PathBuf {
    if overwrite {
        return dir.join(base_name);
    }
    let base = dir.join(base_name);
    if !base.exists() {
        return base;
    }
    let stem = base
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = base
        .extension()
        .map(|e| e.to_string_lossy().to_string())
        .unwrap_or_else(|| "dxf".into());
    let mut index = 1u32;
    loop {
        let candidate = dir.join(format!("{}_{}.{}", stem, index, ext));
        if !candidate.exists() {
            return candidate;
        }
        index += 1;
        if index > 9999 {
            // защита от бесконечного цикла в экзотических случаях
            return candidate;
        }
    }
}
