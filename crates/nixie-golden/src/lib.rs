//! Golden-network test harness and gate suites.
//!
//! Loads `testdata/golden/<name>/{input,expected}/`, compares Nixie output
//! against the human-owned expected files using named tolerances
//! (ADR-0012, ADR-0019, ADR-0020), and defines the gate suites.

mod compare;
mod diff;
pub mod fixture;
pub mod gate0;
pub mod model;
mod tolerance;

pub use compare::{
    assert_hcm, assert_landxml_roundtrip, assert_lane_graph, assert_lane_groups, assert_movements,
    compare_hcm, compare_landxml_roundtrip, compare_lane_graph, compare_lane_groups,
    compare_movements,
};
pub use diff::{Diff, Entry, Side};
pub use fixture::{ExpectedFile, Fixture, FixtureError, Provenance, golden_root};
pub use tolerance::{InvalidToleranceName, ToleranceName, ToleranceTable, Toleranced, Tolerances};
