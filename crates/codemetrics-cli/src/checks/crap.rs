//! Crap check module.
//! Checks CRAP (Change Risk Anti-Patterns) scores for functions.

use serde_json::Value;
use std::time::Instant;

use crate::config::Config;
use crate::project::ProjectProfile;
use codemetrics_common::{crap_score, parse_lcov, CoverageRecord};

/// Result type for check functions
pub struct CheckResult {
    pub name: String,
    pub passed: bool,
    pub score: Option<f64>,
    pub threshold: Option<f64>,
    pub message: String,
    pub details: Value,
    pub severity: Option<String>,
    pub help: Option<String>,
    pub rule_id: Option<String>,
}

/// Scan all source functions and calculate CRAP scores
fn scan_source_functions<T, F>(path: &str, recursive: bool, mut predicate: F) -> (usize, Vec<T>)
where
    F: FnMut(ast_parse_ts::FunctionInfo) -> Option<T>,
{
    let extensions = [
        "rs", "py", "js", "ts", "go", "java", "c", "cpp", "cs", "php", "rb", "swift",
    ];
    let files = crate::find_source_files(path, recursive, &extensions);
    let mut total = 0;
    let mut results = Vec::new();

    for file in files {
        let functions = ast_parse_ts::parse_complexity_file(&file);
        total += functions.len();
        for func in &functions {
            if let Some(item) = predicate(func) {
                results.push(item);
            }
        }
    }
    (total, results)
}

/// Get coverage percentage for a function
fn function_coverage(coverage_records: &[CoverageRecord], func_name: &str) -> f64 {
    coverage_records
        .iter()
        .find(|r| r.function == func_name)
        .map_or(0.0, |r| if r.hits > 0 { 1.0 } else { 0.0 })
}

/// Run CRAP check
pub fn check_crap(
    path: &str,
    recursive: bool,
    coverage_path: &Option<String>,
    max_crap: f64,
) -> CheckResult {
    let coverage_data: Option<Vec<CoverageRecord>> = coverage_path
        .as_ref()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .map(|c| parse_lcov(&c));

    let (total, functions) = scan_source_functions(path, recursive, |func| {
        let cov_pct = if let Some(ref cov_data) = coverage_data {
            function_coverage(cov_data, &func.name)
        } else {
            0.0
        };
        let score = crap_score(func.complexity, cov_pct);
        Some((func.name.clone(), func.complexity, cov_pct, score))
    });

    let avg_crap = if total > 0 {
        functions.iter().map(|f| f.3).sum::<f64>() / total as f64
    } else {
        0.0
    };

    let crappy: Vec<_> = functions.iter().filter(|f| f.3 > 30.0).collect();

    let (severity, rule_id, help) = if avg_crap <= max_crap {
        (
            "info".to_string(),
            "crap-pass".to_string(),
            "CRAP score is within acceptable limits.".to_string(),
        )
    } else if avg_crap > max_crap * 1.5 {
        (
            "error".to_string(),
            "crap-error".to_string(),
            "Reduce function complexity or increase test coverage to lower CRAP score. Aim for CRAP < 30 per function.".to_string(),
        )
    } else {
        (
            "warning".to_string(),
            "crap-warning".to_string(),
            "CRAP score is approaching threshold. Consider refactoring complex functions or adding tests.".to_string(),
        )
    };

    CheckResult {
        name: "crap".to_string(),
        passed: avg_crap <= max_crap,
        score: Some(avg_crap),
        threshold: Some(max_crap),
        message: if avg_crap <= max_crap {
            format!("Average CRAP {:.1} <= {:.0}", avg_crap, max_crap)
        } else {
            format!(
                "Average CRAP {:.1} > {:.0} ({} functions above 30)",
                avg_crap,
                max_crap,
                crappy.len()
            )
        },
        details: serde_json::json!({
            "total_functions": total,
            "avg_crap": avg_crap,
            "crappy_count": crappy.len(),
            "excellent_count": functions.iter().filter(|f| f.3 <= 10.0).count(),
            "top_offenders": crappy.iter().take(5).map(|f| {
                serde_json::json!({
                    "name": f.0, "complexity": f.1, "coverage": f.2, "crap": f.3
                })
            }).collect::<Vec<_>>(),
        }),
        severity: Some(severity),
        help: Some(help),
        rule_id: Some(rule_id),
    }
}
