//!
//! CLI definition for codemetrics.
//! Defines the main CLI structure and all subcommands using clap derive macros.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "codemetrics",
    about = "Unified code quality tool for Rust. Headless-first, JSON output, CI-ready.",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run all CodeMetrics checks and report results
    Check {
        /// Path to analyze
        path: String,

        /// Recursive scan
        #[arg(short, long)]
        recursive: bool,

        /// Output format: json (default) or text
        #[arg(short, long, default_value = "json")]
        format: String,

        /// Path to lcov coverage file
        #[arg(long)]
        coverage: Option<String>,

        /// Max average CRAP score (fail if exceeded)
        #[arg(long, default_value = "30")]
        max_crap: f64,

        /// Min doc coverage percentage (fail if below)
        #[arg(long, default_value = "50")]
        min_doc: f64,

        /// Max technical debt markers (fail if exceeded)
        #[arg(long, default_value = "100")]
        max_debt: usize,

        /// Max number of functions with complexity >= 10 allowed before failing (default: 0 = strict)
        #[arg(long, default_value = "0")]
        max_complexity_violations: usize,

        /// Max taint violations (default: 0)
        #[arg(long, default_value = "0")]
        max_taint: usize,

        /// Max code duplication percentage (default: 5.0)
        #[arg(long, default_value = "5.0")]
        max_duplication: f64,

        /// Max allowed file risk score (default: 10.0)
        #[arg(long, default_value = "10.0")]
        max_risk: f64,

        /// Max allowed architectural coupling issues (default: 5)
        #[arg(long, default_value = "5")]
        max_coupling: usize,

        /// Min property test coverage percentage (default: 0.0)
        #[arg(long, default_value = "0.0")]
        min_propcov: f64,

        /// Max unprotected fuzzable endpoints (default: 0)
        #[arg(long, default_value = "0")]
        max_fuzz_risk: usize,

        /// Max functions/files exceeding line length limits (default: 0)
        #[arg(long, default_value = "0")]
        max_linelen: usize,

        /// Max estimated bugs from Halstead metrics per file (default: 2.0)
        #[arg(long, default_value = "2.0")]
        max_halstead_bugs: f64,

        /// Max hardcoded secret findings (default: 0)
        #[arg(long, default_value = "0")]
        max_secrets: usize,

        /// Max dead code findings (default: 10)
        #[arg(long, default_value = "10")]
        max_deadcode: usize,

        /// Max LCOM4 cohesion violations (default: 5)
        #[arg(long, default_value = "5")]
        max_cohesion: usize,

        /// Minimum comment ratio 0.0–1.0 (default: 0.05 = 5%)
        #[arg(long, default_value = "0.05")]
        min_comment_ratio: f64,

        /// Max error handling violations (unwrap/expect/panic/discard, default: 50)
        #[arg(long, default_value = "50")]
        max_errhandle: usize,

        /// Minimum type annotation coverage % for Python/JS/TS (default: 0 = off)
        #[arg(long, default_value = "0.0")]
        min_typecov: f64,

        /// Max critical CVEs from dependency scan (default: 0)
        #[arg(long, default_value = "0")]
        max_vuln_critical: usize,

        /// Max high CVEs from dependency scan (default: 0)
        #[arg(long, default_value = "0")]
        max_vuln_high: usize,

        /// Max SAST findings — SQL injection, XSS, path traversal, cmd injection (default: 0)
        #[arg(long, default_value = "0")]
        max_sast: usize,

        /// Max crypto findings — weak hash, insecure random, ECB, disabled TLS (default: 0)
        #[arg(long, default_value = "0")]
        max_crypto: usize,

        /// Max OSS license violations (default: 0)
        #[arg(long, default_value = "0")]
        max_license_violations: usize,

        /// Max direct dependencies that are a full major version behind latest (default: 0, requires cargo-outdated)
        #[arg(long, default_value = "0")]
        max_outdated: usize,

        /// Skip specific checks (comma-separated: crap,debt,doc,complexity,taint,risk,coupling,propcov,fuzz,linelen,halstead,secrets,deadcode,cohesion,comments,errhandle,typecov,vulnscan,sast,crypto,licenses)
        #[arg(long)]
        skip: Option<String>,

        /// Run only these checks (comma-separated); takes precedence over --skip
        #[arg(long)]
        only: Option<String>,

        /// CI mode: JSON output, no TTY colors or progress (equivalent to --format json + CODEMETRICS_NO_PROGRESS=1)
        #[arg(long)]
        ci: bool,

        /// Show top offenders (file:line) for every check, not just failed ones
        #[arg(long)]
        verbose: bool,
    },

    /// Verify environment dependencies (doctor)
    Setup,

    /// CRAP metric only
    Crap {
        path: String,
        #[arg(short, long)]
        recursive: bool,
        #[arg(long)]
        coverage: Option<String>,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Technical debt only
    Debt {
        path: String,
        #[arg(short, long)]
        recursive: bool,
        #[arg(long)]
        marker: Option<String>,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Documentation coverage only
    Doccov {
        path: String,
        #[arg(short, long)]
        recursive: bool,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Code duplication only
    Dupfind {
        path: String,
        #[arg(short, long)]
        recursive: bool,
        #[arg(long, default_value = "5")]
        min_lines: usize,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Cyclomatic complexity report
    Complexity {
        path: String,
        #[arg(short, long)]
        recursive: bool,
        #[arg(long, default_value = "5")]
        min_complexity: u32,
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Generate default config file
    Init {
        /// Output path (default: .quality.toml)
        #[arg(long, default_value = ".quality.toml")]
        output: String,

        /// Full CI bootstrap: also writes GitHub Actions workflow, installs pre-commit hook, seeds baseline, records history
        #[arg(long)]
        ci: bool,
    },

    /// Run all CodeMetrics tools in batch mode using .quality.toml config
    Run {
        /// Path to the crate root (directory with Cargo.toml)
        path: String,

        /// Config file (default: .quality.toml)
        #[arg(long, default_value = ".quality.toml")]
        config: String,

        /// Output format (table, json, or sarif)
        #[arg(short, long, default_value = "table")]
        format: String,

        /// Baseline SARIF/JSON file: only emit new/regressed results
        #[arg(long)]
        baseline: Option<String>,

        /// Do not exit 1 on baseline regression (useful for seeding a new baseline)
        #[arg(long)]
        no_fail_on_regression: bool,
    },

    /// Record or display CodeMetrics history
    History {
        /// Action: record (append current run to history) or show (print trend table)
        #[arg(default_value = "show")]
        action: String,

        /// History directory (default: .codemetrics-history)
        #[arg(long, default_value = ".codemetrics-history")]
        dir: String,

        /// Number of recent runs to show
        #[arg(long, default_value = "10")]
        last: usize,

        /// Path to a JSON run report to record (default: stdin)
        #[arg(long)]
        report: Option<String>,
    },

    /// Install a CodeMetrics pre-commit git hook
    InstallHooks {
        /// Git repo root (default: current directory)
        #[arg(default_value = ".")]
        repo: String,

        /// Install a lightweight hook that skips test execution (metrics only)
        #[arg(long)]
        fast: bool,
    },

    /// Remove the CodeMetrics pre-commit git hook
    UninstallHooks {
        /// Git repo root (default: current directory)
        #[arg(default_value = ".")]
        repo: String,
    },

    /// Watch for file changes and re-run relevant checks
    Watch {
        /// Path to watch
        #[arg(default_value = ".")]
        path: String,

        /// Which checks to run on change (comma-separated: crap,debt,doc,complexity)
        #[arg(long, default_value = "debt,doc,crap")]
        checks: String,

        /// Debounce delay in milliseconds
        #[arg(long, default_value = "500")]
        debounce_ms: u64,

        /// Skip running tests and coverage collection (metrics-only mode)
        #[arg(long)]
        no_tests: bool,

        /// Run all available checks every cycle (equivalent to codemetrics check)
        #[arg(long)]
        full: bool,
    },

    /// Discover available CodeMetrics tools and their capabilities
    Discover {
        /// Output format: json (default) or text
        #[arg(short, long, default_value = "json")]
        format: String,
    },

    /// Generate HTML or Markdown audit report
    Report {
        /// Path to analyze
        path: String,

        /// Output format: html (default) or markdown
        #[arg(long, default_value = "html")]
        format: String,

        /// Output file path (default: report.html or report.md)
        #[arg(long)]
        output: Option<String>,

        /// Generate from an existing JSON snapshot (from `codemetrics check . --format json`)
        #[arg(long)]
        from_json: Option<String>,

        /// Auto-launch report in browser after generation
        #[arg(long)]
        open: bool,
    },

    /// Compare two JSON snapshots and show regressions/fixes
    Diff {
        /// Path to old JSON snapshot
        old: String,

        /// Path to new JSON snapshot
        new: String,

        /// Output format: text (default) or json
        #[arg(short, long, default_value = "text")]
        format: String,
    },
}
