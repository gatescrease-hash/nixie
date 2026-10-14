# ADR-0017: Apache-2.0 core, DCO sign-off, copyleft out of process

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

- Core licence: **Apache-2.0**, accepting that a vendor may build a closed fork in exchange for maximum adoption.
- Every contribution carries a **Developer Certificate of Origin** sign-off.
- Copyleft dependencies (GPL, EPL, AGPL) run **out of process**. LGPL (GEOS) only dynamically linked, native builds only.
- Nixie stays independent until proven (Gate 1 passed **and** two outside organisations using it), then moves to a foundation.

## Enforcement

`cargo deny` licence allow-list in CI. DCO check on PRs.
