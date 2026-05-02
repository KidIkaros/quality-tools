---
name: quality-tools
description: AI-native code quality audit toolkit - 10 automated tools for CI/CD pipelines and AI agents
version: 0.1.0
author: KidIkaros
license: Apache-2.0 OR OPL-1.1
platforms: [macos, linux]
metadata:
  hermes:
    tags: [Code-Quality, Rust, Metrics, Security, Testing, CI-CD]
    related_skills: [claude-code, opencode, codex]
    requires_toolsets: [terminal]
    fallback_for_tools: []
---

# quality-tools

Code quality audit toolkit with 10 tools for automated analysis. Designed for CI/CD pipelines and AI agents.

## When to Use

- User asks to audit code quality
- Before merging significant changes
- User asks about test coverage or risk
- Setting up CI/CD quality gates
- Evaluating code maintainability

## Quick Reference

| Tool | Purpose | Command |
|------|---------|---------|
| `quality` | Full batch audit | `cargo run -p quality-cli -- run . --format sarif` |
| `crap` | Risk scores | `cargo run -p crap-metric -- ./src --recursive` |
| `mutate` | Test quality | `cargo run -p mutation-test -- . -p {crate} --max 5` |
| `riskmap` | High-risk files | `cargo run -p risk-map -- . --min-risk 30` |
| `debt` | TODOs/FIXMEs | `cargo run -p debt-scan -- ./src --recursive` |
| `doccov` | Doc coverage | `cargo run -p doc-coverage -- ./src --recursive` |
| `taint` | Security | `cargo run -p taint-scan -- ./src --recursive` |

## Prerequisites

Build the tools first:

```bash
cargo build --release
```

Or install binaries to PATH via `cargo install`.

## Procedure

### 1. Full Audit (Recommended for CI/CD)

```bash
# Run all 10 tools, output SARIF for GitHub Security tab
cargo run -p quality-cli -- run . --format sarif > results.sarif

# Or simpler output
cargo run -p quality-cli -- run .
```

### 2. Quick Risk Check

```bash
# Find high-risk functions (CRAP > 15)
cargo run -p crap-metric -- ./src --recursive --format json

# Find complex/churned files
cargo run -p risk-map -- . --format json
```

### 3. Test Quality Check

```bash
# Requires: cargo test must pass first
# For a specific crate in workspace:
cargo run -p mutation-test -- . -p ast-parse-ts --max 5 --timeout 30
```

### 4. Technical Debt

```bash
# Find TODO/FIXME/HACK markers
cargo run -p debt-scan -- ./src --recursive
```

## Tool Details

### crap (CRAP Score Calculator)

- **Purpose**: Find functions with high maintenance risk
- **Formula**: CRAP = comp² × (1 - coverage/100)³ + comp
- **Threshold**: > 15 is risky, > 30 is critical
- **Requires**: Test coverage data (optional)

### mutate (Mutation Testing)

- **Purpose**: Evaluate test suite quality
- **Precondition**: `cargo test` must pass
- **Output**: Mutation score (0-100%)
- **Notes**: Won't work on crates with failing tests

### riskmap (Risk Map)

- **Purpose**: Identify files that change often AND are complex
- **Data**: Cross-references git churn with code complexity
- **Use case**: Prioritize code reviews

### taint (Taint Analysis)

- **Purpose**: Detect sensitive data flow
- **Checks**: passwords, keys, PII to sinks
- **Use case**: Security audits

## CI/CD Integration

### GitHub Actions

```yaml
- name: Quality Audit
  run: |
    cargo build --release
    quality run . --format sarif > results.sarif
- name: Upload SARIF
  uses: github/codeql-action/upload-sarif@v3
  with:
    sarif_file: results.sarif
```

### Pre-commit Hook

Run quick checks before commit:

```bash
# Fast debt check
cargo run -p debt-scan -- ./src --recursive

# Risk score check  
cargo run -p crap-metric -- ./src --recursive --min-score 20
```

## Output Formats

| Format | Use Case | Command |
|--------|----------|---------|
| `json` | Programmatic | `--format json` |
| `sarif` | GitHub Security | `--format sarif` |
| `ndjson` | Streaming | `--format ndjson` |
| `text` | Human readable | `--format text` |

## Pitfalls

1. **mutate fails**: Ensure `cargo test` passes first
2. **Coverage required for accurate CRAP**: Use `--coverage` or `--coveragePct`
3. **Workspace crates**: Use `-p crate-name` with mutate
4. **Build required**: Run `cargo build --release` before first use

## Verification

```bash
# Check all tools work
cargo run -p quality-cli -- run .

# Check specific tool
cargo run -p crap-metric -- ./src --recursive --format json | head
```

## Rules

1. Run full audit (`quality run`) before major merges
2. Fix CRAP > 30 immediately
3. Use `mutate` to verify test suites catch bugs
4. Zero tolerance for TODO/FIXME in production code
5. Address riskmap findings in code reviews

## See Also

- Repository: https://github.com/KidIkaros/quality-tools
- UTCP Manual: `utcps/quality-tools.json`
- Claude Code: `CLAUDE.md`
- OpenCode: `AGENTS.md`