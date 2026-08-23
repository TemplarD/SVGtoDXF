//! SVG to DXF Converter Core Module
//! 
//! Этот модуль предоставляет основную функциональность для конвертации SVG файлов в DXF формат.
//! Включает в себя парсинг SVG, трансформацию в DXF примитивы и обработку ошибок.

pub mod converter;
pub mod error;
pub mod raster;

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
pub fn convert_svg_to_dxf_with_options(
    input_path: &str,
    output_path: &str,
    options: ConversionOptions,
) -> Result<()> {
    tracing::info!("Начало конвертации: {} -> {}", input_path, output_path);

    let input_path = Path::new(input_path);
    let output_path = Path::new(output_path);

    if !input_path.exists() {
        return Err(anyhow::anyhow!(
            "Входной файл не существует: {}",
            input_path.display()
        ));
    }

    let mut converter = SvgConverter::with_options(options);
    converter.convert_file(input_path, output_path)?;

    tracing::info!("Конвертация завершена успешно: {}", output_path.display());
    Ok(())
}

pub fn convert_svg_to_dxf(input_path: &str, output_path: &str) -> Result<()> {
    convert_svg_to_dxf_with_options(input_path, output_path, ConversionOptions::default())
}
