# Handoff: Scanner fail:true Rejections + C Baseline Cache

**Date:** 2026-03-07
**Branch:** `main`
**State:** Green — fmt/clippy clean, all tests pass, committed.

## Where things stand

All 11 EVENT_UNEXPECTED compliance failures are fixed — `fail:true` inputs the scanner was incorrectly accepting. Event compliance: 346 PASS (+11), 0 UNEXPECTED (-11), 5 EXPECTED (unchanged). Token compliance: 334 PASS, 2 MISMATCH, 15 EXPECTED (all `fail:true`). 143 scanner + 65 parser + 702 compliance tests pass.

Also shipped: `fyaml-tokenize --batch` mode and a C baseline cache (`just c-baseline`) that eliminates ~700 subprocess spawns per compliance run.

## Decisions made

- **Document state tracking in the scanner** — 6 new fields on `Scanner` (`in_directive_prologue`, `seen_directive`, `seen_yaml_directive`, `document_start_line`, `root_token_emitted`, `tag_handles`) to validate directives, tag handles, block structure, and root node count during tokenization.
- **Tag handle scope validation at scanner level** — `%TAG`-declared handles are tracked per document. Named handles (`!name!suffix`) are validated against the registry; primary `!` and secondary `!!` are always valid.
- **`root_token_emitted` only for Scalar/Alias** — anchors and tags are node properties, not standalone root values. Setting the flag on anchor/tag purge caused 9 false positive regressions.
- **C baseline cache** — `generate_baseline` binary drives `fyaml-tokenize --batch` (2 process spawns for all 351 test cases × 2 modes). Compliance tests read from `target/c-baseline-*.json` via `OnceLock`, fall back to subprocess if cache is missing.

## What's next

1. **Regenerate baseline** — `just c-baseline` to update cache with the new scanner behavior.
2. **Next milestone** — composer/document-builder, serde deserializer, CST layer, or `yg` CLI. See previous handoff for details.

## Landmines

- **Error paths in `fetch_next_token` must use `self.error` + `continue`, never direct `return Some(Err(...))`** — the `next()` method drains queued tokens before checking state. Direct returns skip the queue drain, and without `state = Done`, create infinite iterators. This caused a memory leak (64 GB per process) that crashed the machine. The existing `self.error` drain at the top of the loop already sets `state = Done`.
- **`fetch_value` inline indent-rolling bypasses `roll_indent()`** — the `document_start_line` check for 9KBC/CXX2 exists in both `roll_indent()` AND the simple key resolution path in `fetch_value()`. If you add a new check to `roll_indent`, check if `fetch_value`'s inline path needs it too.
- **EB22's YAML input has `# comment`** — the test suite file has `scalar1 # comment`, not bare `scalar1`. Without the comment, plain scalar continuation at indent -1 eats `%YAML 1.2` as scalar content and the directive check never fires.
- **`just c-baseline` must be re-run** after changes to `fyaml-tokenize` or the YAML test suite. The cache files are in `target/` (not committed).

