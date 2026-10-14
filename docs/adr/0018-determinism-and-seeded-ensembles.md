# ADR-0018: Determinism, stated honestly

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

- Counter-based RNG (e.g. Philox) seeded per agent: runs are **bit-identical on the same build and platform**.
- Across platforms and GPUs, floating-point summation order differs, so runs are **statistically reproducible** only.
- Results are reported as **seeded ensembles with confidence bands**, not single values.
- Ghost traffic (queue-based meso in the browser) is drawn visibly as ghost traffic and **never presented as an analysis result**.
