# ADR-0016: Adopt leaf libraries, adapt engines behind interfaces, build only the core

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

Test for every candidate: *does adopting it force its data model, language or licence onto Nixie's core?* If yes, it sits behind an adapter or stays a reference.

- **Build:** data model and network compiler; geometry kernel and junction zone generator; interactive design editor; (later) native GPU simulation.
- **Adapt (separate process, adapter):** SUMO (first micro engine), AequilibraE (assignment, via Python bridge), LibreDWG, IfcOpenShell (sidecar/dynamic link).
- **Adopt (in core):** cch crate, geo/geozero crates, MapLibre GL, Automerge or Yrs, GMNS/OpenDRIVE/GTFS/OMX/UTDF as exchange formats.
- **Reference/interop only:** MATSim, LPSim, Conveyal R5, Streetmix (data import only), osm2streets (import only).

Engines consume the compiled analysis network through a stable interface; results map back **by stable ID**.

**Upstream first, fork by rule.** Contribute changes upstream. Fork only when upstream rejects a change Nixie depends on, goes quiet ~3 months on a blocking need, or changes licence. Every outside library sits behind a Nixie interface so swapping or forking is cheap.
