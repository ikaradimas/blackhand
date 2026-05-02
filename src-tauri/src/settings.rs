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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_disable_limits_and_use_os_dir() {
        let s = AppSettings::default();
        assert!(s.download_dir.is_empty());
        assert_eq!(s.listen_port_min, 0);
        assert_eq!(s.listen_port_max, 0);
        assert!(s.enable_upnp);
        assert!(s.enable_dht);
        assert_eq!(s.upload_limit_kbps, 0);
        assert_eq!(s.download_limit_kbps, 0);
    }

    #[test]
    fn kbps_to_nz_bps_zero_is_unlimited() {
        assert!(kbps_to_nz_bps(0).is_none());
    }

    #[test]
    fn kbps_to_nz_bps_one_kbps_is_1024_bps() {
        // The bug we shipped briefly was treating this as bytes-per-second
        // directly (×125 for "megabits" math). 1 KB/s = 1024 B/s; verify.
        assert_eq!(kbps_to_nz_bps(1).unwrap().get(), 1024);
    }

    #[test]
    fn kbps_to_nz_bps_1000_kbps_is_one_mibps() {
        assert_eq!(kbps_to_nz_bps(1000).unwrap().get(), 1_024_000);
    }

    #[test]
    fn kbps_to_nz_bps_saturates_does_not_panic() {
        // saturating_mul guards against u32 overflow; verify we don't crash.
        let v = kbps_to_nz_bps(u32::MAX).unwrap();
        assert_eq!(v.get(), u32::MAX);
    }

    #[test]
    fn settings_serde_round_trip_preserves_all_fields() {
        let s = AppSettings {
            download_dir: "/tmp/dl".into(),
            listen_port_min: 6881,
            listen_port_max: 6889,
            enable_upnp: false,
            enable_dht: true,
            upload_limit_kbps: 500,
            download_limit_kbps: 2000,
        };
        let json = serde_json::to_string(&s).unwrap();
        let back: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(back.download_dir, s.download_dir);
        assert_eq!(back.listen_port_min, s.listen_port_min);
        assert_eq!(back.listen_port_max, s.listen_port_max);
        assert_eq!(back.enable_upnp, s.enable_upnp);
        assert_eq!(back.enable_dht, s.enable_dht);
        assert_eq!(back.upload_limit_kbps, s.upload_limit_kbps);
        assert_eq!(back.download_limit_kbps, s.download_limit_kbps);
    }
}
