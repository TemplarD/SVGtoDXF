//! Основной модуль конвертации SVG в DXF

use crate::error::{ConversionError, ConversionResult};
use dxf::{Drawing};
use dxf::enums::AcadVersion;
use dxf::entities::{Entity, EntityType, LwPolyline};
use dxf::LwPolylineVertex;
use std::path::Path;
use std::fs;
use usvg::{NodeKind, Tree, TreeParsing};
use tiny_skia_path::{PathSegment as Seg};
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
    pub status: String,
    pub progress: f32,
    pub message: String,
}

/// Структура для конвертации SVG в DXF
pub struct SvgConverter {
    svg_options: usvg::Options,
    current_layer: String,
    doc_height: f64,
}

impl SvgConverter {
    pub fn new() -> Self {
        Self {
            svg_options: usvg::Options::default(),
            current_layer: "0".to_string(),
            doc_height: 0.0,
        }
    }

    pub fn set_layer(&mut self, layer: impl Into<String>) {
        self.current_layer = layer.into();
    }

    pub fn convert_file(&mut self, input_path: &Path, output_path: &Path) -> ConversionResult<()> {
        debug!("Чтение SVG файла: {}", input_path.display());

        let svg_content = fs::read_to_string(input_path)
            .map_err(|e| ConversionError::svg_read_error(format!("Не удалось прочитать файл: {}", e)))?;

        let tree = Tree::from_str(&svg_content, &self.svg_options)
            .map_err(|e| ConversionError::svg_parse_error(format!("Ошибка парсинга SVG: {}", e)))?;

        self.doc_height = tree.size.height() as f64;

        let mut drawing = Drawing::new();
        // R2013 (AC1027) обязателен для поддержки LWPOLYLINE
        drawing.header.version = AcadVersion::R2013;

        self.convert_tree(&tree, &mut drawing)?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ConversionError::file_system_error(format!("Не удалось создать директорию: {}", e))
            })?;
        }

        drawing.save_file(output_path).map_err(|e| {
            ConversionError::dxf_write_error(format!("Не удалось сохранить DXF: {}", e))
        })?;

        debug!("DXF файл сохранен: {}", output_path.display());
        Ok(())
    }

    fn convert_tree(&mut self, tree: &Tree, drawing: &mut Drawing) -> ConversionResult<()> {
        let size = tree.size;
        trace!(
            "Обработка SVG дерева с размерами: {}x{}",
            size.width(),
            size.height()
        );

        for node in tree.root.children() {
            self.convert_node(&node, drawing)?;
        }
        Ok(())
    }

    fn convert_node(&mut self, node: &usvg::Node, drawing: &mut Drawing) -> ConversionResult<()> {
        match *node.borrow() {
            NodeKind::Path(ref path) => {
                self.convert_path(path, drawing)?;
            }
            NodeKind::Group(ref _group) => {
                for child in node.children() {
                    self.convert_node(&child, drawing)?;
                }
            }
            NodeKind::Image(_) => {
                warn!("Изображения не поддерживаются в DXF конвертации");
            }
            NodeKind::Text(_) => {
                warn!("Текстовые элементы не поддерживаются, будут пропущены");
            }
        }
        Ok(())
    }

    /// Конвертирует SVG Path в LWPOLYLINE (с тесселяцией кривых).
    /// Координаты в path.data уже абсолютные (по документации usvg).
    fn convert_path(&self, path: &usvg::Path, drawing: &mut Drawing) -> ConversionResult<()> {
        const CURVE_STEPS: usize = 24;

        let mut points: Vec<(f64, f64)> = Vec::new();
        let mut current = (0.0f64, 0.0f64);
        let mut start = (0.0f64, 0.0f64);
        let mut pen_down = false;

        for seg in path.data.segments() {
            match seg {
                Seg::MoveTo(p) => {
                    current = (p.x as f64, p.y as f64);
                    start = current;
                    points.push(current);
                    pen_down = true;
                }
                Seg::LineTo(p) => {
                    current = (p.x as f64, p.y as f64);
                    points.push(current);
                }
                Seg::CubicTo(p1, p2, p) => {
                    let (sx, sy) = current;
                    let (x1, y1) = (p1.x as f64, p1.y as f64);
                    let (x2, y2) = (p2.x as f64, p2.y as f64);
                    let (x, y) = (p.x as f64, p.y as f64);
                    for i in 1..=CURVE_STEPS {
                        let t = i as f64 / CURVE_STEPS as f64;
                        let pt = cubic_bezier(sx, sy, x1, y1, x2, y2, x, y, t);
                        points.push(pt);
                    }
                    current = (x, y);
                }
                Seg::QuadTo(p1, p) => {
                    let (sx, sy) = current;
                    let (x1, y1) = (p1.x as f64, p1.y as f64);
                    let (x, y) = (p.x as f64, p.y as f64);
                    for i in 1..=CURVE_STEPS {
                        let t = i as f64 / CURVE_STEPS as f64;
                        let pt = quadratic_bezier(sx, sy, x1, y1, x, y, t);
                        points.push(pt);
                    }
                    current = (x, y);
                }
                Seg::Close => {
                    points.push(start);
                    pen_down = false;
                }
            }
        }

        if points.len() < 2 {
            trace!("Path содержит <2 точек, пропуск");
            return Ok(());
        }

        let mut poly = LwPolyline::default();
        for (x, y) in points.iter() {
            poly.vertices.push(LwPolylineVertex {
                x: *x,
                y: self.invert_y(*y),
                ..Default::default()
            });
        }
        if !pen_down && points.first() == points.last() {
            poly.flags |= 1; // бит Closed для LWPOLYLINE
        }

        drawing.add_entity(Entity::new(EntityType::LwPolyline(poly)));
        Ok(())
    }

    /// Инверсия оси Y: SVG (0 сверху) -> DXF (0 снизу)
    fn invert_y(&self, y: f64) -> f64 {
        if self.doc_height > 0.0 {
            self.doc_height - y
        } else {
            y
        }
    }
}

/// Кубическая кривая Безье
fn cubic_bezier(
    p0x: f64, p0y: f64,
    p1x: f64, p1y: f64,
    p2x: f64, p2y: f64,
    p3x: f64, p3y: f64,
    t: f64,
) -> (f64, f64) {
    let u = 1.0 - t;
    let x = u * u * u * p0x + 3.0 * u * u * t * p1x + 3.0 * u * t * t * p2x + t * t * t * p3x;
    let y = u * u * u * p0y + 3.0 * u * u * t * p1y + 3.0 * u * t * t * p2y + t * t * t * p3y;
    (x, y)
}

/// Квадратичная кривая Безье
fn quadratic_bezier(
    p0x: f64, p0y: f64,
    p1x: f64, p1y: f64,
    p2x: f64, p2y: f64,
    t: f64,
) -> (f64, f64) {
    let u = 1.0 - t;
    let x = u * u * p0x + 2.0 * u * t * p1x + t * t * p2x;
    let y = u * u * p0y + 2.0 * u * t * p1y + t * t * p2y;
    (x, y)
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
            <path d="M10 10 L90 10 L90 90 L10 90 Z"/>
        </svg>"#;

        let temp_dir = tempdir().unwrap();
        let svg_path = temp_dir.path().join("test.svg");
        let dxf_path = temp_dir.path().join("test.dxf");

        fs::write(&svg_path, svg_content).unwrap();

        let mut converter = SvgConverter::new();
        let result = converter.convert_file(&svg_path, &dxf_path);

        assert!(result.is_ok(), "conversion failed: {:?}", result.err());
        assert!(dxf_path.exists());
    }
}
