# Known Issues

This document tracks known issues in the CodeMetrics codebase.

## Pre-existing Test Failures

### test_python_docstring (ast-parse-ts/src/lib.rs:532)
**Status:** Known pre-existing issue - fails even when running directly  
**Description:** Test calls parse_doc_coverage() on a 2-function Python snippet and asserts documentation stats. Test fails with "attempt to subtract with overflow" panic.  
**Impact:** Not related to our changes (Solidity/OCaml support or UTCP integration)  
**Root Cause:** Likely in parse_doc_coverage() or stats calculation logic  
**Workaround:** Test fails consistently, but all 10 tools pass in full audit

**Note:** This test was failing before our session started (see git history).
