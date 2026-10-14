---
id: P0-12
title: "schema: stable IDs, movement keys and anchors"
labels: [type:task, crate:schema, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-08]
---
## Goal
The identity and location primitives every entity uses.

## Context
- ADR-0003 (anchoring and defaults table), ADR-0004 (stable IDs)

## Scope
`crates/nixie-schema/**`

## Acceptance
- [ ] `AuthoredId` (random 128-bit) and `DerivedId` = platform-stable hash (e.g. BLAKE3, truncated to 128 bits) of an ordered list of `AuthoredId`s plus a `Role`
- [ ] `DerivedId` constructors cannot accept a `DerivedId` as input (compile-fail test via `trybuild`)
- [ ] `MovementKey { junction, approach_leg, departure_leg, turn: Left | Through | Right | UTurn }`
- [ ] `Anchor::{Station, Absolute, SnapTo}`; per-type default anchors exactly as ADR-0003's table
- [ ] Hash-stability test with fixed inputs and committed expected values
