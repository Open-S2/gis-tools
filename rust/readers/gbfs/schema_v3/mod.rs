mod gbfs;
mod gbfs_versions;
mod geofencing_zones;
mod manifest;
mod station_information;
mod station_status;
mod system_alerts;
mod system_information;
mod system_pricing_plans;
mod system_regions;
mod vehicle_status;
mod vehicle_types;

use crate::{
    geometry::{ConvertFeature, convert_geometry_to_vector},
    parsers::FeatureReader,
    util::fetch_url,
};
use alloc::{format, string::String, vec, vec::Vec};
pub use gbfs::*;
pub use gbfs_versions::*;
pub use geofencing_zones::*;
pub use manifest::*;
use s2json::{Features, MValue, Properties, VectorFeature, VectorGeometry, VectorPoint};
use serde::{Deserialize, Serialize};
pub use station_information::*;
pub use station_status::*;
pub use system_alerts::*;
pub use system_information::*;
pub use system_pricing_plans::*;
pub use system_regions::*;
pub use vehicle_status::*;
pub use vehicle_types::*;

/// Geofencing Feature
pub type GBFSGeofencingFeatureV3 = VectorFeature<(), GBFSGeofencingZonesV3Properties, MValue>;

/// Station Information feature properties
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSStationV3FeaturesProperties {
    /// Station ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short name
    pub short_name: Option<String>,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Station Opening Hours
    pub station_opening_hours: Option<String>,
    /// Is Virtual Station
    pub is_virtual_station: Option<bool>,
    /// Parking Type
    pub parking_type: Option<String>,
    /// Parking Hoop
    pub parking_hoop: Option<bool>,
    /// Contact Phone
    pub contact_phone: Option<String>,
    /// Capacity
    pub capacity: Option<u64>,
    /// Is Valet Station
    pub is_valet_station: Option<bool>,
    /// Is Charging Station
    pub is_charging_station: Option<bool>,
}
impl GBFSStationV3FeaturesProperties {
    /// Create a new GBFSStationV3FeaturesProperties
    pub fn new(station: &GBFSStationV3, locale: &str) -> Self {
        // station.name is an array of objects with `language` and `text` params. Filter by language == locale and return `text`
        let name = &station
            .name
            .iter()
            .find(|n| n.language == locale)
            .or_else(|| station.name.first())
            .map_or(String::new(), |n| n.text.clone());
        let short_name = station.short_name.as_ref().and_then(|s| {
            s.iter().find(|name| name.language == locale).map(|name| name.text.clone())
        });
        GBFSStationV3FeaturesProperties {
            station_id: station.station_id.clone(),
            name: name.clone(),
            short_name,
            address: station.address.clone(),
            cross_street: station.cross_street.clone(),
            region_id: station.region_id.clone(),
            post_code: station.post_code.clone(),
            station_opening_hours: station.station_opening_hours.clone(),
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

/// Station Information Point Feature
pub type GBFSStationFeatureV3 = VectorFeature<(), GBFSStationV3FeaturesProperties, MValue>;

/// Vehicle Point Feature
pub type GBFSVehicleFeatureV3 = VectorFeature<(), GBFSVehicleV3, MValue>;

/// GBFS Version 3 Reader
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSReaderV3 {
    /// User defined locale (defaults to "en")
    pub locale: String,
    /// The GBFS information
    pub gbfs: GBFSV3,
    /// The feeds for the GBFS
    pub gbfs_versions: Option<GBFSVersionsV3>,
    /// The system information
    pub system_information: GBFSSystemInformationV3,
    /// The station information
    pub station_information: Option<GBFSStationInformationV3>,
    /// The station status
    pub station_status: Option<GBFSStationStatusV3>,
    /// The vehicle status
    pub vehicle_status: Option<GBFSVehicleStatusV3>,
    /// The vehicle types
    pub vehicle_types: Option<GBFSVehicleTypesV3>,
    /// The system alerts
    pub system_alerts: Option<GBFSSystemAlertsV3>,
    /// The system regions
    pub system_regions: Option<GBFSSystemRegionsV3>,
    /// The system pricing plans
    pub system_pricing_plans: Option<GBFSSystemPricingPlansV3>,
    /// The geofencing zones
    pub geofencing_zones: Option<GBFSGeofencingZonesV3>,
    /// The manifest
    pub manifest: Option<GBFSManifestV3>,
}
impl GBFSReaderV3 {
    /// Get all features from the GBFS V3 data
    pub fn features(&self) -> Vec<VectorFeature> {
        let mut res = vec![];

        res.extend(self.station_features().iter().map(|f| f.to_m_vector_feature(|_| None)));
        res.extend(self.geofencing_features().iter().map(|f| f.to_m_vector_feature(|_| None)));
        res.extend(self.vehicle_features().iter().map(|f| f.to_m_vector_feature(|_| None)));

        res
    }

    /// Get all station features from the GBFS V3 data
    pub fn station_features(&self) -> Vec<GBFSStationFeatureV3> {
        let mut res = vec![];

        if let Some(station_information) = &self.station_information {
            for station in &station_information.data.stations {
                let properties = GBFSStationV3FeaturesProperties::new(station, &self.locale);
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

    /// Get Geofencing features from the GBFS V3 data
    pub fn geofencing_features(&self) -> Vec<GBFSGeofencingFeatureV3> {
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

    /// Get vehicle features for the GBFS V3 data
    pub fn vehicle_features(&self) -> Vec<GBFSVehicleFeatureV3> {
        let mut res = vec![];

        if let Some(vehicle_status) = &self.vehicle_status {
            for vehicle in &vehicle_status.data.vehicles {
                if let (Some(lat), Some(lon)) = (vehicle.lat, vehicle.lon) {
                    res.push(VectorFeature::new_wm(
                        None,
                        vehicle.clone(),
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
pub struct GBFSIteratorV3 {
    features: Vec<VectorFeature>,
    index: usize,
    len: usize,
}
impl Iterator for GBFSIteratorV3 {
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
impl FeatureReader<(), Properties, MValue> for GBFSReaderV3 {
    type FeatureIterator<'a> = GBFSIteratorV3;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        let features = self.features();
        let len = features.len();
        GBFSIteratorV3 { features, index: 0, len }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        let features = self.features();
        let start = features.len() * thread_id / pool_size;
        let end = features.len() * (thread_id + 1) / pool_size;
        GBFSIteratorV3 { features, index: start, len: end }
    }
}

/// Parse a GBFS V3 schema and build a V3 GBFS reader
///
/// ## Parameters
/// - `gbfs`: the GBFS schema to parse
/// - `locale`: the locale to use if provided, otherwise default to en
/// - `path`: if provided, will use this path instead of the url (for testing)
///
/// ## Returns
/// The GBFS reader
pub async fn build_gbfs_reader_v3(
    gbfs: &GBFSV3,
    locale: Option<String>,
    path: Option<String>,
) -> GBFSReaderV3 {
    let feeds = gbfs.data.feeds.clone();

    let mut reader = GBFSReaderV3 {
        locale: locale.unwrap_or("en".into()),
        gbfs: gbfs.clone(),
        ..Default::default()
    };

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

        if let Ok(url_data) = fetch_url(&url, &[]).await {
            match feed.name {
                GBFSV30FeedsName::Gbfs => {}
                GBFSV30FeedsName::GbfsVersions => {
                    reader.gbfs_versions = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::StationInformation => {
                    reader.station_information = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::StationStatus => {
                    reader.station_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::SystemAlerts => {
                    reader.system_alerts = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::SystemInformation => {
                    reader.system_information = serde_json::from_slice(&url_data).unwrap();
                }
                GBFSV30FeedsName::SystemPricingPlans => {
                    reader.system_pricing_plans = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::SystemRegions => {
                    reader.system_regions = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::VehicleStatus => {
                    reader.vehicle_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::VehicleTypes => {
                    reader.vehicle_types = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV30FeedsName::GeofencingZones => {
                    reader.geofencing_zones = Some(serde_json::from_slice(&url_data).unwrap());
                }
            }
        }
    }

    if let Some(manifest_url) = reader.system_information.data.manifest_url.clone() {
        let manifest_url = if let Some(ref path) = path {
            format!("{}/manifest.json", path)
        } else {
            manifest_url
        };
        let manifest_data = fetch_url(&manifest_url, &[]).await.unwrap();
        reader.manifest = serde_json::from_slice(&manifest_data).unwrap_or(None);
    }

    reader
}
