// https://gtfs.org/documentation/schedule/reference/#agencytxt
mod agency;
mod areas;
mod attributions;
mod booking_rules;
mod calendar;
mod calendar_dates;
mod fare_attributes;
mod fare_leg_join_rules;
mod fare_leg_rules;
mod fare_media;
mod fare_products;
mod fare_rules;
mod fare_transfer_rules;
mod feed_info;
mod frequencies;
mod levels;
mod location_group_stops;
mod location_groups;
mod networks;
mod pathways;
mod route_networks;
mod routes;
mod shapes;
mod stop_areas;
mod stop_times;
mod stops;
mod timeframes;
mod transfers;
mod translations;
mod trips;

use crate::{
    parsers::FeatureReader,
    readers::json::{JSONCollectionReader, ToGisJSON},
    util::iter_zip_folder,
};
pub use agency::*;
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
pub use areas::*;
pub use attributions::*;
pub use booking_rules::*;
pub use calendar::*;
pub use calendar_dates::*;
pub use fare_attributes::*;
pub use fare_leg_join_rules::*;
pub use fare_leg_rules::*;
pub use fare_media::*;
pub use fare_products::*;
pub use fare_rules::*;
pub use fare_transfer_rules::*;
pub use feed_info::*;
pub use frequencies::*;
pub use levels::*;
pub use location_group_stops::*;
pub use location_groups::*;
pub use networks::*;
pub use pathways::*;
pub use route_networks::*;
pub use routes::*;
use s2json::{MValue, MValueCompatible, Properties, VectorFeature};
use serde::{Deserialize, Serialize};
pub use shapes::*;
pub use stop_areas::*;
pub use stop_times::*;
pub use stops::*;
pub use timeframes::*;
pub use transfers::*;
pub use translations::*;
pub use trips::*;

// TODO: postprocess all interactions like `Trips -> shape_id [Link]` & `StopTime -> On-demand Service Routing Behavior [Link]`

/// A piece of the GTFS schedule
#[derive(Debug, Clone, PartialEq)]
pub struct Piece {
    /// The name of the file
    pub filename: String,
    /// The contents of the file
    pub data: String,
}

/// `locations.geojson` data properties
/// Defines zones where riders can request either pickup or drop off by on-demand services.
/// These zones are represented as GeoJSON polygons.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
pub struct GTFSLocationsProperties {
    /// The name of the stop
    pub stop_name: String,
    /// The description of the stop
    pub stop_desc: String,
}

