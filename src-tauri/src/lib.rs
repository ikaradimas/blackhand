mod commands;
mod paths;
mod session;
mod stats;
mod types;

use tauri::Manager;
use tauri_specta::{collect_commands, collect_events, Builder};

fn make_specta_builder() -> Builder<tauri::Wry> {
    Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::add_magnet,
            commands::add_torrent_file,
            commands::list_torrents,
            commands::pause,
            commands::resume,
            commands::forget,
            commands::delete,
            commands::session_stats,
        ])
        .events(collect_events![
            stats::TorrentsSnapshotEvent,
            stats::SessionStatsEvent,
        ])
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let specta_builder = make_specta_builder();

    #[cfg(debug_assertions)]
    specta_builder
        .export(
            specta_typescript::Typescript::default(),
            "../src/lib/bindings.ts",
        )
        .expect("failed to export tauri-specta bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            specta_builder.mount_events(app);

            let api = tauri::async_runtime::block_on(session::build_api())?;
            app.manage(api.clone());

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(stats::run(handle, api));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        make_specta_builder()
            .export(
                specta_typescript::Typescript::default(),
                "../src/lib/bindings.ts",
            )
            .expect("failed to export tauri-specta bindings");
    }
}
