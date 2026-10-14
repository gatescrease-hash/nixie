# Nixie: vision, data model and architecture

Oct 13, 2026 · @will

Nixie is a free, open-source transportation design and modelling suite built on one rule: there is exactly one authored model of the street, and every other representation (CAD linework, the simulation network, HCM inputs, the presentation map) is compiled from it. Existing tool chains break at that seam (Civil 3D drawing, re-digitised into VISSIM, re-keyed into Synchro), so that is where Nixie wins or loses.

The data model and core architecture below are meant to be fixed now. Everything else is blue-sky and can be phased. The open-source review concludes that Nixie should adopt standards and leaf libraries freely, run SUMO and AequilibraE behind adapters for the first years, and build only its data model, compiler, geometry kernel and design editor from scratch. Work starts with a one-intersection vertical slice that proves the compiler before the editor is built.

## Vision and scope

A town should be able to build its whole road network in one afternoon, with high detail only where it chooses, and have it all stay connected. Nixie targets municipalities and private consultancies, designed around Ontario practice but configurable for any jurisdiction.

- **Design:** CAD-grade geometry and design controls with a game-like, plug-and-play interface for non-designers.
- **Interop:** import and export with AutoCAD, Civil 3D, MicroStation/OpenRoads; cross-sections importable from Streetmix.
- **Presentation:** simple, configurable colour schemes over aerial imagery.
- **Analysis:** microsimulation (SUMO/VISSIM class), deterministic capacity analysis (Synchro class), meso and macro models, roundabouts, transit and all modes, on the same network.
- **Demand:** trips from individual addresses and driveways, easy import of trip generation, counts and signal timings, automatic multimodal distribution and mode choice.
- **Performance:** the fastest credible routing and simulation methods available, accelerated on modern CPUs and GPUs.
- **2D first:** a 2D interface throughout (see principle 7 on keeping data 2.5D-ready).

## Core principle: one model, many compilations

_Diagram omitted from this Markdown snapshot: core architecture · one authored model, compiled outputs._

Only the highlighted model is ever edited. The compiler regenerates everything below it in milliseconds, and an external engine such as SUMO consumes the analysis network through an adapter, with results mapped back by stable ID. Safety surrogates, quantities and HD maps are further compile targets of the same model.

## Data model principles

These thirteen rules are fixed from day one; changing any of them later means a schema break. Each comes with a policy for the case that breaks naive versions of it.

