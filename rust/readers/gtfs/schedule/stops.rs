use crate::readers::csv::parse_csv_as_record;
use alloc::{collections::BTreeMap, string::String};
use s2json::{MValueCompatible, VectorFeature, VectorGeometry, VectorPoint};

/// Properties object from GTFS stops
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSStopProperties {
    /// Stop ID
    pub stop_id: String,
    /// Stop code
    pub stop_code: String,
    /// Stop name
    pub stop_name: String,
    /// TTS stop name
    pub tts_stop_name: String,
    /// Stop description
    pub stop_desc: String,
    /// Stop zone id
    pub zone_id: String,
    /// Stop url
    pub stop_url: String,
    /// Location type
    pub location_type: i8,
    /// Parent station
    pub parent_station: String,
    /// Stop timezone
    pub stop_timezone: String,
    /// Wheelchair boarding
    pub wheelchair_boarding: i8,
    /// Level id
    pub level_id: String,
    /// Platform code
    pub platform_code: String,
}
impl From<&GTFSStop> for GTFSStopProperties {
    fn from(stop: &GTFSStop) -> Self {
        GTFSStopProperties {
            stop_id: stop.stop_id.clone(),
            stop_code: stop.stop_code.clone(),
            stop_name: stop.stop_name.clone(),
            tts_stop_name: stop.tts_stop_name.clone(),
            stop_desc: stop.stop_desc.clone(),
            zone_id: stop.zone_id.clone(),
            stop_url: stop.stop_url.clone(),
            location_type: stop.location_type.unwrap_or(0),
            parent_station: stop.parent_station.clone(),
            stop_timezone: stop.stop_timezone.clone(),
            wheelchair_boarding: stop.wheelchair_boarding.unwrap_or(0),
            level_id: stop.level_id.clone(),
            platform_code: stop.platform_code.clone(),
        }
    }
}

/// Location type. Valid options:
/// - 0 or empty = Stop/Platform,
/// - 1 = Station,
/// - 2 = Entrance/Exit,
/// - 3 = Generic Node,
/// - 4 = Boarding Area.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum GTFSStopLocationType {
    /// Stop
    Stop = 0,
    /// Station
    Station = 1,
    /// Entrance
    Entrance = 2,
    /// Generic Node
    GenericNode = 3,
    /// Boarding Area
    BoardingArea = 4,
}
impl From<i8> for GTFSStopLocationType {
    fn from(value: i8) -> Self {
        match value {
            1 => GTFSStopLocationType::Station,
            2 => GTFSStopLocationType::Entrance,
            3 => GTFSStopLocationType::GenericNode,
            4 => GTFSStopLocationType::BoardingArea,
            _ => GTFSStopLocationType::Stop,
        }
    }
}

/// # Stop Information
///
/// ## Details
/// **Conditionally Required** - Stops where vehicles pick up or drop off riders.
/// Also defines stations, entrances, etc.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSStop {
    /// **Required**
    /// Identifies a location: stop/platform, station, entrance/exit, generic node, or boarding area.
    /// Must be unique across:
    /// - stops.stop_id
    /// - locations.geojson id
    /// - location_groups.location_group_id
    ///
    /// Multiple routes may use the same `stop_id`.
    pub stop_id: String,
    /// **Optional**
    /// Short text or a number that identifies the location for riders.
    pub stop_code: String,
    /// **Conditionally Required**
    /// Name of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
    pub stop_name: String,
    /// **Optional**
    /// Readable version of the stop_name for text-to-speech systems.
    pub tts_stop_name: String,
    /// **Optional**
    /// Description providing useful information about the location.
    /// Should not be a duplicate of `name`.
    pub stop_desc: String,
    /// **Conditionally Required**
    /// Latitude of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
    pub stop_lat: Option<f64>,
    /// **Conditionally Required**
    /// Longitude of the location. Required if `location_type` is 0, 1, or 2. Optional otherwise.
    pub stop_lon: Option<f64>,
    /// **Optional**
    /// Identifies the fare zone for a stop.
    pub zone_id: String,
    /// **Optional**
    /// URL of a web page about this location.
    pub stop_url: String,
    /// **Optional**
    /// Location type. Valid options:
    /// 0 or empty = Stop/Platform,
    /// 1 = Station,
    /// 2 = Entrance/Exit,
    /// 3 = Generic Node,
    /// 4 = Boarding Area.
    pub location_type: Option<i8>,
    /// **Conditionally Required**
    /// Defines hierarchy between different locations. Required if `location_type` is 2, 3, or 4.
    pub parent_station: String,
    /// **Optional**
    /// Timezone of the location. Inherits from parent station if not specified.
    pub stop_timezone: String,
    /// **Optional**
    /// Indicates whether wheelchair boardings are possible at this location.
    /// For parentless stops: 0 = no info, 1 = possible, 2 = not possible.
    /// For child stops, entrance/exits: inherits or overrides parent station accessibility.
    pub wheelchair_boarding: Option<i8>,
    /// **Optional**
    /// Level of the location. References levels.level_id.
    pub level_id: String,
    /// **Optional**
    /// Platform identifier for a platform stop.
    pub platform_code: String,
}
impl GTFSStop {
    /// Create a new GTFSStop
    pub fn new(source: &str) -> BTreeMap<String, GTFSStop> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSStop>(source, None, None) {
            res.insert(record.stop_id.clone(), record);
        }
        res
    }
    /// Get the location_type
    pub fn get_location_type(&self) -> Option<GTFSStopLocationType> {
        self.location_type.map(GTFSStopLocationType::from)
    }
    /// Convert to a feature
    pub fn to_feature(&self) -> Option<VectorFeature> {
        if self.stop_lon.is_none() || self.stop_lat.is_none() {
            return None;
        }
        let properties: GTFSStopProperties = self.into();
        Some(VectorFeature {
            properties: properties.into(),
            geometry: VectorGeometry::new_point(
                VectorPoint::new_xy(self.stop_lon.unwrap(), self.stop_lat.unwrap(), None),
                None,
            ),
            ..Default::default()
        })
    }
}
