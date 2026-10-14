# ADR-0011: SI units internally; time in seconds from a scenario epoch

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

All internal quantities are SI. Time is seconds from a scenario epoch. Conversions happen only at I/O edges.

## Enforcement

`nixie-units` provides newtypes (`Metres`, `Seconds`, `MetresPerSecond`, …). Public APIs in core crates take newtypes, not bare `f64`, for physical quantities.
