---
id: P0-27
title: "storage: GeoPackage native file with op log and checksummed projections"
labels: [type:task, area:storage, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-15, P0-18]
---
## Goal
Projects save to and open from a GeoPackage where the op log is authoritative.

## Context
- ADR-0015; ADR-0014 (SQLite is C: native builds only)

## Scope
new `crates/nixie-store/**` behind a `native` feature

## Acceptance
- [ ] Tables: op log, metadata (schema version, projection checksums), projection feature tables for alignments and lanes
- [ ] Open = replay op log; projection tables regenerated on save
- [ ] Tampering with a projection table is detected on open and reported (reject only in Phase 0)
- [ ] File opens in QGIS (manual check noted in PR, screenshot attached)
- [ ] Not reachable from the wasm32 build
