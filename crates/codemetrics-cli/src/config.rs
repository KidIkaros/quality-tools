// ═══════════════════════════════════════════
// CONFIG — .quality.toml parsing
// ═══════════════════════════════════════════

use serde::Deserialize;
use colored::Colorize;

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub ecosystem: Option<String>,
    pub test_cmd: Option<String>,
    pub coverage_cmd: Option<String>,
    pub lcov_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CrapConfig {
    pub threshold: Option<f64>,
    pub warn_at: Option<f64>,
    pub max_avg: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct DebtConfig {
    pub max_items: Option<usize>,
    pub max_markers: Option<usize>,
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct DocConfig {
    pub min_coverage: Option<f64>,
    pub min_pct: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ComplexityConfig {
    pub max_violations: Option<usize>,
    pub threshold: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct TaintConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct DuplicationConfig {
    pub max_duplication: Option<f64>,
    pub max_duplicates: Option<f64>,
    pub min_lines: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct RiskConfig {
    pub max_score: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct CouplingConfig {
    pub max_coupling: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct MutationConfig {
    pub min_score: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SecurityConfig {
    pub max_vulnerabilities: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct SecretsConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct LicensesConfig {
    pub deny: Option<Vec<String>>,
    pub allow: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct DeadCodeConfig {
    pub max_findings: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct TypeCoverageConfig {
    pub min_coverage: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct HalsteadConfig {
    pub max_bug_estimate: Option<f64>,
}

/// Validate a loaded config and print warnings for common issues.
/// Returns true if the config is valid, false if there are critical errors.
pub fn validate_config(config: &Config) -> bool {
    let mut valid = true;
    let mut warnings = Vec::new();

    // Check for conflicting thresholds
    if let Some(ref crap) = config.crap {
        if let Some(threshold) = crap.threshold {
            if threshold <= 0.0 {
                warnings.push("crap.threshold should be positive".to_string());
            }
            if threshold > 100.0 {
                warnings.push("crap.threshold > 100 is very permissive".to_string());
            }
        }
        if let Some(max_avg) = crap.max_avg {
            if max_avg <= 0.0 {
                warnings.push("crap.max_avg should be positive".to_string());
                valid = false;
            }
        }
    }

    if let Some(ref doc) = config.doc {
        if let Some(min_pct) = doc.min_pct.or(doc.min_coverage) {
            if !(0.0..=100.0).contains(&min_pct) {
                warnings.push(format!("doc.min_pct should be 0-100, got {}", min_pct));
                valid = false;
            }
        }
    }

    if let Some(ref debt) = config.debt {
        if let Some(max_items) = debt.max_items.or(debt.max_markers) {
            if max_items > 10000 {
                warnings.push(format!("debt.max_items={} is very high, analysis may be slow", max_items));
            }
        }
    }

    if let Some(ref complexity) = config.complexity {
        if let Some(max_violations) = complexity.max_violations {
            if max_violations > 1000 {
                warnings.push(format!("complexity.max_violations={} is very high", max_violations));
            }
        }
    }

    // Check for unknown/unused project settings
    if let Some(ref project) = config.project {
        if let Some(ref eco) = project.ecosystem {
            let known = ["rust", "python", "javascript", "typescript", "go"];
            if !known.contains(&eco.to_lowercase().as_str()) {
                warnings.push(format!("unknown ecosystem '{}', expected one of: {:?}", eco, known));
            }
        }
    }

    // Print warnings
    for warning in &warnings {
        eprintln!("  {} config: {}", "⚠".yellow(), warning);
    }

    valid
}

/// Load and validate config from a file path.
/// Returns the config and whether it's valid.
pub fn load_and_validate(config_path: &str) -> (Config, bool) {
    let content = match std::fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("  {} could not read {}", "✗".red().bold(), config_path);
            return (Config::default(), false);
        }
    };

    let config: Config = match toml::from_str(&content) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("  {} invalid TOML in {}: {}", "✗".red().bold(), config_path, e);
            return (Config::default(), false);
        }
    };

    let valid = validate_config(&config);
    (config, valid)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project: None,
            crap: None,
            debt: None,
            doc: None,
            complexity: None,
            taint: None,
            duplication: None,
            risk: None,
            coupling: None,
            mutation: None,
            security: None,
            secrets: None,
            licenses: None,
            dead_code: None,
            type_coverage: None,
            halstead: None,
        }
    }
}
