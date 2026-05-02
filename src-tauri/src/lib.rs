mod commands;
mod paths;
mod session;
mod settings;
mod stats;
mod types;

use std::sync::Arc;

use librqbit::{AddTorrent, Api};
use tauri::Manager;
use tauri_specta::{collect_commands, collect_events, Builder};

fn add_magnet_url(api: Arc<Api>, url: String) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = api.api_add_torrent(AddTorrent::from_url(url), None).await {
            eprintln!("deep-link add failed: {e:#}");
        }
    });
}

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
            commands::get_settings,
            commands::save_settings,
            commands::restart_app,
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

    let mut tauri_builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        tauri_builder = tauri_builder.plugin(tauri_plugin_single_instance::init(
            |app, _argv, _cwd| {
                // Bring the running instance forward; the deep-link plugin
                // (with the "deep-link" feature) forwards the URL through
                // its own on_open_url channel, so we don't need to parse argv here.
                if let Some(w) = app.get_webview_window("main") {
                    let _ = w.unminimize();
                    let _ = w.show();
                    let _ = w.set_focus();
                }
            },
        ));
    }

    tauri_builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            use tauri_plugin_deep_link::DeepLinkExt;

            specta_builder.mount_events(app);

            let cfg = settings::load().unwrap_or_default();
            let api = tauri::async_runtime::block_on(session::build_api(&cfg))?;
            app.manage(api.clone());

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(stats::run(handle, api.clone()));

            // Cold-start deep links (app launched *with* a magnet URL).
            if let Ok(Some(urls)) = app.deep_link().get_current() {
                for url in urls {
                    add_magnet_url(api.clone(), url.to_string());
                }
            }

            // Runtime deep links (already running, OS hands us another URL).
            let api_for_cb = api.clone();
            app.deep_link().on_open_url(move |event| {
                for url in event.urls() {
                    add_magnet_url(api_for_cb.clone(), url.to_string());
                }
            });

            // Linux/Windows need explicit runtime registration to test in dev.
            // macOS uses Info.plist, written by `tauri build`.
            #[cfg(any(windows, target_os = "linux"))]
            app.deep_link().register_all()?;

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
