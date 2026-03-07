# Handoff: Simple Key Stack and Compliance Push

**Date:** 2026-03-07
**Branch:** `main` (4 PRs merged: #30–#33)
**State:** Green

> Green = `just check` passes, safe to continue.

## Where things stand

YAML Test Suite compliance at 296/351 (84.3%, up from 246/351). The scanner now uses a proper simple key stack for deferred key resolution, replacing the eager peek-for-`:` approach. 110 scanner unit tests pass. 9 mismatches and 46 UNEXPECTED (error validation) remain.

## Decisions made

- **Simple key stack replaces `pending_prefix` buffer.** Deferred resolution via `SimpleKey` struct with monotonic token IDs. `needs_more_tokens()` guard prevents premature yielding. `VecDeque::insert` for retroactive Key/BlockMappingStart insertion. `scanner.rs:49–66, 265–340`.
- **`simple_key_allowed` resets on all line crossings.** Not just in `scan_to_next_token` — also after multi-line plain scalars, block scalars, and quoted scalars. `scanner.rs:211, 824, 1023, 1385`.
- **`adjacent_value_offset` uses exact byte offset, not boolean.** Prevents false positives like `[:x]` treating `:` as value indicator. `scanner.rs:108, 1388, 1563`.
- **Flow collections save simple key at OUTER flow level.** Before `flow_level++`, so `[a,b]: value` resolves correctly. `scanner.rs:505`.
- **Anchor/alias names allow `:`** per YAML 1.2 §6.9.2. `scanner.rs:922`.
- **Explicit key (`?`) recognized in flow context.** Not just block. `scanner.rs:1556`.

## What's next

Remaining 9 mismatches, in priority order:

1. **Block scalar + document end (M7A3, W4TN).** `fetch_block_scalar` content loop doesn't check for `...` at column 0. Add the same check as `scan_plain_scalar_text` uses for `---`/`...`. `scanner.rs:932–985`.

2. **Cross-line flow colon (5MUD, K3WX).** `"foo"\n  :bar` in flow mapping — colon on next line after quoted scalar. Needs `adjacent_value_offset` to survive across `scan_to_next_token` whitespace, or a different mechanism for multi-line flow keys.

3. **Tab escape folding (DE56).** `\t` before line fold in double-quoted scalar is stripped by `result.pop()` trimming, but escape-produced characters should be preserved. Need to distinguish literal trailing whitespace from escape-generated content. `scanner.rs:1126–1128`.

4. **Tag URI encoding (6CK3).** `!e!tag!` should encode the trailing `!` as `%21` in the tag suffix. `scanner.rs:856–890`.

5. **Error tests (9KBC, BD7L, CXX2).** Rust scanner is too permissive — produces tokens where C errors. Needs position validation: no mapping on `---` line, no mapping after sequence at same indent, no anchor on `---` line. Different workstream from token production.

6. **UNEXPECTED (46 tests).** Error test cases where Rust succeeds. Low priority — validation gaps, not token bugs.

## Landmines

- **`VecDeque::insert` in `fetch_value` bypasses `enqueue()`.** Every `queue.insert()` must be paired with `self.next_token_id += 1`. Forgetting this causes `queue_position()` math to fail for all subsequent simple keys. `scanner.rs:619–631`.
- **`needs_more_tokens()` + `unroll_indent` interaction.** The old `continue` after `unroll_indent` caused infinite loops with the simple key guard. Removed in PR #30 — don't re-add it.
- **`pending_complex_key_column` must be checked BEFORE simple key resolution** in `fetch_value`. Otherwise `? foo: bar` emits Key twice. `scanner.rs:583–597`.
- **`remove_simple_key()` is for structural tokens only.** Do NOT call it from `fetch_plain_scalar`, `push_quoted_scalar`, `fetch_tag`, or `fetch_anchor_or_alias`. These are content tokens — removing the simple key would discard a preceding tag/anchor's key mark. This caused 20 regressions when first attempted.
