use std::path::Path;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use yew::prelude::*;

pub mod bindings;
pub mod components;
pub mod state;

#[cfg(test)]
mod tests;

use bindings::{convert_files, get_file_size, select_files, select_output_folder};
use state::{ConversionOptions, FileItem, FileStatus};

/// Форматирует размер файла: в КБ, если меньше 1 МБ, иначе в МБ.
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * 1024;
    if bytes < MB {
        format!("{:.1} КБ", bytes as f64 / KB as f64)
    } else {
        format!("{:.2} МБ", bytes as f64 / MB as f64)
    }
}

/// Главный компонент приложения
#[function_component]
pub fn App() -> Html {
    let files = use_state(|| Vec::<FileItem>::new());
    let status_message = use_state(|| "Готов к работе".to_string());
    let output_folder = use_state(|| "".to_string());
    let is_busy = use_state(|| false);
    let options = use_state(ConversionOptions::default);

    // Выбор SVG файлов
    let on_open_file_dialog = {
        let files = files.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            let files_clone = files.clone();
            let status_clone = status_message.clone();
            spawn_local(async move {
                match select_files().await {
                    Ok(paths) if !paths.is_empty() => {
                        let mut new_files = (*files_clone).clone();
                        for (i, p) in paths.iter().enumerate() {
                            let name = Path::new(p)
                                .file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("unknown")
                                .to_string();
                            // реальный размер файла через Tauri-команду
                            let size = get_file_size(p.clone()).await.unwrap_or(0);
                            new_files.push(FileItem {
                                id: format!("file_{}", i),
                                name,
                                path: p.clone(),
                                size,
                                status: FileStatus::Pending,
                            });
                        }
                        files_clone.set(new_files);
                        status_clone.set(format!("Добавлено {} файлов", paths.len()));
                    }
                    Ok(_) => status_clone.set("Файлы не выбраны".to_string()),
                    Err(e) => status_clone.set(format!("Ошибка выбора: {:?}", e)),
                }
            });
        })
    };

    // Выбор выходной папки
    let on_folder_select = {
        let output_folder = output_folder.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            let output_folder_clone = output_folder.clone();
            let status_clone = status_message.clone();
            spawn_local(async move {
                match select_output_folder().await {
                    Ok(folder) => {
                        output_folder_clone.set(folder.clone());
                        status_clone.set("Выходная папка выбрана".to_string());
                    }
                    Err(e) => status_clone.set(format!("Ошибка выбора папки: {:?}", e)),
                }
            });
        })
    };

    // Конвертация
    let on_convert = {
        let files = files.clone();
        let status_message = status_message.clone();
        let output_folder = output_folder.clone();
        let is_busy = is_busy.clone();
        let options = options.clone();
        Callback::from(move |_| {
            let current_files = (*files).clone();
            let current_output = (*output_folder).clone();
            let current_options = (*options).clone();
            if current_files.is_empty() {
                status_message.set("Сначала выберите файлы".to_string());
                return;
            }
            if current_output.is_empty() {
                status_message.set("Сначала выберите выходную папку".to_string());
                return;
            }
            status_message.set("Начинаю конвертацию...".to_string());
            is_busy.set(true);

            let files_clone = files.clone();
            let status_clone = status_message.clone();
            let is_busy_clone = is_busy.clone();
            spawn_local(async move {
                let paths: Vec<String> = current_files.iter().map(|f| f.path.clone()).collect();
                match convert_files(paths, current_output, current_options).await {
                    Ok(results) => {
                        let mut updated = (*files_clone).clone();
                        for (i, res) in results.iter().enumerate() {
                            let success = bindings::result_success(res);
                            let error = bindings::result_error(res);
                            if let Some(item) = updated.get_mut(i) {
                                item.status = if success {
                                    FileStatus::Completed
                                } else {
                                    FileStatus::Error(error.unwrap_or_else(|| "Ошибка".into()))
                                };
                            }
                        }
                        files_clone.set(updated);
                        status_clone.set("Конвертация завершена!".to_string());
                    }
                    Err(e) => {
                        status_clone.set(format!("Ошибка конвертации: {:?}", e));
                    }
                }
                is_busy_clone.set(false);
            });
        })
    };

    let on_clear = {
        let files = files.clone();
        let status_message = status_message.clone();
        Callback::from(move |_| {
            files.set(Vec::new());
            status_message.set("Очищено".to_string());
        })
    };

    // Переключатели настроек конвертации
    let options_toggle = {
        let options = options.clone();
        move |key: &str, val: bool| {
            let mut o = (*options).clone();
            match key {
                "fill_as_lines" => o.fill_as_lines = val,
                "preserve_colors" => o.preserve_colors = val,
                "true_color" => o.true_color = val,
                "trace_raster" => o.trace_raster = val,
                "overwrite" => o.overwrite = val,
                "add_color_suffix" => o.add_color_suffix = val,
                "add_hatch_suffix" => o.add_hatch_suffix = val,
                _ => {}
            }
            options.set(o);
        }
    };
    let on_toggle = {
        let t = options_toggle.clone();
        move |key: &'static str| {
            let t = t.clone();
            Callback::from(move |e: web_sys::Event| {
                let checked = e
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                    .map(|i| i.checked())
                    .unwrap_or(false);
                t(key, checked);
            })
        }
    };
    let on_toggle_fill = on_toggle("fill_as_lines");
    let on_toggle_colors = on_toggle("preserve_colors");
    let on_toggle_true_color = on_toggle("true_color");
    let on_toggle_trace = on_toggle("trace_raster");
    let on_toggle_overwrite = on_toggle("overwrite");
    let on_toggle_color_suffix = on_toggle("add_color_suffix");
    let on_toggle_hatch_suffix = on_toggle("add_hatch_suffix");

    // Что ещё нужно сделать, чтобы стала доступна конвертация
    let files_empty = (*files).is_empty();
    let folder_empty = (*output_folder).is_empty();
    let needs_action = files_empty || folder_empty;
    let action_hint: &str = if files_empty && folder_empty {
        "Сначала выберите SVG и папку вывода"
    } else if files_empty {
        "Выберите SVG файлы для конвертации"
    } else {
        "Выберите папку вывода"
    };

    html! {
        <div class="app">
            <style>{ include_str!("style.css") }</style>

            <header class="app-header">
                <div class="brand">
                    <span class="brand-icon">{"⬡"}</span>
                    <div>
                        <h1>{"SVG → DXF"}</h1>
                        <p class="subtitle">{"Конвертер векторной графики"}</p>
                    </div>
                </div>
                <div class="header-actions">
                    <div class="convert-bar">
                        <button class={classes!("btn", "btn-primary", if needs_action { "needs-action" } else { "" })}
                            onclick={on_convert}
                            disabled={(*files).is_empty() || (*output_folder).is_empty() || *is_busy}>
                            <span>{"🔄"}</span>{"Конвертировать"}
                        </button>
                        <label class="opt" title="Заменять существующие DXF. Если выключено — к имени добавляется индекс (_1, _2, …), чтобы не перезаписать результат.">
                            <input type="checkbox" checked={(*options).overwrite} onchange={on_toggle_overwrite}/>
                            <span>{"Заменять файлы"}</span>
                        </label>
                    </div>
                    if needs_action {
                        <div class="needs-action-hint">
                            <span class="arrow">{"👉"}</span>{action_hint}
                        </div>
                    }
                </div>
            </header>

            <main class="main-content">
                <section class="card">
                    <div class="card-head">
                        <h2>{"📁 Файлы"}</h2>
                        <div class="card-actions">
                            <button class="btn btn-ghost" onclick={on_open_file_dialog}>{"Выбрать SVG"}</button>
                            <button class="btn btn-ghost" onclick={on_folder_select}>{"Папка вывода"}</button>
                            <button class="btn btn-ghost danger" onclick={on_clear} disabled={(*files).is_empty()}>{"Очистить"}</button>
                        </div>
                    </div>

                    if (*output_folder).is_empty() {
                        <div class="hint">{"Выходная папка не выбрана — файлы не конвертируются"}</div>
                    } else {
                        <div class="hint ok">{"Папка вывода: "}{(*output_folder).clone()}</div>
                    }

                    <div class="file-list">
                        if (*files).is_empty() {
                            <div class="empty">
                                <div class="empty-icon">{"📂"}</div>
                                <p>{"Нет файлов. Нажмите «Выбрать SVG»."}</p>
                            </div>
                        } else {
                            {for (*files).iter().map(|file| {
                                let status_class = match file.status {
                                    FileStatus::Pending => "pending",
                                    FileStatus::Processing => "processing",
                                    FileStatus::Completed => "completed",
                                    FileStatus::Error(_) => "error",
                                };
                                let status_text = match &file.status {
                                    FileStatus::Pending => "Ожидание",
                                    FileStatus::Processing => "Обработка…",
                                    FileStatus::Completed => "Готово",
                                    FileStatus::Error(_) => "Ошибка",
                                };
                                html! {
                                    <div class="file-row">
                                        <span class="file-ico">{"📄"}</span>
                                        <div class="file-meta">
                                            <div class="file-name">{&file.name}</div>
                                            <div class="file-size">{format_size(file.size)}</div>
                                        </div>
                                        <span class={format!("badge {}", status_class)}>{status_text}</span>
                                    </div>
                                }
                            })}
                        }
                    </div>
                </section>

                <section class="card">
                    <div class="card-head"><h2>{"⚙️ Настройки конвертации"}</h2></div>

                    <div class="settings-group">
                        <h3>{"Качество"}</h3>
                        <div class="options">
                            <label class="opt" title="Рисовать заливку параллельными линиями внутри замкнутых фигур (вместо пустого контура).">
                                <input type="checkbox" checked={(*options).fill_as_lines} onchange={on_toggle_fill}/>
                                <span>{"Заливка линиями"}</span>
                            </label>
                            <label class="opt" title="Переносить цвета SVG (fill/stroke) в DXF. Выключите для чёрно-белого результата.">
                                <input type="checkbox" checked={(*options).preserve_colors} onchange={on_toggle_colors}/>
                                <span>{"Сохранять цвета"}</span>
                            </label>
                            <label class="opt" title="Точные цвета (true-color, группа 420) вместо приближённых ACI. Только при включённом «Сохранять цвета».">
                                <input type="checkbox" checked={(*options).true_color} onchange={on_toggle_true_color} disabled={!(*options).preserve_colors}/>
                                <span>{"Точные цвета"}</span>
                            </label>
                            <label class="opt" title="Трассировать растровые изображения (PNG/JPEG/GIF) в вектор через marching squares.">
                                <input type="checkbox" checked={(*options).trace_raster} onchange={on_toggle_trace}/>
                                <span>{"Трассировка растра"}</span>
                            </label>
                        </div>
                    </div>

                    <div class="settings-group">
                        <h3>{"Имя файла"}</h3>
                        <div class="options">
                            <label class={classes!("opt", if !(*options).preserve_colors { "disabled" } else { "" })}
                                title={if (*options).preserve_colors { "Добавлять к имени суффикс _color (например drawing_color.dxf)." } else { "Доступно только при включённом «Сохранять цвета»." }}>
                                <input type="checkbox" checked={(*options).add_color_suffix} onchange={on_toggle_color_suffix} disabled={!(*options).preserve_colors}/>
                                <span>{"+ _color"}</span>
                            </label>
                            <label class={classes!("opt", if !(*options).fill_as_lines { "disabled" } else { "" })}
                                title={if (*options).fill_as_lines { "Добавлять к имени суффикс _hatch (например drawing_hatch.dxf)." } else { "Доступно только при включённом «Заливка линиями»." }}>
                                <input type="checkbox" checked={(*options).add_hatch_suffix} onchange={on_toggle_hatch_suffix} disabled={!(*options).fill_as_lines}/>
                                <span>{"+ _hatch"}</span>
                            </label>
                        </div>
                        <div class="hint" style="margin-top:8px;margin-bottom:0;">
                            {"Без замены файлов (чекбокс у кнопки) имена создаются с индексом _1, _2, … при совпадении."}
                        </div>
                    </div>
                </section>

                <section class="card">
                    <div class="card-head"><h2>{"ℹ️ Статус"}</h2></div>
                    <div class="status-line">{(*status_message).clone()}</div>
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

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"WASM module loaded".into());
    run_app();
}
