#[cfg_attr(mobile, tauri::mobile_entry_point)]



mod search_devices;


pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            crate::search_devices::search_devices // Registra o comando aqui
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
