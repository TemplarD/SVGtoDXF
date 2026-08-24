//! Правильный интерфейс к Tauri v2 backend (через @tauri-apps/api/core).
//! Использует глобальный window.__TAURI__.core.invoke (включается
//! опцией withGlobalTauri в tauri.conf.json) либо прямой импорт.

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;

/// Вызывает Tauri-команду. Работает и при withGlobalTauri=true
/// (window.__TAURI__.core.invoke), и если __TAURI_INTERNALS__ доступен.
async fn invoke_tauri(command: &str, args: &JsValue) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or(JsValue::from_str("no window"))?;

    // Путь 1: window.__TAURI__.core.invoke (withGlobalTauri)
    if let Ok(tauri) = Reflect::get(&window, &JsValue::from_str("__TAURI__")) {
        if let Ok(core) = Reflect::get(&tauri, &JsValue::from_str("core")) {
            if let Ok(invoke) = Reflect::get(&core, &JsValue::from_str("invoke")) {
                if let Ok(invoke_fn) = invoke.dyn_into::<Function>() {
                    let promise =
                        invoke_fn.call2(&JsValue::NULL, &JsValue::from_str(command), args)?;
                    let promise = promise.dyn_into::<js_sys::Promise>()?;
                    return JsFuture::from(promise).await;
                }
            }
        }
    }

    // Путь 2: нативный import (если бандлер предоставил)
    if let Ok(internals) = Reflect::get(&window, &JsValue::from_str("__TAURI_INTERNALS__")) {
        if let Ok(invoke) = Reflect::get(&internals, &JsValue::from_str("invoke")) {
            if let Ok(invoke_fn) = invoke.dyn_into::<Function>() {
                let promise = invoke_fn.call2(&JsValue::NULL, &JsValue::from_str(command), args)?;
                let promise = promise.dyn_into::<js_sys::Promise>()?;
                return JsFuture::from(promise).await;
            }
        }
    }

    Err(JsValue::from_str("Tauri runtime недоступен"))
}

/// Выбор SVG файлов через системный диалог.
/// Возвращает Vec<String> (абсолютные пути).
pub async fn select_files() -> Result<Vec<String>, JsValue> {
    let result = invoke_tauri("api_select_files", &JsValue::NULL).await?;
    let array = result.dyn_into::<js_sys::Array>()?;
    let mut out = Vec::new();
    for i in 0..array.length() {
        if let Some(s) = array.get(i).as_string() {
            out.push(s);
        }
    }
    Ok(out)
}

/// Выбор выходной папки через системный диалог.
/// Возвращает String (путь) или ошибку.
pub async fn select_output_folder() -> Result<String, JsValue> {
    let result = invoke_tauri("api_select_output_folder", &JsValue::NULL).await?;
    result
        .as_string()
        .ok_or(JsValue::from_str("не выбрана папка"))
}

/// Конвертация списка файлов в выходную папку.
/// Возвращает Vec<JsValue> (объекты FileConversionResult).
pub async fn convert_files(
    files: Vec<String>,
    output_folder: String,
    options: crate::state::ConversionOptions,
) -> Result<Vec<JsValue>, JsValue> {
    let args = Object::new();
    let files_js = js_sys::Array::new();
    for f in files {
        files_js.push(&JsValue::from_str(&f));
    }
    Reflect::set(&args, &JsValue::from_str("files"), &files_js)?;
    Reflect::set(
        &args,
        &JsValue::from_str("outputFolder"),
        &JsValue::from_str(&output_folder),
    )?;
    let options_js = JsValue::from_serde(&options)
        .map_err(|e| JsValue::from_str(&format!("ошибка сериализации options: {}", e)))?;
    Reflect::set(&args, &JsValue::from_str("options"), &options_js)?;

    let result = invoke_tauri("api_convert_files", &args).await?;
    let array = result.dyn_into::<js_sys::Array>()?;
    let mut out = Vec::new();
    for i in 0..array.length() {
        out.push(array.get(i));
    }
    Ok(out)
}

/// Получить размер файла в байтах через Tauri-команду.
pub async fn get_file_size(path: String) -> Result<u64, JsValue> {
    let args = Object::new();
    Reflect::set(&args, &JsValue::from_str("path"), &JsValue::from_str(&path))?;
    let result = invoke_tauri("api_get_file_size", &args).await?;
    result
        .as_f64()
        .map(|v| v as u64)
        .ok_or(JsValue::from_str("не удалось получить размер"))
}

/// Получить статус приложения.
pub async fn get_status() -> Result<String, JsValue> {
    let result = invoke_tauri("api_get_status", &JsValue::NULL).await?;
    Ok(result.as_string().unwrap_or_default())
}

/// Загрузить сохранённые между запусками папки.
/// Возвращает JsValue вида { output_folder, last_input_dir } (возможно пустые).
pub async fn load_folders() -> Result<JsValue, JsValue> {
    invoke_tauri("api_load_folders", &JsValue::NULL).await
}

/// Сохранить последние папки между запусками.
pub async fn save_folders(output_folder: String, last_input_dir: String) -> Result<(), JsValue> {
    let args = Object::new();
    let obj = Object::new();
    Reflect::set(
        &obj,
        &JsValue::from_str("output_folder"),
        &JsValue::from_str(&output_folder),
    )?;
    Reflect::set(
        &obj,
        &JsValue::from_str("last_input_dir"),
        &JsValue::from_str(&last_input_dir),
    )?;
    Reflect::set(&args, &JsValue::from_str("folders"), &obj)?;
    invoke_tauri("api_save_folders", &args).await?;
    Ok(())
}

/// Подписаться на событие от backend (например, "conversion_progress").
/// Возвращает id подписки; колбэк вызывается с распарсенным объектом события.
pub async fn listen_event(event: &str, callback: js_sys::Function) -> Result<JsValue, JsValue> {
    let window = web_sys::window().ok_or(JsValue::from_str("no window"))?;
    if let Ok(tauri) = Reflect::get(&window, &JsValue::from_str("__TAURI__")) {
        if let Ok(event_mod) = Reflect::get(&tauri, &JsValue::from_str("event")) {
            if let Ok(listen) = Reflect::get(&event_mod, &JsValue::from_str("listen")) {
                if let Ok(listen_fn) = listen.dyn_into::<Function>() {
                    let event_js = JsValue::from_str(event);
                    // listen(event, handler) -> Promise<unlisten>
                    let promise = listen_fn.call2(&JsValue::NULL, &event_js, &callback)?;
                    let promise = promise.dyn_into::<js_sys::Promise>()?;
                    return JsFuture::from(promise).await;
                }
            }
        }
    }
    Err(JsValue::from_str("Tauri runtime недоступен"))
}

/// Извлечь поле success из объекта FileConversionResult.
pub fn result_success(obj: &JsValue) -> bool {
    Reflect::get(obj, &JsValue::from_str("success"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

/// Извлечь поле error (строка) из объекта FileConversionResult.
pub fn result_error(obj: &JsValue) -> Option<String> {
    Reflect::get(obj, &JsValue::from_str("error"))
        .ok()
        .and_then(|v| v.as_string())
}

/// Тестовая функция (для проверки сборки wasm).
pub fn bindings_ready() -> bool {
    console::log_1(&"bindings module loaded".into());
    true
}
