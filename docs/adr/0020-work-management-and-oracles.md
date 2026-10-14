# ADR-0020: Work lives in issues; golden oracles are human-owned

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

Previous projects accumulated historical Markdown files (plans, status, gates) that bloated and drifted. This project will be built largely by AI agents pulling tickets.

## Decision

- **Work** is GitHub Issues grouped by Milestone (one per phase) and tracked on one Project board. No work-tracking Markdown in the repo.
- **Gates** are CI suites in `nixie-golden`, not documents.
- **Status** is read from milestones and CI. `ROADMAP.md` is edited only at gate transitions.
- **Decisions** are ADRs; accepted ADRs are never edited, only superseded.
- **Rolling-wave planning:** only the current phase is ticketed in detail.
- Tickets are one PR, one crate where possible, with acceptance criteria written as commands.
- **Golden expected outputs, tolerances and budgets are human-owned.** Agents never write both the code and the oracle that judges it.

## Enforcement

`CODEOWNERS` assigns `testdata/golden/**`, `docs/adr/**` and `crates/nixie-units/src/tol.rs` to maintainers; branch protection requires their review.
