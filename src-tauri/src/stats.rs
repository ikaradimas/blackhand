use std::sync::Arc;
use std::time::Duration;

use librqbit::api::ApiTorrentListOpts;
use librqbit::Api;
use tauri::{AppHandle, Emitter};

const TICK: Duration = Duration::from_millis(500);

pub async fn run(app: AppHandle, api: Arc<Api>) {
    let mut interval = tokio::time::interval(TICK);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        let snapshot = api.api_torrent_list_ext(ApiTorrentListOpts { with_stats: true });
        let session = api.api_session_stats();

        if let Err(e) = app.emit("torrents:snapshot", &snapshot) {
            eprintln!("emit torrents:snapshot failed: {e}");
        }
        if let Err(e) = app.emit("session:stats", &session) {
            eprintln!("emit session:stats failed: {e}");
        }
    }
}