1. **Authored vs derived, with an override policy.** Users only edit semantic objects; lane polygons, simulation links and HCM lane groups are compiled. A manual tweak to derived data is stored as an authored override that targets authored anchors. When a topology change removes its target, the compiler rebases it if exactly one successor exists, otherwise orphans it and flags it for review. Overrides are never silently dropped.
2. **Exact centrelines, bounded offsets.** Alignments (lines, arcs, clothoids) are stored analytically. An offset of a clothoid is not a clothoid, and variable-width strips along tapers have no closed form, so lane edges and curb lines are approximations (biarcs or polylines) generated to a stated tolerance, with the achieved error recorded.
3. **Linear referencing with explicit anchoring.** Every located object declares an anchor: *station* (moves with the alignment), *absolute* (stays put and is re-projected) or *snap-to-feature* (follows a junction, parcel access or another object). Driveways default to snap-to-parcel, so fixing an upstream curve does not slide them. Station equations (chainage breaks) are first-class, but only as a display and interop layer: internally every position is true distance along the alignment. Where an ahead station is lower than the back station, a station value occurs twice, so each typed station stores which region it belongs to. Default anchors per object type are listed below.
4. **Stable IDs anchored only to authored objects.** Derived IDs hash authored IDs plus a semantic role, never other derived objects. A movement is (junction, approach leg, departure leg, turn type: left, through, right, U-turn), which is how counts are collected. Adding a turn lane changes lane groups but not movement IDs, so counts and timings stay attached.
5. **Scenarios are diffs on a versioned operation log.** Every operation carries its schema version. Upcasters migrate old operations when the schema bumps, and periodic snapshots bound replay time.
6. **Merge is not validation.** CRDT merging guarantees that copies converge, not that the result is buildable. Every merged state passes through the compiler's validator; invalid results (e.g. an impossible junction) become conflicts handled in three tiers: auto-heal what can safely be regenerated (logged); offer a one-click keep mine, keep theirs or edit choice on the map; otherwise flag to the scenario owner. The person whose merge caused a conflict is prompted first. Analysis may run on a flagged scenario: flagged objects use their last valid compile, results are stamped "Provisional: N unresolved conflicts", and provisional results cannot be exported as final reports or pass nixie check.
7. **Multi-fidelity per object.** Each junction declares its fidelity: macro node, HCM node or full micro geometry. Lower fidelities derive from higher ones, so a town can be coarse with a few detailed intersections in one connected network.
8. **Time-varying attributes.** Strips, lanes and controls carry schedules by time of day, day of week and season: peak parking bans, reversible lanes, flex curbs, reduced winter widths and work-zone stages. This maps to GMNS time-of-day tables and must be in the schema from the start.
9. **2D interface, 2.5D-ready data.** The UI stays 2D, but alignments reserve an optional vertical profile and cross-slope for truck performance, sight distance and Civil 3D round-tripping.
10. **Jurisdiction packs, not hardcoding.** Driving side, units, design vehicles, signal conventions, turn-on-red rules and design standards live in a swappable bundle. Ontario is the first pack; a mirrored left-hand-traffic pack is a required test case.
11. **SI units internally; time in seconds from a scenario epoch.** Conversions only at the edges.
12. **Named tolerances everywhere.** Every geometric and numeric comparison uses a named project tolerance (see Tolerances, budgets and test networks), never a literal epsilon.
13. **Published, versioned schema.** An open specification with migrations and a conformance test suite, mapped explicitly to IFC 4.3, OpenDRIVE, GMNS and GTFS where they overlap.

### Anchoring defaults

Every object can override its default. If an absolute object ends up too far from a redesigned road, or a snap target is deleted, the object is flagged and orphaned under the same policy as overrides.

| Object | Default anchor | Why |
| --- | --- | --- |
| Driveways, site access | Snap to parcel | They exist where the property is |
| Stop bars, crosswalks, signal poles | Snap to junction | They belong to the intersection |
| Stop-line and advance detectors | Snap to stop bar, at a set distance upstream | Signal timing depends on that distance |
| Near-side and far-side transit stops | Snap to junction | Defined relative to the intersection |
| Mid-block transit stops | Absolute | Physical locations moved only on purpose |
| Stop and yield signs | Snap to junction | Tied to its control |
| Other signs, trees, utilities | Absolute | Physically placed |
| Cross-section ranges, tapers, speed zones | Station | Inherently defined along the road |

Station equations are imported from LandXML, entered by clicking a point on the alignment and typing the back and ahead values, or edited in a table view.

## Data model entity layers

Nine authored layers, each mapped to the open standard it should interoperate with.

| Layer | Key entities | Maps to |
| --- | --- | --- |
| Reference | CRS, local origin, imagery sources, background CAD/GIS, survey control | EPSG / PROJ |
| Alignment | Horizontal elements (line, arc, clothoid), stationing with station equations, optional vertical profile | IFC 4.3 IfcAlignment, LandXML, OpenDRIVE reference line |
| Corridor | Cross-section templates as ordered strips (lane, bike lane, buffer, curb, boulevard, sidewalk, median), station ranges, transitions, time-of-day and seasonal schedules | IFC 4.3 road parts, Streetmix segments, OpenDRIVE lane sections |
| Junction | Intersections, roundabouts, merges/diverges, crossings, driveways; authored legs; movements keyed by leg pair and turn type | OpenDRIVE junctions, Lanelet2 regulatory elements |
| Control | Signal controllers (NEMA ring-barrier, plans, coordination, detectors, preemption, TSP), stop/yield, Ontario PXOs, time-of-day plans, work-zone control stages | Synchro UTDF, NTCIP, SUMO tlLogic |
| Transit | Routes, patterns, stops, schedules | GTFS, GTFS-Realtime |
| Land use and demand | Sites, parcels, access points (edge + station), TAZs, trip rates, OD matrices by mode and period, agent plans | GMNS zones, OMX matrices, MATSim plans |
| Observations | Turning movement counts, ATR counts, travel times, speeds, high-resolution signal logs | Miovision exports, ATSPM event logs |
| Results | Per scenario, run and seed; keyed to stable IDs; stored apart from the model | Parquet / Arrow |

