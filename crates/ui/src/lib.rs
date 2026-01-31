use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlInputElement, console};
use js_sys::Reflect;
use std::path::Path;

pub mod state;
pub mod components;
pub mod bindings;

#[cfg(test)]
mod tests;

use state::{FileItem, FileStatus};

/// Главный компонент приложения
#[function_component]
pub fn App() -> Html {
    let files = use_state(|| Vec::<FileItem>::new());
    let status_message = use_state(|| "Готов к работе".to_string());
    let output_folder = use_state(|| "".to_string());
    let debug_mode = use_state(|| false);
    
    // Слушаем события F12
    let debug_mode_clone = debug_mode.clone();
    use_effect_with((), |_| {
        let window = web_sys::window().unwrap();
        let listener = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "F12" {
                debug_mode_clone.set(!*debug_mode_clone);
                web_sys::console::log_1(&"🔧 F12 - переключение дебаг режима".into());
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        
        window.add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref()).unwrap();
        listener.forget();
        
        || ()
    });
    
    let on_folder_select = {
        let output_folder = output_folder.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            console::log_1(&"on_folder_select triggered".into());
            let output_folder_clone = output_folder.clone();
            let status_message_clone = status_message.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"Starting folder selection".into());
                
                if let Some(window) = window() {
                    if let Some(tauri) = Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__TAURI__")).ok() {
                        if let Some(invoke) = Reflect::get(&tauri, &wasm_bindgen::JsValue::from_str("invoke")).ok() {
                            if let Some(invoke_fn) = invoke.dyn_into::<js_sys::Function>().ok() {
                                let result = invoke_fn.call1(
                                    &wasm_bindgen::JsValue::NULL,
                                    &wasm_bindgen::JsValue::from_str("api_select_output_folder")
                                );
                                
                                match result {
                                    Ok(promise) => {
                                        let promise = promise.dyn_into::<js_sys::Promise>().unwrap();
                                        let future = wasm_bindgen_futures::JsFuture::from(promise);
                                        
                                        match future.await {
                                            Ok(folder) => {
                                                console::log_1(&"Promise resolved".into());
                                                if let Some(folder_str) = folder.as_string() {
                                                    console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Folder selected: {}", folder_str)));
                                                    output_folder_clone.set(folder_str);
                                                    status_message_clone.set("Выходная папка выбрана".to_string());
                                                }
                                            }
                                            Err(e) => {
                                                console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Promise error: {:?}", e)));
                                                status_message_clone.set("Ошибка выбора папки".to_string());
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Invoke error: {:?}", e)));
                                        status_message_clone.set("Ошибка вызова Tauri".to_string());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    status_message_clone.set("Tauri API недоступен".to_string());
                }
            });
        })
    };
    
    let on_convert = {
        let files = files.clone();
        let status_message = status_message.clone();
        let output_folder = output_folder.clone();
        Callback::from(move |_| {
            console::log_1(&"on_convert triggered".into());
            let current_files = (*files).clone();
            let current_output_folder = (*output_folder).clone();
            
            if current_files.is_empty() {
                status_message.set("Сначала выберите файлы".to_string());
                return;
            }
            
            if current_output_folder.is_empty() {
                status_message.set("Сначала выберите выходную папку".to_string());
                return;
            }
            
            status_message.set("Начинаю конвертацию...".to_string());
            
            let files_clone = files.clone();
            let status_clone = status_message.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                let mut updated_files = (*files_clone).clone();
                let file_names: Vec<String> = updated_files.iter().map(|f| f.name.clone()).collect();
                
                if let Some(window) = window() {
                    if let Some(tauri) = Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__TAURI__")).ok() {
                        if let Some(invoke) = Reflect::get(&tauri, &wasm_bindgen::JsValue::from_str("invoke")).ok() {
                            if let Some(invoke_fn) = invoke.dyn_into::<js_sys::Function>().ok() {
                                let args = serde_json::json!({
                                    "files": file_names,
                                    "output_folder": current_output_folder
                                });
                                
                                let result = invoke_fn.call2(
                                    &wasm_bindgen::JsValue::NULL,
                                    &wasm_bindgen::JsValue::from_str("api_convert_files"),
                                    &wasm_bindgen::JsValue::from_str(&args.to_string())
                                );
                                
                                match result {
                                    Ok(promise) => {
                                        let promise = promise.dyn_into::<js_sys::Promise>().unwrap();
                                        let future = wasm_bindgen_futures::JsFuture::from(promise);
                                        
                                        match future.await {
                                            Ok(_results) => {
                                                console::log_1(&"Conversion completed".into());
                                                status_clone.set("Конвертация завершена!".to_string());
                                                
                                                if let Ok(results_array) = _results.dyn_into::<js_sys::Array>() {
                                                    let mut updated_files = (*files_clone).clone();
                                                    for i in 0..results_array.length() {
                                                        let i_usize = i as usize;
                                                        if let Some(result) = results_array.get(i).dyn_into::<js_sys::Object>().ok() {
                                                            let success = js_sys::Reflect::get(&result, &wasm_bindgen::JsValue::from_str("success"))
                                                                .and_then(|v| v.as_bool().ok_or(wasm_bindgen::JsValue::NULL))
                                                                .unwrap_or(false);
                                                            
                                                            if success {
                                                                if let Some(file_item) = updated_files.get_mut(i_usize) {
                                                                    file_item.status = FileStatus::Completed;
                                                                }
                                                            } else {
                                                                if let Some(file_item) = updated_files.get_mut(i_usize) {
                                                                    let error_msg = js_sys::Reflect::get(&result, &wasm_bindgen::JsValue::from_str("error"))
                                                                        .and_then(|v| v.as_string().ok_or(wasm_bindgen::JsValue::NULL))
                                                                        .unwrap_or_else(|_| "Ошибка конвертации".to_string());
                                                                    file_item.status = FileStatus::Error(error_msg);
                                                                }
                                                            }
                                                        }
                                                    }
                                                    files_clone.set(updated_files);
                                                }
                                            }
                                            Err(e) => {
                                                console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Conversion error: {:?}", e)));
                                                status_clone.set("Ошибка конвертации".to_string());
                                                
                                                for file in updated_files.iter_mut() {
                                                    file.status = FileStatus::Error("Ошибка конвертации".to_string());
                                                }
                                                files_clone.set(updated_files);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Invoke error: {:?}", e)));
                                        status_clone.set("Ошибка вызова конвертации".to_string());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    status_clone.set("Tauri API недоступен".to_string());
                }
            });
        })
    };
    
    let on_clear = {
        let files = files.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            console::log_1(&"on_clear triggered".into());
            files.set(Vec::new());
            status_message.set("Очищено".to_string());
        })
    };
    
    let on_open_file_dialog = {
        let files = files.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            console::log_1(&"on_open_file_dialog triggered".into());
            let files_clone = files.clone();
            let status_clone = status_message.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                console::log_1(&"Starting file selection".into());
                
                if let Some(window) = window() {
                    if let Some(tauri) = Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__TAURI__")).ok() {
                        if let Some(invoke) = Reflect::get(&tauri, &wasm_bindgen::JsValue::from_str("invoke")).ok() {
                            if let Some(invoke_fn) = invoke.dyn_into::<js_sys::Function>().ok() {
                                let result = invoke_fn.call1(
                                    &wasm_bindgen::JsValue::NULL,
                                    &wasm_bindgen::JsValue::from_str("api_select_files")
                                );
                                
                                match result {
                                    Ok(promise) => {
                                        let promise = promise.dyn_into::<js_sys::Promise>().unwrap();
                                        let future = wasm_bindgen_futures::JsFuture::from(promise);
                                        
                                        match future.await {
                                            Ok(selected_files) => {
                                                console::log_1(&"Files selected".into());
                                                if let Ok(files_array) = selected_files.dyn_into::<js_sys::Array>() {
                                                    let mut new_files = (*files_clone).clone();
                                                    for i in 0..files_array.length() {
                                                        if let Some(file_path) = files_array.get(i).as_string() {
                                                            let file_name = Path::new(&file_path)
                                                                .file_name()
                                                                .and_then(|n| n.to_str())
                                                                .unwrap_or("unknown")
                                                                .to_string();
                                                            
                                                            let file_item = FileItem {
                                                                id: format!("file_{}", i),
                                                                name: file_name,
                                                                size: 0,
                                                                status: FileStatus::Pending,
                                                            };
                                                            new_files.push(file_item);
                                                        }
                                                    }
                                                    files_clone.set(new_files);
                                                    status_clone.set(format!("Добавлено {} файлов", files_array.length()));
                                                }
                                            }
                                            Err(e) => {
                                                console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Promise error: {:?}", e)));
                                                status_clone.set("Ошибка выбора файлов".to_string());
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        console::log_1(&wasm_bindgen::JsValue::from_str(&format!("Invoke error: {:?}", e)));
                                        status_clone.set("Ошибка вызова Tauri".to_string());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    status_clone.set("Tauri API недоступен".to_string());
                }
            });
        })
    };

    html! {
        <div class="app">
            <style>
                { include_str!("style.css") }
            </style>
            
            // Индикатор дебаг режима
            if *debug_mode {
                <div style="position: fixed; top: 10px; left: 10px; background: #ff6b6b; color: white; padding: 5px 10px; border-radius: 5px; z-index: 1000; font-size: 12px;">
                    {"🔧 ДЕБУГ РЕЖИМ (F12)"}
                </div>
            }
            
            <header>
                <h1>{"SVG to DXF Converter"}</h1>
                <p class="subtitle">{"Современный конвертер векторной графики"}</p>
            </header>
            
            <main class="main-content">
                <section class="section">
                    <h2 class="section-title">
                        <span class="icon">{"📁"}</span>
                        {"Управление файлами"}
                    </h2>
                    
                    <div class="controls">
                        <button class="btn" onclick={on_open_file_dialog}>
                            <span>{"📄"}</span>
                            {"Выбрать SVG файлы"}
                        </button>
                        
                        <button class="btn btn-secondary" onclick={on_folder_select}>
                            <span>{"📁"}</span>
                            {"Выбрать выходную папку"}
                        </button>
                        
                        <button class="btn" onclick={on_convert} disabled={(*files).is_empty() || (*output_folder).is_empty()}>
                            <span>{"🔄"}</span>
                            {"Конвертировать в DXF"}
                        </button>
                        
                        <button class="btn btn-secondary" onclick={on_clear} disabled={(*files).is_empty()}>
                            <span>{"🗑️"}</span>
                            {"Очистить"}
                        </button>
                    </div>
                    
                    if !(*output_folder).is_empty() {
                        <div class="status-message" style="margin-top: 15px;">
                            {"Выходная папка: "}{(*output_folder).clone()}
                        </div>
                    }
                </section>
                
                <section class="section">
                    <h2 class="section-title">
                        <span class="icon">{"📋"}</span>
                        {"Список файлов"}
                    </h2>
                    
                    <div class="file-list">
                        if (*files).is_empty() {
                            <div class="empty-state">
                                <div class="empty-state-icon">{"📁"}</div>
                                <p>{"Нет выбранных файлов"}</p>
                                <p>{"Нажмите 'Выбрать SVG файлы' для добавления"}</p>
                            </div>
                        } else {
                            {for (*files).iter().map(|file| {
                                let status_class = match file.status {
                                    FileStatus::Pending => "status-pending",
                                    FileStatus::Processing => "status-processing", 
                                    FileStatus::Completed => "status-completed",
                                    FileStatus::Error(_) => "status-error",
                                };
                                
                                let status_text = match file.status {
                                    FileStatus::Pending => "Ожидание",
                                    FileStatus::Processing => "Обработка...",
                                    FileStatus::Completed => "Завершено",
                                    FileStatus::Error(_) => "Ошибка",
                                };
                                
                                html! {
                                    <div class="file-item">
                                        <div class="file-info">
                                            <span>{"📄"}</span>
                                            <div>
                                                <div class="file-name">{&file.name}</div>
                                                <div class="file-size">{format!("{} байт", file.size)}</div>
                                            </div>
                                        </div>
                                        <span class={format!("file-status {}", status_class)}>
                                            {status_text}
                                        </span>
                                    </div>
                                }
                            })}
                        }
                    </div>
                </section>
                
                <section class="section status-section">
                    <h2 class="section-title">
                        <span class="icon">{"ℹ️"}</span>
                        {"Статус"}
                    </h2>
                    
                    <div class="status-message">
                        {(*status_message).clone()}
                    </div>
                </section>
            </main>
        </div>
    }
}

/// Главная точка входа Yew приложения
pub fn run_app() {
    console::log_1(&"SVG to DXF Converter starting".into());
    yew::Renderer::<App>::new().render();
}

// Инициализация при загрузке WASM
#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"WASM module loaded".into());
    run_app();
}
