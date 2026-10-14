#!/usr/bin/env python3
"""
One-time bootstrap: create labels, milestones and the Phase 0 tickets as
GitHub issues, then delete this `bootstrap/` folder in the same PR.

Requires the GitHub CLI (`gh auth login`) and must run inside the repo
checkout (or pass --repo owner/name).

    python3 bootstrap/seed.py --dry-run      # print what would happen
    python3 bootstrap/seed.py                # do it

Ticket files: bootstrap/tickets/<phase>/<ID>-slug.md with front matter:

    ---
    id: P0-07
    title: "golden: harness crate and red Gate 0 suite"
    labels: [type:task, crate:golden, agent-ok, size:M]
    milestone: "Phase 0 — Vertical slice"
    blocked_by: [P0-06]
    ---
    body markdown, may mention other tickets as P0-xx

Pass 1 creates every issue (in ID order) and records ID -> issue number.
Pass 2 rewrites P0-xx references to #N and appends a "Blocked by" line.
Issues with no blockers get `ready`; the rest get `blocked`.
Re-running skips issues whose exact title already exists.
"""
import argparse, json, pathlib, re, subprocess, sys, tempfile

HERE = pathlib.Path(__file__).resolve().parent
REF = re.compile(r"\b(P\d+-\d{2})\b")


def sh(args, dry, capture=True):
    if dry:
        print("  $ " + " ".join(repr(a) if " " in a else a for a in args))
        return ""
    r = subprocess.run(args, text=True, capture_output=capture)
    if r.returncode != 0:
        sys.exit(f"command failed: {' '.join(args)}\n{r.stderr}")
    return r.stdout


def parse(path):
    text = path.read_text()
    m = re.match(r"^---\n(.*?)\n---\n(.*)$", text, re.S)
    if not m:
        sys.exit(f"{path}: missing front matter")
    meta = {}
    for line in m.group(1).splitlines():
        if not line.strip():
            continue
        k, v = line.split(":", 1)
        v = v.strip()
        if v.startswith("["):
            meta[k.strip()] = [x.strip().strip('"') for x in v[1:-1].split(",") if x.strip()]
        else:
            meta[k.strip()] = v.strip('"')
    meta.setdefault("blocked_by", [])
    meta["body"] = m.group(2).strip() + "\n"
    return meta


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--repo", help="owner/name (default: current checkout)")
    a = ap.parse_args()
    dry = a.dry_run
    R = ["--repo", a.repo] if a.repo else []
    api_repo = a.repo or "{owner}/{repo}"

    tickets = sorted((parse(p) for p in HERE.glob("tickets/*/*.md")), key=lambda t: t["id"])
    ids = {t["id"] for t in tickets}
    for t in tickets:
        for b in t["blocked_by"] + REF.findall(t["body"]):
            if b not in ids:
                sys.exit(f"{t['id']} references unknown ticket {b}")
    print(f"{len(tickets)} tickets")

    print("labels")
    for l in json.loads((HERE / "labels.json").read_text()):
        sh(["gh", "label", "create", l["name"], "--color", l["color"],
            "--description", l["description"], "--force", *R], dry)

    print("milestones")
    existing = set()
    if not dry:
        existing = {m["title"] for m in json.loads(sh(
            ["gh", "api", f"repos/{api_repo}/milestones?state=all&per_page=100"], dry))}
    for m in json.loads((HERE / "milestones.json").read_text()):
        if m["title"] in existing:
            continue
        sh(["gh", "api", f"repos/{api_repo}/milestones", "-f", f"title={m['title']}",
            "-f", f"description={m['description']}"], dry)

    have = {}
    if not dry:
        for i in json.loads(sh(["gh", "issue", "list", "--state", "all", "--limit", "1000",
                                "--json", "number,title", *R], dry)):
            have[i["title"]] = i["number"]

    print("pass 1: create issues")
    num = {}
    for n, t in enumerate(tickets, start=1):
        if t["title"] in have:
            num[t["id"]] = have[t["title"]]
            print(f"  skip {t['id']} (exists as #{num[t['id']]})")
            continue
        state = "blocked" if t["blocked_by"] else "ready"
        labels = ",".join(t["labels"] + [state])
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False) as f:
            f.write(t["body"])
        out = sh(["gh", "issue", "create", "--title", t["title"], "--body-file", f.name,
                  "--label", labels, "--milestone", t["milestone"], *R], dry)
        num[t["id"]] = int(out.strip().rsplit("/", 1)[-1]) if not dry else 1000 + n

    print("pass 2: link references")
    for t in tickets:
        body = REF.sub(lambda m: f"#{num[m.group(1)]}", t["body"])
        if t["blocked_by"]:
            refs = ", ".join(f"#{num[b]}" for b in t["blocked_by"])
            body += f"\n## Dependencies\n\n**Blocked by:** {refs}\n"
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False) as f:
            f.write(body)
        sh(["gh", "issue", "edit", str(num[t["id"]]), "--body-file", f.name, *R], dry)

    print("done. Now: create a Project board, add the Phase 0 milestone issues, "
          "and delete bootstrap/ in your next commit.")


if __name__ == "__main__":
    main()