Derived (never edited) objects: offset curves to tolerance, junction zones and curb returns, lane graph, conflict zones, analysis links and nodes, HCM lane groups, quantities, safety surrogates, and the read-only GIS projection tables.

## System architecture

One Rust core runs in three tiers, and the split is explicit: the browser edits and analyses, while desktop and server also run the external engines.

| Tier | Runs | Geometry and CRS |
| --- | --- | --- |
| Browser (WASM) | Editor, compiler, validator, HCM, ghost traffic (queue-based meso), presentation | Pure-Rust geometry; CRS grid shifts precomputed per project |
| Desktop (Tauri) | Everything in the browser, plus SUMO, Python (AequilibraE), CAD converters, GPU compute | PROJ with NTv2 grids, GEOS |
| Server (headless) | Desktop engines, multi-user PostGIS, ensemble batches, `nixie check` in CI | PROJ, GEOS |

- **Core in Rust, pure Rust in the browser.** C libraries (PROJ, GEOS, IfcOpenShell) are linked only in native builds; the wasm32 build stays C-free.
- **Incremental compiler:** a dependency graph from authored to derived objects, so moving one curb return recompiles one junction. Its budgets are set in Tolerances, budgets and test networks.
- **Native file = GeoPackage, with the GIS tables as a read-only projection.** The operation log is authoritative. Projection tables are regenerated from it and checksummed; edits made outside Nixie (e.g. in QGIS) are detected on open and either imported as operations or rejected, never silently trusted.
- **Operation log with a validation gate:** scenarios, undo, audit and CRDT collaboration replay the same log, and every merge passes the validator (principle 6).
- **Frontend: one rendering path.** TypeScript UI over MapLibre GL (WebGL2), with Nixie's layers drawn into the same context through custom layers from buffers produced by the WASM core. WebGPU is revisited only when the map stack supports it; wgpu is used for GPU compute on desktop and server, not for UI.
- **Extensibility:** Python bindings for analysts and notebooks; WASM plugins for behaviour models, mode choice models and design checks.
- **Determinism, stated honestly:** counter-based random numbers (e.g. Philox) seeded per agent make runs bit-identical on the same build and platform. Across platforms and GPUs, floating-point summation order differs, so runs are statistically reproducible. Results are therefore reported as seeded ensembles with confidence bands, not single values.
- **Engine boundary:** engines consume the compiled analysis network through a stable interface, so SUMO or MATSim can sit behind an adapter.
- **Headless CLI:** `nixie check` runs the compiler and design-standard checks without a UI, so an agency can fail a submission automatically when a taper is short or a sight triangle is blocked.

## Known hard problems

Six problems will consume a disproportionate share of effort; each has a policy now so it is budgeted rather than discovered.

| Problem | Why it is hard | Policy |
| --- | --- | --- |
| Junction zone generation | Curb returns between offset curves from different alignments, conflict zones and movement paths. This is where A/B Street, netedit and RoadRunner all struggle, and it is roughly half the compiler. | Its own workstream from Phase 0, a golden junction library, and authored overrides as the escape hatch. |
| Offset geometry | Clothoid offsets and variable-width tapers have no closed form. | Exact centrelines only; offsets approximated to tolerance with recorded error (principle 2). |
| Overrides after topology change | Deleting a lane can remove an override's target. | Rebase, orphan or flag; never drop (principle 1). |
| Merge validity | CRDTs converge but can produce impossible geometry. | Validator gate and a conflict-resolution view (principle 6). |
| Operation-log migration | Every schema bump must still replay years of history. | Upcasters, snapshots, and a CI test that replays every historic operation. |
| Rust to WASM with C libraries | C toolchains in wasm32 are painful, and NAD83(CSRS) transforms need grid files. | Pure Rust in the browser; PROJ and GEOS only in native builds. |

