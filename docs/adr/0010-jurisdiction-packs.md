# ADR-0010: Jurisdiction packs, not hardcoding

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

Driving side, units, design vehicles, signal conventions, turn-on-red rules and design standards live in a **swappable bundle**. Ontario is the first pack. A **mirrored left-hand-traffic pack is a required test case** for every golden network.

## Enforcement

Every golden network has a mirrored LHT twin whose expected outputs are the mirror of the RHT outputs. Code reads driving side from the active pack; review rejects driving-side assumptions outside `jurisdiction` modules.
