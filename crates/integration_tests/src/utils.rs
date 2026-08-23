//! Утилиты для тестирования

use wasm_bindgen::prelude::*;
use web_sys::*;

/// Проверяет наличие элемента по ID
pub fn element_exists(id: &str) -> bool {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    document.get_element_by_id(id).is_some()
}

/// Симулирует клик по элементу
pub fn simulate_click(element_id: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    if let Some(element) = document.get_element_by_id(element_id) {
        let event = MouseEvent::new("click")?;
        element.dispatch_event(&event)?;
        Ok(())
    } else {
        Err(JsValue::from_str(&format!("Element {} not found", element_id)))
    }
}

/// Симулирует нажатие клавиши
pub fn simulate_keydown(_key: &str) -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Упрощенная версия без KeyboardEventInit
    let event = KeyboardEvent::new("keydown")?;
    
    document.dispatch_event(&event)?;
    Ok(())
}
