//! Gate 0 (ROADMAP.md): the Phase 0 slice runs end to end on golden g01 and
//! its mirrored LHT twin.
//!
//! The gate is red until every fixture below exists and every stage is
//! wired. Each red item names what is missing and the issue that owns it.
//! A ticket that delivers a stage replaces its `run: None` with a function
//! that drives the stage on a fixture and compares against its expected
//! files.

use std::fmt;
use std::path::Path;

use crate::fixture::{Fixture, FixtureError};
use crate::tolerance::Tolerances;

/// A golden fixture the gate runs.
#[derive(Debug, Clone, Copy)]
pub struct FixtureReq {
    pub name: &'static str,
    pub owner: &'static str,
}

/// Runs a stage on one fixture. `Err` carries a readable reason, usually a
/// formatted [`Diff`](crate::Diff).
pub type StageFn = fn(&Fixture, &dyn Tolerances) -> Result<(), String>;

/// A stage of the slice the gate exercises.
#[derive(Debug, Clone, Copy)]
pub struct StageReq {
    pub name: &'static str,
    pub owner: &'static str,
    /// `None` until the owning ticket wires the stage in.
    pub run: Option<StageFn>,
}

pub const FIXTURES: &[FixtureReq] = &[
    FixtureReq {
        name: "g01-signalized",
        owner: "#3",
    },
    FixtureReq {
        name: "g01-signalized-lht",
        owner: "#25",
    },
];

pub const STAGES: &[StageReq] = &[
    StageReq {
        name: "input: replay authored op log",
        owner: "#15",
        run: None,
    },
    StageReq {
        name: "movements and lane graph",
        owner: "#19",
        run: None,
    },
    StageReq {
        name: "HCM lane groups",
        owner: "#20",
        run: None,
    },
    StageReq {
        name: "HCM signalized analysis",
        owner: "#21",
        run: None,
    },
    StageReq {
        name: "LandXML alignment round trip",
        owner: "#22",
        run: None,
    },
    StageReq {
        name: "SUMO network export",
        owner: "#23",
        run: None,
    },
    StageReq {
        name: "SUMO run and result mapping",
        owner: "#24",
        run: None,
    },
    StageReq {
        name: "single-junction recompile budget",
        owner: "#26",
        run: None,
    },
];

/// The project tolerance registry the gate judges fixtures with.
///
/// `None` until `nixie_units::tol` exists (#8) and is wired in here.
pub fn tolerances() -> Option<&'static dyn Tolerances> {
    None
}

/// Why the gate is red. Empty means green.
#[derive(Debug, Clone, Default)]
pub struct Report {
    pub items: Vec<String>,
}

impl Report {
    pub fn is_green(&self) -> bool {
        self.items.is_empty()
    }
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_green() {
            return write!(f, "Gate 0 is green");
        }
        write!(f, "Gate 0 is red: {} item(s)", self.items.len())?;
        for (i, item) in self.items.iter().enumerate() {
            write!(f, "\n  {}. {}", i + 1, item.replace('\n', "\n     "))?;
        }
        Ok(())
    }
}

/// Evaluates Gate 0 against the fixtures under `root`.
pub fn evaluate(root: &Path) -> Report {
    evaluate_with(root, FIXTURES, STAGES, tolerances())
}

