# ADR-0004: Stable IDs anchored only to authored objects

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

Counts, signal timings and results must stay attached through redesigns.

## Decision

- Authored objects get random, permanent IDs.
- Derived IDs are a **hash of authored IDs plus a semantic role**, never of other derived objects.
- A **movement** is keyed by `(junction, approach leg, departure leg, turn type ∈ {left, through, right, U-turn})`. Adding a turn lane changes lane groups but **not** movement IDs.

## Enforcement

Golden test: adding a turn lane to golden g01 leaves every movement ID and attached count unchanged. Lint/review: no derived-ID constructor accepts a derived ID as input.
