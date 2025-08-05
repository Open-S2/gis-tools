use crate::readers::{GBFSRentalUri, gbfs_bool_or_int};
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// # Free Bike Status Schema V1.1 OR Free Bike Status Schema V1.0
/// Describes the vehicles that are available for rent.
///
/// ## Links
/// - [GBFS Specification V1.1](https://github.com/MobilityData/gbfs/blob/v1.1/gbfs.md#free_bike_statusjson)
/// - [GBFS Specification V1.0](https://github.com/MobilityData/gbfs/blob/v1.0/gbfs.md#free_bike_statusjson)
pub type GBFSFreeBikeStatusV1 = GBFSFreeBikeStatusV11;

/// Free Bike Status Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeV11 {
    /// Bike ID
    pub bike_id: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Is the bike reserved
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the bike disabled
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
    /// Rental URIs
    pub rental_uris: Option<GBFSRentalUri>,
}

/// Data containing an array of bikes.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeDataV11 {
    /// Data containing an array of bikes.
    pub bikes: Vec<GBFSFreeBikeV11>,
}

/// Free Bike Status Schema V1.1 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusV11 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// GBFS version number (1.1).
    pub version: String,
    /// Data containing an array of bikes.
    pub data: GBFSFreeBikeDataV11,
}

/// Free Bike Status Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeV10 {
    /// Bike ID
    pub bike_id: String,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Is the bike reserved
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_reserved: bool,
    /// Is the bike disabled
    #[serde(deserialize_with = "gbfs_bool_or_int")]
    pub is_disabled: bool,
}

/// Data containing an array of bikes.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeDataV10 {
    /// Data containing an array of bikes.
    pub bikes: Vec<GBFSFreeBikeV10>,
}

/// Free Bike Status Schema V1.0 Interface
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct GBFSFreeBikeStatusV10 {
    /// Last time the data in the feed was updated in POSIX time.
    pub last_updated: u64,
    /// Number of seconds before the data in the feed will be updated again.
    pub ttl: u64,
    /// Data containing an array of bikes.
    pub data: GBFSFreeBikeDataV10,
}
