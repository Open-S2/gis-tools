use crate::{parsers::RGBA, readers::csv::parse_csv_as_record};
use alloc::{collections::BTreeMap, string::String};
use s2json::MValueCompatible;

/// Indicates the type of transportation used on a route.
/// Valid options include:
/// 0 - Tram
/// 1 - Subway
/// 2 - Rail
/// 3 - Bus
/// 4 - Ferry,
/// 5 - Cable tram
/// 6 - Aerial lift
/// 7 - Funicular,
/// 11 - Trolleybus
/// 12 - Monorail.
pub enum GTFSRouteType {
    /// 0 - Tram
    Tram = 0,
    /// 1 - Subway
    Subway = 1,
    /// 2 - Rail
    Rail = 2,
    /// 3 - Bus
    Bus = 3,
    /// 4 - Ferry
    Ferry = 4,
    /// 5 - Cable tram
    CableTram = 5,
    /// 6 - Aerial lift
    AerialLift = 6,
    /// 7 - Funicular
    Funicular = 7,
    /// 11 - Trolleybus
    Trolleybus = 11,
    /// 12 - Monorail
    Monorail = 12,
}
impl From<i8> for GTFSRouteType {
    fn from(s: i8) -> Self {
        match s {
            1 => GTFSRouteType::Subway,
            2 => GTFSRouteType::Rail,
            3 => GTFSRouteType::Bus,
            4 => GTFSRouteType::Ferry,
            5 => GTFSRouteType::CableTram,
            6 => GTFSRouteType::AerialLift,
            7 => GTFSRouteType::Funicular,
            11 => GTFSRouteType::Trolleybus,
            12 => GTFSRouteType::Monorail,
            _ => GTFSRouteType::Tram,
        }
    }
}

/// Continuous pickup setting for the entire route.
/// 0 - Continuous stopping pickup,
/// 1/empty - No continuous stopping pickup,
/// 2 - Must phone agency,
/// 3 - Must coordinate with driver.
pub enum GTFSRoutePickupType {
    /// 0 - Continuous stopping pickup
    ContinuousStoppingPickup = 0,
    /// 1/empty - No continuous stopping pickup
    NoContinuousStoppingPickup = 1,
    /// 2 - Must phone agency
    MustPhoneAgency = 2,
    /// 3 - Must coordinate with driver
    MustCoordinateWithDriver = 3,
}
impl From<i8> for GTFSRoutePickupType {
    fn from(s: i8) -> Self {
        match s {
            0 => GTFSRoutePickupType::ContinuousStoppingPickup,
            2 => GTFSRoutePickupType::MustPhoneAgency,
            3 => GTFSRoutePickupType::MustCoordinateWithDriver,
            _ => GTFSRoutePickupType::NoContinuousStoppingPickup,
        }
    }
}

/// # Route Information
///
/// ## Details
/// **Required** - Transit routes. A route is a group of trips that are displayed to riders as a
/// single service.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GTFSRoute {
    /// **Required**
    /// Identifies a route.
    pub route_id: String,
    /// **Conditionally Required**
    /// Agency for the specified route.
    /// Required if multiple agencies are defined in agency.txt.
    pub agency_id: Option<String>,
    /// **Conditionally Required**
    /// Short name of a route, e.g. "32", "100X", "Green".
    /// Required if `route_long_name` is empty. Recommended otherwise.
    pub route_short_name: Option<String>,
    /// **Conditionally Required**
    /// Full name of a route, generally more descriptive than `shortName`.
    /// Required if `route_short_name` is empty. Optional otherwise.
    pub route_long_name: Option<String>,
    /// **Optional**
    /// Description of a route providing useful info, not a duplicate of short/long name.
    pub route_desc: Option<String>,
    /// **Required**
    /// Indicates the type of transportation used on a route.
    /// Valid options include:
    /// 0 - Tram, 1 - Subway, 2 - Rail, 3 - Bus, 4 - Ferry,
    /// 5 - Cable tram, 6 - Aerial lift, 7 - Funicular,
    /// 11 - Trolleybus, 12 - Monorail.
    pub route_type: i8,
    /// **Optional**
    /// URL of a web page about the route. Should differ from `agency.agency_url`.
    pub route_url: Option<String>,
    /// **Optional**
    /// Route color (hex) matching public-facing material. Defaults to `FFFFFF` if empty.
    pub route_color: Option<String>,
    /// **Optional**
    /// Text color (hex) used against the `route_color`. Defaults to `000000` if empty.
    pub route_text_color: Option<String>,
    /// **Optional**
    /// Orders routes for ideal presentation (smaller values displayed first).
    pub route_sort_order: Option<i8>,
    /// **Conditionally Forbidden**
    /// Continuous pickup setting for the entire route.
    /// 0 - Continuous stopping pickup,
    /// 1/empty - No continuous stopping pickup,
    /// 2 - Must phone agency,
    /// 3 - Must coordinate with driver.
    ///
    /// Forbidden if `stop_times.start_pickup_drop_off_window`
    /// or `stop_times.end_pickup_drop_off_window` are used.
    pub continuous_pickup: Option<i8>,
    /// **Conditionally Forbidden**
    /// Continuous drop-off setting for the entire route.
    /// 0 - Continuous stopping drop off,
    /// 1/empty - No continuous stopping drop off,
    /// 2 - Must phone agency,
    /// 3 - Must coordinate with driver.
    ///
    /// Forbidden if `stop_times.start_pickup_drop_off_window`
    /// or `stop_times.end_pickup_drop_off_window` are used.
    pub continuous_drop_off: Option<i8>,
    /// **Conditionally Forbidden**
    /// Identifies a group of routes. Multiple rows may share the same `network_id`.
    /// Forbidden if `route_networks.txt` is used.
    pub network_id: Option<String>,
}
impl GTFSRoute {
    /// Create a new GTFSRoute
    pub fn new(source: &str) -> BTreeMap<String, GTFSRoute> {
        let mut res = BTreeMap::new();
        for record in parse_csv_as_record::<GTFSRoute>(source, None, None) {
            res.insert(record.route_id.clone(), record);
        }
        res
    }
    /// Get route type
    pub fn get_route_type(&self) -> GTFSRouteType {
        GTFSRouteType::from(self.route_type)
    }
    /// Get the continuous_pickup
    pub fn get_continuous_pickup(&self) -> Option<GTFSRoutePickupType> {
        self.continuous_pickup.map(GTFSRoutePickupType::from)
    }
    /// Get the continuous_drop_off
    pub fn get_continuous_drop_off(&self) -> Option<GTFSRoutePickupType> {
        self.continuous_drop_off.map(GTFSRoutePickupType::from)
    }
    /// Get the route color
    pub fn get_route_color(&self) -> Option<RGBA> {
        self.route_color.as_ref().map(|c| RGBA::from_hex(c))
    }
    /// Get the route text color
    pub fn get_route_text_color(&self) -> Option<RGBA> {
        self.route_text_color.as_ref().map(|c| RGBA::from_hex(c))
    }
}
