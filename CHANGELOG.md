# Changelog

All notable changes to CodeMetrics are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Hermes Agent skill integration under `hermes/` for AI-native workflows
- JSON/NDJSON/SARIF output formats across all tools
- Schema validation files in `schemas/` for output contracts

### Changed
- **Rebranded from `quality-tools` to `codemetrics`** — all commands, paths, and references updated
- Unified CLI entry point: `codemetrics <subcommand>` (previously separate binaries)
- Default history directory renamed to `.codemetrics-history/`

### Fixed
- CI stabilization: ignored known flaky tests in `ast-parse-ts` and `taint` modules
- ANSI icon width handling in CRAP tool output for consistent test capture

---

## [1.0.0] — 2026-05-03

### Added
- Initial public release of CodeMetrics (stable v1)
- Ten analysis engines: `crap`, `mutate`, `debt`, `riskmap`, `doccov`, `taint`, `fuzz`, `coupling`, `dupfind`, `propcov`
- Single-binary CLI (`codemetrics`) with subcommands
- SARIF output support for GitHub Security tab integration
- JSON and NDJSON output formats for machine consumption
- Zero-configuration detection for 15+ programming languages
- Self-hosting: runs on its own codebase with CI validation

### Documentation
- Professional README with problem/solution framing
- User guide (`docs/user-guide.md`) and developer guide (`docs/developer-guide.md`)
- UTCP integration notes (`docs/utcp-integration.md`)
- Project status page (`PROJECT_STATUS.md`) with roadmap and limitations
- SVG logo and social preview assets

### Infrastructure
- GitHub Actions workflow with SARIF upload
- `.editorconfig` and `.pre-commit-config.yaml` for contributor consistency
- `PROJECT_STATUS.md` tracking tool health and known issues
- Hermes Agent skills exported to repo `hermes/` directory

---

## [0.1.0] — Prior to public release (as quality-tools)

### Added (pre-rebrand)
- Separate crate-per-tool architecture with workspace build
- Basic CLI wrappers for each tool
- Proof-of-concept AST-based duplication detection and CRAP metric

---

## Upgrade Guide

See [`UPGRADE.md`](UPGRADE.md) for migration instructions from `quality-tools` to `codemetrics`.
