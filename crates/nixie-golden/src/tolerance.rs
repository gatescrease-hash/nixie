//! Expected values carry a tolerance *name*; numbers come from a registry.
//!
//! ADR-0012 and ADR-0019: tolerances live in code once, in
//! `nixie_units::tol`. A fixture never writes a tolerance as a number, so
//! changing a tolerance never means editing an oracle file.

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// The name of a project tolerance, e.g. `OFFSET_CAD`.
///
/// Names are `SCREAMING_SNAKE_CASE`, matching the constants in
/// `nixie_units::tol`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ToleranceName(String);

impl ToleranceName {
    pub fn new(name: impl Into<String>) -> Result<Self, InvalidToleranceName> {
        let name = name.into();
        let mut chars = name.chars();
        let valid = chars.next().is_some_and(|c| c.is_ascii_uppercase())
            && chars.all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if valid {
            Ok(Self(name))
        } else {
            Err(InvalidToleranceName(name))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ToleranceName {
    type Error = InvalidToleranceName;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Self::new(name)
    }
}

impl From<ToleranceName> for String {
    fn from(name: ToleranceName) -> Self {
        name.0
    }
}

impl fmt::Display for ToleranceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidToleranceName(pub String);

impl fmt::Display for InvalidToleranceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid tolerance name `{}`: expected SCREAMING_SNAKE_CASE, e.g. `OFFSET_CAD`",
            self.0
        )
    }
}

impl std::error::Error for InvalidToleranceName {}

/// An expected numeric value and the name of the tolerance it is judged by.
///
/// In TOML: `length = { value = 42.0, tol = "OFFSET_CAD" }`. A bare number
/// is rejected when the fixture is loaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toleranced {
    pub value: f64,
    pub tol: ToleranceName,
}

/// Resolves tolerance names to values, in the same SI unit as the quantity
/// being compared.
pub trait Tolerances {
    fn resolve(&self, name: &ToleranceName) -> Option<f64>;
}

/// A fixed name → value table. Use it in tests; production code resolves
/// through `nixie_units::tol`.
#[derive(Debug, Clone, Default)]
pub struct ToleranceTable(BTreeMap<ToleranceName, f64>);

impl ToleranceTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an entry. Panics if `name` is not a valid tolerance name.
    pub fn with(mut self, name: &str, value: f64) -> Self {
        let name = ToleranceName::new(name).unwrap_or_else(|e| panic!("{e}"));
        self.0.insert(name, value);
        self
    }
}

impl Tolerances for ToleranceTable {
    fn resolve(&self, name: &ToleranceName) -> Option<f64> {
        self.0.get(name).copied()
    }
}

/// The outcome of comparing one value against its expectation.
#[derive(Debug, Clone)]
pub(crate) enum Check {
    Within,
    Outside {
        tol: f64,
    },
    UnknownTolerance,
    /// The tolerance resolved to something that cannot bound a difference.
    InvalidTolerance {
        tol: f64,
    },
}

pub(crate) fn check(expected: &Toleranced, actual: f64, tolerances: &dyn Tolerances) -> Check {
    let Some(tol) = tolerances.resolve(&expected.tol) else {
        return Check::UnknownTolerance;
    };
    if !(tol.is_finite() && tol >= 0.0) {
        return Check::InvalidTolerance { tol };
    }
    // Written so that NaN on either side lands outside.
    if (actual - expected.value).abs() <= tol {
        Check::Within
    } else {
        Check::Outside { tol }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exp(value: f64, tol: &str) -> Toleranced {
        Toleranced {
            value,
            tol: ToleranceName::new(tol).unwrap(),
        }
    }

    fn table() -> ToleranceTable {
        ToleranceTable::new()
            .with("COARSE", 0.5)
            .with("NEGATIVE", -1.0)
    }

    #[test]
    fn names_must_be_screaming_snake_case() {
        assert!(ToleranceName::new("OFFSET_CAD").is_ok());
        assert!(ToleranceName::new("HCM_DELAY_0_1S").is_ok());
        for bad in [
            "",
            "offset_cad",
            "Offset",
            "_LEADING",
            "1ST",
            "WITH SPACE",
            "A-B",
        ] {
            assert!(ToleranceName::new(bad).is_err(), "{bad:?} accepted");
        }
    }

    #[test]
    fn within_tolerance_is_inclusive() {
        let t = table();
        assert!(matches!(
            check(&exp(10.0, "COARSE"), 10.5, &t),
            Check::Within
        ));
        assert!(matches!(
            check(&exp(10.0, "COARSE"), 9.5, &t),
            Check::Within
        ));
        assert!(matches!(
            check(&exp(10.0, "COARSE"), 10.75, &t),
            Check::Outside { .. }
        ));
    }

    #[test]
    fn nan_is_never_within() {
        let t = table();
        assert!(matches!(
            check(&exp(10.0, "COARSE"), f64::NAN, &t),
            Check::Outside { .. }
        ));
        assert!(matches!(
            check(&exp(f64::NAN, "COARSE"), 10.0, &t),
            Check::Outside { .. }
        ));
    }

    #[test]
    fn unknown_and_invalid_tolerances_are_reported() {
        let t = table();
        assert!(matches!(
            check(&exp(1.0, "MISSING"), 1.0, &t),
            Check::UnknownTolerance
        ));
        assert!(matches!(
            check(&exp(1.0, "NEGATIVE"), 1.0, &t),
            Check::InvalidTolerance { .. }
        ));
    }

    #[test]
    fn toleranced_requires_a_name_not_a_number() {
        #[derive(Deserialize)]
        struct Doc {
            #[allow(dead_code)]
            x: Toleranced,
        }
        assert!(toml::from_str::<Doc>(r#"x = { value = 1.0, tol = "OFFSET_CAD" }"#).is_ok());
        for bad in [
            "x = 1.0",
            "x = { value = 1.0 }",
            "x = { value = 1.0, tol = 0.5 }",
            r#"x = { value = 1.0, tol = "offset" }"#,
            r#"x = { value = 1.0, tol = "OFFSET_CAD", abs = 0.5 }"#,
        ] {
            assert!(toml::from_str::<Doc>(bad).is_err(), "{bad} accepted");
        }
    }
}
