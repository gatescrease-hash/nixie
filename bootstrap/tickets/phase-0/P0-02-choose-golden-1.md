---
id: P0-02
title: "golden: choose golden intersection g01 and its HCM oracle"
labels: [type:task, crate:golden, needs-human, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: []
---
## Goal
Pick the one signalized intersection Phase 0 must handle, with an **independent source of truth** for every expected value.

## Context
- ADR-0019 (golden networks; HCM results must reproduce published example problems)
- ADR-0020 (oracles are human-owned)

## Recommendation
Base it on a **published HCM 7 signalized example problem** so the HCM expected values have a published source. Make the geometry realistic and slightly awkward on purpose:
- four legs, at least one exclusive left-turn lane and one shared lane;
- protected-permissive left on one approach, pedestrian phases;
- at least one approach alignment that includes a clothoid, so geometry is exercised;
- volumes and timings taken from the example problem.

## Scope
`testdata/golden/g01-signalized/PROVENANCE.toml` only.

## Acceptance
- [ ] `PROVENANCE.toml` records: HCM example problem reference, geometry source, volume and timing source, any deviations from the example and why
- [ ] Comment on this issue summarising the choice so P0-03 can start

## Out of scope
Authoring the expected outputs (P0-03).
