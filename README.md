# Nixie

Free, open-source transportation design and modelling suite built on one rule: **there is exactly one authored model of the street**, and every other representation (CAD linework, simulation network, HCM inputs, presentation map) is compiled from it.

- **Where we are:** [`ROADMAP.md`](ROADMAP.md)
- **Why things are the way they are:** [`docs/adr/`](docs/adr/README.md)
- **Working in this repo (humans and agents):** [`AGENTS.md`](AGENTS.md)
- **Original vision document:** [`docs/spec/`](docs/spec/README.md) (frozen snapshot; ADRs win where they differ)

Licensed under Apache-2.0. Every commit needs a DCO sign-off (`git commit -s`).

## First-time setup

1. `gh auth login`, then `python3 bootstrap/seed.py --dry-run` and `python3 bootstrap/seed.py` to create labels, milestones and the Phase 0 issues.
2. Delete `bootstrap/` in the next commit. From then on, work lives only in GitHub Issues.
