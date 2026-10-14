//! Loading `testdata/golden/<name>/`.
//!
//! ```text
//! testdata/golden/<name>/
//!   PROVENANCE.toml        where every expected value comes from
//!   input/                 the authored model (op-log fixture)
//!   expected/              human-owned oracle files (ADR-0020)
//!     movements.toml
//!     lane_graph.toml
//!     lane_groups.toml
//!     hcm.toml
//!     landxml.toml
//! ```
//!
//! Each expected file is optional: a fixture only declares the stages it
//! has oracle values for.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::model::{Hcm, LandXml, LaneGraph, LaneGroups, Movements};
use crate::tolerance::Toleranced;

/// Name of the directory that shows the fixture layout. It is not a golden
/// network and no gate runs it.
pub const TEMPLATE: &str = "_template";

pub const PROVENANCE_FILE: &str = "PROVENANCE.toml";
pub const INPUT_DIR: &str = "input";
pub const EXPECTED_DIR: &str = "expected";

/// `testdata/golden/` in this repository.
pub fn golden_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../testdata/golden")
}

/// The expected-output files a fixture may contain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpectedFile {
    Movements,
    LaneGraph,
    LaneGroups,
    Hcm,
    Landxml,
}

impl ExpectedFile {
    pub const ALL: [ExpectedFile; 5] = [
        ExpectedFile::Movements,
        ExpectedFile::LaneGraph,
        ExpectedFile::LaneGroups,
        ExpectedFile::Hcm,
        ExpectedFile::Landxml,
    ];

    pub fn file_name(self) -> &'static str {
        match self {
            ExpectedFile::Movements => "movements.toml",
            ExpectedFile::LaneGraph => "lane_graph.toml",
            ExpectedFile::LaneGroups => "lane_groups.toml",
            ExpectedFile::Hcm => "hcm.toml",
            ExpectedFile::Landxml => "landxml.toml",
        }
    }
}

/// `PROVENANCE.toml`: where every expected value in the fixture comes from.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub fixture: FixtureInfo,
    pub sources: Sources,
    pub oracle: Oracle,
    #[serde(default, rename = "deviation")]
    pub deviations: Vec<Deviation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureInfo {
    /// Must equal the fixture's directory name.
    pub name: String,
    pub description: String,
    /// The issue that tracks the fixture's oracle values.
    pub issue: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sources {
    /// Published HCM example problem the HCM values come from, if any.
    pub hcm_example: Option<String>,
    pub geometry: String,
    pub volumes: String,
    pub timings: Option<String>,
    /// Licence or permission under which the source material is used.
    pub licence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Oracle {
    pub method: OracleMethod,
    pub authored_by: String,
    pub reviewed_by: Option<String>,
}

/// How the expected values were obtained. Never "Nixie output" (ADR-0020).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OracleMethod {
    /// Copied from a published source, e.g. an HCM example problem.
    Published,
    /// Calculated by hand by a human.
    HandCalculation,
    /// Derived by a tool from another fixture's oracle (e.g. the LHT mirror),
    /// then approved by a Code Owner.
    Derived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deviation {
    pub what: String,
    pub why: String,
}

#[derive(Debug)]
pub enum FixtureError {
    /// The fixture directory does not exist.
    Missing {
        name: String,
        dir: PathBuf,
    },
    /// A required file or directory inside the fixture does not exist.
    MissingPart {
        name: String,
        path: PathBuf,
    },
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Parse {
        path: PathBuf,
        message: String,
    },
    NameMismatch {
        dir_name: String,
        declared: String,
    },
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixtureError::Missing { name, dir } => {
                write!(
                    f,
                    "fixture {name} missing (no directory at {})",
                    dir.display()
                )
            }
            FixtureError::MissingPart { name, path } => {
                write!(
                    f,
                    "fixture {name} is incomplete: {} missing",
                    path.display()
                )
            }
            FixtureError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            FixtureError::Parse { path, message } => {
                write!(f, "{}: {}", path.display(), message.trim_end())
            }
            FixtureError::NameMismatch { dir_name, declared } => write!(
                f,
                "fixture directory {dir_name} declares fixture.name = {declared:?} in {PROVENANCE_FILE}"
            ),
        }
    }
}

