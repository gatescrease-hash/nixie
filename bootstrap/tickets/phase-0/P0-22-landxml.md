---
id: P0-22
title: "io-landxml: alignment export and import with station equations"
labels: [type:task, crate:io-landxml, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-10, P0-14]
---
## Goal
Alignments go out to LandXML 1.2 and come back unchanged — the only round-trip Gate 1 tests.

## Context
- ADR-0002, ADR-0003

## Scope
`crates/nixie-io-landxml/**`

## Acceptance
- [ ] Export `<Alignment>` with `<CoordGeom>` `Line`, `Curve`, `Spiral` (clothoid) and `<StaEquation>`; metric units
- [ ] Import the same; property test: export → import yields identical element types and parameters within `OFFSET_CAD` / `COORD_RESOLUTION`
- [ ] Gate 0 `assert_landxml_roundtrip` passes for golden g01
- [ ] Unsupported elements on import produce typed diagnostics, not panics

## Notes
If a maintainer can supply a LandXML exported from Civil 3D, add it as an import fixture (ask on this issue).
