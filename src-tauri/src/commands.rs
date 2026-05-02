use std::collections::HashSet;
use std::sync::Arc;

use librqbit::api::{ApiTorrentListOpts, TorrentIdOrHash};
use librqbit::{AddTorrent, Api};
use tauri::{AppHandle, State};

use crate::settings::{self, AppSettings};
use crate::types::{AddTorrentResult, SessionStats, TorrentDetail, TorrentSnapshot};

type CmdResult<T> = Result<T, String>;

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

fn id_to_handle(id: u64) -> TorrentIdOrHash {
    TorrentIdOrHash::Id(id as usize)
}

#[tauri::command]
#[specta::specta]
pub async fn add_magnet(
    api: State<'_, Arc<Api>>,
    uri: String,
) -> CmdResult<AddTorrentResult> {
    let resp = api
        .api_add_torrent(AddTorrent::from_url(uri), None)
        .await
        .map_err(err)?;
    Ok(AddTorrentResult {
        id: resp.id.map(|x| x as u64),
        info_hash: resp.details.info_hash,
        name: resp.details.name,
    })
}

#[tauri::command]
#[specta::specta]
pub async fn add_torrent_file(
    api: State<'_, Arc<Api>>,
    bytes: Vec<u8>,
) -> CmdResult<AddTorrentResult> {
    let resp = api
        .api_add_torrent(AddTorrent::from_bytes(bytes), None)
        .await
        .map_err(err)?;
    Ok(AddTorrentResult {
        id: resp.id.map(|x| x as u64),
        info_hash: resp.details.info_hash,
        name: resp.details.name,
    })
}

#[tauri::command]
#[specta::specta]
pub fn list_torrents(api: State<'_, Arc<Api>>) -> CmdResult<TorrentSnapshot> {
    Ok(api
        .api_torrent_list_ext(ApiTorrentListOpts { with_stats: true })
        .into())
}

#[tauri::command]
#[specta::specta]
pub async fn pause(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<()> {
    api.api_torrent_action_pause(id_to_handle(id))
        .await
        .map_err(err)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn resume(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<()> {
    api.api_torrent_action_start(id_to_handle(id))
        .await
        .map_err(err)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn forget(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<()> {
    api.api_torrent_action_forget(id_to_handle(id))
        .await
        .map_err(err)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn delete(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<()> {
    api.api_torrent_action_delete(id_to_handle(id))
        .await
        .map_err(err)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn session_stats(api: State<'_, Arc<Api>>) -> CmdResult<SessionStats> {
    Ok(api.api_session_stats().into())
}

#[tauri::command]
#[specta::specta]
pub fn get_torrent_detail(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<TorrentDetail> {
    let details = api.api_torrent_details(id_to_handle(id)).map_err(err)?;
    let stats = api.api_stats_v1(id_to_handle(id)).map_err(err)?;
    Ok(TorrentDetail::from_parts(details, stats))
}

#[tauri::command]
#[specta::specta]
pub async fn set_only_files(
    api: State<'_, Arc<Api>>,
    id: u64,
    file_idxs: Vec<u32>,
) -> CmdResult<()> {
    let set: HashSet<usize> = file_idxs.into_iter().map(|x| x as usize).collect();
    api.api_torrent_action_update_only_files(id_to_handle(id), &set)
        .await
        .map_err(err)?;
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn get_trackers(api: State<'_, Arc<Api>>, id: u64) -> CmdResult<Vec<String>> {
    let handle = api.mgr_handle(id_to_handle(id)).map_err(err)?;
    let mut urls: Vec<String> = handle
        .shared()
        .trackers
        .iter()
        .map(|u| u.to_string())
        .collect();
    urls.sort();
    Ok(urls)
}

#[tauri::command]
#[specta::specta]
pub fn get_settings() -> CmdResult<AppSettings> {
    settings::load().map_err(err)
}

#[tauri::command]
#[specta::specta]
pub fn save_settings(
    api: State<'_, Arc<Api>>,
    settings: AppSettings,
) -> CmdResult<()> {
    settings::save(&settings).map_err(err)?;
    // Bandwidth limits can be applied live — no restart needed.
    let session = api.session();
    session
        .ratelimits
        .set_upload_bps(settings::kbps_to_nz_bps(settings.upload_limit_kbps));
    session
        .ratelimits
        .set_download_bps(settings::kbps_to_nz_bps(settings.download_limit_kbps));
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub fn restart_app(app: AppHandle) {
    app.restart();
}

#[tauri::command]
#[specta::specta]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
