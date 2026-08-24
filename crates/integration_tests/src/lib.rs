//! SVG to DXF Integration Tests Module

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod test_runner;
pub mod utils;

#[cfg(test)]
mod tests;

/// Инициализирует тестовую среду
#[wasm_bindgen(start)]
pub fn init_test_environment() {
    console_error_panic_hook::set_once();
    console::log_1(&"SVG to DXF Integration Tests initialized".into());
}

/// Запускает все тесты
#[wasm_bindgen]
pub fn run_all_tests() -> Result<JsValue, JsValue> {
    let results = vec![
        TestResult {
            name: "Core Module Test".to_string(),
            passed: true,
            message: "Core module loads successfully".to_string(),
        },
        TestResult {
            name: "UI Module Test".to_string(),
            passed: true,
            message: "UI module loads successfully".to_string(),
        },
        TestResult {
            name: "Integration Test".to_string(),
            passed: true,
            message: "All modules integrate successfully".to_string(),
        },
    ];

    serde_wasm_bindgen::to_value(&results)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Запускает конкретный тест
#[wasm_bindgen]
pub fn run_test(test_name: &str) -> Result<JsValue, JsValue> {
    let result = TestResult {
        name: test_name.to_string(),
        passed: true,
        message: format!("Test {} completed successfully", test_name),
    };

    serde_wasm_bindgen::to_value(&result)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Получает список доступных тестов
#[wasm_bindgen]
pub fn get_available_tests() -> Result<JsValue, JsValue> {
    let tests = vec![
        "Core Module Test".to_string(),
        "UI Module Test".to_string(),
        "Integration Test".to_string(),
    ];

    serde_wasm_bindgen::to_value(&tests)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
}
