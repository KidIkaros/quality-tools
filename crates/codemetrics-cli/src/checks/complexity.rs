//! Complexity check module.
//! Checks cyclomatic complexity of functions.

use serde_json::Value;
use std::collections::HashSet;

use crate::find_source_files;
use ast_parse_ts::{parse_complexity_file, Language};

/// Run complexity check
pub fn check_complexity(
    path: &str,
    recursive: bool,
    min_complexity: u32,
    max_violations: usize,
) -> super::CheckResult {
    let all_exts = [
        "rs", "py", "pyi", "js", "mjs", "cjs", "ts", "tsx", "mts", "go", "c", "h", "cpp", "cc",
        "cxx", "hpp", "cs", "java", "php", "rb", "swift",
    ];
    let files = crate::find_source_files(path, recursive, &all_exts);

    let mut total = 0usize;
    let mut complex_funcs: Vec<Value> = Vec::new();
    let mut langs_seen: HashSet<String> = Default::default();

    for file in &files {
        let lang = Language::from_extension(file);
        langs_seen.insert(lang.to_string());
        let funcs = parse_complexity_file(file);
        for func in funcs {
            total += 1;
            if func.complexity >= min_complexity {
                complex_funcs.push(Value::json!({
                    "name": func.name,
                    "file": func.file,
                    "line": func.line,
                    "complexity": func.complexity,
                    "language": func.language.to_string(),
                }));
            }
        }
    }

    let mut langs_vec: Vec<String> = langs_seen.into_iter().collect();
    langs_vec.sort();

    let passed = complex_funcs.len() <= max_violations;

    let (severity, rule_id, help) = if passed && complex_funcs.is_empty() {
        (
            "info".to_string(),
            "complexity-pass".to_string(),
            "No functions with excessive complexity.".to_string(),
        )
    } else if passed {
        (
            "info".to_string(),
            "complexity-pass".to_string(),
            format!(
                "Complexity violations within allowed limit (<= {})",
                max_violations
            ),
        )
    } else if complex_funcs.len() > 10 {
        (
            "error".to_string(),
            "complexity-high".to_string(),
            "Multiple functions with high complexity. Refactor to reduce decision points."
                .to_string(),
        )
    } else {
        (
            "warning".to_string(),
            "complexity-moderate".to_string(),
            "Some functions with high complexity. Consider refactoring.".to_string(),
        )
    };

    super::CheckResult {
        name: "complexity".to_string(),
        passed,
        score: Some(complex_funcs.len() as f64),
        threshold: Some(max_violations as f64),
        message: if passed && complex_funcs.is_empty() {
            format!(
                "No functions above complexity threshold (languages: {})",
                langs_vec.join(", ")
            )
        } else if passed {
            format!(
                "{} complex functions <= allowed {} (languages: {})",
                complex_funcs.len(),
                max_violations,
                langs_vec.join(", ")
            )
        } else {
            format!(
                "{} functions with complexity >= {} > allowed {} (languages: {})",
                complex_funcs.len(),
                min_complexity,
                max_violations,
                langs_vec.join(", ")
            )
        },
        details: Value::json!({
            "total_functions": total,
            "complex_count": complex_funcs.len(),
            "max_violations_allowed": max_violations,
            "languages": langs_vec,
            "functions": complex_funcs.iter().take(10).collect::<Vec<_>>(),
        }),
        severity: Some(severity),
        help: Some(help),
        rule_id: Some(rule_id),
    }
}