impl std::error::Error for FixtureError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FixtureError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// A golden fixture on disk, with its provenance read and checked.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub name: String,
    pub dir: PathBuf,
    pub provenance: Provenance,
}

impl Fixture {
    /// Loads `<root>/<name>/`, requiring `PROVENANCE.toml`, `input/` and
    /// `expected/`. Expected files are read on demand.
    pub fn load(root: &Path, name: &str) -> Result<Self, FixtureError> {
        let dir = root.join(name);
        if !dir.is_dir() {
            return Err(FixtureError::Missing {
                name: name.to_owned(),
                dir,
            });
        }
        for sub in [INPUT_DIR, EXPECTED_DIR] {
            let path = dir.join(sub);
            if !path.is_dir() {
                return Err(FixtureError::MissingPart {
                    name: name.to_owned(),
                    path,
                });
            }
        }
        let path = dir.join(PROVENANCE_FILE);
        let provenance: Provenance =
            read_toml(&path)?.ok_or_else(|| FixtureError::MissingPart {
                name: name.to_owned(),
                path: path.clone(),
            })?;
        if provenance.fixture.name != name {
            return Err(FixtureError::NameMismatch {
                dir_name: name.to_owned(),
                declared: provenance.fixture.name,
            });
        }
        Ok(Self {
            name: name.to_owned(),
            dir,
            provenance,
        })
    }

    pub fn input_dir(&self) -> PathBuf {
        self.dir.join(INPUT_DIR)
    }

    pub fn expected_dir(&self) -> PathBuf {
        self.dir.join(EXPECTED_DIR)
    }

    pub fn expected_path(&self, file: ExpectedFile) -> PathBuf {
        self.expected_dir().join(file.file_name())
    }

    /// Whether the fixture declares oracle values for `file`.
    pub fn has_expected(&self, file: ExpectedFile) -> bool {
        self.expected_path(file).is_file()
    }

    pub fn movements(&self) -> Result<Option<Movements<Toleranced>>, FixtureError> {
        read_toml(&self.expected_path(ExpectedFile::Movements))
    }

    pub fn lane_graph(&self) -> Result<Option<LaneGraph>, FixtureError> {
        read_toml(&self.expected_path(ExpectedFile::LaneGraph))
    }

    pub fn lane_groups(&self) -> Result<Option<LaneGroups>, FixtureError> {
        read_toml(&self.expected_path(ExpectedFile::LaneGroups))
    }

    pub fn hcm(&self) -> Result<Option<Hcm<Toleranced>>, FixtureError> {
        read_toml(&self.expected_path(ExpectedFile::Hcm))
    }

    pub fn landxml(&self) -> Result<Option<LandXml<Toleranced>>, FixtureError> {
        read_toml(&self.expected_path(ExpectedFile::Landxml))
    }

    /// Parses every expected file present, so format errors surface as soon
    /// as the fixture is loaded rather than when a stage first runs.
    pub fn check_expected_files(&self) -> Result<(), FixtureError> {
        self.movements()?;
        self.lane_graph()?;
        self.lane_groups()?;
        self.hcm()?;
        self.landxml()?;
        Ok(())
    }
}