## Tolerances, budgets and test networks

These are proposed targets, to be confirmed or revised during Phase 0; once set, they become CI assertions.

| Quantity | Proposed target | Reason |
| --- | --- | --- |
| Offset curve vs true offset (CAD output) | 1 mm | Below construction and survey precision |
| Display tessellation error | 0.5 px at current zoom | Invisible on screen |
| Coordinate resolution | 0.1 mm via local origin | Doubles stay precise across a municipality |
| Single-junction incremental recompile | under 50 ms | Drag edits stay interactive |
| Full compile, 1,000-junction town | under 10 s | Whole-network edits stay practical |
| Editor frame rate while dragging | 60 fps | Game-like feel |
| Ghost traffic response after an edit | under 1 s | Queue reaction feels live |
| HCM results | Reproduce published HCM example problems to reported precision | Agency acceptance |

**Golden test networks**, each stored with expected compiler output, HCM results and LandXML round-trip results, and run in CI:

- One signalized intersection (the Phase 0 slice).
- Single-lane and two-lane roundabouts.
- A skewed, dog-leg intersection with channelized right turns.
- A divided arterial with tapers, driveways and a station equation.
- Diamond interchange ramp terminals.
- A mirrored left-hand-traffic copy of each.
- A small Ontario town network from the pilot partner.

## Blue-sky feature set

### Design

- Paint roads like a city-builder game, but with real alignment geometry: snap to aerials, drag lane strips, pull curb radii.
- Live design checks like a spell-checker: radius below minimum for design speed (TAC Geometric Design Guide), short tapers, blocked sight triangles, AODA crossing requirements.
- Swept-path analysis with TAC design vehicles (WB-20, buses, fire trucks) built in.
- Cross-section editor with Streetmix import; strips carry width, mode permissions, direction and surface.

### CAD interop

- DXF, LandXML and IFC 4.3 export with true arcs and spirals, mapped to configurable municipal or MTO layer standards.
- Civil 3D corridors and OpenRoads objects do not survive neutral formats; they explode into plain linework. True round-tripping needs Nixie add-ins for Civil 3D (.NET API) and OpenRoads, built in-house in Phase 4.

### Simulation on one network, all resolutions

- **Micro:** IDM, Wiedemann and Krauss car-following, MOBIL-style lane changing, social-force pedestrians, cyclists, transit dwell; GPU-accelerated.
- **Meso:** queue-based or cell transmission models for the wider network, running hybrid with micro in the study area.
- **Macro:** bush-based user equilibrium (Algorithm B / TAPAS family) for tight convergence, plus dynamic traffic assignment. Customizable Contraction Hierarchies for road routing (link costs change every iteration); RAPTOR for transit.
- **Deterministic:** HCM 7 signalized, TWSC, AWSC and roundabout methods, with Synchro-style timing optimization.

### Demand

- Sites and driveways generate trips from rates the user imports. ITE data is proprietary, so Nixie ships the machinery, not the numbers.
- TTS survey data as the Ontario anchor for distribution and mode split.
- Nested logit mode choice with skims from the network itself, including bike Level of Traffic Stress computed from cross-section strips. A painted bike lane changes mode share automatically.

### Calibration

- Import Miovision TMCs, Synchro UTDF timings and ATSPM logs.
- Automatic OD matrix estimation and behaviour parameter tuning against GEH targets; one-click calibration report for agency submission.

### Network seeding

- Bootstrap from the Ontario Road Network or OpenStreetMap with inferred cross-sections, then upgrade chosen intersections to full detail.

Three of the ideas below touch the architecture and are protected now: time-varying cross-sections (principle 8), the safety compile target, and always-on ghost traffic (browser tier). The rest can land later without schema changes.

### Streets as code

