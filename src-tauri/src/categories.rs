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

impl CategoryData {
    fn category_for(&self, info_hash: &str) -> Option<String> {
        self.assignments.get(info_hash).cloned()
    }

    fn list_with_counts(&self, info_hashes: &[String]) -> Vec<CategoryInfo> {
        let mut counts: HashMap<&str, u32> = HashMap::new();
        for h in info_hashes {
            if let Some(c) = self.assignments.get(h) {
                *counts.entry(c.as_str()).or_default() += 1;
            }
        }
        self.categories
            .iter()
            .map(|c| CategoryInfo {
                name: c.clone(),
                count: counts.get(c.as_str()).copied().unwrap_or(0),
            })
            .collect()
    }

    fn set_category(&mut self, info_hash: String, category: Option<String>) {
        match category.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
            None => {
                self.assignments.remove(&info_hash);
            }
            Some(name) => {
                if !self.categories.iter().any(|c| c == name) {
                    self.categories.push(name.to_string());
                    self.categories.sort();
                }
                self.assignments.insert(info_hash, name.to_string());
            }
        }
    }
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
        self.inner.read().ok()?.category_for(info_hash)
    }

    /// Tally each known category against the live torrent set, plus add any
    /// orphan categories (no torrents) so the user can still see and pick them.
    pub fn list_with_counts(&self, info_hashes: &[String]) -> Vec<CategoryInfo> {
        let Ok(data) = self.inner.read() else {
            return vec![];
        };
        data.list_with_counts(info_hashes)
    }

    /// Assign or unassign. Empty/whitespace-only names unassign.
    /// New names are auto-added to the categories list (sorted).
    pub fn set_category(&self, info_hash: String, category: Option<String>) -> Result<()> {
        let mut data = self
            .inner
            .write()
            .map_err(|_| anyhow::anyhow!("category store poisoned"))?;
        data.set_category(info_hash, category);
        Self::save(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn set_category_assigns_new_name_and_sorts_alphabetically() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("zeta")));
        d.set_category(h("bbb"), Some(h("alpha")));
        d.set_category(h("ccc"), Some(h("mu")));
        assert_eq!(d.categories, vec!["alpha", "mu", "zeta"]);
        assert_eq!(d.category_for("aaa"), Some(h("zeta")));
        assert_eq!(d.category_for("bbb"), Some(h("alpha")));
    }

    #[test]
    fn set_category_dedupes_known_names() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("movies")));
        d.set_category(h("bbb"), Some(h("movies")));
        d.set_category(h("ccc"), Some(h("movies")));
        assert_eq!(d.categories, vec!["movies"]);
    }

    #[test]
    fn set_category_with_none_unassigns_but_keeps_category_in_list() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("movies")));
        d.set_category(h("aaa"), None);
        assert_eq!(d.category_for("aaa"), None);
        // The category remains available for picking again.
        assert_eq!(d.categories, vec!["movies"]);
    }

    #[test]
    fn set_category_with_empty_string_unassigns() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("movies")));
        d.set_category(h("aaa"), Some(h("   ")));
        assert_eq!(d.category_for("aaa"), None);
    }

    #[test]
    fn set_category_reassigning_overwrites_previous() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("movies")));
        d.set_category(h("aaa"), Some(h("books")));
        assert_eq!(d.category_for("aaa"), Some(h("books")));
        // Both names remain in the list.
        assert_eq!(d.categories, vec!["books", "movies"]);
    }

    #[test]
    fn category_for_returns_none_when_unassigned() {
        let d = CategoryData::default();
        assert_eq!(d.category_for("nope"), None);
    }

    #[test]
    fn list_with_counts_tallies_only_present_torrents() {
        let mut d = CategoryData::default();
        d.set_category(h("a"), Some(h("movies")));
        d.set_category(h("b"), Some(h("movies")));
        d.set_category(h("c"), Some(h("books")));
        // d still knows about a torrent that's no longer in the live set.
        d.set_category(h("ghost"), Some(h("movies")));

        let live = vec![h("a"), h("b"), h("c")];
        let counts = d.list_with_counts(&live);

        // Sorted alphabetically (matches `set_category` insertion ordering).
        let by_name: std::collections::HashMap<_, _> =
            counts.into_iter().map(|c| (c.name, c.count)).collect();
        assert_eq!(by_name.get("movies").copied(), Some(2));
        assert_eq!(by_name.get("books").copied(), Some(1));
    }

    #[test]
    fn list_with_counts_includes_orphan_categories_with_zero_count() {
        let mut d = CategoryData::default();
        d.set_category(h("a"), Some(h("movies")));
        d.set_category(h("a"), None);
        // "movies" is now an orphan — no assignments, but still in the list.
        let counts = d.list_with_counts(&[]);
        assert_eq!(counts.len(), 1);
        assert_eq!(counts[0].name, "movies");
        assert_eq!(counts[0].count, 0);
    }

    #[test]
    fn category_data_serde_round_trip() {
        let mut d = CategoryData::default();
        d.set_category(h("aaa"), Some(h("movies")));
        d.set_category(h("bbb"), Some(h("books")));

        let json = serde_json::to_string(&d).unwrap();
        let back: CategoryData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.categories, d.categories);
        assert_eq!(back.assignments, d.assignments);
    }

    #[test]
    fn category_data_default_is_empty() {
        let d = CategoryData::default();
        assert!(d.categories.is_empty());
        assert!(d.assignments.is_empty());
        assert!(d.list_with_counts(&[]).is_empty());
    }
}
