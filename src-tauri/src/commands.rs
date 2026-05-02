use std::sync::Arc;

use librqbit::api::{ApiTorrentListOpts, TorrentIdOrHash};
use librqbit::{AddTorrent, Api};
use tauri::State;

use crate::types::{AddTorrentResult, SessionStats, TorrentSnapshot};

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