/// [`evaluate`] with every input explicit, for testing the gate itself.
pub fn evaluate_with(
    root: &Path,
    fixtures: &[FixtureReq],
    stages: &[StageReq],
    tolerances: Option<&dyn Tolerances>,
) -> Report {
    let mut items = Vec::new();

    let mut loaded = Vec::new();
    for req in fixtures {
        let fixture = Fixture::load(root, req.name).and_then(|f| {
            f.check_expected_files()?;
            Ok(f)
        });
        match fixture {
            Ok(fixture) => loaded.push(fixture),
            Err(FixtureError::Missing { .. }) => {
                items.push(format!(
                    "fixture {} missing (owned by {})",
                    req.name, req.owner
                ));
            }
            Err(e) => items.push(format!(
                "fixture {} invalid (owned by {}): {e}",
                req.name, req.owner
            )),
        }
    }

    if tolerances.is_none() {
        items.push(
            "tolerance registry missing: nixie_units::tol is not wired into nixie-golden (owned by #8)"
                .to_owned(),
        );
    }

    for stage in stages {
        let Some(run) = stage.run else {
            items.push(format!(
                "stage \"{}\" missing (owned by {})",
                stage.name, stage.owner
            ));
            continue;
        };
        let Some(tolerances) = tolerances else {
            continue;
        };
        for fixture in &loaded {
            if let Err(reason) = run(fixture, tolerances) {
                items.push(format!(
                    "stage \"{}\" failed on {} (owned by {}):\n{reason}",
                    stage.name, fixture.name, stage.owner
                ));
            }
        }
    }

    Report { items }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::golden_root;
    use crate::fixture::tests::{fixture_from_template, scratch_dir};
    use crate::tolerance::ToleranceTable;

    const ONE: &[FixtureReq] = &[FixtureReq {
        name: "g01-test",
        owner: "#100",
    }];

    fn passes(_: &Fixture, _: &dyn Tolerances) -> Result<(), String> {
        Ok(())
    }

    fn fails(f: &Fixture, _: &dyn Tolerances) -> Result<(), String> {
        Err(format!("hcm: 1 difference\n  - missing     {}", f.name))
    }

    #[test]
    fn real_gate_names_every_missing_fixture_and_stage() {
        let report = evaluate(&golden_root());
        let text = report.to_string();
        for f in FIXTURES {
            if !golden_root().join(f.name).is_dir() {
                assert!(
                    text.contains(&format!(
                        "fixture {} missing (owned by {})",
                        f.name, f.owner
                    )),
                    "{text}"
                );
            }
        }
        for s in STAGES.iter().filter(|s| s.run.is_none()) {
            assert!(
                text.contains(&format!(
                    "stage \"{}\" missing (owned by {})",
                    s.name, s.owner
                )),
                "{text}"
            );
        }
    }

    #[test]
    fn green_when_fixtures_exist_and_stages_pass() {
        let root = scratch_dir("gate-green");
        fixture_from_template(&root, "g01-test");
        let stages = [StageReq {
            name: "stage a",
            owner: "#101",
            run: Some(passes),
        }];
        let report = evaluate_with(&root, ONE, &stages, Some(&ToleranceTable::new()));
        assert!(report.is_green(), "{report}");
        assert_eq!(report.to_string(), "Gate 0 is green");
    }

    #[test]
    fn red_items_are_numbered_and_name_their_owner() {
        let root = scratch_dir("gate-red");
        let stages = [
            StageReq {
                name: "stage a",
                owner: "#101",
                run: None,
            },
            StageReq {
                name: "stage b",
                owner: "#102",
                run: Some(passes),
            },
        ];
        let report = evaluate_with(&root, ONE, &stages, None);
        assert_eq!(
            report.to_string(),
            "Gate 0 is red: 3 item(s)\n  \
             1. fixture g01-test missing (owned by #100)\n  \
             2. tolerance registry missing: nixie_units::tol is not wired into nixie-golden (owned by #8)\n  \
             3. stage \"stage a\" missing (owned by #101)"
        );
    }

    #[test]
    fn failing_stage_reports_fixture_and_diff() {
        let root = scratch_dir("gate-fail");
        fixture_from_template(&root, "g01-test");
        let stages = [StageReq {
            name: "stage b",
            owner: "#102",
            run: Some(fails),
        }];
        let report = evaluate_with(&root, ONE, &stages, Some(&ToleranceTable::new()));
        assert_eq!(
            report.to_string(),
            "Gate 0 is red: 1 item(s)\n  \
             1. stage \"stage b\" failed on g01-test (owned by #102):\n     \
             hcm: 1 difference\n       - missing     g01-test"
        );
    }

    #[test]
    fn invalid_fixture_is_red_with_the_parse_error() {
        let root = scratch_dir("gate-invalid");
        let dir = fixture_from_template(&root, "g01-test");
        std::fs::write(dir.join("expected/hcm.toml"), "[[lane_group]]\nbogus = 1\n").unwrap();
        let report = evaluate_with(&root, ONE, &[], Some(&ToleranceTable::new()));
        assert_eq!(report.items.len(), 1, "{report}");
        assert!(
            report.items[0].starts_with("fixture g01-test invalid (owned by #100): "),
            "{report}"
        );
        assert!(report.items[0].contains("hcm.toml"), "{report}");
    }
}