/// Reads and parses a TOML file; `Ok(None)` if it does not exist.
fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<Option<T>, FixtureError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(source) => {
            return Err(FixtureError::Io {
                path: path.to_owned(),
                source,
            });
        }
    };
    toml::from_str(&text)
        .map(Some)
        .map_err(|e| FixtureError::Parse {
            path: path.to_owned(),
            message: e.to_string(),
        })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A fresh, empty directory under the system temp dir.
    pub(crate) fn scratch_dir(label: &str) -> PathBuf {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir =
            std::env::temp_dir().join(format!("nixie-golden-{label}-{}-{n}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Copies the template into `root/<name>` with `fixture.name` set to
    /// `name`.
    pub(crate) fn fixture_from_template(root: &Path, name: &str) -> PathBuf {
        let src = golden_root().join(TEMPLATE);
        let dst = root.join(name);
        for sub in [INPUT_DIR, EXPECTED_DIR] {
            fs::create_dir_all(dst.join(sub)).unwrap();
        }
        for file in ExpectedFile::ALL {
            let rel = Path::new(EXPECTED_DIR).join(file.file_name());
            fs::copy(src.join(&rel), dst.join(&rel)).unwrap();
        }
        let provenance = fs::read_to_string(src.join(PROVENANCE_FILE))
            .unwrap()
            .replace(
                &format!("name = \"{TEMPLATE}\""),
                &format!("name = \"{name}\""),
            );
        fs::write(dst.join(PROVENANCE_FILE), provenance).unwrap();
        dst
    }

    #[test]
    fn template_loads_and_every_expected_file_parses() {
        let fixture = Fixture::load(&golden_root(), TEMPLATE).unwrap();
        assert_eq!(fixture.provenance.fixture.name, TEMPLATE);
        for file in ExpectedFile::ALL {
            assert!(
                fixture.has_expected(file),
                "template lacks {}",
                file.file_name()
            );
        }
        fixture.check_expected_files().unwrap();
        assert!(!fixture.movements().unwrap().unwrap().movements.is_empty());
        assert!(!fixture.lane_graph().unwrap().unwrap().legs.is_empty());
        assert!(
            !fixture
                .lane_groups()
                .unwrap()
                .unwrap()
                .lane_groups
                .is_empty()
        );
        assert!(!fixture.hcm().unwrap().unwrap().lane_groups.is_empty());
        assert!(!fixture.landxml().unwrap().unwrap().alignments.is_empty());
    }

    #[test]
    fn missing_fixture_is_named() {
        let root = scratch_dir("missing");
        let err = Fixture::load(&root, "g99-nowhere").unwrap_err();
        assert!(matches!(err, FixtureError::Missing { .. }));
        assert!(err.to_string().starts_with("fixture g99-nowhere missing"));
    }

    #[test]
    fn incomplete_fixture_names_the_missing_part() {
        let root = scratch_dir("incomplete");
        let dir = fixture_from_template(&root, "g98");
        fs::remove_dir_all(dir.join(INPUT_DIR)).unwrap();
        let err = Fixture::load(&root, "g98").unwrap_err();
        assert!(
            matches!(err, FixtureError::MissingPart { ref path, .. } if path.ends_with(INPUT_DIR))
        );

        fs::create_dir(dir.join(INPUT_DIR)).unwrap();
        fs::remove_file(dir.join(PROVENANCE_FILE)).unwrap();
        let err = Fixture::load(&root, "g98").unwrap_err();
        assert!(
            matches!(err, FixtureError::MissingPart { ref path, .. } if path.ends_with(PROVENANCE_FILE))
        );
    }

    #[test]
    fn provenance_name_must_match_directory() {
        let root = scratch_dir("mismatch");
        fixture_from_template(&root, "g97");
        fs::rename(root.join("g97"), root.join("g96")).unwrap();
        let err = Fixture::load(&root, "g96").unwrap_err();
        assert!(matches!(err, FixtureError::NameMismatch { .. }), "{err}");
    }

    #[test]
    fn absent_expected_file_is_none_and_bad_one_is_a_parse_error() {
        let root = scratch_dir("expected");
        let dir = fixture_from_template(&root, "g95");
        let fixture = Fixture::load(&root, "g95").unwrap();

        fs::remove_file(fixture.expected_path(ExpectedFile::Hcm)).unwrap();
        assert!(fixture.hcm().unwrap().is_none());

        let movements = dir.join(EXPECTED_DIR).join("movements.toml");
        fs::write(
            &movements,
            "[[movement]]\njunction = \"J1\"\napproach = \"N\"\ndeparture = \"S\"\nturn = \"left\"\nvolume = 0.1\n",
        )
        .unwrap();
        let err = fixture.check_expected_files().unwrap_err();
        assert!(matches!(err, FixtureError::Parse { .. }));
        assert!(err.to_string().contains("movements.toml"), "{err}");
    }
}
