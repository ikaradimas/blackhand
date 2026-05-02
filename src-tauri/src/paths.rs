use std::path::PathBuf;

use anyhow::{Context, Result};

const APP_DIRNAME: &str = "blackhand";

pub fn data_dir() -> Result<PathBuf> {
    let base = dirs::data_dir().context("could not determine OS data dir")?;
    let p = base.join(APP_DIRNAME);
    std::fs::create_dir_all(&p).with_context(|| format!("creating {p:?}"))?;
    Ok(p)
}

pub fn default_downloads_dir() -> Result<PathBuf> {
    let base = dirs::download_dir().context("could not determine OS downloads dir")?;
    let p = base.join("BlackHand");
    std::fs::create_dir_all(&p).with_context(|| format!("creating {p:?}"))?;
    Ok(p)
}
