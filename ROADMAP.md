# Roadmap

One page. Updated **only** when a gate passes or a phase's scope changes (via ADR). Day-to-day status lives in the milestone pages and the Project board, never here.

**Current phase: 0 — Vertical slice**

| Phase | Goal | Gate (a CI suite, not a checklist) | Milestone |
| --- | --- | --- | --- |
| **0 Vertical slice** | One signalized intersection: authored model → lane graph → HCM → SUMO run, alignments out and back through LandXML, behind a deliberately plain UI. | **Gate 0:** slice runs end to end on golden intersection g01 and its mirrored LHT copy; budgets in ADR-0019 confirmed or revised. `cargo test -p nixie-golden --features gate0` | Phase 0 |
| 1 Foundation | Junction generator, scaled compiler, design editor, DXF/GMNS, OSM/ORN seeding. | **Gate 1:** alignments round-trip via LandXML within 1 mm across all golden networks; full-compile and recompile budgets hold. | Phase 1 |
| 2 Analysis | HCM at scale, UTDF timings, SUMO adapter hardened, TMC import, safety surrogates. | **Gate 2:** matches HCS/Synchro on pilot intersections. | Phase 2 |
| 3 Network modelling | AequilibraE assignment, trip generation, mode choice, GEH calibration, GTFS, ghost traffic. | **Gate 3:** pilot model meets GEH targets. | Phase 3 |
| 4 Native engines | GPU micro/meso, Civil 3D and OpenRoads add-ins, live collaboration. | **Gate 4:** native engine matches SUMO statistically. | Phase 4 |

## Planning rule (rolling wave)

Only the **current** phase has detailed tickets. Later phases exist as a milestone with the one-line goal above. When a gate is about to pass, the next phase is broken into epics and tickets — not before.

## Gate history

| Gate | Passed | Commit | Notes |
| --- | --- | --- | --- |
| 0 | — | — | — |
