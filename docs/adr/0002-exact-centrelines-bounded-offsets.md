# ADR-0002: Exact centrelines, bounded offsets

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

An offset of a clothoid is not a clothoid, and variable-width strips along tapers have no closed form.

## Decision

- Horizontal alignment elements (line, circular arc, clothoid) are stored **analytically**.
- Lane edges and curb lines are **approximations** (biarcs or polylines) generated to a **stated named tolerance**, and each derived curve records the **achieved maximum error**.

## Consequences

Exports to CAD can emit true arcs/spirals for centrelines; offsets carry an error bound downstream consumers can check.

## Enforcement

Property tests in `nixie-geom` assert achieved error ≤ `tol::OFFSET_CAD` (ADR-0019) for sampled alignments, including clothoid–arc–clothoid sequences and tapers.
