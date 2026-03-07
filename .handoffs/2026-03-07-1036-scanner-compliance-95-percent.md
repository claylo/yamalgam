# Handoff: Scanner Compliance Push to 95.4%

**Date:** 2026-03-07
**Branch:** `main` (3 PRs merged this session: #40–#42)
**State:** Green

> Green = `just check` passes, safe to continue.

## Where things stand

YAML Test Suite compliance at 335/351 (95.4%), up from 314/351 (89.5%) at session start. 126 scanner unit tests, 548 total workspace tests. 8 UNEXPECTED remain (Rust too permissive), 4 EXPECTED (Rust correctly stricter than C on `fail:true` tests), 4 MISMATCH (parser-level/C-bug). Removed `yamalgam-mcp` crate (unused). Added scanner testing guide to `AGENTS.md`.

## Decisions made

- **SimpleKey.end_line field** tracks where key content actually ends, not where the reader landed after failed continuation lookahead. `scan_plain_scalar_text` returns `(Cow, u32)` tuple. `scanner.rs:62, 873`.
- **ns-plain-first is first-line only.** `scan_plain_scalar_line(is_first_line: bool)` — `-`, `?`, `:` followed by blank/flow-indicator rejected only on the first line, not continuation lines. `scanner.rs:807–832`.
- **Multiline key rules differ by context.** Block: never allowed. Flow: plain keys require `:` on same line as key end; json_key (quoted) allows `:` on a different line. `scanner.rs:785–802`.
- **Tab-as-indentation** only rejected when `indent >= 0` (active block structure). Tabs before the first block token are fine (e.g., `\t[`). `scanner.rs:242–250`.
- **Quoted scalar indent check** uses `(indent + 1).max(0)` not `.max(1)` — doc-level scalars at indent -1 can continue at column 0. `scanner.rs:1446, 1685`.
- **Flow collection close** validation allows `:` after `}`/`]` for patterns like `[a]: value`. `scanner.rs:621–643`.
- **Block entry same-line rejection** uses `last_block_token_line` (Value/Anchor line tracking), excludes explicit key values so `? key : - seq` works. `scanner.rs:691–704, 837–840`.
- **Flow indent tracking** — `flow_indent` field records block indent when entering flow context; content at or below this indent on a new line is rejected. `scanner.rs:278–289, 597–600`.
- **Compliance harness** now converts all YAML Test Suite visual markers: `—` (em-dash tab filler), `↵` (trailing newline marker), `∎` (no-final-newline). `compliance.rs:85–100`.
- **Removed `yamalgam-mcp`** — unused MCP server, token comparison is entirely CLI-driven.

## What's next

Remaining 8 UNEXPECTED (all `fail: true`, Rust too permissive):

1. **Block scalar indent edge cases** (5LLU, S98Z, W9L4) — spaces-only lines with deeper indent than first content line. Need `detect_block_indent_lookahead` to ignore spaces-only lines or validate indent consistency. `scanner.rs:1297–1323`.
2. **Anchor/tag placement** (G9HC, GT5M, H7J7) — anchors/tags at column 0 between block entries or at wrong indent relative to block structure. Hard to catch without parser context. Check if `last_block_token_line` approach can extend.
3. **Wrong indentation in mapping** (EW3V) — `k1: v1\n k2: v2` where `k2` at column 1 creates ambiguous structure. C says "invalid multiline plain key". May need plain scalar continuation to reject lines containing `: `.
4. **Flow seq implicit key across lines** (ZXT5) — `[ "key"\n  :value ]` in flow sequence context. C rejects but flow mapping equivalent is valid. Difference is container type.

Also 4 EXPECTED (we correctly reject, C incorrectly accepts): 62EZ, CML9, DK95, U99R — no action needed.

Also 4 MISMATCH (no scanner fix): M7A3 (C bug), 9KBC/BD7L/CXX2 (parser-level token ordering).

## Landmines

- **`purge_stale_simple_keys` uses `sk.end_line`** not `sk.mark.line` for staleness in block context. This was changed so multiline tokens survive long enough for `fetch_value` to run the multiline key check. If you change simple key lifetime logic, verify 7LBH, D49Q, G7JE, DK4H all still pass.
- **`scan_plain_scalar_text` consumes newlines on failed continuation.** The reader advances past `\n` + whitespace even when `scan_plain_scalar_line` returns empty. This is why `content_end_line` is returned separately — `end_mark.line` from the reader is wrong for single-line scalars that peeked ahead.
- **`last_block_token_line` must NOT be set for explicit key values.** `? key\n: - seq` is valid; `key: - seq` is not. The `is_explicit_key_value` flag gates this. See 5WE3, A2M4, KK5P test cases.
- **Flow indent check only fires when `flow_indent >= 0`** (flow entered from within a block structure). Flow at document level (`flow_indent = -1`) has no minimum indent.
- **C harness needs rebuilding** after any libfyaml changes: `cd tools/fyaml-tokenize && make clean && make`.
- **AGENTS.md now documents the full testing workflow** — scanner unit tests, compliance harness commands, result categories, and how to drill into specific cases. New sessions should read it.
