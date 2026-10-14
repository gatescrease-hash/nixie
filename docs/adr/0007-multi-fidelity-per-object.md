# ADR-0007: Multi-fidelity per object

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

Each junction declares its fidelity: **macro node**, **HCM node** or **full micro geometry**. Lower fidelities derive from higher ones, so a town can be coarse with a few detailed intersections in one connected network.

## Consequences

Phase 0 only needs full micro for golden g01, but the schema field exists from v0 and the compiler must reject a fidelity it cannot yet produce rather than guess.
