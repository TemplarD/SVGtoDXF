//! Точка входа Tauri приложения

// На Windows собираем как GUI-приложение (windows subsystem), чтобы при
// запуске .exe не всплывало чёрное окно консоли, закрытие которого
// завершало бы всё приложение. На других ОС атрибут игнорируется.
#![cfg_attr(windows, windows_subsystem = "windows")]

fn main() {
    svg2dxf_tauri_app::run();
}