- **Pull requests for streets:** a visual red/green diff between scenarios, review comments pinned at (alignment, station), and merge for approved Class EA options.
- **Standards as automated tests:** `nixie check` fails a submission when a design check fails, so agencies can review in CI.
- **AI agent as a collaborator:** a request like "add a protected bike lane from Main to King" becomes a branch of operations that a person reviews like any other changeset.
- **Bidirectional edits (moonshot):** change a lane group in an HCM view and Nixie infers the authored change. Until then, overrides cover this.

### Design by intent

- **Declared geometry:** state "WB-20 turns right from lane 2 with no encroachment" and a constraint solver produces the minimum curb radius. Swept path becomes an input, not an after-the-fact check.
- **Generative design:** fast deterministic runs make thousands of cross-section and timing variants affordable, shown as a Pareto front across delay, safety, bike LTS and cost.

### Ghost traffic and ensembles

- **Always-on ghost traffic (protected):** low-fidelity traffic runs continuously in the editor, so dragging a curb shows the queue react, like hot reload. The browser runs a simple queue-based meso model. It shows queues and spillback well but not lane changes, weaving or pedestrian conflicts, so it is drawn visibly as ghost traffic and never presented as an analysis result. Desktop can upgrade it to micro.
- **Results as distributions:** seeded ensembles report LOS and delay with confidence bands instead of a single letter.
- **Digital twin:** ingest ATSPM logs nightly, recalibrate automatically, and flag intersections drifting from the model.

### More compile targets

- **Safety (protected):** SSAM-style conflict detection from micro trajectories and HSM crash prediction from geometry, compiled from the same model. Vision Zero-ready out of the box.
- **Quantities and cost:** strips already carry surface type, so they compile into Class D quantity and cost estimates; emissions and noise follow the same pattern.
- **Construction staging:** work zones as time-bound scenario diffs that compile into Ontario Traffic Manual Book 7 traffic control plans.
- **Public engagement:** a shareable link where residents flip between options and comment on a spot, with a simple street-level view generated from cross-sections.
- **Equity:** RAPTOR plus walking isochrones over census data show who gains access to jobs in each scenario.

### Ecosystem

- **Jurisdiction pack registry:** community-maintained packs with design checks shipped as WASM plugins, like a package manager for standards.
- **HD map export:** OpenDRIVE and Lanelet2 output for autonomous-vehicle testing (e.g. CARLA), a second market and possible funding source.
- **Schema as a standard:** propose it to Zephyr or buildingSMART as the missing semantic street model, with the conformance suite, once the structure and system are proven in practice.

## Open-source landscape: adopt, adapt or build

Borrow standards and leaf libraries freely; adapt mature engines behind interfaces; build only what defines Nixie. The test for each candidate: does adopting it force its data model, language or license onto Nixie's core? If yes, it sits behind an adapter or stays a reference.

