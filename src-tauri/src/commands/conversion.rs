use anyhow::{Result, Context};
use dxf::{Drawing, entities};
use dxf::entities::{Entity, EntityType};
use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversionResult {
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

/// Конвертирует SVG файл в DXF формат
pub async fn convert_svg_to_dxf(svg_path: &str, output_path: &str) -> Result<ConversionResult> {
    let svg_path = Path::new(svg_path);
    let output_path = Path::new(output_path);
    
    // Проверяем существует ли исходный файл
    if !svg_path.exists() {
        return Ok(ConversionResult {
            success: false,
            input_file: svg_path.to_string_lossy().to_string(),
            output_file: output_path.to_string_lossy().to_string(),
            message: "Исходный файл не найден".to_string(),
            error: Some("File not found".to_string()),
        });
    }
    
    // Читаем SVG файл
    let svg_content = fs::read_to_string(svg_path)
        .context("Не удалось прочитать SVG файл")?;
    
    // Создаем DXF чертеж
    let mut drawing = Drawing::new();
    
    // Парсим SVG и конвертируем элементы
    let mut conversion_data = ConversionData::new();
    parse_svg_to_dxf(&svg_content, &mut drawing, &mut conversion_data)?;
    
    // Убеждаемся что выходная директория существует
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .context("Не удалось создать выходную директорию")?;
    }
    
    // Сохраняем DXF файл
    use std::io::BufWriter;
    use std::fs::File;
    
    let file = File::create(output_path)
        .context("Не удалось создать файл")?;
    let mut writer = BufWriter::new(file);
    
    drawing.save(&mut writer)
        .context("Не удалось сохранить DXF файл")?;
    
    Ok(ConversionResult {
        success: true,
        input_file: svg_path.to_string_lossy().to_string(),
        output_file: output_path.to_string_lossy().to_string(),
        message: format!("Успешно конвертировано: {} элементов", conversion_data.elements_count),
        error: None,
    })
}

#[derive(Debug)]
struct ConversionData {
    elements_count: usize,
    current_path: Vec<dxf::Point>,
}

impl ConversionData {
    fn new() -> Self {
        Self {
            elements_count: 0,
            current_path: Vec::new(),
        }
    }
}

/// Парсит SVG и добавляет элементы в DXF чертеж (полная версия)
fn parse_svg_to_dxf(svg_content: &str, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    use xmlparser::Tokenizer;
    
    let mut tokenizer = Tokenizer::from(svg_content);
    let mut current_element = String::new();
    let mut attributes: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    
    while let Some(token) = tokenizer.next() {
        match token {
            Ok(xmlparser::Token::ElementStart { local, .. }) => {
                current_element = local.to_string();
                attributes.clear();
            },
            Ok(xmlparser::Token::ElementEnd { .. }) => {
                // Обрабатываем элемент после чтения всех атрибутов
                match current_element.as_str() {
                    "path" => {
                        if let Some(d) = attributes.get("d") {
                            convert_path_to_dxf_full(d, drawing, data)?;
                        }
                    },
                    "rect" => {
                        convert_rect_to_dxf_full(&attributes, drawing, data)?;
                    },
                    "circle" => {
                        convert_circle_to_dxf_full(&attributes, drawing, data)?;
                    },
                    "ellipse" => {
                        convert_ellipse_to_dxf_full(&attributes, drawing, data)?;
                    },
                    "line" => {
                        convert_line_to_dxf_full(&attributes, drawing, data)?;
                    },
                    "polygon" => {
                        if let Some(points) = attributes.get("points") {
                            convert_polygon_to_dxf(points, drawing, data)?;
                        }
                    },
                    "polyline" => {
                        if let Some(points) = attributes.get("points") {
                            convert_polyline_to_dxf(points, drawing, data)?;
                        }
                    },
                    "text" => {
                        convert_text_to_dxf(&attributes, drawing, data)?;
                    },
                    _ => {
                        log::debug!("Пропуск элемента: {}", current_element);
                    }
                }
            },
            Ok(xmlparser::Token::Attribute { local, value, .. }) => {
                attributes.insert(local.to_string(), value.to_string());
            },
            _ => {}
        }
    }
    
    Ok(())
}

