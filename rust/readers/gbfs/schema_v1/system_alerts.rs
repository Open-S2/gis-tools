use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Alerts Schema V1.1 OR GBFS System Alerts Schema V1.0
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#system_alertsjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#system_alertsjson)
pub type GBFSSystemAlertsV1 = GBFSSystemAlertsV11;

/// GBFS System Alerts Alert Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemAlertsAlertType {
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

/// GBFS System Alerts Alert Times
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertTimes {
    /// Start time in POSIX time
    pub start: u64,
    /// End time in POSIX time
    pub end: Option<u64>,
}

/// GBFS System Alerts Alert
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlert {
    /// Alert ID
    pub alert_id: String,
    /// Alert type
    pub r#type: GBFSSystemAlertsAlertType,
    /// List of times when the alert is active
    pub times: Option<Vec<GBFSSystemAlertsAlertTimes>>,
    /// List of affected stations
    pub station_ids: Option<Vec<String>>,
    /// List of affected regions
    pub regions_ids: Option<Vec<String>>,
    /// URL
    pub url: Option<String>,
    /// Summary
    pub summary: String,
    /// Description
    pub description: Option<String>,
    /// Last time the alert was updated
    pub last_updated: Option<u64>,
}

/// GBFS System Alerts Alerts
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlerts {
    /// Data containing ad-hoc alerts for the system.
    pub alerts: Vec<GBFSSystemAlertsAlert>,
}

/// GBFS System Alerts Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing ad-hoc alerts for the system.
    pub data: GBFSSystemAlertsAlerts,
}

/// GBFS System Alerts Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing ad-hoc alerts for the system.
    pub data: GBFSSystemAlertsAlerts,
}
