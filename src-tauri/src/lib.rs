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
        commands::get_log_directory,
        // Система логирования
        commands::init_logging_system,
        commands::write_log,
        commands::get_log_files,
        commands::read_log_file,
        commands::cleanup_old_logs
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      
      // Инициализируем систему логирования
      tauri::async_runtime::spawn(async move {
          if let Err(e) = commands::init_logging_system().await {
              log::error!("Не удалось инициализировать систему логирования: {}", e);
          }
      });
      
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
