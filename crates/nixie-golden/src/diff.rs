//! A readable list of differences between expected and actual results.

use std::collections::BTreeMap;
use std::fmt;

use crate::tolerance::{self, Check, ToleranceName, Toleranced, Tolerances};

/// Every difference found when comparing one stage's output against its
/// expected file. Empty means the stage matches.
#[derive(Debug, Clone)]
pub struct Diff {
    stage: &'static str,
    entries: Vec<Entry>,
}

#[derive(Debug, Clone)]
pub enum Entry {
    /// Expected but not produced.
    Missing { item: String },
    /// Produced but not expected.
    Unexpected { item: String },
    /// The same key appears more than once on one side.
    Duplicate { item: String, side: Side },
    /// An exactly-compared field differs.
    Mismatch {
        item: String,
        field: String,
        expected: String,
        actual: String,
    },
    OutOfTolerance {
        item: String,
        field: String,
        expected: f64,
        actual: f64,
        tol_name: ToleranceName,
        tol: f64,
    },
    UnknownTolerance {
        item: String,
        field: String,
        tol_name: ToleranceName,
    },
    InvalidTolerance {
        item: String,
        field: String,
        tol_name: ToleranceName,
        tol: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Expected,
    Actual,
}

impl Diff {
    pub fn new(stage: &'static str) -> Self {
        Self {
            stage,
            entries: Vec::new(),
        }
    }

    pub fn stage(&self) -> &'static str {
        self.stage
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Panics with the formatted diff unless it is empty.
    #[track_caller]
    pub fn assert_empty(&self) {
        if !self.is_empty() {
            panic!("{self}");
        }
    }

    pub(crate) fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    /// Compares an exactly-matched field.
    pub(crate) fn exact<T: PartialEq + fmt::Display>(
        &mut self,
        item: &str,
        field: &str,
        expected: &T,
        actual: &T,
    ) {
        if expected != actual {
            self.push(Entry::Mismatch {
                item: item.to_owned(),
                field: field.to_owned(),
                expected: expected.to_string(),
                actual: actual.to_string(),
            });
        }
    }

    /// Compares a value against its expectation and named tolerance.
    pub(crate) fn value(
        &mut self,
        item: &str,
        field: &str,
        expected: &Toleranced,
        actual: f64,
        tolerances: &dyn Tolerances,
    ) {
        let item = item.to_owned();
        let field = field.to_owned();
        let tol_name = expected.tol.clone();
        match tolerance::check(expected, actual, tolerances) {
            Check::Within => {}
            Check::Outside { tol } => self.push(Entry::OutOfTolerance {
                item,
                field,
                expected: expected.value,
                actual,
                tol_name,
                tol,
            }),
            Check::UnknownTolerance => self.push(Entry::UnknownTolerance {
                item,
                field,
                tol_name,
            }),
            Check::InvalidTolerance { tol } => self.push(Entry::InvalidTolerance {
                item,
                field,
                tol_name,
                tol,
            }),
        }
    }

    /// Matches records by key and reports missing, unexpected and duplicate
    /// keys. `fields` compares each matched pair.
    pub(crate) fn keyed<E, A>(
        &mut self,
        expected: &[E],
        actual: &[A],
        expected_key: impl Fn(&E) -> String,
        actual_key: impl Fn(&A) -> String,
        mut fields: impl FnMut(&mut Diff, &str, &E, &A),
    ) {
        let mut by_key: BTreeMap<String, (Vec<&E>, Vec<&A>)> = BTreeMap::new();
        for e in expected {
            by_key.entry(expected_key(e)).or_default().0.push(e);
        }
        for a in actual {
            by_key.entry(actual_key(a)).or_default().1.push(a);
        }
        for (item, (es, as_)) in by_key {
            if es.len() > 1 {
                self.push(Entry::Duplicate {
                    item: item.clone(),
                    side: Side::Expected,
                });
            }
            if as_.len() > 1 {
                self.push(Entry::Duplicate {
                    item: item.clone(),
                    side: Side::Actual,
                });
            }
            match (es.as_slice(), as_.as_slice()) {
                ([e], [a]) => fields(self, &item, e, a),
                ([_, ..], []) => self.push(Entry::Missing { item }),
                ([], [_, ..]) => self.push(Entry::Unexpected { item }),
                _ => {}
            }
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Missing { item } => write!(f, "- missing     {item}"),
            Entry::Unexpected { item } => write!(f, "+ unexpected  {item}"),
            Entry::Duplicate { item, side } => {
                let side = match side {
                    Side::Expected => "expected file",
                    Side::Actual => "actual output",
                };
                write!(f, "! duplicate   {item} (more than once in the {side})")
            }
            Entry::Mismatch {
                item,
                field,
                expected,
                actual,
            } => write!(f, "~ {item}: {field}: expected {expected}, actual {actual}"),
            Entry::OutOfTolerance {
                item,
                field,
                expected,
                actual,
                tol_name,
                tol,
            } => write!(
                f,
                "~ {item}: {field}: expected {expected} ± {tol} ({tol_name}), actual {actual} (off by {:.*})",
                decimals_for(*tol),
                (actual - expected).abs()
            ),
            Entry::UnknownTolerance {
                item,
                field,
                tol_name,
            } => write!(
                f,
                "? {item}: {field}: tolerance {tol_name} is not in the registry"
            ),
            Entry::InvalidTolerance {
                item,
                field,
                tol_name,
                tol,
            } => write!(
                f,
                "? {item}: {field}: tolerance {tol_name} resolves to {tol}, which is not a finite, non-negative bound"
            ),
        }
    }
}

/// Enough decimals to show a difference one digit finer than the tolerance,
/// without printing float noise.
fn decimals_for(tol: f64) -> usize {
    const MAX: usize = 12;
    if tol > 0.0 {
        let digits = (-tol.log10()).ceil().max(0.0) as usize;
        (digits + 1).min(MAX)
    } else {
        MAX
    }
}

impl fmt::Display for Diff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.entries.len() {
            0 => write!(f, "{}: matches expected", self.stage),
            n => {
                let s = if n == 1 { "" } else { "s" };
                write!(f, "{}: {n} difference{s}", self.stage)?;
                for entry in &self.entries {
                    write!(f, "\n  {entry}")?;
                }
                Ok(())
            }
        }
    }
}
