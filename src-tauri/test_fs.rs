use std::path::Path;

#[tokio::main]
async fn main() {
    println!("🧪 Тест файловой системы");
    
    // Пробуем прочитать текущую директорию
    let path = Path::new(".");
    match path.read_dir() {
        Ok(entries) => {
            println!("✅ Файлы в текущей директории:");
            for entry in entries {
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                let is_dir = entry.file_type().unwrap().is_dir();
                println!("📁 {}: {}", name, if is_dir { "папка" } else { "файл" });
            }
        }
        Err(e) => {
            println!("❌ Ошибка: {}", e);
        }
    }
    
    // Пробуем прочитать /home
    let home_path = Path::new("/home");
    match home_path.read_dir() {
        Ok(entries) => {
            println!("✅ Файлы в /home:");
            for entry in entries.take(5) { // Только первые 5
                let entry = entry.unwrap();
                let name = entry.file_name().into_string().unwrap();
                let is_dir = entry.file_type().unwrap().is_dir();
                println!("📁 {}: {}", name, if is_dir { "папка" } else { "файл" });
            }
        }
        Err(e) => {
            println!("❌ Ошибка чтения /home: {}", e);
        }
    }
}
