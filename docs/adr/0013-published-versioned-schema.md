# ADR-0013: Published, versioned schema

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

The schema is an open specification with migrations and a conformance test suite, mapped explicitly to **IFC 4.3, OpenDRIVE, GMNS and GTFS** where they overlap. It is not pitched to a standards body until proven in real projects.

## Enforcement

Schema is generated from Rust types (JSON Schema) and committed; CI fails if the generated schema differs from the committed one without a version bump.
