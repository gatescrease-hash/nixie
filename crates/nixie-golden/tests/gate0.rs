//! Gate 0 suite: `cargo test -p nixie-golden --features gate0`.
//!
//! Red until the Phase 0 slice is complete; the failure lists every missing
//! fixture and stage with the issue that owns it.

#![cfg(feature = "gate0")]

use nixie_golden::gate0;

#[test]
fn gate0() {
    let report = gate0::evaluate(&nixie_golden::golden_root());
    assert!(report.is_green(), "{report}");
}
