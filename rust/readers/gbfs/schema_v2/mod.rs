mod free_bike_status;
mod gbfs;
mod gbfs_versions;
mod geofencing_zones;
mod station_information;
mod station_status;
mod system_alerts;
mod system_calendar;
mod system_hours;
mod system_information;
mod system_pricing_plans;
mod system_regions;
mod vehicle_types;

use crate::{
    geometry::{ConvertFeature, convert_geometry_to_vector},
    parsers::FeatureReader,
    util::fetch_url,
};
use alloc::{format, string::String, vec, vec::Vec};
pub use free_bike_status::*;
pub use gbfs::*;
pub use gbfs_versions::*;
pub use geofencing_zones::*;
use s2json::{Features, MValue, Properties, VectorFeature, VectorGeometry, VectorPoint};
use serde::{Deserialize, Serialize};
pub use station_information::*;
pub use station_status::*;
pub use system_alerts::*;
pub use system_calendar::*;
pub use system_hours::*;
pub use system_information::*;
pub use system_pricing_plans::*;
pub use system_regions::*;
pub use vehicle_types::*;

/// Geofencing Feature */
pub type GBFSGeofencingFeatureV2 = VectorFeature<(), GBFSGeofencingZonesV2Properties, MValue>;

/// Station Information feature properties
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSStationV2FeaturesProperties {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short name
    pub short_name: Option<String>,
    /// Address
    pub address: Option<String>,
    /// Cross street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post code
    pub post_code: Option<String>,
    /// Is virtual
    pub is_virtual_station: Option<bool>,
    /// Parking type
    pub parking_type: Option<String>,
    /// Parking hoop
    pub parking_hoop: Option<bool>,
    /// Contact phone
    pub contact_phone: Option<String>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Is valet station
    pub is_valet_station: Option<bool>,
    /// Is charging station
    pub is_charging_station: Option<bool>,
}
impl From<&GBFSStationInformationStationV23> for GBFSStationV2FeaturesProperties {
    fn from(station: &GBFSStationInformationStationV23) -> Self {
        GBFSStationV2FeaturesProperties {
            station_id: station.station_id.clone(),
            name: station.name.clone(),
            short_name: station.short_name.clone(),
            address: station.address.clone(),
            cross_street: station.cross_street.clone(),
            region_id: station.region_id.clone(),
            post_code: station.post_code.clone(),
            is_virtual_station: station.is_virtual_station,
            parking_type: station.parking_type.clone().map(|s| serde_json::to_string(&s).unwrap()),
            parking_hoop: station.parking_hoop,
            contact_phone: station.contact_phone.clone(),
            capacity: station.capacity,
            is_valet_station: station.is_valet_station,
            is_charging_station: station.is_charging_station,
        }
    }
}

/// Station Information Point Feature */
pub type GBFSStationFeatureV2 = VectorFeature<(), GBFSStationV2FeaturesProperties, MValue>;

/// Bike Information Point Feature
pub type GBFSBikeFeatureV2 = VectorFeature<(), GBFSFreeBikeStatusBikeV23, MValue>;

/// GBFS Version 2 Reader
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSReaderV2 {
    /// Free Bike Status
    pub free_bike_status: Option<GBFSFreeBikeStatusV2>,
    /// GBFS
    pub gbfs: GBFSV2,
    /// GBFS Versions
    pub gbfs_versions: Option<GBFSVersionsV2>,
    /// Geofencing
    pub geofencing_zones: Option<GBFSGeofencingZonesV2>,
    /// Station Information
    pub station_information: Option<GBFSStationInformationV2>,
    /// Station Status
    pub station_status: Option<GBFSStationStatusV2>,
    /// System Alerts
    pub system_alerts: Option<GBFSSystemAlertsV2>,
    /// System Calendar
    pub system_calendar: Option<GBFSSystemCalendarV2>,
    /// System Hours
    pub system_hours: Option<GBFSSystemHoursV2>,
    /// System Information
    pub system_information: GBFSSystemInformationV2,
    /// System Pricing Plans
    pub system_pricing_plans: Option<GBFSSystemPricingPlansV2>,
    /// System Regions
    pub system_regions: Option<GBFSSystemRegionsV2>,
    /// Vehicle Types
    pub vehicle_types: Option<GBFSVehicleTypesV2>,
}
impl GBFSReaderV2 {
    /// Get all features from the GBFS V2 data
    pub fn features(&self) -> Vec<VectorFeature> {
        let mut res = vec![];

        res.extend(self.station_features().iter().map(|f| f.to_m_vector_feature(|_| None)));
        res.extend(self.bike_features().iter().map(|f| f.to_m_vector_feature(|_| None)));
        res.extend(self.geofencing_features().iter().map(|f| f.to_m_vector_feature(|_| None)));

        res
    }

