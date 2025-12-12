mod free_bike_status;
mod gbfs;
mod gbfs_versions;
mod station_information;
mod station_status;
mod system_alerts;
mod system_calendar;
mod system_hours;
mod system_information;
mod system_pricing_plans;
mod system_regions;

use crate::{parsers::FeatureReader, util::fetch_url};
use alloc::{format, string::String, vec, vec::Vec};
pub use free_bike_status::*;
pub use gbfs::*;
pub use gbfs_versions::*;
use s2json::{MValue, Properties, VectorFeature, VectorGeometry, VectorPoint};
use serde::{Deserialize, Serialize};
pub use station_information::*;
pub use station_status::*;
pub use system_alerts::*;
pub use system_calendar::*;
pub use system_hours::*;
pub use system_information::*;
pub use system_pricing_plans::*;
pub use system_regions::*;

/// Station Information feature properties
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, MValue)]
pub struct GBFSStationV1FeaturesProperties {
    /// ID
    pub station_id: String,
    /// Name
    pub name: String,
    /// Short Name
    pub short_name: Option<String>,
    /// Address
    pub address: Option<String>,
    /// Cross Street
    pub cross_street: Option<String>,
    /// Region ID
    pub region_id: Option<String>,
    /// Post Code
    pub post_code: Option<String>,
    /// Capacity
    pub capacity: Option<u64>,
}
impl From<&GBFSStationInformationV11Station> for GBFSStationV1FeaturesProperties {
    fn from(station: &GBFSStationInformationV11Station) -> Self {
        GBFSStationV1FeaturesProperties {
            station_id: station.station_id.clone(),
            name: station.name.clone(),
            short_name: station.short_name.clone(),
            address: station.address.clone(),
            cross_street: station.cross_street.clone(),
            region_id: station.region_id.clone(),
            post_code: station.post_code.clone(),
            capacity: station.capacity,
        }
    }
}

/// Station Information Point Feature
pub type GBFSStationFeatureV1 = VectorFeature<(), GBFSStationV1FeaturesProperties, MValue>;

/// Bike Information Point Feature
pub type GBFSBikeFeatureV1 = VectorFeature<(), GBFSFreeBikeV11, MValue>;

/// GBFS Version 1 Reader
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSReaderV1 {
    /// Free Bike Status
    pub free_bike_status: Option<GBFSFreeBikeStatusV1>,
    /// GBFS information
    pub gbfs: GBFSV1,
    /// GBFS versions
    pub gbfs_versions: Option<GBFSVersionsV1>,
    /// System Information
    pub station_information: Option<GBFSStationInformationV1>,
    /// Station Status
    pub station_status: Option<GBFSStationStatusV1>,
    /// System Alerts
    pub system_alerts: Option<GBFSSystemAlertsV1>,
    /// System Calendar
    pub system_calendar: Option<GBFSSystemCalendarV1>,
    /// System Hours
    pub system_hours: Option<GBFSSystemHoursV1>,
    /// System Information
    pub system_information: GBFSSystemInformationV1,
    /// System Pricing Plans
    pub system_pricing_plans: Option<GBFSSystemPricingPlansV1>,
    /// System Regions
    pub system_regions: Option<GBFSSystemRegionsV1>,
}
impl GBFSReaderV1 {
    /// Get all features from the GBFS V1 data
    pub fn features(&self) -> Vec<VectorFeature> {
        let mut res = vec![];

        res.extend(self.station_features().iter().map(|f| f.to_m_vector_feature(|_| None)));
        res.extend(self.bike_features().iter().map(|f| f.to_m_vector_feature(|_| None)));

        res
    }

    /// Get all station features from the GBFS V1 data
    pub fn station_features(&self) -> Vec<GBFSStationFeatureV1> {
        let mut res = vec![];

        if let Some(station_information) = &self.station_information {
            for station in &station_information.data.stations {
                res.push(VectorFeature::new_wm(
                    None,
                    GBFSStationV1FeaturesProperties::from(station),
                    VectorGeometry::new_point(VectorPoint::from_xy(station.lon, station.lat), None),
                    None,
                ));
            }
        }

        res
    }

    /// Get all bike features from the GBFS V1 data
    pub fn bike_features(&self) -> Vec<GBFSBikeFeatureV1> {
        let mut res = vec![];

        if let Some(free_bike_status) = &self.free_bike_status {
            for bike in &free_bike_status.data.bikes {
                res.push(VectorFeature::new_wm(
                    None,
                    bike.clone(),
                    VectorGeometry::new_point(VectorPoint::from_xy(bike.lon, bike.lat), None),
                    None,
                ));
            }
        }

        res
    }
}

/// The GBFS V1 Iterator tool
#[derive(Debug)]
pub struct GBFSIteratorV1 {
    features: Vec<VectorFeature>,
    index: usize,
    len: usize,
}
impl Iterator for GBFSIteratorV1 {
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
impl FeatureReader<(), Properties, MValue> for GBFSReaderV1 {
    type FeatureIterator<'a> = GBFSIteratorV1;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        let features: Vec<VectorFeature> =
            self.features().iter().map(|f| f.to_m_vector_feature(|_| None)).collect();
        let len = features.len();
        GBFSIteratorV1 { features, index: 0, len }
    }

    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        let features: Vec<VectorFeature> =
            self.features().iter().map(|f| f.to_m_vector_feature(|_| None)).collect();
        let start = features.len() * thread_id / pool_size;
        let end = features.len() * (thread_id + 1) / pool_size;
        GBFSIteratorV1 { features, index: start, len: end }
    }
}

/// Parse a GBFSV1 schema and build a V1 GBFS reader
///
/// ## Parameters
/// - `gbfs`: the GBFS schema to parse
/// - `locale`: the locale to use if provided, otherwise default to en
/// - `path`: if provided, will use this path instead of the url (for testing)
///
/// ## Returns
/// The GBFS reader
pub async fn build_gbfs_reader_v1(
    gbfs: &GBFSV1,
    locale: Option<String>,
    path: Option<String>,
) -> GBFSReaderV1 {
    let locale = locale.unwrap_or("en".into());
    let data = &gbfs.data;
    let first_locale = data.keys().next().unwrap();
    let feeds = data.get(&locale).unwrap_or(data.get(first_locale).unwrap());
    let feeds = feeds.feeds.clone();

    let mut reader = GBFSReaderV1 { gbfs: gbfs.clone(), ..Default::default() };

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
                GBFSV11FeedsName::FreeBikeStatus => {
                    reader.free_bike_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::Gbfs => {}
                GBFSV11FeedsName::GbfsVersions => {
                    reader.gbfs_versions = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::StationInformation => {
                    reader.station_information = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::StationStatus => {
                    reader.station_status = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::SystemAlerts => {
                    reader.system_alerts = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::SystemCalendar => {
                    reader.system_calendar = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::SystemHours => {
                    reader.system_hours = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::SystemInformation => {
                    reader.system_information = serde_json::from_slice(&url_data).unwrap();
                }
                GBFSV11FeedsName::SystemPricingPlans => {
                    reader.system_pricing_plans = Some(serde_json::from_slice(&url_data).unwrap());
                }
                GBFSV11FeedsName::SystemRegions => {
                    reader.system_regions = Some(serde_json::from_slice(&url_data).unwrap());
                }
            }
        }
    }

    reader
}
