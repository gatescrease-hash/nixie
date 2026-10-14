# ADR-0015: Native file is GeoPackage; the op log is authoritative

- **Status:** Accepted
- **Date:** 2026-10-13
- **Deciders:** @will

## Decision

- The project file is a **GeoPackage** (SQLite). The **operation log** inside it is authoritative.
- GIS tables are a **read-only projection** regenerated from the log and **checksummed**.
- Edits made outside Nixie (e.g. in QGIS) are detected on open and either imported as operations or rejected — never silently trusted.
- Server mode uses PostGIS with the same model.

## Enforcement

Test: tamper with a projection table, reopen, expect a detected-edit result.
