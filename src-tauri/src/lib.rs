mod commands;
mod paths;
mod session;
mod stats;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let api = tauri::async_runtime::block_on(session::build_api())?;
            app.manage(api.clone());

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(stats::run(handle, api));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_magnet,
            commands::add_torrent_file,
            commands::list_torrents,
            commands::get_torrent,
            commands::pause,
            commands::resume,
            commands::forget,
            commands::delete,
            commands::session_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
