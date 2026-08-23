//! Трассировка растровых изображений (SVG <image>) в вектор (LWPOLYLINE).
//!
//! Подход: декодируем байты (PNG/JPEG/GIF) -> яркость -> бинаризация по
//! порогу -> marching squares (изолиния) -> набор сегментов -> LWPOLYLINE.
//! Для производительности растр уменьшается до MAX_DIM пикселей по большей
//! стороне.

use crate::converter::SvgConverter;
use crate::error::{ConversionError, ConversionResult};
use dxf::entities::{Entity, EntityType, LwPolyline};
use dxf::LwPolylineVertex;
use dxf::{Drawing, Point};
use usvg::{Image, ImageKind};

const MAX_DIM: u32 = 256;
const THRESHOLD: f32 = 0.5;

/// Декодирует ImageKind в матрицу яркости (0.0 = белый, 1.0 = чёрный).
/// Возвращает (width, height, Vec яркости построчно).
fn decode_luma(img: &ImageKind) -> ConversionResult<(u32, u32, Vec<f32>)> {
    let bytes: &[u8] = match img {
        ImageKind::JPEG(d) => d,
        ImageKind::PNG(d) => d,
        ImageKind::GIF(d) => d,
        ImageKind::SVG(_) => {
            return Err(ConversionError::unsupported_format(
                "Вложенный SVG в <image> пока не трассируется (отложено)".to_string(),
            ));
        }
    };

    let dyn_img = image::load_from_memory(bytes)
        .map_err(|e| ConversionError::svg_parse_error(format!("Не удалось декодировать растр: {}", e)))?;

    // Уменьшаем для скорости
    let (w, h) = (dyn_img.width(), dyn_img.height());
    let scale = (w.max(h) as f32 / MAX_DIM as f32).max(1.0);
    let nw = (w as f32 / scale).max(1.0) as u32;
    let nh = (h as f32 / scale).max(1.0) as u32;
    let resized = dyn_img.resize(nw, nh, image::imageops::FilterType::Nearest);

    let gray = resized.to_luma8();
    let mut luma = Vec::with_capacity((nw * nh) as usize);
    for p in gray.pixels() {
        // инвертируем: тёмный пиксель -> 1.0 (линия)
        luma.push(1.0 - (p.0[0] as f32 / 255.0));
    }
    Ok((nw, nh, luma))
}

/// Marching squares: строит сегменты изолинии для порога THRESHOLD.
/// Каждый сегмент -> отдельная 2-точечная LWPOLYLINE.
pub fn trace_image(
    converter: &SvgConverter,
    img: &Image,
    drawing: &mut Drawing,
) -> ConversionResult<()> {
    let (w, h, luma) = decode_luma(&img.kind)?;

    // Размеры и позиция в SVG (view_box: x, y, w, h)
    let vb = img.view_box.rect;
    let ox = vb.x() as f64;
    let oy = vb.y() as f64;
    let sx = (vb.width() as f64 / w as f64).max(1e-6);
    let sy = (vb.height() as f64 / h as f64).max(1e-6);

    let at = |x: u32, y: u32| -> f32 {
        if x >= w || y >= h {
            return 0.0;
        }
        luma[(y * w + x) as usize]
    };

    let mut segments: Vec<((f64, f64), (f64, f64))> = Vec::new();

    for y in 0..h.saturating_sub(1) {
        for x in 0..w.saturating_sub(1) {
            let tl = at(x, y);
            let tr = at(x + 1, y);
            let br = at(x + 1, y + 1);
            let bl = at(x, y + 1);

            let mut code = 0u8;
            if tl > THRESHOLD {
                code |= 8;
            }
            if tr > THRESHOLD {
                code |= 4;
            }
            if br > THRESHOLD {
                code |= 2;
            }
            if bl > THRESHOLD {
                code |= 1;
            }
            if code == 0 || code == 15 {
                continue;
            }

            // координаты углов ячейки в SVG-пространстве
            let x0 = ox + x as f64 * sx;
            let x1 = ox + (x + 1) as f64 * sx;
            let y0 = oy + y as f64 * sy;
            let y1 = oy + (y + 1) as f64 * sy;
            let mid = |a: f64, b: f64| (a + b) / 2.0;

            let top = (mid(x0, x1), y0);
            let right = (x1, mid(y0, y1));
            let bottom = (mid(x0, x1), y1);
            let left = (x0, mid(y0, y1));

            // стандартная таблица marching squares (без интерполяции)
            let push = |segs: &mut Vec<((f64, f64), (f64, f64))>, a: (f64, f64), b: (f64, f64)| {
                segs.push((a, b));
            };
            match code {
                1 | 14 => push(&mut segments, bottom, left),
                2 | 13 => push(&mut segments, bottom, right),
                3 | 12 => push(&mut segments, left, right),
                4 | 11 => push(&mut segments, top, right),
                5 => {
                    push(&mut segments, top, left);
                    push(&mut segments, bottom, right);
                }
                6 | 9 => push(&mut segments, top, bottom),
                7 | 8 => push(&mut segments, top, left),
                10 => {
                    push(&mut segments, top, right);
                    push(&mut segments, bottom, left);
                }
                _ => {}
            }
        }
    }

    if segments.is_empty() {
        return Ok(());
    }

    for (a, b) in &segments {
        let poly = LwPolyline {
            vertices: vec![
                LwPolylineVertex {
                    x: a.0,
                    y: converter.invert_y(a.1),
                    ..Default::default()
                },
                LwPolylineVertex {
                    x: b.0,
                    y: converter.invert_y(b.1),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        drawing.add_entity(Entity::new(EntityType::LwPolyline(poly)));
    }

    tracing::debug!(
        "Растр оттрассирован: {} сегментов ({}x{} сетка)",
        segments.len(),
        w,
        h
    );
    Ok(())
}
