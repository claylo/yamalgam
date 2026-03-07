# Handoff: Parser Milestone Complete — 95.4% Event Compliance

**Date:** 2026-03-07
**Branch:** `main` (PRs #48-#53 merged this session)
**State:** Green

> Green = `just check` passes, safe to continue.

## Where things stand

YAML pull parser is feature-complete. 22 parser states, 13 event types, 1117 lines in `parser.rs`. Event compliance at 335/351 (95.4%) with 0 EVENT_MISMATCH. Token compliance unchanged at 343/351 (97.7%). 65 parser unit tests, 707 compliance tests (token + event), all passing.

The parser is a StAX-style pull parser — `Iterator<Item = Result<Event, ParseError>>`. It is the primary interface for streaming YAML processing, not an intermediate layer for tree-building. See `docs/plans/2026-03-07-parser-layer-design.md`.

## Decisions made

- **ADR-0005: Directives as separate events** — `VersionDirective` and `TagDirective` are events in the stream, not consumed into `DocumentStart`. Preserves full fidelity for CST round-tripping. Comparison harness filters them before comparing with libfyaml.
- **Pull parser as primary interface** — `yg`, serde, linter, validators consume the event stream directly. Only DOM/CST builds a tree. Design doc updated to reflect this.
- **Comments deferred** — Future plan: scanner emits `TokenKind::Comment` inline, parser skips them, CST builder attaches to nodes during single-pass construction. No sidecar index.
- **C harness `--events` mode** — `tools/fyaml-tokenize/fyaml-tokenize --events` uses `fy_parser_parse()` for event-level comparison.

## What's next

Next milestone options (same as scanner handoff, now with parser complete):

1. **Composer/document-builder** — anchor table, alias resolution, merge key (`<<`) expansion. This is where `!include`, `$ref`, and merge-all-of live.
2. **Serde `Deserializer`** — backed directly by the pull parser. No intermediate tree. `ref/feature-checklist.txt` tracks this.
3. **CST layer** — round-trip comment/style preservation. Requires scanner comment tokens first.
4. **`yg` CLI tool** — streaming jq/yq equivalent using the pull parser. Listed in `ref/feature-checklist.txt`.

## Landmines

- **`parse_node()` BlockEntry context check** (`parser.rs:435-460`) inspects `state_stack.last()` to decide indentless sequence vs empty scalar. If you add new mapping-like states, update the `in_mapping_context` check or FH7J/PW8X will regress.
- **Tag comparison skipped in event compliance** — `events_match()` in `compare.rs` ignores tag values because libfyaml resolves to full URIs while yamalgam keeps shorthand forms. When building tag resolution, revisit this.
- **11 EVENT_UNEXPECTED** — all `fail:true` tests where the scanner is too permissive. These are scanner-level issues (9HCY, 9KBC, 9MMA, BS4K, C2SP, CXX2, EB22, KS4U, QLJ7, RHX7, SF5V). Parser can't fix what the scanner accepts.
- **`cargo fmt --all && just clippy`** must run before every commit. PR #52 failed lint CI because this was skipped.
- **C harness `--events` mode** does NOT call `fy_parser_set_default_document_state()` — that's scanner-only API. Don't add it to the events path.
