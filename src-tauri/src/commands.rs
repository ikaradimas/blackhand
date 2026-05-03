use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use librqbit::api::{ApiTorrentListOpts, TorrentIdOrHash};
use librqbit::{AddTorrent, Api};
use tauri::{AppHandle, State};

use crate::categories::{CategoryInfo, CategoryStore};
use crate::settings::{self, AppSettings};
use crate::types::{AddTorrentResult, DiskSpace, SessionStats, TorrentDetail, TorrentSnapshot};

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
pub fn list_torrents(
    api: State<'_, Arc<Api>>,
    cats: State<'_, Arc<CategoryStore>>,
) -> CmdResult<TorrentSnapshot> {
    Ok(TorrentSnapshot::from_response(
        api.api_torrent_list_ext(ApiTorrentListOpts { with_stats: true }),
        Some(cats.inner()),
    ))
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
pub fn list_categories(
    api: State<'_, Arc<Api>>,
    cats: State<'_, Arc<CategoryStore>>,
) -> CmdResult<Vec<CategoryInfo>> {
    let resp = api.api_torrent_list_ext(ApiTorrentListOpts { with_stats: false });
    let info_hashes: Vec<String> = resp.torrents.into_iter().map(|t| t.info_hash).collect();
    Ok(cats.inner().list_with_counts(&info_hashes))
}

#[tauri::command]
#[specta::specta]
pub fn set_torrent_category(
    cats: State<'_, Arc<CategoryStore>>,
    info_hash: String,
    category: Option<String>,
) -> CmdResult<()> {
    cats.inner().set_category(info_hash, category).map_err(err)
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

/// Walk up the path until we find an existing ancestor. Returns None only
/// if even the root doesn't exist (essentially impossible in normal use).
fn first_existing(p: &Path) -> Option<&Path> {
    let mut cur: Option<&Path> = Some(p);
    while let Some(c) = cur {
        if c.exists() {
            return Some(c);
        }
        cur = c.parent();
    }
    None
}

/// Disk capacity + available space for the filesystem hosting `path`.
/// If `path` is None or empty, resolves to the currently-configured download
/// directory (creating it if needed via the existing settings::resolve helper).
#[tauri::command]
#[specta::specta]
pub fn disk_space(path: Option<String>) -> CmdResult<DiskSpace> {
    let target: PathBuf = match path.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        Some(p) => PathBuf::from(p),
        None => {
            let cfg = settings::load().unwrap_or_default();
            settings::resolve_download_dir(&cfg).map_err(err)?
        }
    };
    let queried = first_existing(&target)
        .ok_or_else(|| "no existing parent directory".to_string())?;
    let total = fs2::total_space(queried).map_err(err)?;
    let free = fs2::available_space(queried).map_err(err)?;
    Ok(DiskSpace {
        total_bytes: total,
        free_bytes: free,
        path: queried.to_string_lossy().into_owned(),
    })
}
