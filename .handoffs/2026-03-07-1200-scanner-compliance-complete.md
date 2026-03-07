# Handoff: Scanner Compliance Complete — 97.7%, 0 UNEXPECTED

**Date:** 2026-03-07
**Branch:** `main` (PRs #44–#46 merged this session)
**State:** Green

> Green = `just check` passes, safe to continue.

## Where things stand

YAML Test Suite compliance at 343/351 (97.7%), up from 335/351 (95.4%) at session start. 0 UNEXPECTED remaining — all `fail:true` tests that the Rust scanner should reject are now rejected. 126 scanner unit tests, 548+ total workspace tests. The scanner milestone is effectively complete.

## Decisions made

- **Block scalar spaces-only indent validation** — `detect_block_indent_lookahead` now returns `(indent, max_empty_spaces)` tuple. When spaces-only lines before the first content line exceed the detected indent, error. Fixes 5LLU, S98Z, W9L4. `scanner.rs:1393–1420, 1280–1290`.
- **Anchor/tag indent via `SimpleKey.required`** — activated the dormant `required` field. `purge_stale_simple_keys` now errors when discarding a required anchor/tag key. Fixes G9HC, GT5M, H7J7. `scanner.rs:430–460`.
- **Multiline plain key proactive check** — `fetch_plain_scalar` peeks for `:` after multiline scan in block context. Guarded by `pending_complex_key_column < 0` to allow explicit `?` keys. Fixes EW3V. `scanner.rs:1100–1115`.
- **Flow context type tracking** — `flow_is_mapping: Vec<bool>` stack distinguishes `{...}` from `[...]`. Multiline implicit keys rejected in flow sequences, allowed in flow mappings. Fixes ZXT5. `scanner.rs:130, 664, 687, 824–828`.

## What's next

The scanner is as good as it gets without a parser. Remaining 8 non-PASS cases:

| Category | Tests | Why unfixable at scanner level |
|----------|-------|-------------------------------|
| MISMATCH | M7A3 | C baseline bug (`%!PS` misinterpreted as directive) |
| MISMATCH | 9KBC, BD7L, CXX2 | Parser-level token ordering on `fail:true` input |
| EXPECTED | 62EZ, CML9, DK95, U99R | We correctly reject; C incorrectly accepts |

Next milestone options:
1. **Parser layer** — build the YAML parser on top of the scanner (events → document tree)
2. **API design** — implement the three-layer API (serde + DOM + CST) per `docs/plans/`
3. **serde integration** — `Deserializer` backed by the scanner

## Landmines

- **`purge_stale_simple_keys` now has early-return error path.** If you add new simple key types or change staleness logic, ensure the anchor/tag check at `scanner.rs:430–460` still fires correctly. Verify G9HC, GT5M, H7J7 still pass.
- **`flow_is_mapping` must stay in sync with `flow_level`.** Push on `fetch_flow_collection_start`, pop on `fetch_flow_collection_end`. If you add other flow-level manipulation, update the stack too.
- **The multiline plain key check skips explicit keys.** If `pending_complex_key_column >= 0`, the check is bypassed. Changing explicit key handling could re-expose EW3V-style regressions. Verify JTV5 and EW3V both pass after any `pending_complex_key_column` changes.
- **`detect_block_indent_lookahead` tuple return** changed the function signature. Any new callers must destructure both values.
