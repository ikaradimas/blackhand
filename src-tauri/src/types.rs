use serde::{Deserialize, Serialize};
use specta::Type;

use librqbit::api::{TorrentDetailsResponse, TorrentListResponse};
use librqbit::session_stats::snapshot::SessionStatsSnapshot;
use librqbit::{TorrentStats, TorrentStatsState};

use crate::categories::CategoryStore;

#[derive(Serialize, Deserialize, Type, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TorrentState {
    Initializing,
    Live,
    Paused,
    Error,
}

impl From<TorrentStatsState> for TorrentState {
    fn from(s: TorrentStatsState) -> Self {
        match s {
            TorrentStatsState::Initializing => Self::Initializing,
            TorrentStatsState::Live => Self::Live,
            TorrentStatsState::Paused => Self::Paused,
            TorrentStatsState::Error => Self::Error,
        }
    }
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct TorrentSummary {
    pub id: u64,
    pub info_hash: String,
    pub name: Option<String>,
    pub output_folder: String,
    pub state: TorrentState,
    pub finished: bool,
    pub progress_bytes: u64,
    pub uploaded_bytes: u64,
    pub total_bytes: u64,
    /// 0.0 .. 100.0
    pub progress_pct: f32,
    pub down_bps: u64,
    pub up_bps: u64,
    pub peers_live: u32,
    pub eta_secs: Option<u64>,
    pub error: Option<String>,
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct TorrentSnapshot {
    pub torrents: Vec<TorrentSummary>,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct SessionStats {
    pub down_bps: u64,
    pub up_bps: u64,
    pub fetched_bytes: u64,
    pub uploaded_bytes: u64,
    pub peers_live: u32,
    pub uptime_secs: u64,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct AddTorrentResult {
    pub id: Option<u64>,
    pub info_hash: String,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct TorrentFile {
    /// Position in the torrent's file table — the value used by `set_only_files`.
    pub idx: u32,
    pub name: String,
    pub components: Vec<String>,
    pub length: u64,
    pub included: bool,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct TorrentDetail {
    pub id: u64,
    pub info_hash: String,
    pub name: Option<String>,
    pub output_folder: String,
    pub state: TorrentState,
    pub finished: bool,
    pub progress_bytes: u64,
    pub uploaded_bytes: u64,
    pub total_bytes: u64,
    pub progress_pct: f32,
    pub error: Option<String>,
    pub files: Vec<TorrentFile>,
}

impl TorrentDetail {
    pub fn from_parts(d: TorrentDetailsResponse, stats: TorrentStats) -> Self {
        let files = d
            .files
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(idx, f)| TorrentFile {
                idx: idx as u32,
                name: f.name,
                components: f.components,
                length: f.length,
                included: f.included,
            })
            .collect();
        let progress_pct = if stats.total_bytes == 0 {
            0.0
        } else {
            (stats.progress_bytes as f32 / stats.total_bytes as f32) * 100.0
        };
        Self {
            id: d.id.map(|x| x as u64).unwrap_or(0),
            info_hash: d.info_hash,
            name: d.name,
            output_folder: d.output_folder,
            state: stats.state.into(),
            finished: stats.finished,
            progress_bytes: stats.progress_bytes,
            uploaded_bytes: stats.uploaded_bytes,
            total_bytes: stats.total_bytes,
            progress_pct,
            error: stats.error.clone(),
            files,
        }
    }
}

// librqbit's `Speed.mbps` is misleadingly named — its Display impl prints
// `"{:.2} MiB/s"`, so it's actually mebibytes per second.
const MIB_TO_BYTES: f64 = 1024.0 * 1024.0;

fn mibps_to_bps(mibps: f64) -> u64 {
    if !mibps.is_finite() || mibps < 0.0 {
        0
    } else {
        (mibps * MIB_TO_BYTES).round() as u64
    }
}

impl From<SessionStatsSnapshot> for SessionStats {
    fn from(s: SessionStatsSnapshot) -> Self {
        Self {
            down_bps: mibps_to_bps(s.download_speed.mbps),
            up_bps: mibps_to_bps(s.upload_speed.mbps),
            fetched_bytes: s.fetched_bytes,
            uploaded_bytes: s.uploaded_bytes,
            peers_live: s.peers.live as u32,
            uptime_secs: s.uptime_seconds,
        }
    }
}

impl TorrentSnapshot {
    pub fn from_response(resp: TorrentListResponse, store: Option<&CategoryStore>) -> Self {
        let torrents = resp
            .torrents
            .into_iter()
            .filter_map(|t| {
                let id = t.id? as u64;
                let s = t.stats.as_ref();
                let total_bytes = s.map(|s| s.total_bytes).unwrap_or(0);
                let progress_bytes = s.map(|s| s.progress_bytes).unwrap_or(0);
                let progress_pct = if total_bytes == 0 {
                    0.0
                } else {
                    (progress_bytes as f32 / total_bytes as f32) * 100.0
                };

                let live = s.and_then(|s| s.live.as_ref());
                let down_bps = live.map(|l| mibps_to_bps(l.download_speed.mbps)).unwrap_or(0);
                let up_bps = live.map(|l| mibps_to_bps(l.upload_speed.mbps)).unwrap_or(0);
                let peers_live = live
                    .map(|l| l.snapshot.peer_stats.live as u32)
                    .unwrap_or(0);
                // DurationWithHumanReadable's inner Duration is private; pull `secs` via serde.
                let eta_secs = live.and_then(|l| {
                    let d = l.time_remaining.as_ref()?;
                    serde_json::to_value(d)
                        .ok()?
                        .get("duration")?
                        .get("secs")?
                        .as_u64()
                });

                let category = store.and_then(|s| s.category_for(&t.info_hash));

                Some(TorrentSummary {
                    id,
                    info_hash: t.info_hash,
                    name: t.name,
                    output_folder: t.output_folder,
                    state: s
                        .map(|s| s.state.into())
                        .unwrap_or(TorrentState::Initializing),
                    finished: s.map(|s| s.finished).unwrap_or(false),
                    progress_bytes,
                    uploaded_bytes: s.map(|s| s.uploaded_bytes).unwrap_or(0),
                    total_bytes,
                    progress_pct,
                    down_bps,
                    up_bps,
                    peers_live,
                    eta_secs,
                    error: s.and_then(|s| s.error.clone()),
                    category,
                })
            })
            .collect();
        Self { torrents }
    }
}

impl From<TorrentListResponse> for TorrentSnapshot {
    fn from(resp: TorrentListResponse) -> Self {
        Self::from_response(resp, None)
    }
}
