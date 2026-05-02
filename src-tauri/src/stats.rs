use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

use librqbit::api::ApiTorrentListOpts;
use librqbit::Api;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
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

    // Tracks which torrents we've already announced as finished, so a single
    // completion fires a single notification (not one per stats tick).
    let mut announced_done = HashSet::<u64>::new();

    loop {
        interval.tick().await;

        let snapshot: TorrentSnapshot = api
            .api_torrent_list_ext(ApiTorrentListOpts { with_stats: true })
            .into();
        let session: SessionStats = api.api_session_stats().into();

        // Detect finished-transitions and notify once per torrent.
        for t in &snapshot.torrents {
            if t.finished && !announced_done.contains(&t.id) {
                announced_done.insert(t.id);
                let title = "BlackHand — torrent finished";
                let body = t.name.clone().unwrap_or_else(|| t.info_hash.clone());
                if let Err(e) = app
                    .notification()
                    .builder()
                    .title(title)
                    .body(body)
                    .show()
                {
                    eprintln!("notification.show failed: {e}");
                }
            }
        }

        // Drop ids that are no longer in the list (forget/delete) so a re-add
        // can notify again.
        let current_ids: HashSet<u64> = snapshot.torrents.iter().map(|t| t.id).collect();
        announced_done.retain(|id| current_ids.contains(id));

        if let Err(e) = TorrentsSnapshotEvent(snapshot).emit(&app) {
            eprintln!("emit torrents-snapshot failed: {e}");
        }
        if let Err(e) = SessionStatsEvent(session).emit(&app) {
            eprintln!("emit session-stats failed: {e}");
        }
    }
}
