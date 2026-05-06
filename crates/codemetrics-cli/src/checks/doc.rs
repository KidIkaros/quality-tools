//! Documentation coverage check module.
//! Checks public API documentation coverage for Rust, Python, JS/TS.

use serde_json::Value;
use std::time::Instant;

use crate::find_source_files;
use ast_parse_ts::{parse_doc_coverage_file, Language};

/// Run doc coverage check
pub fn check_doc_coverage(path: &str, recursive: bool, min_doc: f64) -> super::CheckResult {
    let mut total = 0usize;
    let mut documented = 0usize;
    let mut langs_seen: std::collections::HashSet<String> = Default::default();

    // Rust files via syn (high-fidelity)
    let rust_files = crate::find_source_files(path, recursive, &["rs"]);
    if !rust_files.is_empty() {
        langs_seen.insert("Rust".to_string());
    }
    let mut counter = DocCounter {
        total: 0,
        documented: 0,
    };
    for file in &rust_files {
        if let Ok(source) = std::fs::read_to_string(file) {
            if let Ok(ast) = syn::parse_file(&source) {
                counter.visit_file(&ast);
            }
        }
    }
    total += counter.total;
    documented += counter.documented;

    // Non-Rust files via tree-sitter
    let all_exts = ["py", "pyi", "js", "mjs", "ts", "tsx", "go"];
    let other_files: Vec<String> = crate::find_source_files(path, recursive, &all_exts)
        .into_iter()
        .filter(|f| !f.ends_with(".rs"))
        .collect();

    for file in &other_files {
        let lang = Language::from_extension(file);
        langs_seen.insert(lang.to_string());
        let stats = parse_doc_coverage_file(file);
        total += stats.total_public;
        documented += stats.documented;
    }

    let pct = if total > 0 {
        documented as f64 / total as f64 * 100.0
    } else {
        100.0
    };

    let mut langs_vec: Vec<String> = langs_seen.into_iter().collect();
    langs_vec.sort();

    let (severity, rule_id, help) = if pct >= min_doc {
        (
            "info".to_string(),
            "doccov-pass".to_string(),
            "Documentation coverage is within acceptable limits.".to_string(),
        )
    } else if pct < min_doc * 0.5 {
        (
            "error".to_string(),
            "doccov-low".to_string(),
            "Very low documentation coverage. Add documentation to public APIs to improve maintainability.".to_string(),
        )
    } else {
        (
            "warning".to_string(),
            "doccov-moderate".to_string(),
            "Moderate documentation coverage. Add documentation to remaining public APIs."
                .to_string(),
        )
    };

    super::CheckResult {
        name: "doc_coverage".to_string(),
        passed: pct >= min_doc,
        score: Some(pct),
        threshold: Some(min_doc),
        message: if pct >= min_doc {
            format!(
                "Doc coverage {:.0}% >= {:.0}% (langs: {})",
                pct,
                min_doc,
                langs_vec.join(", ")
            )
        } else {
            format!(
                "Doc coverage {:.0}% < {:.0}% (langs: {})",
                pct,
                min_doc,
                langs_vec.join(", ")
            )
        },
        details: Value::json!({
            "total_public": total,
            "documented": documented,
            "coverage_pct": pct,
            "languages": langs_vec,
        }),
        severity: Some(severity),
        help: Some(help),
        rule_id: Some(rule_id),
    }
}

struct DocCounter {
    total: usize,
    documented: usize,
}

impl<'a> syn::visit::Visit<'a> for DocCounter {
    fn visit_item_fn(&mut self, node: &'a syn::ItemFn) {
        if matches!(node.vis, syn::Visibility::Public(_)) {
            self.total += 1;
            if node.attrs.iter().any(|a| a.path().is_ident("doc")) {
                self.documented += 1;
            }
        }
    }
    fn visit_item_struct(&mut self, node: &'a syn::ItemStruct) {
        if matches!(node.vis, syn::Visibility::Public(_)) {
            self.total += 1;
            if node.attrs.iter().any(|a| a.path().is_ident("doc")) {
                self.documented += 1;
            }
        }
    }
    fn visit_item_enum(&mut self, node: &'a syn::ItemEnum) {
        if matches!(node.vis, syn::Visibility::Public(_)) {
            self.total += 1;
            if node.attrs.iter().any(|a| a.path().is_ident("doc")) {
                self.documented += 1;
            }
        }
    }
    fn visit_item_trait(&mut self, node: &'a syn::ItemTrait) {
        if matches!(node.vis, syn::Visibility::Public(_)) {
            self.total += 1;
            if node.attrs.iter().any(|a| a.path().is_ident("doc")) {
                self.documented += 1;
            }
        }
    }
    fn visit_impl_item_fn(&mut self, node: &'a syn::ImplItemFn) {
        if matches!(node.vis, syn::Visibility::Public(_)) {
            self.total += 1;
            if node.attrs.iter().any(|a| a.path().is_ident("doc")) {
                self.documented += 1;
            }
        }
    }
}
