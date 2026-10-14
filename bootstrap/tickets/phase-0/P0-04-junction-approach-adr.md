---
id: P0-04
title: "junction: decide the Phase 0 junction-zone approach (ADR-0021)"
labels: [type:adr, crate:junction, needs-human, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: []
---
## Goal
A decision, recorded as ADR-0021, on how much junction-zone generation Phase 0 builds, so P0-19 can be implemented without guessing.

## Context
- Junction zone generation is roughly half the compiler and is where A/B Street, netedit and RoadRunner all struggle (spec: Known hard problems).
- ADR-0001: authored overrides are the escape hatch.

## Options to weigh
1. **Connectors only.** Lane-end to lane-end connector curves per movement; junction polygon = hull of lane ends. Fastest; no curb returns.
2. **Authored curb returns + generated connectors.** Curb-return radius per corner is authored; connectors generated; junction polygon bounded by returns. Moderate.
3. **Full generator.** Curb returns derived from offsets of different alignments, conflict zones, movement paths. Phase 1 work.

## Acceptance
- [ ] ADR-0021 merged with the chosen option, the connector curve type, and what is deliberately deferred to Phase 1
- [ ] P0-19's Scope and Acceptance updated to match
- [ ] Initial list of golden junction-library cases for Phase 1 recorded in the ADR
