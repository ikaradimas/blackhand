mod categories;
mod commands;
mod paths;
mod session;
mod settings;
mod stats;
mod types;

use std::sync::Arc;

use categories::CategoryStore;
use librqbit::api::{ApiTorrentListOpts, TorrentIdOrHash};
use librqbit::{AddTorrent, Api};
use settings::AppSettings;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};
use tauri_specta::{collect_commands, collect_events, Builder};

fn add_magnet_url(api: Arc<Api>, url: String) {
    tauri::async_runtime::spawn(async move {
        if let Err(e) = api.api_add_torrent(AddTorrent::from_url(url), None).await {
            log::warn!("deep-link add failed: {e:#}");
        }
    });
}

/// Scan an argv-like iterator for `.torrent` file paths and add them.
/// Used both at cold start (`std::env::args`) and when single-instance
/// hands us a second-launch argv from a "Open with…" file association.
fn add_torrent_files_from_argv<I, S>(api: Arc<Api>, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    for arg in args {
        let path = arg.as_ref().to_string();
        if !path.to_lowercase().ends_with(".torrent") {
            continue;
        }
        let api = api.clone();
        tauri::async_runtime::spawn(async move {
            match AddTorrent::from_local_filename(&path) {
                Ok(add) => {
                    if let Err(e) = api.api_add_torrent(add, None).await {
                        log::warn!("file-association add failed for {path}: {e:#}");
                    }
                }
                Err(e) => log::warn!("read torrent file {path} failed: {e:#}"),
            }
        });
    }
}

fn toggle_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let visible = w.is_visible().unwrap_or(false);
        if visible {
            let _ = w.hide();
        } else {
            let _ = w.unminimize();
            let _ = w.show();
            let _ = w.set_focus();
        }
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.unminimize();
        let _ = w.show();
        let _ = w.set_focus();
    }
}

/// Iterate the current torrents and pause (or resume) each. Used by the tray menu.
fn bulk_action(app: &AppHandle, pause: bool) {
    let Some(api) = app.try_state::<Arc<Api>>() else {
        return;
    };
    let api = api.inner().clone();
    let ids: Vec<u64> = api
        .api_torrent_list_ext(ApiTorrentListOpts { with_stats: false })
        .torrents
        .into_iter()
        .filter_map(|t| t.id.map(|i| i as u64))
        .collect();
    tauri::async_runtime::spawn(async move {
        for id in ids {
            let handle = TorrentIdOrHash::Id(id as usize);
            let res = if pause {
                api.api_torrent_action_pause(handle).await
            } else {
                api.api_torrent_action_start(handle).await
            };
            if let Err(e) = res {
                log::warn!("tray bulk {} failed for id={id}: {e:#}", if pause { "pause" } else { "resume" });
            }
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
            commands::get_torrent_detail,
            commands::set_only_files,
            commands::get_trackers,
            commands::list_categories,
            commands::set_torrent_category,
            commands::get_settings,
            commands::save_settings,
            commands::restart_app,
            commands::app_version,
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
            |app, argv, _cwd| {
                // Bring the running instance forward; the deep-link plugin
                // (with the "deep-link" feature) forwards magnet URLs through
                // its own on_open_url channel, so we don't parse those here.
                show_main_window(app);
                // .torrent file paths from "Open with…" come through as plain
                // argv entries — process them ourselves.
                if let Some(api) = app.try_state::<Arc<Api>>() {
                    add_torrent_files_from_argv(api.inner().clone(), argv);
                }
            },
        ));
    }

    tauri_builder
        .on_window_event(|window, event| {
            // Route window close + minimize through the tray when the user
            // has hide_to_tray enabled. Reopen via tray-icon click or menu.
            let app = window.app_handle();
            let Some(cfg) = app.try_state::<Arc<AppSettings>>() else {
                return;
            };
            if !cfg.hide_to_tray {
                return;
            }
            match event {
                tauri::WindowEvent::Resized(_) => {
                    if window.is_minimized().unwrap_or(false) {
                        let _ = window.unminimize();
                        let _ = window.hide();
                    }
                }
                tauri::WindowEvent::CloseRequested { api: close_api, .. } => {
                    close_api.prevent_close();
                    let _ = window.hide();
                }
                _ => {}
            }
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("blackhand".into()),
                    }),
                ])
                .level(log::LevelFilter::Info)
                .max_file_size(5 * 1024 * 1024 /* 5 MB */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepOne)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(specta_builder.invoke_handler())
        .setup(move |app| {
            use tauri_plugin_deep_link::DeepLinkExt;

            specta_builder.mount_events(app);

            let cfg = settings::load().unwrap_or_default();
            let api = tauri::async_runtime::block_on(session::build_api(&cfg))?;
            app.manage(api.clone());
            app.manage(Arc::new(cfg));

            let cats = Arc::new(CategoryStore::load());
            app.manage(cats.clone());

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(stats::run(handle, api.clone(), cats.clone()));

            // Cold-start deep links (app launched *with* a magnet URL).
            if let Ok(Some(urls)) = app.deep_link().get_current() {
                for url in urls {
                    add_magnet_url(api.clone(), url.to_string());
                }
            }

            // Cold-start .torrent file paths (app launched via "Open with…").
            // Skip arg 0 (program name).
            add_torrent_files_from_argv(api.clone(), std::env::args().skip(1));

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

            // System tray with menu.
            let show_i = MenuItem::with_id(app, "show", "Show BlackHand", true, None::<&str>)?;
            let pause_i = MenuItem::with_id(app, "pause_all", "Pause all", true, None::<&str>)?;
            let resume_i = MenuItem::with_id(app, "resume_all", "Resume all", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &pause_i, &resume_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => show_main_window(app),
                    "pause_all" => bulk_action(app, true),
                    "resume_all" => bulk_action(app, false),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_main_window(tray.app_handle());
                    }
                })
                .build(app)?;

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
