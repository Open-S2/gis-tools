use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS Alerts Schema V2.3, V2.2, V2.1, OR V2.0
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification V2.3](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_alertsjson)
/// - [GBFS Specification V2.2](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_alertsjson)
/// - [GBFS Specification V2.1](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_alertsjson)
/// - [GBFS Specification V2.0](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_alertsjson)
pub type GBFSSystemAlertsV2 = GBFSSystemAlertsV23;

/// # GBFS System Alerts V2.3
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.3/gbfs.md#system_alertsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV23 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.3
    pub version: String,
    /// Contains system alerts data.
    pub data: GBFSSystemAlertsDataV21,
}

/// # GBFS System Alerts V2.2
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.2/gbfs.md#system_alertsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV22 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.2
    pub version: String,
    /// Contains system alerts data.
    pub data: GBFSSystemAlertsDataV21,
}

/// GBFS System Alerts Alert Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemAlertsAlertTypeV21 {
    /// System Closure
    #[serde(rename = "system_closure")]
    SystemClosure,
    /// Station Closure
    #[serde(rename = "station_closure")]
    StationClosure,
    /// Station Move
    #[serde(rename = "station_move")]
    StationMove,
    /// Other
    #[default]
    #[serde(rename = "other")]
    Other,
}

/// GBFS System Alerts Alert V2.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertV21 {
    /// Unique ID for the alert.
    pub alert_id: String,
    /// Type of alert.
    pub r#type: GBFSSystemAlertsAlertTypeV21,
    /// Times the alert is in effect.
    pub times: Option<Vec<GBFSSystemAlertsAlertTimesV20>>,
    /// IDs of affected stations.
    pub station_ids: Option<Vec<String>>,
    /// IDs of affected regions.
    pub region_ids: Option<Vec<String>>,
    /// URL to more information about the alert.
    pub url: Option<String>,
    /// Summary of the alert.
    pub summary: String,
    /// Description of the alert.
    pub description: Option<String>,
    /// Last time the alert was updated in POSIX time.
    pub last_updated: Option<u64>,
}

/// GBFS System Alerts Data V2.1
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsDataV21 {
    /// List of system alerts.
    pub alerts: Vec<GBFSSystemAlertsAlertV21>,
}

/// # GBFS System Alerts V2.1
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.1/gbfs.md#system_alertsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV21 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.1
    pub version: String,
    /// Contains system alerts data.
    pub data: GBFSSystemAlertsDataV21,
}

/// GBFS System Alerts Alert Type V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemAlertsAlertTypeV20 {
    /// System Closure
    #[serde(rename = "SYSTEM_CLOSURE")]
    SystemClosure,
    /// Station Closure
    #[serde(rename = "STATION_CLOSURE")]
    StationClosure,
    /// Station Move
    #[serde(rename = "STATION_MOVE")]
    StationMove,
    /// Other
    #[default]
    #[serde(rename = "OTHER")]
    Other,
}

/// GBFS System Alerts Alert Times V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertTimesV20 {
    /// Start time in POSIX time
    pub start: u64,
    /// End time in POSIX time
    pub end: Option<u64>,
}

/// GBFS System Alerts Alert V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertV20 {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub r#type: GBFSSystemAlertsAlertTypeV20,
    /// Alert times
    pub times: Option<Vec<GBFSSystemAlertsAlertTimesV20>>,
    /// Affected station IDs
    pub station_ids: Option<Vec<String>>,
    /// Affected region IDs
    pub region_ids: Option<Vec<String>>,
    /// Alert URL
    pub url: Option<String>,
    /// Alert summary
    pub summary: String,
    /// Alert description
    pub description: Option<String>,
    /// Last updated time in POSIX time
    pub last_updated: Option<u64>,
}

/// GBFS System Alerts Data V2.0
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsDataV20 {
    /// Contains system alerts data.
    pub alerts: Vec<GBFSSystemAlertsAlertV20>,
}

/// # GBFS System Alerts V2.0
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v2.0/gbfs.md#system_alertsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV20 {
    /// Last time the data in the feed was updated in POSIX time.
    /// **Minimum**: 1450155600
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms, according to the versioning framework.
    /// **Const**: 2.0
    pub version: String,
    /// Contains system alerts data.
    pub data: GBFSSystemAlertsDataV20,
}
