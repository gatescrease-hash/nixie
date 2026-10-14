---
id: P0-15
title: "oplog: operations, replay, snapshots and upcasters"
labels: [type:task, crate:oplog, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-13]
---
## Goal
All model state comes from replaying a versioned operation log.

## Context
- ADR-0005

## Scope
`crates/nixie-oplog/**`, `testdata/oplog/**`

## Acceptance
- [ ] Operation envelope `{ schema_version, op_id, author, payload }`
- [ ] Ops sufficient for golden g01: create/edit alignment, create corridor, set strips, create junction, set control, set volumes, add override, delete
- [ ] `replay(ops) -> Result<Model>`; snapshot every N ops; replay from snapshot equals full replay (property test)
- [ ] `Upcaster` trait and registry; a dummy v0 → v1 upcaster test proves the mechanism
- [ ] CI test replays every file in `testdata/oplog/` with current upcasters

## Out of scope
CRDT merge, scenario branching.
