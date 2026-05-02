use std::num::NonZeroU32;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::paths;

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct AppSettings {
    /// Empty string -> resolve to ~/Downloads/BlackHand.
    pub download_dir: String,
    /// 0 (either side) -> use librqbit defaults.
    pub listen_port_min: u16,
    pub listen_port_max: u16,
    pub enable_upnp: bool,
    pub enable_dht: bool,
    /// 0 -> unlimited.
    pub upload_limit_kbps: u32,
    pub download_limit_kbps: u32,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            download_dir: String::new(),
            listen_port_min: 0,
            listen_port_max: 0,
            enable_upnp: true,
            enable_dht: true,
            upload_limit_kbps: 0,
            download_limit_kbps: 0,
        }
    }
}

fn settings_path() -> Result<PathBuf> {
    Ok(paths::data_dir()?.join("settings.json"))
}

pub fn load() -> Result<AppSettings> {
    let path = settings_path()?;
    if !path.exists() {
        return Ok(AppSettings::default());
    }
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {path:?}"))?;
    let s: AppSettings = serde_json::from_str(&content)
        .with_context(|| format!("parsing {path:?}"))?;
    Ok(s)
}

pub fn save(s: &AppSettings) -> Result<()> {
    let path = settings_path()?;
    let json = serde_json::to_string_pretty(s)?;
    std::fs::write(&path, json).with_context(|| format!("writing {path:?}"))?;
    Ok(())
}

/// Resolve the download directory, falling back to the OS default if empty.
pub fn resolve_download_dir(s: &AppSettings) -> Result<PathBuf> {
    if s.download_dir.is_empty() {
        paths::default_downloads_dir()
    } else {
        let p = PathBuf::from(&s.download_dir);
        std::fs::create_dir_all(&p)
            .with_context(|| format!("creating {p:?}"))?;
        Ok(p)
    }
}

pub fn kbps_to_nz_bps(kbps: u32) -> Option<NonZeroU32> {
    NonZeroU32::new(kbps.saturating_mul(1024))
}
