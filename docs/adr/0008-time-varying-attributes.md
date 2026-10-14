# ADR-0008: Time-varying attributes are in the schema from day one

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

Strips, lanes and controls carry **schedules** by time of day, day of week and season: peak parking bans, reversible lanes, flex curbs, reduced winter widths, work-zone stages. Maps to GMNS time-of-day tables.

## Consequences

Phase 0 may compile only the "always" schedule, but the type exists in v0 so adding schedules later is not a schema break.
