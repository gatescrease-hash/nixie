//! The shape of each golden stage's results.
//!
//! Types with a value parameter `V` are read from `expected/*.toml` as
//! `V = Toleranced` (value plus tolerance name) and produced by Nixie as
//! `V = f64`. Counts are integers and compare exactly.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Turn type of a movement (ADR-0004).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Turn {
    Left,
    Through,
    Right,
    UTurn,
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Turn::Left => "left",
            Turn::Through => "through",
            Turn::Right => "right",
            Turn::UTurn => "u_turn",
        })
    }
}

/// HCM level of service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Los {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl fmt::Display for Los {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

/// `expected/movements.toml`: every movement key with its volume.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Movements<V> {
    #[serde(default = "Vec::new", rename = "movement")]
    pub movements: Vec<Movement<V>>,
}

impl<V> Default for Movements<V> {
    fn default() -> Self {
        Self {
            movements: Vec::new(),
        }
    }
}

/// A movement, keyed by `(junction, approach, departure, turn)` (ADR-0004).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Movement<V> {
    pub junction: String,
    pub approach: String,
    pub departure: String,
    pub turn: Turn,
    /// Demand volume, veh/s.
    pub volume: V,
}

/// `expected/lane_graph.toml`: lanes per leg and one connector set per
/// movement.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneGraph {
    #[serde(default = "Vec::new", rename = "leg")]
    pub legs: Vec<LegLanes>,
    #[serde(default = "Vec::new", rename = "connector_set")]
    pub connector_sets: Vec<ConnectorSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegLanes {
    pub junction: String,
    pub leg: String,
    pub approach_lanes: u32,
    pub departure_lanes: u32,
}

/// The lane connectors that carry one movement through the junction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectorSet {
    pub junction: String,
    pub approach: String,
    pub departure: String,
    pub turn: Turn,
    pub connectors: u32,
}

/// `expected/lane_groups.toml`: HCM lane groups per approach.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneGroups {
    #[serde(default = "Vec::new", rename = "lane_group")]
    pub lane_groups: Vec<LaneGroup>,
}

/// A lane group is identified by its approach and the turns it serves.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneGroup {
    pub junction: String,
    pub approach: String,
    pub turns: Vec<Turn>,
    pub lanes: u32,
}

/// `expected/hcm.toml`: control delay, v/c and LOS per lane group, approach
/// and intersection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Hcm<V> {
    #[serde(default = "Vec::new", rename = "lane_group")]
    pub lane_groups: Vec<HcmLaneGroup<V>>,
    #[serde(default = "Vec::new", rename = "approach")]
    pub approaches: Vec<HcmApproach<V>>,
    #[serde(default = "Vec::new", rename = "intersection")]
    pub intersections: Vec<HcmIntersection<V>>,
}

impl<V> Default for Hcm<V> {
    fn default() -> Self {
        Self {
            lane_groups: Vec::new(),
            approaches: Vec::new(),
            intersections: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HcmLaneGroup<V> {
    pub junction: String,
    pub approach: String,
    pub turns: Vec<Turn>,
    /// Control delay, s/veh.
    pub control_delay: V,
    pub volume_to_capacity: V,
    pub los: Los,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HcmApproach<V> {
    pub junction: String,
    pub approach: String,
    /// Control delay, s/veh.
    pub control_delay: V,
    pub los: Los,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HcmIntersection<V> {
    pub junction: String,
    /// Control delay, s/veh.
    pub control_delay: V,
    pub los: Los,
}

/// `expected/landxml.toml`: element types, lengths and stations for each
/// alignment after export and re-import.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LandXml<V> {
    #[serde(default = "Vec::new", rename = "alignment")]
    pub alignments: Vec<Alignment<V>>,
}

impl<V> Default for LandXml<V> {
    fn default() -> Self {
        Self {
            alignments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Alignment<V> {
    pub name: String,
    /// Station at the start of the alignment, m.
    pub start_station: V,
    #[serde(default = "Vec::new", rename = "element")]
    pub elements: Vec<Element<V>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Element<V> {
    pub kind: ElementKind,
    /// Length along the centreline, m.
    pub length: V,
    /// Station at the start of the element, m.
    pub start_station: V,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementKind {
    Line,
    Arc,
    Clothoid,
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ElementKind::Line => "line",
            ElementKind::Arc => "arc",
            ElementKind::Clothoid => "clothoid",
        })
    }
}
