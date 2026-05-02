use std::sync::Arc;
use std::time::Duration;

use librqbit::api::ApiTorrentListOpts;
use librqbit::Api;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;
use tauri_specta::Event;

use crate::types::{SessionStats, TorrentSnapshot};

const TICK: Duration = Duration::from_millis(500);

#[derive(Serialize, Deserialize, Type, Clone, Debug, Event)]
pub struct TorrentsSnapshotEvent(pub TorrentSnapshot);

#[derive(Serialize, Deserialize, Type, Clone, Debug, Event)]
pub struct SessionStatsEvent(pub SessionStats);

pub async fn run(app: AppHandle, api: Arc<Api>) {
    let mut interval = tokio::time::interval(TICK);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        let snapshot: TorrentSnapshot = api
            .api_torrent_list_ext(ApiTorrentListOpts { with_stats: true })
            .into();
        let session: SessionStats = api.api_session_stats().into();

        if let Err(e) = TorrentsSnapshotEvent(snapshot).emit(&app) {
            eprintln!("emit torrents-snapshot failed: {e}");
        }
        if let Err(e) = SessionStatsEvent(session).emit(&app) {
            eprintln!("emit session-stats failed: {e}");
        }
    }
}
