# ADR-0019: Tolerances and performance budgets are CI assertions

- **Status:** Proposed — to be confirmed or revised at Gate 0, then Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

| Name in `nixie_units::tol` / budget | Target | Reason |
| --- | --- | --- |
| `OFFSET_CAD` | 1 mm | Below construction and survey precision |
| `DISPLAY_TESSELLATION` | 0.5 px at current zoom | Invisible on screen |
| `COORD_RESOLUTION` | 0.1 mm via local origin | Doubles stay precise across a municipality |
| Single-junction incremental recompile | < 50 ms | Drag edits stay interactive |
| Full compile, 1,000-junction town | < 10 s | Whole-network edits stay practical |
| Editor frame rate while dragging | 60 fps | Game-like feel |
| Ghost traffic response after an edit | < 1 s | Queue reaction feels live |
| HCM results | Reproduce published HCM example problems to reported precision | Agency acceptance |

Golden test networks, each stored with expected compiler output, HCM results and LandXML round-trip results, run in CI:

1. One signalized intersection (Phase 0 slice).
2. Single-lane and two-lane roundabouts.
3. Skewed dog-leg intersection with channelized right turns.
4. Divided arterial with tapers, driveways and a station equation.
5. Diamond interchange ramp terminals.
6. A mirrored left-hand-traffic copy of each.
7. A small Ontario town network from the pilot partner.

## Enforcement

Values live in code once, in `nixie-units`. Budget tests run in CI on a pinned runner class; a regression beyond 10% fails the build. Changing a value requires a superseding ADR.
