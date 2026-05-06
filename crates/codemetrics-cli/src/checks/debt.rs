//! Debt check module.
//! Scans for technical debt markers (TODO, FIXME, HACK, XXX, BUG).

use serde_json::Value;
use std::time::Instant;

use crate::find_source_files;

/// Run technical debt check
pub fn check_debt(path: &str, recursive: bool, max_debt: usize) -> super::CheckResult {
    let extensions = [
        "rs", "py", "js", "ts", "go", "c", "cpp", "h", "java", "cs", "php", "rb", "swift",
    ];
    let files = find_source_files(path, recursive, &extensions);

    let markers = ["TODO", "FIXME", "HACK", "XXX", "BUG"];
    let mut count = 0;
    let mut items = Vec::new();

    for file in &files {
        if let Ok(source) = std::fs::read_to_string(file) {
            for (line_num, line) in source.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("//")
                    || trimmed.starts_with("/*")
                    || trimmed.starts_with('*')
                {
                    for marker in &markers {
                        if trimmed.contains(marker) {
                            count += 1;
                            items.push(Value::json!({
                                "file": file, "line": line_num + 1, "type": marker
                            }));
                        }
                    }
                }
            }
        }
    }

    let (severity, rule_id, help) = if count <= max_debt {
        (
            "info".to_string(),
            "debt-pass".to_string(),
            "Technical debt is within acceptable limits.".to_string(),
        )
    } else if count > max_debt * 2 {
        (
            "error".to_string(),
            "debt-high".to_string(),
            "Excessive technical debt. Address TODO/FIXME/HACK markers to improve code maintainability.".to_string(),
        )
    } else {
        (
            "warning".to_string(),
            "debt-moderate".to_string(),
            "Moderate technical debt. Consider addressing high-priority markers first.".to_string(),
        )
    };

    super::CheckResult {
        name: "debt".to_string(),
        passed: count <= max_debt,
        score: Some(count as f64),
        threshold: Some(max_debt as f64),
        message: if count <= max_debt {
            format!("{} debt markers <= {}", count, max_debt)
        } else {
            format!("{} debt markers > {}", count, max_debt)
        },
        details: Value::json!({
            "total_markers": count,
            "items": items.iter().take(20).collect::<Vec<_>>(),
        }),
        severity: Some(severity),
        help: Some(help),
        rule_id: Some(rule_id),
    }
}
