# AGENTS.md

How to work in this repo. Applies to humans and AI agents alike. Keep this file short; if it grows past ~150 lines, something belongs in an ADR instead.

## What Nixie is (two lines)

One authored, semantic model of the street (alignments, cross-section strips, junctions, controls). Everything else — lane graph, HCM lane groups, SUMO network, LandXML, map layers — is **compiled** from it and never edited directly.

## Non-negotiables

Read the ADR before touching anything it governs. Violating one is a failed PR regardless of tests.

| Rule | ADR |
| --- | --- |
| Users edit authored objects only; derived data is regenerated. Manual tweaks are stored as overrides, never dropped silently. | 0001 |
| Centrelines are exact (line/arc/clothoid); offsets are approximations with recorded error. | 0002 |
| Every located object declares an anchor (station / absolute / snap-to-feature). Internally, position = true distance. | 0003 |
| Derived IDs hash **authored IDs + semantic role**, never other derived objects. | 0004 |
| All state changes are versioned operations in the op log. | 0005 |
| SI units internally; conversions only at I/O edges. | 0011 |
| **No literal epsilons.** Every comparison uses a named tolerance from `nixie-units`. | 0012 |
| No C dependencies reachable from the `wasm32` build. | 0014 |
| No copyleft code linked into the core. GPL/EPL/AGPL tools run as separate processes. | 0016, 0017 |

## Repo map

```
crates/nixie-units        SI newtypes, named tolerances (leaf, no deps)
crates/nixie-geom         line/arc/clothoid kernel, offsets to tolerance
crates/nixie-schema       authored entities, stable IDs, anchors
crates/nixie-oplog        operations, replay, snapshots, upcasters
crates/nixie-compiler     incremental dependency graph, authored → derived
crates/nixie-junction     junction zones, movement connectors
crates/nixie-hcm          HcmEngine trait + adapters
crates/nixie-io-landxml   LandXML import/export
crates/nixie-adapter-sumo SUMO export, run, result mapping
crates/nixie-cli          `nixie check`, `nixie compile`
crates/nixie-golden       golden-network test harness and gate suites
crates/nixie-store        GeoPackage file (native only)
crates/nixie-wasm         browser bindings
web/                      plain viewer (TypeScript + MapLibre + WASM)
testdata/golden/          fixtures and expected outputs (HUMAN-OWNED)
docs/adr/                 decisions (append-only)
```

## Picking up work

1. Find work: open, unassigned, `ready`, `agent-ok`, in the current milestone.
   ```sh
   gh issue list --milestone "Phase 0 — Vertical slice" \
     --label ready --label agent-ok --search "no:assignee" \
     --json number,title,labels
   ```
   Prefer the lowest-numbered issue; numbering follows dependency order.
2. Claim it: `gh issue edit <n> --add-assignee @me` and comment `Claiming.`
3. Branch: `<n>-short-slug` (e.g. `14-clothoid-element`).
4. Stay inside the **Scope** section of the ticket. Need to touch something outside it? Stop and comment on the issue.
5. Done when every **Acceptance** command passes locally and in CI.
6. PR: title `<crate>: <what>`, body starts with `Closes #<n>`. Use the PR template.

## When stuck or when a decision is needed

Do **not** guess on design questions and do **not** write a design doc. Instead:

- Comment on the issue: what you tried, the exact question, the options you see.
- Add label `needs-human`, remove `agent-ok`, unassign yourself.
- If the answer is a lasting decision, a human will turn it into an ADR.

## Never

- Create new Markdown files outside `docs/adr/` (no notes, logs, plans, summaries, status files).
- Edit anything under `testdata/golden/**/expected/`. Agents may not be the oracle for their own correctness. If you believe an expected value is wrong, comment on the issue with evidence.
- Edit an accepted ADR. Propose a superseding one via the "ADR proposal" issue form.
- Loosen a tolerance or budget to make a test pass.
- Add a dependency without stating its licence in the PR body.
- Mark a test `#[ignore]` without linking the issue that will un-ignore it.

## Commands

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build -p nixie-compiler --target wasm32-unknown-unknown   # must stay C-free
cargo test -p nixie-golden --features gate0                     # Gate 0 suite
```

## Commits

Conventional-ish: `geom: add clothoid evaluation`. Every commit signed off (`git commit -s`) per the DCO (ADR-0017).
