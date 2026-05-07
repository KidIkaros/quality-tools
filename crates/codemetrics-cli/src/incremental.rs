// ═══════════════════════════════════════════
// INCREMENTAL ANALYSIS — only check changed files
// ═══════════════════════════════════════════

use colored::Colorize;
use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;

/// Cache entry for a single file
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct FileCacheEntry {
    path: String,
    last_modified: u64, // Unix timestamp in seconds
    size: u64,
}

/// The on-disk cache format
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct IncrementalCache {
    version: u32,
    files: HashMap<String, FileCacheEntry>,
}

impl IncrementalCache {
    const CURRENT_VERSION: u32 = 1;
    const CACHE_FILE: &'static str = ".codemetrics-cache.json";

    /// Load cache from disk, or create empty if not present/invalid
    fn load() -> Self {
        let path = Path::new(Self::CACHE_FILE);
        if !path.exists() {
            return Self::empty();
        }
        match std::fs::read_to_string(path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Self::empty()),
            Err(_) => Self::empty(),
        }
    }

    /// Save cache to disk
    fn save(&self) -> bool {
        match serde_json::to_string_pretty(self) {
            Ok(json) => std::fs::write(Self::CACHE_FILE, json).is_ok(),
            Err(_) => false,
        }
    }

    fn empty() -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            files: HashMap::new(),
        }
    }

    /// Check if a file has changed since last scan
    fn has_changed(&self, path: &Path) -> bool {
        let key = path.to_string_lossy().to_string();
        match self.files.get(&key) {
            Some(entry) => {
                let Ok(metadata) = path.metadata() else {
                    return true;
                };
                let Ok(modified) = metadata.modified() else {
                    return true;
                };
                let now_ts = modified
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                // Changed if timestamp differs or size differs
                entry.last_modified != now_ts || entry.size != metadata.len()
            }
            None => true, // Not in cache = new file = changed
        }
    }

    /// Update cache entry for a file
    fn update(&mut self, path: &Path) {
        let key = path.to_string_lossy().to_string();
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified) = metadata.modified() {
                let ts = modified
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                self.files.insert(
                    key,
                    FileCacheEntry {
                        path: path.to_string_lossy().to_string(),
                        last_modified: ts,
                        size: metadata.len(),
                    },
                );
            }
        }
    }

    /// Remove entries for files that no longer exist
    fn prune_missing(&mut self) {
        self.files
            .retain(|_, entry| Path::new(&entry.path).exists());
    }
}

/// Filter a list of file paths to only those that have changed since last scan.
/// Returns (changed_files, total_files, skipped_count).
pub fn filter_changed_files(files: Vec<String>) -> (Vec<String>, usize, usize) {
    let cache = IncrementalCache::load();
    let total = files.len();
    let changed: Vec<String> = files
        .into_iter()
        .filter(|f| cache.has_changed(Path::new(f)))
        .collect();
    let skipped = total - changed.len();
    (changed, total, skipped)
}

/// Update the cache with the given file paths after a successful scan.
pub fn update_cache(files: &[String]) {
    let mut cache = IncrementalCache::load();
    cache.prune_missing();
    for f in files {
        cache.update(Path::new(f));
    }
    cache.save();
}

/// Get a summary string for incremental mode
pub fn incremental_summary(changed: usize, total: usize, skipped: usize) -> String {
    if skipped == 0 {
        format!(
            "  {} Analyzing all {} files (no cache found)",
            "ℹ".cyan(),
            total
        )
    } else {
        format!(
            "  {} Incremental: {}/{} files changed ({} skipped)",
            "⚡".yellow(),
            changed,
            total,
            skipped
        )
    }
}

/// Check if incremental mode should be used (cache exists and --incremental flag is set)
pub fn is_incremental_enabled() -> bool {
    Path::new(IncrementalCache::CACHE_FILE).exists()
}
