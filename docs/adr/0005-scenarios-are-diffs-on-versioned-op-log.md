# ADR-0005: Scenarios are diffs on a versioned operation log

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

- All state changes are operations in an append-only log. Scenarios, undo, audit and collaboration replay the same log.
- Every operation carries its **schema version**.
- When the schema bumps, **upcasters** migrate old operations. Periodic **snapshots** bound replay time.

## Enforcement

CI replays every historic operation fixture in `testdata/oplog/` through current upcasters on every PR. A schema bump without an upcaster fails CI.