/// Полная конвертация path элемента SVG в DXF
fn convert_path_to_dxf_full(path_data: &str, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut start_x = 0.0;
    let mut start_y = 0.0;
    
    // Парсим path данные
    let commands = parse_path_commands(path_data)?;
    
    for command in commands {
        match command {
            PathCommand::MoveTo(x, y) => {
                current_x = x;
                current_y = y;
                start_x = x;
                start_y = y;
            },
            PathCommand::LineTo(x, y) => {
                let p1 = dxf::Point::new(current_x, current_y, 0.0);
                let p2 = dxf::Point::new(x, y, 0.0);
                drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                current_x = x;
                current_y = y;
                data.elements_count += 1;
            },
            PathCommand::HorizontalLine(x) => {
                let p1 = dxf::Point::new(current_x, current_y, 0.0);
                let p2 = dxf::Point::new(x, current_y, 0.0);
                drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                current_x = x;
                data.elements_count += 1;
            },
            PathCommand::VerticalLine(y) => {
                let p1 = dxf::Point::new(current_x, current_y, 0.0);
                let p2 = dxf::Point::new(current_x, y, 0.0);
                drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                current_y = y;
                data.elements_count += 1;
            },
            PathCommand::ClosePath => {
                if current_x != start_x || current_y != start_y {
                    let p1 = dxf::Point::new(current_x, current_y, 0.0);
                    let p2 = dxf::Point::new(start_x, start_y, 0.0);
                    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                    data.elements_count += 1;
                }
                current_x = start_x;
                current_y = start_y;
            },
            // Для кривых используем аппроксимацию линиями
            PathCommand::CubicBezier(x1, y1, x2, y2, x, y) => {
                // Аппроксимируем кубическую кривую Безье несколькими линиями
                let steps = 10;
                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    let t2 = t * t;
                    let t3 = t2 * t;
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;
                    let mt3 = mt2 * mt;
                    
                    let new_x = mt3 * current_x + 3.0 * mt2 * t * x1 + 3.0 * mt * t2 * x2 + t3 * x;
                    let new_y = mt3 * current_y + 3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3 * y;
                    
                    let p1 = dxf::Point::new(current_x, current_y, 0.0);
                    let p2 = dxf::Point::new(new_x, new_y, 0.0);
                    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                    data.elements_count += 1;
                    
                    current_x = new_x;
                    current_y = new_y;
                }
            },
            PathCommand::QuadraticBezier(x1, y1, x, y) => {
                // Аппроксимируем квадратичную кривую Безье
                let steps = 8;
                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    let t2 = t * t;
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;
                    
                    let new_x = mt2 * current_x + 2.0 * mt * t * x1 + t2 * x;
                    let new_y = mt2 * current_y + 2.0 * mt * t * y1 + t2 * y;
                    
                    let p1 = dxf::Point::new(current_x, current_y, 0.0);
                    let p2 = dxf::Point::new(new_x, new_y, 0.0);
                    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
                    data.elements_count += 1;
                    
                    current_x = new_x;
                    current_y = new_y;
                }
            },
        }
    }
    
    Ok(())
}

/// Команды SVG path
#[derive(Debug)]
enum PathCommand {
    MoveTo(f64, f64),
    LineTo(f64, f64),
    HorizontalLine(f64),
    VerticalLine(f64),
    ClosePath,
    CubicBezier(f64, f64, f64, f64, f64, f64),
    QuadraticBezier(f64, f64, f64, f64),
}

