use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Describes the type of pathway between two stops or station nodes.
///
/// 1 - Walkway
/// 2 - Stairs
/// 3 - Moving sidewalk (travelator)
/// 4 - Escalator
/// 5 - Elevator
/// 6 - Fare gate (payment gate)
/// 7 - Exit gate
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSPathwayMode {
    /// 1 - Walkway
    Walkway = 1,
    /// 2 - Stairs
    Stairs = 2,
    /// 3 - Moving sidewalk (travelator)
    MovingSidewalk = 3,
    /// 4 - Escalator
    Escalator = 4,
    /// 5 - Elevator
    Elevator = 5,
    /// 6 - Fare gate (payment gate)
    FareGate = 6,
    /// 7 - Exit gate
    ExitGate = 7,
}
impl From<i8> for GTFSPathwayMode {
    fn from(s: i8) -> Self {
        match s {
            2 => GTFSPathwayMode::Stairs,
            3 => GTFSPathwayMode::MovingSidewalk,
            4 => GTFSPathwayMode::Escalator,
            5 => GTFSPathwayMode::Elevator,
            6 => GTFSPathwayMode::FareGate,
            7 => GTFSPathwayMode::ExitGate,
            _ => GTFSPathwayMode::Walkway,
        }
    }
}

/// Indicates whether a pathway can be used in both directions:
///
/// 0 - Unidirectional
/// 1 - Bidirectional
///
/// Note: Exit gates (pathway_mode=7) must not be bidirectional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSIsBidirectional {
    /// 0 - Unidirectional
    Unidirectional = 0,
    /// 1 - Bidirectional
    Bidirectional = 1,
}
impl From<i8> for GTFSIsBidirectional {
    fn from(s: i8) -> Self {
        match s {
            1 => GTFSIsBidirectional::Bidirectional,
            _ => GTFSIsBidirectional::Unidirectional,
        }
    }
}

/// # Pathways
///
/// **Optional**
/// Represents edges in a station graph describing station interiors, connecting
/// platforms, entrances/exits, generic nodes, or boarding areas.
///
/// **Primary Key**: (pathway_id)
///
/// Pathways must be complete if included:
/// - No dangling locations if any pathways exist, except for platforms that have boarding areas.
/// - Platforms with boarding areas must not have pathways directly; their boarding areas do.
/// - Each platform (location_type=0) or boarding area (4) must have at least
///   one path to an entrance/exit (2) unless it’s impossible for riders to exit at that platform.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSPathway {
    /// **Required**
    /// Unique ID for the pathway record.
    pub pathway_id: String,
    /// **Required**
    /// The stop or node from which this pathway begins.
    /// Must be location_type=0, 2, 3, or 4 (platform, entrance/exit, generic node, or boarding area).
    /// Stations (location_type=1) are forbidden here.
    pub from_stop_id: String,
    /// **Required**
    /// The stop or node at which this pathway ends.
    /// Must be location_type=0, 2, 3, or 4 (platform, entrance/exit, generic node, or boarding area).
    /// Stations (location_type=1) are forbidden here.
    pub to_stop_id: String,
    /// **Required**
    /// Pathway mode, e.g. walkway, stairs, escalator.
    pub pathway_mode: i8, // GTFSPathwayMode;
    /// **Required**
    /// 0 = Unidirectional, 1 = Bidirectional
    pub is_bidirectional: i8, // GTFSIsBidirectional;
    /// **Optional**
    /// Horizontal length in meters of the pathway.
    /// Recommended for walkway, fare gate, exit gate.
    pub length: Option<f64>,
    /// **Optional**
    /// Average time in seconds needed to traverse this pathway.
    /// Recommended for moving sidewalk, escalator, elevator.
    pub traversal_time: Option<i64>,
    /// **Optional**
    /// Number of stairs in this pathway.
    /// Positive: fromStopId to toStopId goes upwards
    /// Negative: fromStopId to toStopId goes downwards
    /// Recommended for pathway_mode=2 (stairs).
    pub stair_count: Option<i32>,
    /// **Optional**
    /// Maximum slope ratio. Positive for upwards, negative for downwards.
    /// E.g., 0.083 is an 8.3% slope.
    /// Used for walkway (1) or moving sidewalk (3) if relevant.
    pub max_slope: Option<f64>,
    /// **Optional**
    /// Minimum width of the pathway in meters, recommended if less than 1 meter.
    pub min_width: Option<f64>,
    /// **Optional**
    /// Public facing text on signage to help riders navigate (e.g. "Follow signs to X").
    pub signposted_as: Option<String>,
    /// **Optional**
    /// Public facing text on signage when traversing the pathway in reverse
    /// (toStopId -> fromStopId), if different from `signpostedAs`.
    pub reversed_signposted_as: Option<String>,
}
impl GTFSPathway {
    /// Create a new GTFSPathway
    pub fn new(source: &str) -> BTreeMap<String, GTFSPathway> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSPathway>(source, None, None) {
            res.insert(record.pathway_id.clone(), record);
        }
        res
    }
    /// Get the pathway_mode
    pub fn get_pathway_mode(&self) -> GTFSPathwayMode {
        GTFSPathwayMode::from(self.pathway_mode)
    }
    /// Get the is_bidirectional
    pub fn get_is_bidirectional(&self) -> GTFSIsBidirectional {
        GTFSIsBidirectional::from(self.is_bidirectional)
    }
}
