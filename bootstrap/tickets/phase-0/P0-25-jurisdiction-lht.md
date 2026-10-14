---
id: P0-25
title: "jurisdiction: pack v0 and the mirrored LHT twin of golden g01"
labels: [type:task, area:jurisdiction, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-19, P0-03]
---
## Goal
Driving side and related rules come from a pack, and golden g01 passes mirrored.

## Context
- ADR-0010

## Scope
`crates/nixie-schema/**` (pack types), `crates/nixie-golden/**` (mirror tool), `testdata/golden/g01-signalized-lht/**` (via Code-Owner-reviewed PR)

## Acceptance
- [ ] Pack type with driving side, display units, turn-on-red rule, signal convention placeholder; `ontario` and `lht-mirror-test` packs
- [ ] Mirror tool generates the LHT fixture (input and expected outputs) from golden g01
- [ ] The generated `expected/` is committed in a PR that a Code Owner approves
- [ ] Gate 0 suite runs both golden g01 and its LHT twin
