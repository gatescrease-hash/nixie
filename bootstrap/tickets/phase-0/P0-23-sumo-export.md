---
id: P0-23
title: "adapter-sumo: export a valid SUMO network with stable IDs"
labels: [type:task, crate:adapter-sumo, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-19, P0-13]
---
## Goal
golden g01 compiles to SUMO plain XML that `netconvert` accepts, with SUMO IDs equal to Nixie stable IDs.

## Context
- ADR-0016, ADR-0017 (SUMO runs as a separate process; never linked)

## Scope
`crates/nixie-adapter-sumo/**`, `ci/sumo/Dockerfile` (pinned SUMO version)

## Acceptance
- [ ] Writes `.nod.xml`, `.edg.xml`, `.con.xml`, `.tll.xml` from the lane graph and control
- [ ] `netconvert` runs as a subprocess and succeeds with no errors on golden g01 in CI (container with pinned SUMO)
- [ ] Every SUMO edge, lane and connection ID maps back to a Nixie `DerivedId` (test)
