---
id: P0-03
title: "golden: author expected outputs for golden g01"
labels: [type:task, crate:golden, needs-human, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-02, P0-07, P0-13]
---
## Goal
Hand-authored, sourced expected outputs for golden g01, in the fixture format defined by P0-07.

## Context
- Choice and provenance: P0-02
- Fixture format: P0-07
- Schema v0 (for the authored input): P0-13

## Scope
`testdata/golden/g01-signalized/**`

## Acceptance
- [ ] `input/`: authored model of the intersection as an op-log fixture (agents may draft this; a human checks it against the source)
- [ ] `expected/movements`: every movement key `(junction, approach, departure, turn)` with its volume
- [ ] `expected/lane_graph`: lanes per approach and departure, one connector set per movement
- [ ] `expected/lane_groups`: HCM lane groups per approach
- [ ] `expected/hcm`: control delay, v/c and LOS per lane group, approach and intersection, from the published example, with the reported precision as the tolerance
- [ ] `expected/landxml`: element types, lengths and stations for each alignment
- [ ] `cargo test -p nixie-golden` loads the fixture without format errors

## Notes
Agents may help with formatting, but **values come from the published source or a human hand calculation**, never from Nixie output.
