# ADR-0014: One Rust core in three tiers; pure Rust in the browser

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

| Tier | Runs | Geometry and CRS |
| --- | --- | --- |
| Browser (WASM) | Editor, compiler, validator, HCM, ghost traffic, presentation | Pure-Rust geometry; CRS grid shifts precomputed per project |
| Desktop (Tauri) | Everything above + SUMO, Python (AequilibraE), CAD converters, GPU compute | PROJ with NTv2 grids, GEOS |
| Server (headless) | Desktop engines, multi-user PostGIS, ensembles, `nixie check` in CI | PROJ, GEOS |

- C libraries (PROJ, GEOS, IfcOpenShell) are linked **only in native builds**; the `wasm32` build stays C-free.
- Frontend: TypeScript UI over **MapLibre GL (WebGL2)**, with Nixie layers drawn into the same context via custom layers from buffers produced by the WASM core. wgpu is for desktop/server compute only.
- The compiler is incremental: a dependency graph from authored to derived objects.

## Enforcement

CI builds `nixie-compiler` (and everything it depends on) for `wasm32-unknown-unknown`. Native-only deps sit behind a `native` feature.
