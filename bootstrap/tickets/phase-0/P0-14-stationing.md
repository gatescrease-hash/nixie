---
id: P0-14
title: "schema: stationing and station equations"
labels: [type:task, crate:schema, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-09, P0-12]
---
## Goal
Display stations with chainage breaks, while everything internal stays true distance.

## Context
- ADR-0003

## Scope
`crates/nixie-schema/**`

## Acceptance
- [ ] `TrueDistance` (internal) and `Station { value, region }` (display/interop) types
- [ ] `StationEquation { at: TrueDistance, back, ahead }` list per alignment; conversions both ways
- [ ] Ahead < back case: a repeated station value resolves by `region`; ambiguous input without a region is a typed error
- [ ] Round-trip property test `true → station → true`
