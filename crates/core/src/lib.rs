//! SVG to DXF Converter Core Module
//! 
//! Этот модуль предоставляет основную функциональность для конвертации SVG файлов в DXF формат.
//! Включает в себя парсинг SVG, трансформацию в DXF примитивы и обработку ошибок.

pub mod converter;
pub mod error;

pub use converter::*;
pub use error::*;

use anyhow::Result;
use std::path::Path;

/// Основная функция конвертации SVG в DXF
/// 
/// # Arguments
/// * `input_path` - путь к исходному SVG файлу
/// * `output_path` - путь для сохранения DXF файла
/// 
/// # Returns
/// `Result<()>` - успешная конвертация или ошибка
/// 
/// # Example
/// ```rust
/// use svg2dxf_core::convert_svg_to_dxf;
/// 
/// let result = convert_svg_to_dxf("input.svg", "output.dxf");
/// match result {
///     Ok(()) => println!("Конвертация успешна"),
///     Err(e) => println!("Ошибка: {}", e),
/// }
/// ```
pub fn convert_svg_to_dxf(input_path: &str, output_path: &str) -> Result<()> {
    tracing::info!("Начало конвертации: {} -> {}", input_path, output_path);
    
    let input_path = Path::new(input_path);
    let output_path = Path::new(output_path);
    
    // Проверяем существование входного файла
    if !input_path.exists() {
        return Err(anyhow::anyhow!("Входной файл не существует: {}", input_path.display()));
    }
    
    // Создаем конвертер и выполняем конвертацию
    let mut converter = SvgConverter::new();
    converter.convert_file(input_path, output_path)?;
    
    tracing::info!("Конвертация завершена успешно: {}", output_path.display());
    Ok(())
}
