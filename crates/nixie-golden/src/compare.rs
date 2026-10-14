//! Comparison helpers: one `compare_*` returning a [`Diff`] and one
//! `assert_*` that panics with it, per golden stage.

use crate::diff::{Diff, Entry};
use crate::model::{
    Alignment, ConnectorSet, Hcm, HcmApproach, HcmIntersection, HcmLaneGroup, LandXml, LaneGraph,
    LaneGroup, LaneGroups, LegLanes, Movement, Movements, Turn,
};
use crate::tolerance::{Toleranced, Tolerances};

fn movement_key(junction: &str, approach: &str, departure: &str, turn: Turn) -> String {
    format!("{junction} {approach}→{departure} {turn}")
}

fn turns_key(turns: &[Turn]) -> String {
    let mut turns = turns.to_vec();
    turns.sort();
    let turns: Vec<String> = turns.iter().map(Turn::to_string).collect();
    format!("[{}]", turns.join(", "))
}

pub fn compare_movements(
    expected: &Movements<Toleranced>,
    actual: &Movements<f64>,
    tolerances: &dyn Tolerances,
) -> Diff {
    fn key<V>(m: &Movement<V>) -> String {
        movement_key(&m.junction, &m.approach, &m.departure, m.turn)
    }
    let mut diff = Diff::new("movements");
    diff.keyed(
        &expected.movements,
        &actual.movements,
        key,
        key,
        |diff, item, e, a| diff.value(item, "volume", &e.volume, a.volume, tolerances),
    );
    diff
}

#[track_caller]
pub fn assert_movements(
    expected: &Movements<Toleranced>,
    actual: &Movements<f64>,
    tolerances: &dyn Tolerances,
) {
    compare_movements(expected, actual, tolerances).assert_empty();
}

pub fn compare_lane_graph(expected: &LaneGraph, actual: &LaneGraph) -> Diff {
    fn leg_key(l: &LegLanes) -> String {
        format!("{} leg {}", l.junction, l.leg)
    }
    fn set_key(c: &ConnectorSet) -> String {
        format!(
            "connectors {}",
            movement_key(&c.junction, &c.approach, &c.departure, c.turn)
        )
    }
    let mut diff = Diff::new("lane_graph");
    diff.keyed(
        &expected.legs,
        &actual.legs,
        leg_key,
        leg_key,
        |diff, item, e, a| {
            diff.exact(item, "approach_lanes", &e.approach_lanes, &a.approach_lanes);
            diff.exact(
                item,
                "departure_lanes",
                &e.departure_lanes,
                &a.departure_lanes,
            );
        },
    );
    diff.keyed(
        &expected.connector_sets,
        &actual.connector_sets,
        set_key,
        set_key,
        |diff, item, e, a| diff.exact(item, "connectors", &e.connectors, &a.connectors),
    );
    diff
}

#[track_caller]
pub fn assert_lane_graph(expected: &LaneGraph, actual: &LaneGraph) {
    compare_lane_graph(expected, actual).assert_empty();
}

pub fn compare_lane_groups(expected: &LaneGroups, actual: &LaneGroups) -> Diff {
    fn key(g: &LaneGroup) -> String {
        format!("{} {} {}", g.junction, g.approach, turns_key(&g.turns))
    }
    let mut diff = Diff::new("lane_groups");
    diff.keyed(
        &expected.lane_groups,
        &actual.lane_groups,
        key,
        key,
        |diff, item, e, a| diff.exact(item, "lanes", &e.lanes, &a.lanes),
    );
    diff
}

#[track_caller]
pub fn assert_lane_groups(expected: &LaneGroups, actual: &LaneGroups) {
    compare_lane_groups(expected, actual).assert_empty();
}