| Subsystem | Candidate | Licence | Verdict | Reasoning |
| --- | --- | --- | --- | --- |
| Microsimulation | [Eclipse SUMO](https://sumo.dlr.de/pydoc/sumolib.files.html) | EPL-2.0 or GPL-2.0+ | Adapt (first engine) | Mature, active, all modes. But C++, CPU-bound and has its own network model, so it runs behind an export + TraCI adapter, not as the core. A [pure-Rust TraCI client](https://docs.rs/crate/traci-rs/latest) exists. |
| GPU microsimulation | [LPSim](https://arxiv.org/html/2406.08496v1) | Research code | Reference | Proves regional GPU micro is feasible; learn its partitioning, don't ship it. |
| Static assignment and skims | [AequilibraE](https://github.com/AequilibraE/AequilibraE) | Permissive custom | Adopt early | Full Python modelling package with OMX support and a QGIS plugin. Use through a Python bridge first; port hot paths to Rust later. |
| Road routing | [cch crate](https://docs.rs/crate/cch/latest) (from [RoutingKit](https://github.com/RoutingKit/RoutingKit)) | MIT (BSD-2 origin) | Adopt | Pure-Rust Customizable Contraction Hierarchies, at parity with RoutingKit. Young, so plan to contribute upstream. |
| Transit routing | [Conveyal R5](https://awesome.ecosyste.ms/projects/github.com%2Fconveyal%2Fr5) | MIT | Reference / interop | Java engine that already does transit, bike LTS and accessibility. RAPTOR itself is compact enough to build natively in Rust. |
| Activity-based demand | [MATSim](https://arxiv.org/pdf/2102.07505) | GPL-2.0+ | Interop only | Strong agent replanning loop, but Java and copyleft. Export plans; don't link. |
| HCM methods | [transportations-library](https://pypi.org/project/transportations-library) | Not stated on PyPI; confirm | Evaluate and contribute | Rust core with Python and WASM bindings covering HCM 7 and 7.1 chapters 10 to 24. Single maintainer and young: validate against HCS before relying on it. |
| OSM lane geometry | [osm2streets](https://github.com/a-b-street/osm2streets) | Confirm | Adopt for import only | Rust, produces lane and intersection polygons from OSM. Polyline-based, and its own [geom crate](https://github.com/a-b-street/geom) warns against reuse, so it is not the geometry kernel. |
| Network exchange | [GMNS](https://www.github.com/zephyr-data-specs/GMNS) | Open spec | Adopt as exchange format | FHWA/Zephyr standard with lane, movement and signal timing tables; tooling includes UTDF and OSM converters. |
| IFC 4.3 | [IfcOpenShell](https://www.sourcepulse.org/projects/1481160) | LGPL-3.0+ | Adopt (dynamic link or sidecar) | Parses IFC up to IFC4x3 Add2 and has an active alignment API. |
| DWG | [LibreDWG](https://www.GNU.org/software/libredwg/manual/LibreDWG.pdf) | GPL-3.0+ | Optional separate converter | Reads nearly all DWG versions but writes r2000 by default. GPL-3 would infect the core, so run it as a separate process. Commercial ODA libraries stay out of core. |
| Cross-sections | [Streetmix](https://streetmix.readthedocs.io/en/latest/support/faq/) | AGPL-3.0 code, CC-BY-SA content | Import data only | Read its street JSON through the API; never embed its code or artwork in the core. |
| Geo foundation | PROJ, GDAL, GEOS, georust crates | MIT / LGPL / MIT-Apache | Adopt | Industry standard; native builds only, with pure-Rust equivalents in the browser. |
| Map and rendering | MapLibre GL, wgpu | BSD / MIT-Apache | Adopt | Basemaps and aerials share one WebGL2 context with Nixie's layers; wgpu is for desktop and server compute only. |
| Collaboration | Automerge or Yjs (Yrs) | MIT | Adopt | CRDT engine under the operation log. |

### What Nixie must build itself

- **The data model and network compiler.** Nothing open source unites analytic alignments, cross-section strips, junction movements and multi-fidelity analysis networks. This is Nixie's reason to exist.
- **The geometry kernel and junction zone generator.** Existing Rust geometry is polyline-first, and junction generation is the hardest part of the compiler (see Known hard problems).
- **The interactive design editor.** Streetmix, A/B Street and netedit each solve a slice; none is CAD-grade.
- **The native GPU simulation engine (later).** Deterministic, multi-resolution and fed directly by the compiler. SUMO covers the gap until then.

### Upstream first, fork by rule

When Nixie needs something an adopted project lacks, it contributes the change upstream rather than copying the code and maintaining a private version.

- **cch crate:** contribute pull requests (developer time) rather than money at first, since the crate is young.
- **GMNS:** Nixie's schema is authoritative and GMNS is an export. Lane geometry, movement and time-of-day extensions are proposed upstream once Nixie has proven them.
- **Fork only when** upstream rejects a change Nixie depends on, goes quiet for about three months on a blocking need, or changes its licence.
- **Every outside library sits behind a Nixie interface**, so swapping or forking later is cheap.

### The blue-sky tradeoff

Adopting whole engines (SUMO, MATSim) buys years of validated behaviour but imports foreign network models, which re-creates the very seam Nixie exists to remove. The adapter pattern keeps the seam one-way and automatic: Nixie compiles, the engine consumes, results map back by stable ID. Leaf libraries (PROJ, CCH, IfcOpenShell) carry no such cost and should be adopted without hesitation. Using known projects also helps contributors: engineers who know SUMO or AequilibraE can work on Nixie on day one.

## Recommended stack and licensing

Nixie's core is licensed under Apache-2.0, with every copyleft dependency kept out of process. That lets municipalities and private firms embed, extend and redistribute Nixie freely, which drives adoption.

| Layer | Choice | Licence boundary |
| --- | --- | --- |
| Core language | Rust (native + WASM) | In core |
| Geometry and CRS | georust + custom arc/spiral kernel (all tiers); PROJ, GEOS (native only) | In core; GEOS (LGPL) dynamically linked in native builds only |
| Storage | GeoPackage (SQLite) + PostGIS server mode | In core |
| Routing | cch crate; native RAPTOR | In core |
| Assignment | AequilibraE via Python bridge, then Rust port | Separate Python process |
| Microsimulation | SUMO adapter now; native GPU engine later | Separate process (EPL/GPL) |
| HCM | transportations-library if licence and validation pass; else native | To confirm |
| CAD | DXF and LandXML native; IfcOpenShell sidecar; LibreDWG converter | IFC sidecar (LGPL); DWG separate process (GPL) |
| Exchange formats | GMNS, OpenDRIVE, GTFS, OMX, UTDF | Specs, no code licence |
| Frontend | TypeScript, MapLibre GL (WebGL2) with Nixie custom layers, Tauri desktop; wgpu for native GPU compute only | In core |
| Collaboration | Automerge or Yrs CRDT | In core |
| Scripting | PyO3 Python bindings; WASM plugins | In core |

Apache-2.0 accepts that a vendor may build a closed fork, in exchange for maximum adoption. Every contribution carries a Developer Certificate of Origin sign-off, so ownership of the code stays clean. Nixie stays an independent project until it is proven (Gate 1 passed and two outside organizations using it), then moves to a foundation.

## Phasing and reality check

_Diagram omitted from this Markdown snapshot: phased roadmap · 5 phases, 5 gates._

Phase 0 is the smallest slice that proves the compiler: one signalized intersection from authored model through lane graph, HCM analysis and a SUMO run, with alignments out and back through LandXML, all behind a deliberately plain UI. The game-like editor waits until the compiler passes its golden tests.

The full scope is several SUMO-sized projects, and Phase 1 alone is multi-year; the fixed data model is what makes phasing safe. Adopting SUMO and AequilibraE behind adapters pulls simulation and assignment forward by years. Gate 1 tests only what can truly round-trip, alignments via LandXML; full corridor round-tripping waits for the Civil 3D and OpenRoads add-ins in Phase 4.

Accuracy comes mostly from behaviour models and calibration, not compute. Speed pays off by allowing more calibration iterations and more scenarios per study.

## Decisions and open questions

Nine questions are decided; three are deferred until a pilot partner is in place.

| Question | Decision | Detail |
| --- | --- | --- |
| Core licence | Apache-2.0 | Maximum adoption; DCO sign-off on every contribution (Recommended stack and licensing) |
| Governance | Independent until proven | Move to a foundation after Gate 1 passes and two outside organizations use Nixie |
| Upstream work vs forking | Upstream first, fork by rule | Policy under Open-source landscape |
| Civil 3D and OpenRoads add-ins | In-house, Phase 4 | Not needed for Gate 1, which tests LandXML alignments only |
| Tolerances and budgets | Confirmed | Frozen as CI assertions after Phase 0 |
| Default anchoring and station equations | Per-object defaults; true distance internally | Anchoring defaults table and principle 3 |
| Merge conflicts | Three tiers, least painful first; analysis allowed on flags | Principle 6; flagged results are stamped provisional and cannot be exported as final |
| Ghost traffic model | Simple queue-based meso | Shows queues and spillback only; drawn as ghost traffic, never as analysis |
| Pitching the schema | After the system is proven | Not before the structure holds up in real projects |

**Deferred:**

- [ ] Confirm licences for osm2streets and transportations-library, and validate the latter against HCS example problems.
- [ ] Seed partner: which Ontario municipality or consultancy pilots Phase 0 and Phase 1?
- [ ] Ontario Road Network licence terms for redistribution of seeded networks.
