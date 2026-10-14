---
id: P0-28
title: "web: plain viewer — WASM compile in the browser with a MapLibre custom layer"
labels: [type:task, area:web, agent-ok, size:M]
milestone: "Phase 0 — Vertical slice"
blocked_by: [P0-18]
---
## Goal
A deliberately plain page proving the browser tier: compile in WASM, draw lanes into MapLibre's WebGL2 context, edit one thing live.

## Context
- ADR-0014 (one rendering path)

## Scope
`web/**`, `crates/nixie-wasm/**` (bindings)

## Acceptance
- [ ] Loads an op-log JSON file, compiles it in WASM, draws lanes via a MapLibre custom layer from buffers produced by the core
- [ ] Basemap style URL configurable; no API keys committed
- [ ] One control (strip width) triggers incremental recompile and redraw, showing compile time on screen
- [ ] `wasm-bindgen-test` headless test of compile; `npm run build` passes in CI

## Out of scope
Any editing beyond the one control; styling.
