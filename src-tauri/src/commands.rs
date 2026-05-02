use std::sync::Arc;

use librqbit::{AddTorrent, Api};
use librqbit::api::{ApiTorrentListOpts, TorrentIdOrHash};
use tauri::State;

type CmdResult<T> = Result<T, String>;

fn err<E: std::fmt::Display>(e: E) -> String {
    e.to_string()
}

#[tauri::command]
pub async fn add_magnet(
    api: State<'_, Arc<Api>>,
    uri: String,
) -> CmdResult<serde_json::Value> {
    let resp = api
        .api_add_torrent(AddTorrent::from_url(uri), None)
        .await
        .map_err(err)?;
    serde_json::to_value(resp).map_err(err)
}

#[tauri::command]
pub async fn add_torrent_file(
    api: State<'_, Arc<Api>>,
    bytes: Vec<u8>,
) -> CmdResult<serde_json::Value> {
    let resp = api
        .api_add_torrent(AddTorrent::from_bytes(bytes), None)
        .await
        .map_err(err)?;
    serde_json::to_value(resp).map_err(err)
}

#[tauri::command]
pub fn list_torrents(api: State<'_, Arc<Api>>) -> CmdResult<serde_json::Value> {
    let resp = api.api_torrent_list_ext(ApiTorrentListOpts { with_stats: true });
    serde_json::to_value(resp).map_err(err)
}

#[tauri::command]
pub fn get_torrent(
    api: State<'_, Arc<Api>>,
    id: TorrentIdOrHash,
) -> CmdResult<serde_json::Value> {
    let resp = api.api_torrent_details(id).map_err(err)?;
    serde_json::to_value(resp).map_err(err)
}

#[tauri::command]
pub async fn pause(
    api: State<'_, Arc<Api>>,
    id: TorrentIdOrHash,
) -> CmdResult<()> {
    api.api_torrent_action_pause(id).await.map_err(err)?;
    Ok(())
}

#[tauri::command]
pub async fn resume(
    api: State<'_, Arc<Api>>,
    id: TorrentIdOrHash,
) -> CmdResult<()> {
    api.api_torrent_action_start(id).await.map_err(err)?;
    Ok(())
}

#[tauri::command]
pub async fn forget(
    api: State<'_, Arc<Api>>,
    id: TorrentIdOrHash,
) -> CmdResult<()> {
    api.api_torrent_action_forget(id).await.map_err(err)?;
    Ok(())
}

#[tauri::command]
pub async fn delete(
    api: State<'_, Arc<Api>>,
    id: TorrentIdOrHash,
) -> CmdResult<()> {
    api.api_torrent_action_delete(id).await.map_err(err)?;
    Ok(())
}

#[tauri::command]
pub fn session_stats(api: State<'_, Arc<Api>>) -> CmdResult<serde_json::Value> {
    serde_json::to_value(api.api_session_stats()).map_err(err)
}
