# ADR-0001: Authored vs derived data, with an override policy

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

Existing tool chains break where one representation is re-digitised into another (Civil 3D → VISSIM → Synchro). Nixie's reason to exist is to remove that seam. Users will still need to nudge compiled output.

## Decision

- Users edit **semantic (authored) objects only**. Lane polygons, simulation links, HCM lane groups and all other derived data are compiled.
- A manual tweak to derived data is stored as an **authored override** that targets authored anchors, not derived objects.
- When a topology change removes an override's target, the compiler **rebases** it if exactly one successor exists; otherwise it **orphans and flags** it for review.
- Overrides are **never silently dropped**.

## Consequences

Derived data can be deleted and regenerated at any time. Every derived type needs a clear "successor" rule for rebasing.

## Enforcement

Property test in `nixie-compiler`: for random edit sequences, the count of overrides in = rebased + orphaned + still-attached. No derived type exposes a public mutator.
