//!
//! Configuration loading and generation for `.quality.toml`.
//! This module handles parsing TOML config, applying defaults, and generating new config files.

use serde::{Deserialize, Serialize};
use std::fs;

// ═════════════════════════════════════════
// CONFIG STRUCTS
// ═════════════════════════════════════════

/// Complete configuration for CodeMetrics, loaded from `.quality.toml`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project: Option<ProjectConfig>,
    pub crap: Option<CrapConfig>,
    pub debt: Option<DebtConfig>,
    pub doc: Option<DocConfig>,
    pub complexity: Option<ComplexityConfig>,
    pub taint: Option<TaintConfig>,
    pub duplication: Option<DuplicationConfig>,
    pub risk: Option<RiskConfig>,
    pub coupling: Option<CouplingConfig>,
    pub mutation: Option<MutationConfig>,
    pub security: Option<SecurityConfig>,
    pub secrets: Option<SecretsConfig>,
    pub licenses: Option<LicensesConfig>,
    pub dead_code: Option<DeadCodeConfig>,
    pub type_coverage: Option<TypeCoverageConfig>,
    pub halstead: Option<HalsteadConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub ecosystem: Option<String>,
    pub test_cmd: Option<String>,
    pub coverage_cmd: Option<String>,
    pub lcov_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CrapConfig {
    pub threshold: Option<f64>,
    pub warn_at: Option<f64>,
    pub max_avg: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebtConfig {
    pub max_items: Option<usize>,
    pub max_markers: Option<usize>,
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DocConfig {
    pub min_coverage: Option<f64>,
    pub min_pct: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComplexityConfig {
    pub max_violations: Option<usize>,
    pub threshold: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaintConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DuplicationConfig {
    pub max_duplication: Option<f64>,
    pub max_duplicates: Option<f64>,
    pub min_lines: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RiskConfig {
    pub max_score: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CouplingConfig {
    pub max_coupling: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MutationConfig {
    pub min_score: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityConfig {
    pub max_vulnerabilities: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecretsConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicensesConfig {
    pub deny: Option<Vec<String>>,
    pub allow: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeadCodeConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TypeCoverageConfig {
    pub min_coverage: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HalsteadConfig {
    pub max_bug_estimate: Option<f64>,
}

// ═════════════════════════════════════════
// CONFIG LOADING
// ═════════════════════════════════════════

/// Load thresholds from `.quality.toml` if present, falling back to `defaults`.
/// Uses the toml crate for proper TOML parsing.
pub fn load_config_thresholds(
    config_path: &str,
    defaults: (
        f64,
        f64,
        usize,
        usize,
        f64,
        usize,
        f64,
        usize,
        f64,
        usize,
        usize,
        f64,
        usize,
        usize,
        usize,
        f64,
        usize,
        f64,
        usize,
        usize,
        usize,
        usize,
        usize,
    ),
) -> (
    f64,
    f64,
    usize,
    usize,
    f64,
    usize,
    f64,
    usize,
    f64,
    usize,
    usize,
    f64,
    usize,
    usize,
    usize,
    f64,
    usize,
    f64,
    usize,
    usize,
    usize,
    usize,
    usize,
) {
    let content = match fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => return defaults,
    };

    let config: Config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(_) => return defaults,
    };

    (
        config.crap.and_then(|c| c.max_avg).unwrap_or(defaults.0),
        config
            .doc
            .and_then(|c| c.min_pct.or(c.min_coverage))
            .unwrap_or(defaults.1),
        config
            .debt
            .and_then(|c| c.max_markers.or(c.max_items))
            .unwrap_or(defaults.2),
        config
            .complexity
            .and_then(|c| c.max_violations)
            .unwrap_or(defaults.3),
        config
            .duplication
            .and_then(|c| c.max_duplication.or(c.max_duplicates))
            .unwrap_or(defaults.4),
        config
            .taint
            .and_then(|c| c.max_findings)
            .unwrap_or(defaults.5),
        config.risk.and_then(|c| c.max_score).unwrap_or(defaults.6),
        config
            .coupling
            .and_then(|c| c.max_coupling)
            .unwrap_or(defaults.7),
        0.0, // min_propcov - not in config yet
        0,   // max_fuzz_risk - not in config yet
        0,   // max_linelen - not in config yet
        config
            .halstead
            .and_then(|c| c.max_bug_estimate)
            .unwrap_or(defaults.11),
        config
            .secrets
            .and_then(|c| c.max_findings)
            .unwrap_or(defaults.12),
        config
            .dead_code
            .and_then(|c| c.max_findings)
            .unwrap_or(defaults.13),
        0,   // max_cohesion - not in config yet
        0.0, // min_comment_ratio - not in config yet
        0,   // max_errhandle - not in config yet
        0.0, // min_typecov - not in config yet
        0,   // max_vuln_critical - not in config yet
        0,   // max_vuln_high - not in config yet
        0,   // max_sast - not in config yet
        0,   // max_crypto - not in config yet
        0,   // max_license_violations - not in config yet
    )
}

// ═════════════════════════════════════════
// CONFIG GENERATION
// ═════════════════════════════════════════

/// Generate a `.quality.toml` configuration string for a project profile.
pub fn generate_config(output: &str, profile: &crate::project::ProjectProfile) {
    let config = format!(
        r#"# .quality.toml — CodeMetrics quality thresholds
# Auto-generated for: {ecosystem}
# Used by: codemetrics check . and codemetrics run .
# Run `codemetrics init` at any time to regenerate with updated detection.

[project]
ecosystem = "{ecosystem}"
test_cmd = {test_cmd}
coverage_cmd = {coverage_cmd}
lcov_path = "{lcov_path}"

[crap]
# CRAP = complexity^2 * (1 - coverage)^3 + complexity. Lower is better.
max_avg = {max_crap}

[debt]
max_markers = {max_debt}
types = ["TODO", "FIXME", "HACK", "XXX"]

[doc_coverage]
min_pct = {min_doc}

[complexity]
max_violations = {max_complexity}

[duplication]
max_duplicates = 0
min_lines = 3

[skip]
checks = []
"#,
        ecosystem = profile.ecosystem,
        test_cmd = serde_json::to_string(&profile.test_cmd).unwrap_or_default(),
        coverage_cmd = serde_json::to_string(&profile.coverage_cmd).unwrap_or_default(),
        lcov_path = profile.lcov_path,
        max_crap = profile.max_crap,
        max_debt = profile.max_debt,
        min_doc = profile.min_doc,
        max_complexity = profile.max_complexity_violations,
    );
    fs::write(output, config).expect("Failed to write config");
}
