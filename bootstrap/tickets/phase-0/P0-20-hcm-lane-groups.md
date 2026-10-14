---
id: P0-20
title: "compiler: HCM lane groups from the lane graph"
labels: [type:task, crate:compiler, agent-ok, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-19]
---
## Goal
Derive HCM 7 lane groups from the lane graph and movements.

## Context
- HCM 7 signalized lane-group formation rules (exclusive turn lanes form their own group; shared lanes per HCM rules)

## Scope
`crates/nixie-compiler/**`

## Acceptance
- [ ] Lane groups derived per approach; IDs from authored IDs + role
- [ ] Gate 0 `assert_lane_groups` passes for golden g01
- [ ] Adding a turn lane changes lane groups but not movement IDs or attached volumes
