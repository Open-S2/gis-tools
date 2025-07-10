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
use s2json::{MValueCompatible, VectorFeature};
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
            match filename.as_str() {
                "agency.txt" => res.agencies = GTFSAgency::new(data),
                "areas.txt" => res.areas = GTFSArea::new(data),
                "attributions.txt" => res.attributions = GTFSAttribution::new(data),
                "booking_rules.txt" => res.booking_rules = GTFSBookingRule::new(data),
                "calendar.txt" => res.calendar = GTFSCalendar::new(data),
                "calendar_dates.txt" => res.calendar_dates = GTFSCalendarDate::new(data),
                "fare_attributes.txt" => res.fare_attributes = GTFSFareAttribute::new(data),
                "fare_leg_join_rules.txt" => {
                    res.fare_leg_join_rules = GTFSFareLegJoinRule::new(data)
                }
                "fare_leg_rules.txt" => res.fare_leg_rules = GTFSFareLegRule::new(data),
                "fare_media.txt" => res.fare_media = GTFSFareMedia::new(data),
                "fare_products.txt" => res.fare_products = GTFSFareProduct::new(data),
                "fare_rules.txt" => res.fare_rules = GTFSFareRule::new(data),
                "fare_transfer_rules.txt" => {
                    res.fare_transfer_rules = GTFSFareTransferRule::new(data)
                }
                "feed_info.txt" => res.feed_info = GTFSFeedInfo::new(data),
                "frequencies.txt" => res.frequencies = GTFSFrequency::new(data),
                "levels.txt" => res.levels = GTFSLevel::new(data),
                "location_groups.txt" => res.location_groups = GTFSLocationGroup::new(data),
                "location_group_stops.txt" => {
                    res.location_group_stops = GTFSLocationGroupStop::new(data)
                }
                "networks.txt" => res.networks = GTFSNetwork::new(data),
                "pathways.txt" => res.pathways = GTFSPathway::new(data),
                "route_networks.txt" => res.route_networks = GTFSRouteNetwork::new(data),
                "routes.txt" => res.routes = GTFSRoute::new(data),
                "shapes.txt" => res.shapes = GTFSShape::new(data),
                "stop_areas.txt" => res.stop_areas = GTFSStopArea::new(data),
                "stops.txt" => res.stops = GTFSStop::new(data),
                "stop_times.txt" => res.stop_times = GTFSStopTime::new(data),
                "timeframes.txt" => res.timeframes = GTFSTimeframe::new(data),
                "transfers.txt" => res.transfers = GTFSTransfer::new(data),
                "translations.txt" => res.translations = GTFSTranslation::new(data),
                "trips.txt" => res.trips = GTFSTrip::new(data),
                "locations.geojson" => {
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
                let chunk = String::from_utf8_lossy(&read_data);
                pieces.push(Piece { filename: item.filename, data: chunk.into() });
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
        // add geosjon
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

//   /**
//    * TODO: Add proeprties from other files like "color"
//    * TODO: All features should be parsed as VectorGeometry
//    * Yields all of the shapes
//    * @yields an iterator that contains shapes, stops, location data, and routes
//    */
//   async *[Symbol.asyncIterator](): AsyncGenerator<
//     | VectorFeature<Record<string, unknown>, MValue, GTFSShapeProperties, VectorLineStringGeometry>
//     | VectorFeature<
//         Record<string, unknown>,
//         MValue,
//         GTFSLocationsProperties,
//         VectorMultiPolygonGeometry | VectorPolygonGeometry
//       >
//     | VectorFeature<undefined, MValue, GTFSStopProperties, VectorPointGeometry>
//   > {
//     if (this.geojson !== undefined) {
//       for await (const feature of this.geojson)
//         yield feature as VectorFeature<
//           Record<string, unknown>,
//           MValue,
//           GTFSLocationsProperties,
//           VectorMultiPolygonGeometry | VectorPolygonGeometry
//         >;
//     }
//     if (this.shapes !== undefined) {
//       for (const shape of Object.values(this.shapes)) yield shape;
//     }
//     if (this.stops !== undefined) {
//       for (const stop of Object.values(this.stops)) {
//         const { lon, lat } = stop;
//         if (lon !== undefined && lat !== undefined) {
//           const stopFeature: VectorFeature<
//             undefined,
//             MValue,
//             GTFSStopProperties,
//             VectorPointGeometry
//           > = {
//             type: 'VectorFeature',
//             properties: stop.properties(),
//             geometry: { type: 'Point', is3D: false, coordinates: { x: lon, y: lat } },
//           };
//           yield stopFeature;
//         }
//       }
//     }
//   }
// }
