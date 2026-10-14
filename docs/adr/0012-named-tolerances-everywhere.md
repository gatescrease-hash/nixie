# ADR-0012: Named tolerances everywhere

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

Every geometric and numeric comparison uses a **named project tolerance** from `nixie_units::tol`, never a literal epsilon. Values are set by ADR-0019.

## Enforcement

CI lint fails on float literals smaller than `1e-3` in comparisons outside `nixie-units` (simple regex check plus clippy `float_cmp` denied).
