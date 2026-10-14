---
id: P0-29
title: "gate: Gate 0 review and Phase 1 planning"
labels: [type:gate, needs-human, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-01, P0-02, P0-03, P0-04, P0-05, P0-06, P0-07, P0-08, P0-09, P0-10, P0-11, P0-12, P0-13, P0-14, P0-15, P0-16, P0-17, P0-18, P0-19, P0-20, P0-21, P0-22, P0-23, P0-24, P0-25, P0-26, P0-27, P0-28]
---
## Goal
Declare Gate 0 passed (or not), lock the budgets, and plan the next phase.

## Acceptance
- [ ] `gate0` CI job green on `main` for golden g01 and its LHT twin; job made **required**
- [ ] ADR-0019 values confirmed or revised by a superseding ADR, then Accepted
- [ ] `ROADMAP.md`: gate history row filled; current phase advanced
- [ ] Phase 1 broken into epics (junction generator, scaled compiler, design editor, DXF/GMNS, OSM/ORN) and the first tickets created; nothing beyond Phase 1 ticketed
