use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::paths;

/// Persisted category data — names + per-torrent assignments keyed by info hash.
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct CategoryData {
    categories: Vec<String>,
    assignments: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Type, Clone, Debug)]
pub struct CategoryInfo {
    pub name: String,
    pub count: u32,
}

pub struct CategoryStore {
    inner: RwLock<CategoryData>,
}

fn store_path() -> Result<PathBuf> {
    Ok(paths::data_dir()?.join("categories.json"))
}

impl CategoryStore {
    pub fn load() -> Self {
        let data = (|| -> Result<CategoryData> {
            let path = store_path()?;
            if !path.exists() {
                return Ok(CategoryData::default());
            }
            let s = std::fs::read_to_string(&path).context("reading categories.json")?;
            Ok(serde_json::from_str(&s).unwrap_or_default())
        })()
        .unwrap_or_default();
        Self {
            inner: RwLock::new(data),
        }
    }

    fn save(data: &CategoryData) -> Result<()> {
        let path = store_path()?;
        let json = serde_json::to_string_pretty(data)?;
        std::fs::write(path, json).context("writing categories.json")?;
        Ok(())
    }

    /// Read the assignment for a torrent (cheap, takes a read lock briefly).
    pub fn category_for(&self, info_hash: &str) -> Option<String> {
        self.inner.read().ok()?.assignments.get(info_hash).cloned()
    }

    /// Tally each known category against the live torrent set, plus add any
    /// orphan categories (no torrents) so the user can still see and pick them.
    pub fn list_with_counts(&self, info_hashes: &[String]) -> Vec<CategoryInfo> {
        let Ok(data) = self.inner.read() else {
            return vec![];
        };
        let mut counts: HashMap<String, u32> = HashMap::new();
        for h in info_hashes {
            if let Some(c) = data.assignments.get(h) {
                *counts.entry(c.clone()).or_default() += 1;
            }
        }
        data.categories
            .iter()
            .map(|c| CategoryInfo {
                name: c.clone(),
                count: *counts.get(c).unwrap_or(&0),
            })
            .collect()
    }

    /// Assign or unassign. Empty/whitespace-only names unassign.
    /// New names are auto-added to the categories list (sorted).
    pub fn set_category(&self, info_hash: String, category: Option<String>) -> Result<()> {
        let mut data = self
            .inner
            .write()
            .map_err(|_| anyhow::anyhow!("category store poisoned"))?;
        match category.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            None => {
                data.assignments.remove(&info_hash);
            }
            Some(name) => {
                if !data.categories.iter().any(|c| c == name) {
                    data.categories.push(name.to_string());
                    data.categories.sort();
                }
                data.assignments.insert(info_hash, name.to_string());
            }
        }
        Self::save(&data)
    }
}
