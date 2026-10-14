---
id: P0-24
title: "adapter-sumo: run SUMO and map results back by stable ID"
labels: [type:task, crate:adapter-sumo, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-23]
---
## Goal
A seeded SUMO run of golden g01 whose results attach to Nixie movements.

## Context
- ADR-0018 (seeded ensembles), ADR-0016

## Scope
`crates/nixie-adapter-sumo/**`

## Acceptance
- [ ] Flows generated per `MovementKey` from volumes
- [ ] Runs `sumo` as a subprocess with a recorded seed; N-seed ensemble supported
- [ ] Parses outputs into per-movement throughput and mean delay with ensemble mean and spread, keyed by stable ID
- [ ] Gate 0 check: the run completes and every movement has a result (no accuracy target in Phase 0)
