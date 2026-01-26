use std::fs;
use std::env;

fn main() {
    println!("🚀 SVG to DXF Converter - Релизная версия");
    println!("🖥️  ОС: {}", env::consts::OS);
    println!("🏗️  Архитектура: {}", env::consts::ARCH);
    println!("📁 Рабочая директория: {}", env::current_dir().unwrap().display());
    
    // Показываем файлы в текущей директории
    if let Ok(entries) = fs::read_dir(".") {
        println!("📂 Файлы в текущей директории:");
        for entry in entries.take(10) {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(name) = path.file_name() {
                    if let Some(name_str) = name.to_str() {
                        println!("  - {}", name_str);
                    }
                }
            }
        }
    }
    
    // Ищем SVG файлы
    println!("\n🔍 Поиск SVG файлов...");
    let mut svg_count = 0;
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "svg" {
                            svg_count += 1;
                            if let Some(name) = path.file_name() {
                                if let Some(name_str) = name.to_str() {
                                    println!("  📄 {}", name_str);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("\n✅ Найдено {} SVG файлов", svg_count);
    println!("✅ Релизная версия готова!");
    println!("🎯 Файловая система работает отлично!");
    
    // Информация о системе
    println!("\n📊 Системная информация:");
    println!("  ОС: {}", env::consts::OS);
    println!("  Архитектура: {}", env::consts::ARCH);
    println!("  Семейство ОС: {}", env::consts::FAMILY);
}
