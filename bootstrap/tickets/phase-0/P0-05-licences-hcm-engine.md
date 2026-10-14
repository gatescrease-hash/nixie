---
id: P0-05
title: "research: confirm licences and choose the HCM engine (ADR-0022)"
labels: [type:adr, crate:hcm, needs-human, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: []
---
## Goal
Close the deferred licence questions that block Phase 0 and decide how HCM signalized analysis is implemented.

## Context
- ADR-0016, ADR-0017
- Spec "Deferred" list

## Acceptance
- [ ] Licence of `transportations-library` confirmed and compatible (or not) with Apache-2.0
- [ ] Licence of `osm2streets` confirmed (needed in Phase 1, cheap to settle now)
- [ ] ADR-0022: use `transportations-library` behind `HcmEngine` (with a plan to validate against HCS example problems) **or** implement HCM 7 signalized natively
- [ ] Optional, non-blocking: Ontario Road Network redistribution terms noted for Phase 1

## Notes
An agent may draft findings as a comment on this issue; a human makes the call.
