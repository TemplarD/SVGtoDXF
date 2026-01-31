//! Модуль для взаимодействия с Tauri бекендом

use wasm_bindgen::prelude::*;
use web_sys::console;

/// Тестовая функция
pub async fn test_function() -> Result<String, String> {
    console::log_1(&"test_function called".into());
    Ok("Тест успешен".to_string())
}
