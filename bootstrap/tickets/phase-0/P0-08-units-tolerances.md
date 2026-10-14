---
id: P0-08
title: "units: SI newtypes and named tolerance registry"
labels: [type:task, crate:units, agent-ok, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-06]
---
## Goal
The leaf crate every other crate uses for physical quantities and comparisons.

## Context
- ADR-0011 (SI internally), ADR-0012 (named tolerances), ADR-0019 (values)

## Scope
`crates/nixie-units/**`

## Acceptance
- [ ] Newtypes `Metres`, `Seconds`, `MetresPerSecond`, `Radians`, `Curvature` (1/m) with the arithmetic that makes physical sense and serde support
- [ ] `tol` module with `OFFSET_CAD` (1 mm), `COORD_RESOLUTION` (0.1 mm), `DISPLAY_TESSELLATION_PX` (0.5) and an `approx_eq(a, b, tol)` helper per type
- [ ] Each constant's doc comment cites ADR-0019
- [ ] No dependencies besides `serde`; builds for `wasm32-unknown-unknown`
- [ ] `cargo test -p nixie-units` passes

## Notes
`src/tol.rs` is Code-Owner protected: the values need maintainer review even though the code is agent-written.
