// ═════════════════════════════════════════
// IGNORE — .codemetricsignore file support
// ═════════════════════════════════════════

use std::path::Path;

/// Load ignore patterns from `.codemetricsignore` in the given directory.
/// Returns a Vec of glob-like patterns (one per line, '#' comments supported).
pub fn load_ignore_patterns(dir: &str) -> Vec<String> {
    let ignore_path = Path::new(dir).join(".codemetricsignore");
    if !ignore_path.exists() {
        return Vec::new();
    }
    match std::fs::read_to_string(&ignore_path) {
        Ok(content) => content
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .map(|l| l.to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// Check if a file path matches any ignore pattern.
/// Supports simple glob patterns: `*.ext`, `dir/`, `**/dir/`, exact matches.
pub fn is_ignored(file_path: &str, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }
    let path = Path::new(file_path);
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    for pattern in patterns {
        let pat = pattern.trim();
        if pat.is_empty() {
            continue;
        }

        // **/ prefix — match any directory depth
        if let Some(suffix) = pat.strip_prefix("**/") {
            // Check if any path component or the file itself matches
            for component in path.components() {
                let comp = component.as_os_str().to_string_lossy().to_string();
                if matches_glob(&comp, suffix) {
                    return true;
                }
            }
            // Also check if the path contains the suffix as a substring after a /
            if let Some(dir_suffix) = suffix.strip_suffix('/') {
                for component in path.components() {
                    if component.as_os_str().to_string_lossy() == dir_suffix {
                        return true;
                    }
                }
            } else if file_path.contains(suffix) {
                return true;
            }
            continue;
        }

        // Directory pattern (ends with /)
        if let Some(dir_name) = pat.strip_suffix('/') {
            // Check if any single component matches (e.g., "generated/" matches any "generated" dir)
            for component in path.components() {
                if component.as_os_str().to_string_lossy() == dir_name {
                    return true;
                }
            }
            // Check if the path contains the full directory sequence (e.g., "src/generated/")
            if dir_name.contains('/') && file_path.contains(dir_name) {
                return true;
            }
            continue;
        }

        // *.ext pattern
        if let Some(ext) = pat.strip_prefix("*.") {
            if file_name.ends_with(ext) {
                return true;
            }
            continue;
        }

        // Exact filename match
        if file_name == pat {
            return true;
        }

        // Check if any path component matches exactly
        for component in path.components() {
            if component.as_os_str().to_string_lossy() == pat {
                return true;
            }
        }
    }

    false
}

/// Simple glob matching — supports `*` (any chars) and `?` (single char).
fn matches_glob(s: &str, pattern: &str) -> bool {
    // Fast path: exact match
    if s == pattern {
        return true;
    }
    // Fast path: no wildcards
    if !pattern.contains('*') && !pattern.contains('?') {
        return false;
    }
    // Convert glob to a simple check
    // For our use cases, we mainly need `*.ext` and exact matches
    if pattern == "*" {
        return true;
    }
    if pattern.starts_with("*.") {
        let ext = pattern.strip_prefix("*.").unwrap();
        return s.ends_with(ext);
    }
    if let Some(prefix) = pattern.strip_suffix(".*") {
        return s.starts_with(prefix);
    }
    // Fallback: check if the pattern appears anywhere
    s.contains(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_patterns() {
        assert!(!is_ignored("src/main.rs", &[]));
    }

    #[test]
    fn test_exact_filename() {
        let patterns = vec!["generated.rs".to_string()];
        assert!(is_ignored("src/generated.rs", &patterns));
        assert!(!is_ignored("src/main.rs", &patterns));
    }

    #[test]
    fn test_ext_pattern() {
        let patterns = vec!["*.rs".to_string()];
        assert!(is_ignored("src/main.rs", &patterns));
        assert!(!is_ignored("src/main.c", &patterns));
    }

    #[test]
    fn test_dir_pattern() {
        let patterns = vec!["target/".to_string()];
        assert!(is_ignored("project/target/debug/foo", &patterns));
        assert!(!is_ignored("src/main.rs", &patterns));
    }

    #[test]
    fn test_double_star() {
        let patterns = vec!["**/target/".to_string()];
        assert!(is_ignored("project/target/debug/foo", &patterns));
        assert!(!is_ignored("src/main.rs", &patterns));
    }
}
