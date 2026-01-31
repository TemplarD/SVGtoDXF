//! Основной модуль конвертации SVG в DXF

use crate::error::{ConversionError, ConversionResult};
use dxf::{Drawing, Point};
use dxf::entities::{Entity, EntityType, Line};
use std::path::Path;
use std::fs;
use usvg::{Tree, Options, TreeParsing};
use tracing::{debug, trace, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConversionResult {
    pub success: bool,
    pub input_file: String,
    pub output_file: String,
    pub message: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionProgress {
    pub file: String,
    pub status: String, // "converting", "completed", "error"
    pub progress: f32, // 0.0 to 1.0
    pub message: String,
}

/// Структура для конвертации SVG в DXF
pub struct SvgConverter {
    /// Настройки парсинга SVG
    svg_options: Options,
    /// Текущий слой DXF
    current_layer: String,
}

impl SvgConverter {
    /// Создает новый экземпляр конвертера
    pub fn new() -> Self {
        let options = Options::default();
        
        Self {
            svg_options: options,
            current_layer: "0".to_string(),
        }
    }
    
    /// Устанавливает текущий слой для DXF примитивов
    pub fn set_layer(&mut self, layer: impl Into<String>) {
        self.current_layer = layer.into();
    }
    
    /// Конвертирует SVG файл в DXF
    pub fn convert_file(&mut self, input_path: &Path, output_path: &Path) -> ConversionResult<()> {
        debug!("Чтение SVG файла: {}", input_path.display());
        
        // Читаем SVG содержимое
        let svg_content = fs::read_to_string(input_path)
            .map_err(|e| ConversionError::svg_read_error(format!("Не удалось прочитать файл: {}", e)))?;
        
        // Парсим SVG
        let tree = Tree::from_str(&svg_content, &self.svg_options)
            .map_err(|e| ConversionError::svg_parse_error(format!("Ошибка парсинга SVG: {}", e)))?;
        
        // Создаем DXF документ
        let mut drawing = Drawing::new();
        
        // Конвертируем SVG элементы в DXF
        self.convert_tree(&tree, &mut drawing)?;
        
        // Убеждаемся что выходная директория существует
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| ConversionError::file_system_error(format!("Не удалось создать директорию: {}", e)))?;
        }
        
        // Сохраняем DXF файл
        drawing.save_file(output_path)
            .map_err(|e| ConversionError::dxf_write_error(format!("Не удалось сохранить DXF: {}", e)))?;
        
        debug!("DXF файл сохранен: {}", output_path.display());
        Ok(())
    }
    
    /// Конвертирует SVG дерево в DXF примитивы
    fn convert_tree(&mut self, tree: &Tree, drawing: &mut Drawing) -> ConversionResult<()> {
        let size = tree.size;
        trace!("Обработка SVG дерева с размерами: {}x{}", size.width(), size.height());
        
        // Обрабатываем корневые элементы
        for node in tree.root.children() {
            self.convert_node(&node, drawing)?;
        }
        
        Ok(())
    }
    
    /// Конвертирует отдельный SVG узел в DXF примитив
    fn convert_node(&mut self, node: &usvg::Node, drawing: &mut Drawing) -> ConversionResult<()> {
        match *node.borrow() {
            usvg::NodeKind::Path(ref _path) => {
                // Временная заглушка - просто создаем тестовую линию для каждого пути
                self.create_test_line(drawing)?;
            }
            usvg::NodeKind::Group(ref _group) => {
                // Временно пропускаем группы для упрощения
                trace!("Пропуск группы");
            }
            usvg::NodeKind::Image(_) => {
                warn!("Изображения не поддерживаются в DXF конвертации");
            }
            usvg::NodeKind::Text(_) => {
                warn!("Текстовые элементы не поддерживаются, будут пропущены");
            }
        }
        
        Ok(())
    }
    
    /// Создает тестовую линию в DXF
    fn create_test_line(&self, drawing: &mut Drawing) -> ConversionResult<()> {
        let line = Line {
            p1: Point::new(0.0, 0.0, 0.0),
            p2: Point::new(100.0, 100.0, 0.0),
            thickness: 0.0,
            ..Default::default()
        };
        
        drawing.add_entity(Entity::new(EntityType::Line(line)));
        Ok(())
    }
}

impl Default for SvgConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_converter_creation() {
        let converter = SvgConverter::new();
        assert_eq!(converter.current_layer, "0");
    }
    
    #[test]
    fn test_layer_setting() {
        let mut converter = SvgConverter::new();
        converter.set_layer("test_layer");
        assert_eq!(converter.current_layer, "test_layer");
    }
    
    #[test]
    fn test_simple_svg_conversion() {
        let svg_content = r#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <line x1="0" y1="0" x2="100" y2="100" stroke="black"/>
        </svg>"#;
        
        let temp_dir = tempdir().unwrap();
        let svg_path = temp_dir.path().join("test.svg");
        let dxf_path = temp_dir.path().join("test.dxf");
        
        fs::write(&svg_path, svg_content).unwrap();
        
        let mut converter = SvgConverter::new();
        let result = converter.convert_file(&svg_path, &dxf_path);
        
        assert!(result.is_ok());
        assert!(dxf_path.exists());
    }
}