pub fn compare_hcm(
    expected: &Hcm<Toleranced>,
    actual: &Hcm<f64>,
    tolerances: &dyn Tolerances,
) -> Diff {
    fn group_key<V>(g: &HcmLaneGroup<V>) -> String {
        format!(
            "lane group {} {} {}",
            g.junction,
            g.approach,
            turns_key(&g.turns)
        )
    }
    fn approach_key<V>(a: &HcmApproach<V>) -> String {
        format!("approach {} {}", a.junction, a.approach)
    }
    fn intersection_key<V>(i: &HcmIntersection<V>) -> String {
        format!("intersection {}", i.junction)
    }
    let mut diff = Diff::new("hcm");
    diff.keyed(
        &expected.lane_groups,
        &actual.lane_groups,
        group_key,
        group_key,
        |diff, item, e, a| {
            diff.value(
                item,
                "control_delay",
                &e.control_delay,
                a.control_delay,
                tolerances,
            );
            diff.value(
                item,
                "volume_to_capacity",
                &e.volume_to_capacity,
                a.volume_to_capacity,
                tolerances,
            );
            diff.exact(item, "los", &e.los, &a.los);
        },
    );
    diff.keyed(
        &expected.approaches,
        &actual.approaches,
        approach_key,
        approach_key,
        |diff, item, e, a| {
            diff.value(
                item,
                "control_delay",
                &e.control_delay,
                a.control_delay,
                tolerances,
            );
            diff.exact(item, "los", &e.los, &a.los);
        },
    );
    diff.keyed(
        &expected.intersections,
        &actual.intersections,
        intersection_key,
        intersection_key,
        |diff, item, e, a| {
            diff.value(
                item,
                "control_delay",
                &e.control_delay,
                a.control_delay,
                tolerances,
            );
            diff.exact(item, "los", &e.los, &a.los);
        },
    );
    diff
}

#[track_caller]
pub fn assert_hcm(expected: &Hcm<Toleranced>, actual: &Hcm<f64>, tolerances: &dyn Tolerances) {
    compare_hcm(expected, actual, tolerances).assert_empty();
}

/// Compares alignments re-imported from LandXML against the expected
/// element types, lengths and stations. Elements compare in order.
pub fn compare_landxml_roundtrip(
    expected: &LandXml<Toleranced>,
    roundtripped: &LandXml<f64>,
    tolerances: &dyn Tolerances,
) -> Diff {
    fn key<V>(a: &Alignment<V>) -> String {
        format!("alignment {}", a.name)
    }
    let mut diff = Diff::new("landxml_roundtrip");
    diff.keyed(
        &expected.alignments,
        &roundtripped.alignments,
        key,
        key,
        |diff, item, e, a| {
            diff.value(
                item,
                "start_station",
                &e.start_station,
                a.start_station,
                tolerances,
            );
            let n = e.elements.len().max(a.elements.len());
            for i in 0..n {
                let item = format!("{item} element {}", i + 1);
                match (e.elements.get(i), a.elements.get(i)) {
                    (Some(e), Some(a)) => {
                        diff.exact(&item, "kind", &e.kind, &a.kind);
                        diff.value(&item, "length", &e.length, a.length, tolerances);
                        diff.value(
                            &item,
                            "start_station",
                            &e.start_station,
                            a.start_station,
                            tolerances,
                        );
                    }
                    (Some(_), None) => diff.push(Entry::Missing { item }),
                    (None, Some(_)) => diff.push(Entry::Unexpected { item }),
                    (None, None) => unreachable!(),
                }
            }
        },
    );
    diff
}

