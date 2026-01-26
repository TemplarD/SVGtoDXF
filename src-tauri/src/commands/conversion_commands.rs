use crate::commands::conversion::*;
use tauri::command;
use std::path::Path;

#[command]
pub async fn convert_single_file(
    input_path: String,
    output_path: Option<String>
) -> Result<ConversionResult, String> {
    let output_path = output_path.unwrap_or_else(|| generate_output_path(&input_path, None));
    
    match convert_svg_to_dxf(&input_path, &output_path).await {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Ошибка конвертации: {}", e)),
    }
}

#[command]
pub async fn convert_multiple_files(
    files: Vec<String>,
    alternative_dir: Option<String>
) -> Result<Vec<ConversionResult>, String> {
    let mut results = Vec::new();
    
    for file in files {
        let output_path = generate_output_path(&file, alternative_dir.as_deref());
        
        let result = match convert_svg_to_dxf(&file, &output_path).await {
            Ok(r) => r,
            Err(e) => ConversionResult {
                success: false,
                input_file: file.clone(),
                output_file: output_path,
                message: "Ошибка конвертации".to_string(),
                error: Some(e.to_string()),
            }
        };
        
        results.push(result);
    }
    
    Ok(results)
}

#[command]
pub async fn check_directory_access(path: String) -> Result<bool, String> {
    match check_directory_writable(&path).await {
        Ok(writable) => Ok(writable),
        Err(e) => Err(format!("Ошибка проверки директории: {}", e)),
    }
}

#[command]
pub async fn get_output_path(input_path: String, alternative_dir: Option<String>) -> Result<String, String> {
    Ok(generate_output_path(&input_path, alternative_dir.as_deref()))
}
