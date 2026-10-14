---
id: P0-13
title: "schema: v0 entity types for the Phase 0 slice"
labels: [type:task, crate:schema, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-12, P0-09]
---
## Goal
Just enough of the published schema to author golden g01.

## Context
- Spec: Data model entity layers
- ADR-0007 (fidelity), ADR-0008 (schedules), ADR-0009 (vertical profile reserved), ADR-0013 (published schema)

## Scope
`crates/nixie-schema/**`, `schema/nixie-v0.json`

## Acceptance
- [ ] **Reference:** CRS (EPSG code), local origin
- [ ] **Alignment:** horizontal elements from `nixie-geom`; optional vertical profile and cross-slope fields reserved
- [ ] **Corridor:** cross-section template of ordered strips (kind, width, direction, mode permissions, surface), each strip with an `AuthoredId`; station ranges; `schedule` field supporting only `Always` for now
- [ ] **Junction:** legs (alignment + station + side), `fidelity` enum (macro, HCM, micro), movements by `MovementKey`
- [ ] **Control:** minimal NEMA ring-barrier signal controller (phases, movements served, cycle, splits, yellow, all-red, offset)
- [ ] **Observations (minimal):** turning movement volume per `MovementKey` and period
- [ ] JSON Schema generated with `schemars` and committed; CI test fails if it drifts without a version bump

## Out of scope
Transit, land use and demand, results layers; schedules other than `Always`.
