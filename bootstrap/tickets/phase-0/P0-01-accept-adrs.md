---
id: P0-01
title: "docs: review and accept ADRs 0001–0020; turn on repo protections"
labels: [type:task, area:docs, needs-human, size:S]
milestone: "Phase 0 — Vertical slice"
blocked_by: []
---
## Goal
The decisions in `docs/adr/` are read, adjusted where needed, and merged; the repo enforces them.

## Context
- ADR index: `docs/adr/README.md`
- ADR-0020 (work management and oracles)

## Scope
`docs/adr/**`, `docs/spec/**`, `.github/CODEOWNERS`, repo settings.

## Acceptance
- [ ] ADRs 0001–0018 and 0020 merged as Accepted; 0019 merged as Proposed
- [ ] Spec exported from the Claude Doc and committed as `docs/spec/nixie-vision-v0.1.md`
- [ ] `@OWNER` in `.github/CODEOWNERS` replaced
- [ ] Branch protection on `main`: PR required, CI required, Code Owner review required, DCO check required
- [ ] Project board created with columns Ready / In progress / In review / Done

## Notes
Anything you change in an ADR now is free. After this ticket closes, changes need a superseding ADR.
