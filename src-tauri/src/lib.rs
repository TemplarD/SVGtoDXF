#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod commands;

pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
        // Файловая система
        commands::list_directory,
        commands::get_svg_files_in_directory,
        commands::get_system_roots,
        // Конвертация
        commands::convert_single_file,
        commands::convert_multiple_files,
        commands::check_directory_access,
        commands::get_output_path,
        // Отладка и тестирование
        commands::toggle_debug_mode,
        commands::create_debug_report,
        commands::run_autotest,
        commands::get_log_directory
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