/// Парсит SVG path строку в команды
fn parse_path_commands(path_data: &str) -> Result<Vec<PathCommand>> {
    let mut commands = Vec::new();
    let mut chars = path_data.chars().filter(|c| !c.is_whitespace()).peekable();
    let mut current_command = 'M';
    let mut numbers = Vec::new();
    
    while let Some(&c) = chars.peek() {
        if c.is_alphabetic() {
            // Сохраняем предыдущую команду
            if !numbers.is_empty() {
                process_command(&mut commands, current_command, &numbers)?;
                numbers.clear();
            }
            current_command = chars.next().unwrap();
        } else if c == '-' || c.is_digit(10) || c == '.' {
            // Читаем число
            let mut num_str = String::new();
            while let Some(&c) = chars.peek() {
                if c == '-' || c.is_digit(10) || c == '.' {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if let Ok(num) = num_str.parse::<f64>() {
                numbers.push(num);
            }
        } else {
            chars.next();
        }
    }
    
    // Обрабатываем последнюю команду
    if !numbers.is_empty() {
        process_command(&mut commands, current_command, &numbers)?;
    }
    
    Ok(commands)
}

/// Обрабатывает команду SVG path
fn process_command(commands: &mut Vec<PathCommand>, cmd: char, numbers: &[f64]) -> Result<()> {
    match cmd {
        'M' | 'm' => {
            for chunk in numbers.chunks(2) {
                if chunk.len() == 2 {
                    commands.push(PathCommand::MoveTo(chunk[0], chunk[1]));
                }
            }
        },
        'L' | 'l' => {
            for chunk in numbers.chunks(2) {
                if chunk.len() == 2 {
                    commands.push(PathCommand::LineTo(chunk[0], chunk[1]));
                }
            }
        },
        'H' | 'h' => {
            for &x in numbers {
                commands.push(PathCommand::HorizontalLine(x));
            }
        },
        'V' | 'v' => {
            for &y in numbers {
                commands.push(PathCommand::VerticalLine(y));
            }
        },
        'Z' | 'z' => {
            commands.push(PathCommand::ClosePath);
        },
        'C' | 'c' => {
            for chunk in numbers.chunks(6) {
                if chunk.len() == 6 {
                    commands.push(PathCommand::CubicBezier(chunk[0], chunk[1], chunk[2], chunk[3], chunk[4], chunk[5]));
                }
            }
        },
        'Q' | 'q' => {
            for chunk in numbers.chunks(4) {
                if chunk.len() == 4 {
                    commands.push(PathCommand::QuadraticBezier(chunk[0], chunk[1], chunk[2], chunk[3]));
                }
            }
        },
        _ => {
            log::debug!("Пропуск команды: {}", cmd);
        }
    }
    Ok(())
}

/// Полная конвертация прямоугольника SVG в DXF
fn convert_rect_to_dxf_full(attributes: &std::collections::HashMap<String, String>, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let x = parse_float_attr(attributes.get("x")).unwrap_or(0.0);
    let y = parse_float_attr(attributes.get("y")).unwrap_or(0.0);
    let width = parse_float_attr(attributes.get("width")).unwrap_or(0.0);
    let height = parse_float_attr(attributes.get("height")).unwrap_or(0.0);
    
    // Создаем 4 линии прямоугольника
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x, y, 0.0),
        dxf::Point::new(x + width, y, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x + width, y, 0.0),
        dxf::Point::new(x + width, y + height, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x + width, y + height, 0.0),
        dxf::Point::new(x, y + height, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x, y + height, 0.0),
        dxf::Point::new(x, y, 0.0)
    ))));
    
    data.elements_count += 4;
    Ok(())
}

/// Полная конвертация круга SVG в DXF
fn convert_circle_to_dxf_full(attributes: &std::collections::HashMap<String, String>, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let cx = parse_float_attr(attributes.get("cx")).unwrap_or(0.0);
    let cy = parse_float_attr(attributes.get("cy")).unwrap_or(0.0);
    let r = parse_float_attr(attributes.get("r")).unwrap_or(0.0);
    
    let center = dxf::Point::new(cx, cy, 0.0);
    let circle = entities::Circle::new(center, r);
    drawing.add_entity(Entity::new(EntityType::Circle(circle)));
    data.elements_count += 1;
    
    Ok(())
}

/// Полная конвертация эллипса SVG в DXF
fn convert_ellipse_to_dxf_full(attributes: &std::collections::HashMap<String, String>, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let cx = parse_float_attr(attributes.get("cx")).unwrap_or(0.0);
    let cy = parse_float_attr(attributes.get("cy")).unwrap_or(0.0);
    let rx = parse_float_attr(attributes.get("rx")).unwrap_or(0.0);
    let ry = parse_float_attr(attributes.get("ry")).unwrap_or(0.0);
    
    // Для эллипса используем аппроксимацию ломаной линией
    let steps = 32;
    let mut prev_x = cx + rx;
    let mut prev_y = cy;
    
    for i in 1..=steps {
        let angle = 2.0 * std::f64::consts::PI * i as f64 / steps as f64;
        let x = cx + rx * angle.cos();
        let y = cy + ry * angle.sin();
        
        let p1 = dxf::Point::new(prev_x, prev_y, 0.0);
        let p2 = dxf::Point::new(x, y, 0.0);
        drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
        data.elements_count += 1;
        
        prev_x = x;
        prev_y = y;
    }
    
    Ok(())
}

/// Полная конвертация линии SVG в DXF
fn convert_line_to_dxf_full(attributes: &std::collections::HashMap<String, String>, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let x1 = parse_float_attr(attributes.get("x1")).unwrap_or(0.0);
    let y1 = parse_float_attr(attributes.get("y1")).unwrap_or(0.0);
    let x2 = parse_float_attr(attributes.get("x2")).unwrap_or(0.0);
    let y2 = parse_float_attr(attributes.get("y2")).unwrap_or(0.0);
    
    let p1 = dxf::Point::new(x1, y1, 0.0);
    let p2 = dxf::Point::new(x2, y2, 0.0);
    
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
    data.elements_count += 1;
    
    Ok(())
}

