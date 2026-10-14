# ADR-0003: Linear referencing with explicit anchoring

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Context

When an upstream curve is redesigned, objects downstream must either move with the road or stay put. Getting this implicitly wrong (driveways sliding along the road) destroys trust.

## Decision

Every located object declares an anchor:

- **station** — moves with the alignment;
- **absolute** — stays put and is re-projected;
- **snap-to-feature** — follows a junction, parcel access or another object.

Internally every position is **true distance along the alignment**. Station equations (chainage breaks) are first-class but are a **display and interop layer only**. Where an ahead station is lower than the back station a value occurs twice, so each typed station stores the **region** it belongs to.

Defaults (overridable per object):

| Object | Default anchor |
| --- | --- |
| Driveways, site access | Snap to parcel |
| Stop bars, crosswalks, signal poles | Snap to junction |
| Stop-line and advance detectors | Snap to stop bar at set upstream distance |
| Near/far-side transit stops | Snap to junction |
| Mid-block transit stops | Absolute |
| Stop and yield signs | Snap to junction |
| Other signs, trees, utilities | Absolute |
| Cross-section ranges, tapers, speed zones | Station |

An absolute object that ends up too far from a redesigned road, or whose snap target is deleted, is flagged and orphaned under ADR-0001.

## Enforcement

Type system: a located object cannot be constructed without an `Anchor`. Tests cover a station equation with a repeated station value.
