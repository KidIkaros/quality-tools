# CodeMetrics
<img src="logo.svg" alt="CodeMetrics Logo" width="400"/>

![Quality Audit](https://github.com/KidIkaros/codemetrics/actions/workflows/quality.yml/badge.svg)
[![Docs](https://img.shields.io/badge/docs-available-brightgreen)](./docs/)
[![ONBOARDING](https://img.shields.io/badge/onboarding-available-brightgreen)](./ONBOARDING.md)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/License-Apache--2.0%20%7C%20OPL--1.1-blue)](LICENSE)

**AI-native code quality audit toolkit** — 10 tools for automated code analysis that actually work. Designed for CI/CD pipelines and AI agents.

- **Zero config** — Auto-detects 15 languages from your source files
- **CI-ready** — JSON/SARIF output with severity levels and fix suggestions
- **Production-proven** — Used on real Rust projects
- **No dependencies** — tree-sitter based, no compilation needed

## Quick Demo

```bash
# Full audit with SARIF output (GitHub Security tab compatible)
codemetrics run . --format sarif > results.sarif

# Single tool
codemetrics crap ./src --recursive           # CRAP risk scoring
codemetrics mutate . -p my-crate --max-mutants 5  # Mutation testing
codemetrics doccov ./src --recursive      # Doc coverage
```

### Why CodeMetrics?

| Tool | What it catches |
|------|---------------|
| `crap` | High-risk functions needing tests |
| `mutate` | Weak test suites (mutants survive) |
| `taint` | Sensitive data leaks |
| `riskmap` | Files that change often AND are complex |
| `doccov` | Undocumented public APIs |
| `debt` | TODO/FIXME left behind |

## Known Limitations

- **Python docstring test**: The `test_python_docstring` test in `ast-parse-ts` is a pre-existing test that can fail unrelated to codebase changes. This does not affect the 10 quality tools or production usage.

## Documentation

- [User Guide](./docs/user-guide.md) — How to use CodeMetrics to audit and improve your project
- [Developer Guide](./docs/developer-guide.md) — Architecture, adding new tools, testing patterns
- [Metrics Explained](./docs/metrics-explained.md) — Detailed metric definitions, what scores mean, how to fix
- [Quality Standards](./docs/quality-standards.md) — "Exceeding Standards" targets and quality gates

Code quality metrics for 10+ languages via `tree-sitter`. All analysis is language-agnostic — no compilation required.

## Crates

| Crate | Binary | Purpose |
|-------|--------|---------|
| `ast-parse-ts` | (lib) | Universal AST parsing (tree-sitter) -- 15 languages |
| `quality-common` | (lib) | Shared utilities -- coverage parsing, CRAP scoring |
| `quality-cli` | `quality` | Unified CLI -- JSON/SARIF output |
| `crap-metric` | `crap` | CRAP score calculator |
| `mutation-test` | `mutate` | Mutation testing (Rust-only) |
| `debt-scan` | `debt` | TODO/FIXME/HACK tracking |
| `doc-coverage` | `doccov` | Documentation coverage |
| `duplication` | `dupfind` | Code duplication detection |
| `coupling` | `coupling` | Module coupling analysis |
| `risk-map` | `riskmap` | Churn × complexity map |
| `taint-scan` | `taint` | Taint analysis |
| `fuzz-surface` | `fuzz` | Fuzzable function detection |
| `prop-cov` | `propcov` | Property test coverage |

## Multi-Language Support

The `ast-parse-ts` crate uses tree-sitter grammars (pure Rust, no external dependencies) to analyze source files directly — no compilation needed. Now supports 15 languages: Rust, Python, JavaScript, TypeScript, Go, C, C++, C#, Java, PHP, Ruby, Swift, Kotlin, Solidity, Vyper, and OCaml.

### Supported Languages

| Language | Extensions | Tree-sitter Crate | Status |
|----------|-------------|------------------|--------|
| **Solidity** | `.sol` | `tree-sitter-solidity` | ✅ Implemented |
| **Vyper** | `.vy` | N/A (parsing not available) | ⚠️ Disabled |
| **OCaml** | `.ml`, `.mli` | `tree-sitter-ocaml` | ✅ Implemented |

### Legacy Languages (Pre-existing)

The following smart contract and functional languages were already supported:

| Language | Extension | Tree-sitter Crate | Status |
|----------|-----------|------------------|--------|
| Kotlin | `.kt`, `.kts` | `tree-sitter-kotlin` | ✅ Partial |

## AI-Native Toolkit

Designed for headless AI agent integration with:

- **Tool Discovery**: `quality discover --format json` outputs all available tools, their formats, output fields, and rule IDs for programmatic consumption
- **Self-Contained Findings**: All JSON/NDJSON outputs include `severity`, `help`, and `rule_id` fields so AI agents can explain and fix issues without reading docs
- **Streaming Support**: NDJSON format enables incremental processing for AI pipelines
- **Standardized Output**: Consistent fields across all tools for reliable parsing

| Tool | Rust | Python | JS/TS | Go | C/C++ | C# | Java | PHP | Ruby | Swift | Kotlin |
|------|:----:|:------:|:-----:|:--:|:-----:|:--:|:----:|:---:|:----:|:-----:|:------:|
| `debt-scan` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `taint-scan` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `complexity` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `doc-coverage` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `duplication` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `coupling` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `risk-map` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `crap-metric` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `mutation-test` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `fuzz-surface` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| `prop-cov` | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

**Note:** All tools now support 12 languages: Rust, Python, JavaScript, TypeScript, Go, C, C++, C#, Java, PHP, Ruby, Swift, and Kotlin. Tools run directly on source code via tree-sitter (no compilation needed).

## CRAP Metric

The CRAP (Change Risk Anti-Patterns) score estimates maintenance risk by combining cyclomatic complexity with test coverage:

```
CRAP = comp^2 * (1 - coverage/100)^3 + comp
```

- **comp** = cyclomatic complexity (number of decision points)
- **coverage** = percentage of code covered by automated tests
- Score > 30 = "crappy" code that is risky to maintain

### Usage

```bash
# Analyze a Rust crate (no coverage data)
codemetrics crap ./crates/my-crate/src --recursive

# Analyze Python files
codemetrics crap ./my-python-project --recursive

# Analyze TypeScript files
codemetrics crap ./my-web-app --recursive

# With lcov coverage file
codemetrics crap ./crates/my-crate/src --recursive --coverage coverage.info

# With coverage override
codemetrics crap ./crates/my-crate/src --recursive --coverage-pct 75

# JSON output
codemetrics crap ./crates/my-crate/src --recursive --format json

# Only show high-risk functions
codemetrics crap ./crates/my-crate/src --recursive --min-score 20
```

### Output

```
FUNCTION                       FILE                      LINE COMP   CRAP CATEGORY
──────────────────────────────────────────────────────────────────────────────
parse_era835                   src/lib.rs                 330   54  2970.0 ✗ crappy
carc_description               src/lib.rs                 244   59  3540.0 ✗ crappy
parse_cas                      src/lib.rs                 560    4    20.0 ○ good
parse_svc                      src/lib.rs                 587    2     6.0 ✓ excellent
```

## Mutation Testing

Mutation testing evaluates test suite quality by introducing deliberate changes (mutants) to source code. If tests still pass with a mutation, the test suite has a gap.

### Mutation Strategies

1. **Binary operator swaps**: `+` <-> `-`, `==` <-> `!=`, `&&` <-> `||`, etc.
2. **Boolean literal swaps**: `true` <-> `false`
3. **Boundary mutations**: `<` <-> `<=`, `>` <-> `>=`

### Usage

```bash
# Test a crate (runs cargo test for each mutant)
codemetrics mutate ./crates/my-crate --max-mutants 20

# Test specific files only
codemetrics mutate ./crates/my-crate --files src/lib.rs,src/parser.rs

# With custom timeout
codemetrics mutate ./crates/my-crate --timeout 60

# JSON output
codemetrics mutate ./crates/my-crate --format json

# With environment variables (e.g., CARGO_TARGET_DIR for FAT32)
CARGO_TARGET_DIR=/tmp/build mutate ./crates/my-crate
```

### Output

```
[1/10] Testing mutant 1 (src/lib.rs:569)... ✗ SURVIVED
[2/10] Testing mutant 2 (src/lib.rs:571)... ✓ KILLED
...

SUMMARY
  Total mutants:  10
  Killed:         6 (60%)
  Survived:       4 (40%)
  Mutation Score: 60%
  Verdict:        Weak — many mutations survived
```

## Building

```bash
# Standard build
cargo build

# FAT32 target directory (if build path doesn't support exec permissions)
CARGO_TARGET_DIR=/tmp/CodeMetrics-build cargo build

# Run tests (single crate — safe)
cargo test

# Run tests across the entire workspace without freezing your OS
# (uses batched compilation to cap peak memory)
./test.sh

# Ultra-safe mode: one crate at a time, single build job
./test.sh --safe
```

## Other Tools

### Technical Debt Scanner (`debt`)

```bash
# Scan for TODO/FIXME/HACK/XXX markers
debt ./src --recursive

# Only show FIXME and HACK
debt ./src --recursive --marker fixme,hack

# Sort by author
debt ./src --recursive --sort author
```

### Fuzz Surface Analysis (`fuzz`)

```bash
# Analyze Python files for fuzzable functions
fuzz ./my-python-project --recursive

# Analyze JavaScript/TypeScript files
fuzz ./my-web-app --recursive

# Analyze Go files
fuzz ./my-go-service --recursive

# Only show functions with high fuzzability score
fuzz ./src --recursive --min-score 30

# Show top 10 most fuzzable functions
fuzz ./src --recursive --top 10
```

### Documentation Coverage (`doccov`)

```bash
# Check public API documentation
codemetrics doccov ./src --recursive

# Fail if below 80% coverage
codemetrics doccov ./src --recursive --min 80
```

### Code Duplication (`dupfind`)

```bash
# Find structural duplicates (min 5 lines)
dupfind ./src --recursive

# Stricter: min 10 lines
dupfind ./src --recursive --min-lines 10
```

### Coupling Analysis (`coupling`)

```bash
# Module dependency graph
coupling ./

# Export as Graphviz dot
coupling ./ --format dot > deps.dot && dot -Tpng deps.dot -o deps.png

# Only show tightly coupled modules
coupling ./ --min-coupling 5
```

### Risk Map (`riskmap`)

```bash
# Cross-reference git churn with complexity
riskmap ./

# Only last 3 months
riskmap ./ --since "3 months ago"

# Only show risk score >= 30
riskmap ./ --min-risk 30
```

### Unified CLI (`quality`)

The `quality` CLI runs all tools in one batch, detects languages automatically, and produces CI-ready JSON/SARIF output.

```bash
# Full audit (auto-detects languages)
codemetrics run . --format json

# Watch mode — re-run checks on .rs file changes
quality watch . --checks debt,doc,crap --debounce-ms 500

# Record run to history
codemetrics run . --format json | quality history record --report /dev/stdin

# Show trend history
quality history show

# Install pre-commit hook
quality install-hooks .
quality uninstall-hooks .
```

**Multi-language example:**
```bash
# Scan a mixed Python/JS/Rust repo
codemetrics run ./my-project --format json
# → summary.languages_detected: ["javascript", "python", "rust"]
```

## Performance

`ast-parse-ts` maintains a **thread-local parser pool** — tree-sitter `Parser` instances are created once per thread per language and reused across files. Heavy tools (`duplication`, `taint-scan`, `coupling`) use **bounded `rayon` parallelism** (2 threads) for file scanning, and the batch runner (`quality`) caps concurrent tool execution at 4 to respect CI RAM limits.

## License

Apache-2.0 OR OPL-1.1

## Crates

| Crate | Type | Description |
|-------|------|-------------|
| `codemetrics-cli` | Binary | Main CLI tool (codemetrics binary) |
| `codemetrics-common` | Library | Shared utilities for all tools |
| `codemetrics-server` | Binary | HTTP API server |
| `coupling` | Binary | Code coupling analysis |
| `crap-metric` | Binary | CRAP score calculation |
| `debt-scan` | Binary | Technical debt scanning |
| `doc-coverage` | Binary | Documentation coverage |
| `duplication` | Binary | Code duplication detection |
| `fuzz-surface` | Binary | Fuzz testing surface analysis |
| `mutation-test` | Binary | Mutation testing |
| `prop-cov` | Binary | Property coverage analysis |
| `risk-map` | Binary | Risk mapping (complex/churned files) |
| `taint-scan` | Binary | Taint analysis (sensitive data flow) |

## Example Output

### Full Audit (`codemetrics run . --format table`)
```
$(cat docs/screenshots/run-table.txt)
```

### CRAP Score (`codemetrics crap ./src --recursive`)
```
$(cat docs/screenshots/crap-output.txt)
```

### Technical Debt (`codemetrics debt ./src --recursive`)
```
$(cat docs/screenshots/debt-output.txt)
```