#[track_caller]
pub fn assert_landxml_roundtrip(
    expected: &LandXml<Toleranced>,
    roundtripped: &LandXml<f64>,
    tolerances: &dyn Tolerances,
) {
    compare_landxml_roundtrip(expected, roundtripped, tolerances).assert_empty();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Element, ElementKind, Los};
    use crate::tolerance::{ToleranceName, ToleranceTable};

    fn tols() -> ToleranceTable {
        ToleranceTable::new()
            .with("LENGTH", 0.01)
            .with("DELAY", 0.1)
            .with("RATIO", 0.01)
    }

    fn t(value: f64, tol: &str) -> Toleranced {
        Toleranced {
            value,
            tol: ToleranceName::new(tol).unwrap(),
        }
    }

    fn movement<V>(approach: &str, turn: Turn, volume: V) -> Movement<V> {
        Movement {
            junction: "J1".into(),
            approach: approach.into(),
            departure: "S".into(),
            turn,
            volume,
        }
    }

    fn lines(diff: &Diff) -> Vec<String> {
        diff.entries().iter().map(ToString::to_string).collect()
    }

    #[test]
    fn identical_movements_match() {
        let e = Movements {
            movements: vec![movement("N", Turn::Through, t(0.125, "RATIO"))],
        };
        let a = Movements {
            movements: vec![movement("N", Turn::Through, 0.13)],
        };
        assert_movements(&e, &a, &tols());
    }

    #[test]
    fn movements_report_missing_unexpected_and_out_of_tolerance() {
        let e = Movements {
            movements: vec![
                movement("N", Turn::Through, t(0.125, "RATIO")),
                movement("N", Turn::Left, t(0.05, "RATIO")),
            ],
        };
        let a = Movements {
            movements: vec![
                movement("N", Turn::Through, 0.2),
                movement("E", Turn::Right, 0.1),
            ],
        };
        let diff = compare_movements(&e, &a, &tols());
        assert_eq!(
            lines(&diff),
            [
                "+ unexpected  J1 E→S right",
                "- missing     J1 N→S left",
                "~ J1 N→S through: volume: expected 0.125 ± 0.01 (RATIO), actual 0.2 (off by 0.075)",
            ]
        );
    }

    #[test]
    fn duplicates_are_reported_per_side() {
        let e = Movements {
            movements: vec![
                movement("N", Turn::Left, t(0.1, "RATIO")),
                movement("N", Turn::Left, t(0.1, "RATIO")),
            ],
        };
        let a = Movements {
            movements: vec![movement("N", Turn::Left, 0.1)],
        };
        let diff = compare_movements(&e, &a, &tols());
        assert_eq!(
            lines(&diff),
            ["! duplicate   J1 N→S left (more than once in the expected file)"]
        );
    }

    #[test]
    fn unknown_tolerance_name_is_a_difference_not_a_pass() {
        let e = Movements {
            movements: vec![movement("N", Turn::Left, t(0.1, "NOT_REGISTERED"))],
        };
        let a = Movements {
            movements: vec![movement("N", Turn::Left, 0.1)],
        };
        let diff = compare_movements(&e, &a, &tols());
        assert_eq!(
            lines(&diff),
            ["? J1 N→S left: volume: tolerance NOT_REGISTERED is not in the registry"]
        );
    }

    #[test]
    fn lane_graph_compares_counts_exactly() {
        let leg = |approach_lanes| LegLanes {
            junction: "J1".into(),
            leg: "N".into(),
            approach_lanes,
            departure_lanes: 2,
        };
        let set = |connectors| ConnectorSet {
            junction: "J1".into(),
            approach: "N".into(),
            departure: "S".into(),
            turn: Turn::Through,
            connectors,
        };
        let e = LaneGraph {
            legs: vec![leg(3)],
            connector_sets: vec![set(2)],
        };
        assert_lane_graph(&e, &e.clone());
        let a = LaneGraph {
            legs: vec![leg(2)],
            connector_sets: vec![set(1)],
        };
        assert_eq!(
            lines(&compare_lane_graph(&e, &a)),
            [
                "~ J1 leg N: approach_lanes: expected 3, actual 2",
                "~ connectors J1 N→S through: connectors: expected 2, actual 1",
            ]
        );
    }

    #[test]
    fn lane_groups_are_keyed_by_turn_set_in_any_order() {
        let group = |turns: Vec<Turn>, lanes| LaneGroup {
            junction: "J1".into(),
            approach: "N".into(),
            turns,
            lanes,
        };
        let e = LaneGroups {
            lane_groups: vec![group(vec![Turn::Through, Turn::Right], 2)],
        };
        let a = LaneGroups {
            lane_groups: vec![group(vec![Turn::Right, Turn::Through], 2)],
        };
        assert_lane_groups(&e, &a);

        let a = LaneGroups {
            lane_groups: vec![group(vec![Turn::Through], 2)],
        };
        assert_eq!(
            lines(&compare_lane_groups(&e, &a)),
            [
                "- missing     J1 N [through, right]",
                "+ unexpected  J1 N [through]",
            ]
        );
    }

    fn hcm<V: Clone>(delay: V, vc: V, los: Los) -> Hcm<V> {
        Hcm {
            lane_groups: vec![HcmLaneGroup {
                junction: "J1".into(),
                approach: "N".into(),
                turns: vec![Turn::Left],
                control_delay: delay.clone(),
                volume_to_capacity: vc,
                los,
            }],
            approaches: vec![HcmApproach {
                junction: "J1".into(),
                approach: "N".into(),
                control_delay: delay.clone(),
                los,
            }],
            intersections: vec![HcmIntersection {
                junction: "J1".into(),
                control_delay: delay,
                los,
            }],
        }
    }

    #[test]
    fn hcm_checks_delay_vc_and_los() {
        let e = hcm(t(23.4, "DELAY"), t(0.82, "RATIO"), Los::C);
        assert_hcm(&e, &hcm(23.45, 0.815, Los::C), &tols());

        let diff = compare_hcm(&e, &hcm(23.4, 0.9, Los::D), &tols());
        let lines = lines(&diff);
        assert_eq!(lines.len(), 4, "{diff}");
        assert!(
            lines[0].starts_with("~ lane group J1 N [left]: volume_to_capacity: expected 0.82")
        );
        assert_eq!(
            lines[1],
            "~ lane group J1 N [left]: los: expected C, actual D"
        );
        assert_eq!(lines[2], "~ approach J1 N: los: expected C, actual D");
        assert_eq!(lines[3], "~ intersection J1: los: expected C, actual D");
    }

    fn alignment<V>(start: V, elements: Vec<Element<V>>) -> LandXml<V> {
        LandXml {
            alignments: vec![Alignment {
                name: "Main St".into(),
                start_station: start,
                elements,
            }],
        }
    }

    fn element<V>(kind: ElementKind, length: V, start_station: V) -> Element<V> {
        Element {
            kind,
            length,
            start_station,
        }
    }

    #[test]
    fn landxml_roundtrip_compares_elements_in_order() {
        let e = alignment(
            t(1000.0, "LENGTH"),
            vec![
                element(ElementKind::Line, t(50.0, "LENGTH"), t(1000.0, "LENGTH")),
                element(
                    ElementKind::Clothoid,
                    t(30.0, "LENGTH"),
                    t(1050.0, "LENGTH"),
                ),
            ],
        );
        let same = alignment(
            1000.0,
            vec![
                element(ElementKind::Line, 50.005, 1000.0),
                element(ElementKind::Clothoid, 30.0, 1050.005),
            ],
        );
        assert_landxml_roundtrip(&e, &same, &tols());

        let off = alignment(
            1000.0,
            vec![
                element(ElementKind::Line, 50.0, 1000.0),
                element(ElementKind::Arc, 30.0, 1050.0),
                element(ElementKind::Line, 10.0, 1080.0),
            ],
        );
        assert_eq!(
            lines(&compare_landxml_roundtrip(&e, &off, &tols())),
            [
                "~ alignment Main St element 2: kind: expected clothoid, actual arc",
                "+ unexpected  alignment Main St element 3",
            ]
        );
    }

    #[test]
    fn diff_display_is_readable() {
        let e = Movements {
            movements: vec![movement("N", Turn::Left, t(0.1, "RATIO"))],
        };
        let diff = compare_movements(&e, &Movements::default(), &tols());
        assert_eq!(
            diff.to_string(),
            "movements: 1 difference\n  - missing     J1 N→S left"
        );
        let diff = compare_movements(&Movements::default(), &Movements::default(), &tols());
        assert_eq!(diff.to_string(), "movements: matches expected");
    }

    #[test]
    #[should_panic(expected = "movements: 1 difference")]
    fn assert_panics_with_the_diff() {
        let e = Movements {
            movements: vec![movement("N", Turn::Left, t(0.1, "RATIO"))],
        };
        assert_movements(&e, &Movements::default(), &tols());
    }
}