    /// Get all station features from the GBFS V2 data
    pub fn station_features(&self) -> Vec<GBFSStationFeatureV2> {
        let mut res = vec![];

        if let Some(station_information) = &self.station_information {
            for station in &station_information.data.stations {
                let properties = GBFSStationV2FeaturesProperties::from(station);
                res.push(VectorFeature::new_wm(
                    None,
                    properties.clone(),
                    VectorGeometry::new_point(VectorPoint::from_xy(station.lon, station.lat), None),
                    None,
                ));
                if let Some(station_area) = &station.station_area {
                    res.push(VectorFeature::new_wm(
                        None,
                        properties,
                        convert_geometry_to_vector(station_area, true),
                        None,
                    ));
                }
            }
        }

        res
    }

    /// Get Geofencing features from the GBFS V2 data
    pub fn geofencing_features(&self) -> Vec<GBFSGeofencingFeatureV2> {
        let mut res = vec![];

        if let Some(geofencing_zones) = &self.geofencing_zones {
            for feature in &geofencing_zones.data.geofencing_zones.features {
                match feature {
                    Features::Feature(f) => {
                        res.push(f.to_vector(Some(true)));
                    }
                    Features::VectorFeature(vf) => res.push(vf.clone()),
                }
            }
        }

        res
    }

    /// Get all bike features from the GBFS V2 data
    pub fn bike_features(&self) -> Vec<GBFSBikeFeatureV2> {
        let mut res = vec![];

        if let Some(free_bike_status) = &self.free_bike_status {
            for bike in &free_bike_status.data.bikes {
                if let (Some(lon), Some(lat)) = (bike.lon, bike.lat) {
                    res.push(VectorFeature::new_wm(
                        None,
                        bike.clone(),
                        VectorGeometry::new_point(VectorPoint::from_xy(lon, lat), None),
                        None,
                    ));
                }
            }
        }

        res
    }
}

/// The GBFS V1 Iterator tool
#[derive(Debug)]
pub struct GBFSIteratorV2 {
    features: Vec<VectorFeature>,
    index: usize,
    len: usize,
}
impl Iterator for GBFSIteratorV2 {
    type Item = VectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        self.index += 1;
        self.features.get(self.index - 1).cloned()
    }
}
/// A feature reader trait with a callback-based approach
impl FeatureReader<(), Properties, MValue> for GBFSReaderV2 {
    type FeatureIterator<'a> = GBFSIteratorV2;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        let features = self.features();
        let len = features.len();
        GBFSIteratorV2 { features, index: 0, len }
    }

    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        let features = self.features();
        let start = features.len() * thread_id / pool_size;
        let end = features.len() * (thread_id + 1) / pool_size;
        GBFSIteratorV2 { features, index: start, len: end }
    }
}

/// Parse a GBFS V2 schema and build a V2 GBFS reader
///
/// ## Parameters
/// - `gbfs`: the GBFS schema to parse
/// - `locale`: the locale to use if provided, otherwise default to en
/// - `path`: if provided, will use this path instead of the url (for testing)
///
/// ## Returns
/// The GBFS reader
pub async fn build_gbfs_reader_v2(
    gbfs: &GBFSV2,
    locale: Option<String>,
    path: Option<String>,
) -> GBFSReaderV2 {
    let locale = locale.unwrap_or("en".into());
    let data = &gbfs.data;
    let first_locale = data.keys().next().unwrap();
    let feeds = data.get(&locale).unwrap_or(data.get(first_locale).unwrap());
    let feeds = feeds.feeds.clone();

    let mut reader = GBFSReaderV2 { gbfs: gbfs.clone(), ..Default::default() };

    for feed in feeds {
        let name = serde_json::to_string(&feed.name).unwrap();
        if &name == "gbfs" {
            continue;
        }
        let url = if let Some(ref path) = path {
            format!("{}/{}.json", path, name.trim_matches('"'))
        } else {
            feed.url
        };

        if let Ok(url_data) = fetch_url::<()>(&url, &[], None, None).await {
            match feed.name {
                GBFSV21FeedsName::FreeBikeStatus => {
                    reader.free_bike_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::Gbfs => {}
                GBFSV21FeedsName::GbfsVersions => {
                    reader.gbfs_versions = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::StationInformation => {
                    reader.station_information = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::StationStatus => {
                    reader.station_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::SystemAlerts => {
                    reader.system_alerts = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::SystemCalendar => {
                    reader.system_calendar = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::SystemHours => {
                    reader.system_hours = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::SystemInformation => {
                    reader.system_information = serde_json::from_slice(&url_data).unwrap();
                }
                GBFSV21FeedsName::SystemPricingPlans => {
                    reader.system_pricing_plans = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::SystemRegions => {
                    reader.system_regions = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::VehicleTypes => {
                    reader.vehicle_types = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV21FeedsName::GeofencingZones => {
                    reader.geofencing_zones = Some(serde_json::from_slice(&url_data).unwrap());
                }
            }
        }
    }

    reader
}
