//! Основной модуль конвертации SVG в DXF

use crate::error::{ConversionError, ConversionResult};
use dxf::Drawing;
use dxf::enums::AcadVersion;
use dxf::entities::{Entity, EntityType, LwPolyline, MText};
use dxf::LwPolylineVertex;
use dxf::Point;
use dxf::Color;
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
            NodeKind::Text(ref text) => {
                self.convert_text(text, drawing)?;
            }
            NodeKind::Image(ref image) => {
                crate::raster::trace_image(self, image, drawing)?;
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
            poly.flags |= 1; // бит Closed для LWPOL LWPOLYLINE
        }

        let mut entity = Entity::new(EntityType::LwPolyline(poly));
        if let Some((r, g, b)) = extract_rgb(path) {
            entity.common.color = color_from_rgb(r, g, b);
        }
        drawing.add_entity(entity);
        Ok(())
    }

    /// Конвертирует SVG Text в DXF MText.
    /// Обрабатывает текст по span'ам: учитывает размер шрифта (масштаб)
    /// и семейство шрифта. Позиция берётся из чанка (с инверсией оси Y).
    fn convert_text(&self, text: &usvg::Text, drawing: &mut Drawing) -> ConversionResult<()> {
        for chunk in &text.chunks {
            if chunk.text.trim().is_empty() {
                continue;
            }
            // Позиция чанка (SVG y вниз; инвертируем в DXF)
            let (x, y) = match (chunk.x, chunk.y) {
                (Some(cx), Some(cy)) => (cx as f64, self.invert_y(cy as f64)),
                _ => {
                    if let Some(pos) = text.positions.first() {
                        match (pos.x, pos.y) {
                            (Some(px), Some(py)) => (px as f64, self.invert_y(py as f64)),
                            _ => continue,
                        }
                    } else {
                        continue;
                    }
                }
            };

            // Каждый span может иметь свой размер/шрифт -> отдельная MText-сущность
            for span in &chunk.spans {
                let slice = chunk.text.get(span.start..span.end).unwrap_or("");
                let content = slice.trim();
                if content.is_empty() {
                    continue;
                }
                // Масштаб: размер шрифта SVG -> высота текста DXF
                let font_size = span.font_size.get() as f64;
                // Семейство шрифта (первое из списка; для DXF стиль регистрируется отдельно)
                let font_family = span
                    .font
                    .families
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "default".to_string());

                let mtext = MText {
                    insertion_point: Point::new(x, y, 0.0),
                    initial_text_height: font_size,
                    text: content.to_string(),
                    ..Default::default()
                };
                let mut entity = Entity::new(EntityType::MText(mtext));
                if let Some(fill) = &span.fill {
                    if let usvg::Paint::Color(c) = &fill.paint {
                        entity.common.color = color_from_rgb(c.red, c.green, c.blue);
                    }
                }
                drawing.add_entity(entity);
                debug!(
                    "Текст: '{}' @ ({:.1}, {:.1}) размер={:.1} шрифт={}",
                    content, x, y, font_size, font_family
                );
            }
        }
        Ok(())
    }

    /// Инверсия оси Y: SVG (0 сверху) -> DXF (0 снизу)
    pub(crate) fn invert_y(&self, y: f64) -> f64 {
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

/// Стандартная палитра AutoCAD (ACI 1..255) — усечённая таблица
/// индекс -> (R, G, B). Используется для приближения цвета SVG к
/// ближайшему индексу цвета DXF (dxf 0.5 не поддерживает true-color напрямую).
const ACI_PALETTE: &[(u8, u8, u8, u8)] = &[
    (1, 255, 0, 0),       // red
    (2, 255, 255, 0),     // yellow
    (3, 0, 255, 0),       // green
    (4, 0, 255, 255),     // cyan
    (5, 0, 0, 255),       // blue
    (6, 255, 0, 255),     // magenta
    (7, 255, 255, 255),   // white
    (8, 128, 128, 128),   // dark gray (approx)
    (9, 192, 192, 192),   // light gray
    (10, 255, 128, 128),  // light red
    (11, 255, 214, 128),  // orange-ish
    (12, 255, 255, 128),  // light yellow
    (13, 128, 255, 128),  // light green
    (14, 128, 255, 255),  // light cyan
    (15, 128, 128, 255),  // light blue
    (16, 255, 128, 255),  // light magenta
    (20, 204, 0, 0),      // dark red
    (30, 0, 204, 0),      // dark green
    (40, 0, 0, 204),      // dark blue
    (50, 204, 204, 0),    // olive
    (60, 204, 0, 204),    // purple
    (70, 0, 204, 204),    // teal
    (110, 128, 64, 0),    // brown
    (140, 64, 0, 0),      // maroon
    (200, 255, 255, 255), // near white
    (250, 0, 0, 0),       // black
];

/// Возвращает ближайший индекс цвета ACI (1..255) к заданному RGB.
fn rgb_to_aci(r: u8, g: u8, b: u8) -> u8 {
    let mut best = 7u8; // white fallback
    let mut best_dist = u32::MAX;
    for &(idx, cr, cg, cb) in ACI_PALETTE {
        let dr = r as i32 - cr as i32;
        let dg = g as i32 - cg as i32;
        let db = b as i32 - cb as i32;
        let dist = (dr * dr + dg * dg + db * db) as u32;
        if dist < best_dist {
            best_dist = dist;
            best = idx;
        }
    }
    best
}

/// Извлекает цвет (RGB) из заливки или обводки SVG-элемента.
/// Приоритет: stroke (обводка), затем fill (заливка).
fn extract_rgb(path: &usvg::Path) -> Option<(u8, u8, u8)> {
    if let Some(stroke) = &path.stroke {
        if let usvg::Paint::Color(c) = &stroke.paint {
            return Some((c.red, c.green, c.blue));
        }
    }
    if let Some(fill) = &path.fill {
        if let usvg::Paint::Color(c) = &fill.paint {
            return Some((c.red, c.green, c.blue));
        }
    }
    None
}

/// Формирует dxf::Color из RGB SVG (ближайший ACI).
fn color_from_rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_index(rgb_to_aci(r, g, b))
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