/// Конвертация полигона SVG в DXF
fn convert_polygon_to_dxf(points_str: &str, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let points = parse_points(points_str)?;
    
    if points.len() < 2 {
        return Ok(());
    }
    
    // Рисуем линии между всеми точками
    for i in 0..points.len() {
        let j = (i + 1) % points.len(); // Замыкаем полигон
        
        let p1 = dxf::Point::new(points[i].0, points[i].1, 0.0);
        let p2 = dxf::Point::new(points[j].0, points[j].1, 0.0);
        drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
        data.elements_count += 1;
    }
    
    Ok(())
}

/// Конвертация полилинии SVG в DXF
fn convert_polyline_to_dxf(points_str: &str, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let points = parse_points(points_str)?;
    
    if points.len() < 2 {
        return Ok(());
    }
    
    // Рисуем линии между последовательными точками
    for i in 0..points.len() - 1 {
        let p1 = dxf::Point::new(points[i].0, points[i].1, 0.0);
        let p2 = dxf::Point::new(points[i + 1].0, points[i + 1].1, 0.0);
        drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(p1, p2))));
        data.elements_count += 1;
    }
    
    Ok(())
}

/// Конвертация текста SVG в DXF (как точки или линии)
fn convert_text_to_dxf(attributes: &std::collections::HashMap<String, String>, drawing: &mut Drawing, data: &mut ConversionData) -> Result<()> {
    let x = parse_float_attr(attributes.get("x")).unwrap_or(0.0);
    let y = parse_float_attr(attributes.get("y")).unwrap_or(0.0);
    
    // Для текста создаем простую точку или маленький прямоугольник как индикатор
    let size = 5.0; // Размер индикатора текста
    
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x - size/2.0, y - size/2.0, 0.0),
        dxf::Point::new(x + size/2.0, y - size/2.0, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x + size/2.0, y - size/2.0, 0.0),
        dxf::Point::new(x + size/2.0, y + size/2.0, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x + size/2.0, y + size/2.0, 0.0),
        dxf::Point::new(x - size/2.0, y + size/2.0, 0.0)
    ))));
    drawing.add_entity(Entity::new(EntityType::Line(entities::Line::new(
        dxf::Point::new(x - size/2.0, y + size/2.0, 0.0),
        dxf::Point::new(x - size/2.0, y - size/2.0, 0.0)
    ))));
    
    data.elements_count += 4;
    
    Ok(())
}

/// Парсит строку с точками в вектор координат
fn parse_points(points_str: &str) -> Result<Vec<(f64, f64)>> {
    let mut points = Vec::new();
    let mut coords = Vec::new();
    
    // Разбиваем строку на числа
    for part in points_str.split_whitespace() {
        if let Ok(num) = part.parse::<f64>() {
            coords.push(num);
        }
    }
    
    // Группируем координаты в пары (x, y)
    for chunk in coords.chunks(2) {
        if chunk.len() == 2 {
            points.push((chunk[0], chunk[1]));
        }
    }
    
    Ok(points)
}

/// Парсит float атрибут
fn parse_float_attr(attr: Option<&String>) -> Option<f64> {
    attr?.parse().ok()
}

/// Проверяет доступность папки для записи
pub async fn check_directory_writable(path: &str) -> Result<bool> {
    let path = Path::new(path);
    
    if !path.exists() {
        // Пробуем создать директорию
        fs::create_dir_all(path)?;
        return Ok(true);
    }
    
    // Пробуем создать тестовый файл
    let test_file = path.join(".write_test");
    match fs::write(&test_file, "test") {
        Ok(_) => {
            let _ = fs::remove_file(&test_file);
            Ok(true)
        }
        Err(_) => Ok(false)
    }
}

/// Генерирует путь для выходного файла
pub fn generate_output_path(input_path: &str, alternative_dir: Option<&str>) -> String {
    let input_path = Path::new(input_path);
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    
    // Сначала пробуем сохранить рядом с исходным
    let output_path = parent.with_extension("dxf");
    
    // Если указана альтернативная директория и исходная недоступна
    if let Some(alt_dir) = alternative_dir {
        let filename = input_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("converted.dxf");
        
        let alt_path = Path::new(alt_dir).join(filename);
        return alt_path.with_extension("dxf").to_string_lossy().to_string();
    }
    
    output_path.to_string_lossy().to_string()
}
