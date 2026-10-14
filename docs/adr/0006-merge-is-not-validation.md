# ADR-0006: Merge is not validation

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

CRDT merging guarantees convergence, not buildability. Two valid edits can merge into an impossible junction.

## Decision

Every merged state passes the compiler's validator. Invalid results become conflicts, handled least-painful first:

1. **Auto-heal** what can safely be regenerated (logged).
2. Offer a one-click **keep mine / keep theirs / edit** on the map; the person whose merge caused the conflict is prompted first.
3. Otherwise **flag** to the scenario owner.

Analysis may run on a flagged scenario: flagged objects use their last valid compile, results are stamped **"Provisional: N unresolved conflicts"**, and provisional results cannot be exported as final reports or pass `nixie check`.

## Enforcement

`nixie check` exits non-zero on any provisional result. Report exporters refuse provisional input (tested).
