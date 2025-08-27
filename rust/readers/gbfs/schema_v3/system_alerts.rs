use crate::readers::GBFSName;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # GBFS System Alerts Schema V3.1-RC & V3.0
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification V3.1-RC](https://github.com/MobilityData/gbfs/blob/v3.1-RC/gbfs.md#system_alertsjson)
/// - [GBFS Specification V3.0](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_alertsjson)
pub type GBFSSystemAlertsV3 = GBFSSystemAlertsV30;

/// GBFS System Alerts Alert Type
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum GBFSSystemAlertsAlertTypeV30 {
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

/// GBFS System Alerts Alert Times
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertTimesV30 {
    /// Start time in POSIX time
    pub start: u64,
    /// End time in POSIX time
    pub end: Option<u64>,
}

/// GBFS System Alerts Alert
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertV30 {
    /// Identifier for this alert.
    pub alert_id: String,
    /// Type of alert.
    /// Possible values: 'system_closure', 'station_closure', 'station_move', 'other'.
    pub r#type: GBFSSystemAlertsAlertTypeV30,
    /// Array of objects indicating when the alert is in effect.
    pub times: Option<Vec<GBFSSystemAlertsAlertTimesV30>>,
    /// Array of identifiers of the stations for which this alert applies.
    pub station_ids: Option<Vec<String>>,
    /// Array of identifiers of the regions for which this alert applies.
    pub region_ids: Option<Vec<String>>,
    /// URL where customers can learn more information about this alert.
    pub url: Option<Vec<GBFSName>>,
    /// Short summary of this alert to be displayed to the customer.
    pub summary: Vec<GBFSName>,
    /// Detailed description of the alert.
    pub description: Option<Vec<GBFSName>>,
    /// Indicates the last time the info for the alert was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: Option<String>,
}

/// GBFS System Alerts Alerts
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsAlertsV30 {
    /// Data containing ad-hoc alerts for the system.
    pub alerts: Vec<GBFSSystemAlertsAlertV30>,
}

/// # GBFS System Alerts Schema V3.0
/// Describes ad-hoc changes to the system.
///
/// ## Links
/// - [GBFS Specification](https://github.com/MobilityData/gbfs/blob/v3.0/gbfs.md#system_alertsjson)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSSystemAlertsV30 {
    /// Last time the data in the feed was updated in RFC3339 format.
    /// **Format**: date-time
    pub last_updated: String,
    /// Number of seconds before the data in the feed will be updated again (0 if the data should always be refreshed).
    /// **Minimum**: 0
    pub ttl: u64,
    /// GBFS version number to which the feed conforms.
    /// **Const**: '3.0'
    pub version: String,
    /// Data object containing system alerts.
    pub data: GBFSSystemAlertsAlertsV30,
}