/// # GTFS Schedule Reader
///
/// ## Description
/// Schedule class that pulls in all of the GTFS schedule files and parses them into a single object
/// implements the {@link FeatureIterator} interface.
///
/// ## Usage
/// ```ts
/// import { buildGTFSSchedule } from 'gis-tools-ts';
///
/// const schedule = await buildGTFSSchedule(gzipData);
///
/// for await (const feature of schedule) {
///   console.log(feature);
/// }
/// ```
///
/// ## Links
/// - https://mobilitydatabase.org
/// - https://developers.google.com/transit/gtfs/examples/overview
/// - https://gtfs.org/documentation/schedule/reference/#tripstxt
/// - https://mobilitydata.github.io/
/// - https://www.transit.land
#[derive(Debug, Default, Clone)]
pub struct GTFSScheduleReader {
    /// Agencies
    pub agencies: BTreeMap<String, GTFSAgency>,
    /// Areas
    pub areas: BTreeMap<String, GTFSArea>,
    /// Attributions
    pub attributions: BTreeMap<String, GTFSAttribution>,
    /// Booking Rules
    pub booking_rules: BTreeMap<String, GTFSBookingRule>,
    /// Calendar
    pub calendar: Vec<GTFSCalendar>,
    /// Calendar Dates
    pub calendar_dates: BTreeMap<String, GTFSCalendarDate>,
    /// Fare Attributes
    pub fare_attributes: BTreeMap<String, GTFSFareAttribute>,
    /// Fare Leg Join Rules
    pub fare_leg_join_rules: Vec<GTFSFareLegJoinRule>,
    /// Fare Leg Rules
    pub fare_leg_rules: Vec<GTFSFareLegRule>,
    /// Fare Media
    pub fare_media: BTreeMap<String, GTFSFareMedia>,
    /// Fare Products
    pub fare_products: BTreeMap<String, GTFSFareProduct>,
    /// Fare Rules
    pub fare_rules: Vec<GTFSFareRule>,
    /// Fare Transfer Rules
    pub fare_transfer_rules: Vec<GTFSFareTransferRule>,
    /// Feed Info
    pub feed_info: BTreeMap<String, GTFSFeedInfo>,
    /// Frequencies
    pub frequencies: Vec<GTFSFrequency>,
    /// Levels
    pub levels: BTreeMap<String, GTFSLevel>,
    /// Location Groups
    pub location_groups: BTreeMap<String, GTFSLocationGroup>,
    /// Location Group Stops
    pub location_group_stops: Vec<GTFSLocationGroupStop>,
    /// Networks
    pub networks: BTreeMap<String, GTFSNetwork>,
    /// Pathways
    pub pathways: BTreeMap<String, GTFSPathway>,
    /// Route Networks
    pub route_networks: Vec<GTFSRouteNetwork>,
    /// Routes
    pub routes: BTreeMap<String, GTFSRoute>,
    /// Shapes
    pub shapes: BTreeMap<String, Vec<GTFSShape>>,
    /// Stop Areas
    pub stop_areas: Vec<GTFSStopArea>,
    /// Stop Areas
    pub stops: BTreeMap<String, GTFSStop>,
    /// Stop Times
    pub stop_times: Vec<GTFSStopTime>,
    /// Timeframes
    pub timeframes: BTreeMap<String, GTFSTimeframe>,
    /// Transfers
    pub transfers: Vec<GTFSTransfer>,
    /// Translations
    pub translations: Vec<GTFSTranslation>,
    /// Trips
    pub trips: BTreeMap<String, GTFSTrip>,
    /// Geojson
    pub geojson: Option<JSONCollectionReader>,
}
impl GTFSScheduleReader {
    /// Create a new GTFSScheduleReader
    pub fn new(pieces: &[Piece]) -> Self {
        let mut res = GTFSScheduleReader::default();

        for Piece { filename, data } in pieces {
            let stem = filename.split('.').next().unwrap_or("");
            match stem {
                "agency" => res.agencies = GTFSAgency::new(data),
                "areas" => res.areas = GTFSArea::new(data),
                "attributions" => res.attributions = GTFSAttribution::new(data),
                "booking_rules" => res.booking_rules = GTFSBookingRule::new(data),
                "calendar" => res.calendar = GTFSCalendar::new(data),
                "calendar_dates" => res.calendar_dates = GTFSCalendarDate::new(data),
                "fare_attributes" => res.fare_attributes = GTFSFareAttribute::new(data),
                "fare_leg_join_rules" => res.fare_leg_join_rules = GTFSFareLegJoinRule::new(data),
                "fare_leg_rules.txt" => res.fare_leg_rules = GTFSFareLegRule::new(data),
                "fare_media" => res.fare_media = GTFSFareMedia::new(data),
                "fare_products" => res.fare_products = GTFSFareProduct::new(data),
                "fare_rules" => res.fare_rules = GTFSFareRule::new(data),
                "fare_transfer_rules" => res.fare_transfer_rules = GTFSFareTransferRule::new(data),
                "feed_info" => res.feed_info = GTFSFeedInfo::new(data),
                "frequencies" => res.frequencies = GTFSFrequency::new(data),
                "levels" => res.levels = GTFSLevel::new(data),
                "location_groups" => res.location_groups = GTFSLocationGroup::new(data),
                "location_group_stops" => {
                    res.location_group_stops = GTFSLocationGroupStop::new(data)
                }
                "networks" => res.networks = GTFSNetwork::new(data),
                "pathways" => res.pathways = GTFSPathway::new(data),
                "route_networks" => res.route_networks = GTFSRouteNetwork::new(data),
                "routes" => res.routes = GTFSRoute::new(data),
                "shapes" => res.shapes = GTFSShape::new(data),
                "stop_areas" => res.stop_areas = GTFSStopArea::new(data),
                "stops" => res.stops = GTFSStop::new(data),
                "stop_times" => res.stop_times = GTFSStopTime::new(data),
                "timeframes" => res.timeframes = GTFSTimeframe::new(data),
                "transfers" => res.transfers = GTFSTransfer::new(data),
                "translations" => res.translations = GTFSTranslation::new(data),
                "trips" => res.trips = GTFSTrip::new(data),
                "locations" => {
                    if let Ok(mut feature_collection) = data.as_str().to_feature_collection() {
                        res.geojson = Some(JSONCollectionReader::from(&mut feature_collection));
                    }
                }
                _ => {}
            }
        }

        res
    }

    /// Builds a GTFS Schedule Reader from a gzip folder
    ///
    /// @param gzipData - the gzip folder to parse
    /// @returns - a Schedule class
    pub fn from_gzip(gzip_data: &[u8]) -> Self {
        let mut pieces: Vec<Piece> = vec![];

        for item in iter_zip_folder(gzip_data).unwrap() {
            if let Ok(read_data) = (item.read)() {
                pieces.push(Piece {
                    filename: item.filename,
                    data: String::from_utf8_lossy(&read_data).into(),
                });
            }
        }

        GTFSScheduleReader::new(&pieces)
    }

    /// Build a GTFS Schedule Reader from a standard folder
    #[cfg(feature = "std")]
    pub fn from_folder(folder_path: &str) -> Self {
        let mut pieces: Vec<Piece> = vec![];

        for entry in std::fs::read_dir(folder_path).unwrap().flatten() {
            if let Ok(read_data) = std::fs::read(entry.path()) {
                pieces.push(Piece {
                    filename: entry.file_name().to_str().unwrap().into(),
                    data: String::from_utf8_lossy(&read_data).into(),
                });
            }
        }

        GTFSScheduleReader::new(&pieces)
    }

    /// Collect all known vector features
    pub fn collect_vector_features(&self) -> Vec<VectorFeature> {
        let mut res = vec![];
        // add stops
        for stop in self.stops.values() {
            if let Some(feature) = stop.to_feature() {
                res.push(feature);
            }
        }
        // add geojson
        if let Some(geojson) = &self.geojson {
            for feature in geojson.iter() {
                res.push(feature);
            }
        }
        // add shapes
        for shape in self.shapes.values() {
            let gtfs_shapes: GTFSShapes = shape.into();
            res.push(gtfs_shapes.into());
        }

        res
    }
}

/// The GTFS Schedule Iterator tool
#[derive(Debug)]
pub struct GTFSScheduleIterator {
    features: Vec<VectorFeature>,
    index: usize,
}
impl Iterator for GTFSScheduleIterator {
    type Item = VectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.features.get(self.index - 1).cloned()
    }
}
/// A feature reader trait with a callback-based approach
impl FeatureReader<(), Properties, MValue> for GTFSScheduleReader {
    type FeatureIterator<'a> = GTFSScheduleIterator;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        GTFSScheduleIterator { features: self.collect_vector_features(), index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
