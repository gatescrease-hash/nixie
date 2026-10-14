# Architecture Decision Records

Short, append-only records of decisions. An accepted ADR is **never edited**; to change a decision, write a new ADR that supersedes it and set the old one's status to `Superseded by ADR-XXXX` (the only permitted edit).

To propose one: open an issue with the "ADR proposal" form. A maintainer writes or accepts the ADR.

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-authored-vs-derived-with-override-policy.md) | Authored vs derived, with override policy | Accepted |
| [0002](0002-exact-centrelines-bounded-offsets.md) | Exact centrelines, bounded offsets | Accepted |
| [0003](0003-linear-referencing-and-anchoring.md) | Linear referencing with explicit anchoring | Accepted |
| [0004](0004-stable-ids-anchored-to-authored-objects.md) | Stable IDs anchored only to authored objects | Accepted |
| [0005](0005-scenarios-are-diffs-on-versioned-op-log.md) | Scenarios are diffs on a versioned op log | Accepted |
| [0006](0006-merge-is-not-validation.md) | Merge is not validation | Accepted |
| [0007](0007-multi-fidelity-per-object.md) | Multi-fidelity per object | Accepted |
| [0008](0008-time-varying-attributes.md) | Time-varying attributes from day one | Accepted |
| [0009](0009-2d-interface-2-5d-ready-data.md) | 2D interface, 2.5D-ready data | Accepted |
| [0010](0010-jurisdiction-packs.md) | Jurisdiction packs, not hardcoding | Accepted |
| [0011](0011-si-units-and-scenario-epoch.md) | SI units; time from scenario epoch | Accepted |
| [0012](0012-named-tolerances-everywhere.md) | Named tolerances everywhere | Accepted |
| [0013](0013-published-versioned-schema.md) | Published, versioned schema | Accepted |
| [0014](0014-rust-core-three-tiers.md) | Rust core, three tiers, pure Rust in browser | Accepted |
| [0015](0015-native-file-geopackage-oplog-authoritative.md) | GeoPackage file; op log authoritative | Accepted |
| [0016](0016-adopt-adapt-build-engines-behind-adapters.md) | Adopt / adapt / build; upstream first | Accepted |
| [0017](0017-licensing-apache-2-dco.md) | Apache-2.0, DCO, copyleft out of process | Accepted |
| [0018](0018-determinism-and-seeded-ensembles.md) | Determinism and seeded ensembles | Accepted |
| [0019](0019-tolerances-and-budgets-as-ci-assertions.md) | Tolerances and budgets as CI assertions | Proposed |
| [0020](0020-work-management-and-oracles.md) | Work in issues; human-owned oracles | Accepted |
