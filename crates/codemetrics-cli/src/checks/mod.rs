//!
//! Check functions module for codemetrics CLI.
//! Contains all check_* functions (crap, debt, doc, complexity, etc.)

use serde_json::Value;
use std::time::Instant;

// Re-export check functions from individual modules
pub mod crap;
pub mod debt;
pub mod doc;
pub mod complexity;
pub mod taint;
pub mod duplication;
pub mod risk;
pub mod coupling;
pub mod propcov;
pub mod fuzz;
pub mod linelen;
pub mod halstead;
pub mod secrets;
pub mod deadcode;
pub mod cohesion;
pub mod comments;
pub mod errhandle;
pub mod typecov;
pub mod vulnscan;
pub mod sast;
pub mod crypto;
pub mod licenses;
pub mod outdated;

/// Common result type for all checks
#[derive(Debug, Serialize, Deserialize)]
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
