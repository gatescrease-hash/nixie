# Spec snapshot

Export the Claude Doc "Nixie: vision, data model and architecture" to Markdown and commit it here as `nixie-vision-v0.1.md`.

That file is a **frozen snapshot**. It is never edited after the first commit. Anything that changes a decision in it is an ADR in `docs/adr/`, and the ADR wins. This keeps one living source of truth per kind of information:

| Kind | Home | Lifespan |
| --- | --- | --- |
| Vision | this folder | frozen |
| Decisions | `docs/adr/` | append-only; superseded, never edited |
| Work | GitHub Issues + Milestones | closed when done |
| Gates and budgets | `testdata/golden/` + CI | executable |
| Status | `ROADMAP.md` + milestone pages | updated at gate transitions only |
