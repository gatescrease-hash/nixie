---
id: P0-07
title: "golden: harness crate, fixture format and red Gate 0 suite"
labels: [type:task, crate:golden, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-06]
---
## Goal
`nixie-golden` loads `testdata/golden/<name>/{input,expected}/`, compares Nixie output against it using **named tolerances**, and exposes a Gate 0 suite that is red for clear, named reasons.

## Context
- ADR-0019, ADR-0020
- golden g01 content arrives in P0-03 (human-owned)

## Scope
`crates/nixie-golden/**`, `testdata/golden/_template/**`

## Acceptance
- [ ] Fixture format defined as Rust types with serde (TOML for expected values), each expected numeric value carrying a tolerance **name**, not a number
- [ ] `testdata/golden/_template/` shows the directory layout, including a `PROVENANCE.toml` template
- [ ] Comparison helpers: `assert_movements`, `assert_lane_graph`, `assert_lane_groups`, `assert_hcm`, `assert_landxml_roundtrip`, each producing a readable diff
- [ ] `cargo test -p nixie-golden` passes (unit tests for the comparison logic itself)
- [ ] `cargo test -p nixie-golden --features gate0` **fails**, listing each missing stage or fixture by name (e.g. `fixture g01-signalized missing (owned by P0-03)`), with no `#[ignore]`

## Out of scope
Writing any expected values for real networks.
