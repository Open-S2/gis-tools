#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::readers::{
        GBFSFreeBikeDataV11, GBFSFreeBikeStatusV1, GBFSFreeBikeStatusV11, GBFSFreeBikeV11,
        GBFSRentalUri, GBFSStationInformationDataV11, GBFSStationInformationV1,
        GBFSStationInformationV11, GBFSStationStatusData, GBFSStationStatusV1,
        GBFSStationStatusV11, GBFSSystemAlertsAlerts, GBFSSystemAlertsV1, GBFSSystemAlertsV11,
        GBFSSystemCalendarData, GBFSSystemCalendarV1, GBFSSystemCalendarV11, GBFSSystemHour,
        GBFSSystemHourDay, GBFSSystemHourType, GBFSSystemHoursData, GBFSSystemHoursV1,
        GBFSSystemHoursV11, GBFSSystemInformationDataV11, GBFSSystemInformationRentalApp,
        GBFSSystemInformationRentalApps, GBFSSystemInformationV1, GBFSSystemInformationV11,
        GBFSSystemPricingPlan, GBFSSystemPricingPlansData, GBFSSystemPricingPlansV1,
        GBFSSystemPricingPlansV11, GBFSSystemRegionsData, GBFSSystemRegionsV1,
        GBFSSystemRegionsV11, GBFSV1, GBFSV11, GBFSV11Feeds, GBFSV11FeedsData, GBFSV11FeedsName,
        GBFSVersionsDataV11, GBFSVersionsV1, GBFSVersionsV11, GBFSVersionsVersionV11,
    };
    use std::{collections::BTreeMap, fs, path::PathBuf};

    #[test]
    fn test_gbfs_free_bike_status_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/free_bike_status.json");
        let file_str = fs::read_to_string(path).unwrap();
        let free_bike_status: GBFSFreeBikeStatusV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            free_bike_status,
            GBFSFreeBikeStatusV11 {
                last_updated: 1735301987,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSFreeBikeDataV11 {
                    bikes: vec![
                        GBFSFreeBikeV11 {
                            bike_id: "U3YXAT".into(),
                            lat: 35.968015,
                            lon: -78.90667,
                            is_reserved: true,
                            is_disabled: true,
                            rental_uris: Some(GBFSRentalUri {
                                android: Some(
                                    "https://deeplink.helbiz.com/startRide?code=U3YXAT".into()
                                ),
                                ios: Some(
                                    "https://deeplink.helbiz.com/startRide?code=U3YXAT".into()
                                ),
                                web: None
                            })
                        },
                        GBFSFreeBikeV11 {
                            bike_id: "T60A2".into(),
                            lat: 38.875526,
                            lon: -77.234484,
                            is_reserved: false,
                            is_disabled: true,
                            rental_uris: Some(GBFSRentalUri {
                                android: Some(
                                    "https://deeplink.helbiz.com/startRide?code=T60A2".into()
                                ),
                                ios: Some(
                                    "https://deeplink.helbiz.com/startRide?code=T60A2".into()
                                ),
                                web: None
                            })
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn test_gbfs_gbfs_versions_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/gbfs_versions.json");
        let file_str = fs::read_to_string(path).unwrap();
        let gbfs_versions: GBFSVersionsV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            gbfs_versions,
            GBFSVersionsV11 {
                last_updated: 1735301932,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSVersionsDataV11 {
                    versions: vec![
                        GBFSVersionsVersionV11 {
                            version: "1.1".into(),
                            url: "https://gbfs.helbiz.com/v1.1/durham/gbfs.json".into()
                        },
                        GBFSVersionsVersionV11 {
                            version: "2.0".into(),
                            url: "https://gbfs.helbiz.com/v2.0/durham/gbfs.json".into()
                        },
                        GBFSVersionsVersionV11 {
                            version: "2.1".into(),
                            url: "https://gbfs.helbiz.com/v2.1/durham/gbfs.json".into()
                        },
                        GBFSVersionsVersionV11 {
                            version: "2.2".into(),
                            url: "https://gbfs.helbiz.com/v2.2/durham/gbfs.json".into()
                        },
                        GBFSVersionsVersionV11 {
                            version: "2.2-google".into(),
                            url: "https://gbfs.helbiz.com/v2.2-google/durham/gbfs.json".into()
                        }
                    ]
                }
            }
        );
    }

    #[test]
    fn test_gbfs_gbfs_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/gbfs.json");
        let file_str = fs::read_to_string(path).unwrap();
        let gbfs: GBFSV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            gbfs,
            GBFSV11 {
                last_updated: 1735292928,
                ttl: 0,
                version: "1.1".into(),
                data: BTreeMap::from([(
                    "en".into(),
                    GBFSV11FeedsData {
                        feeds: vec![
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::GbfsVersions,
                                url: "https://gbfs.helbiz.com/v1.1/durham/gbfs_versions.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemInformation,
                                url: "https://gbfs.helbiz.com/v1.1/durham/system_information.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::StationInformation,
                                url: "https://gbfs.helbiz.com/v1.1/durham/station_information.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::StationStatus,
                                url: "https://gbfs.helbiz.com/v1.1/durham/station_status.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::FreeBikeStatus,
                                url: "https://gbfs.helbiz.com/v1.1/durham/free_bike_status.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemHours,
                                url: "https://gbfs.helbiz.com/v1.1/durham/system_hours.json".into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemCalendar,
                                url: "https://gbfs.helbiz.com/v1.1/durham/system_calendar.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemRegions,
                                url: "https://gbfs.helbiz.com/v1.1/durham/system_regions.json"
                                    .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemPricingPlans,
                                url:
                                    "https://gbfs.helbiz.com/v1.1/durham/system_pricing_plans.json"
                                        .into()
                            },
                            GBFSV11Feeds {
                                name: GBFSV11FeedsName::SystemAlerts,
                                url: "https://gbfs.helbiz.com/v1.1/durham/system_alerts.json"
                                    .into()
                            },
                        ]
                    }
                )])
            }
        );
    }

    #[test]
    fn test_gbfs_station_information_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/station_information.json");
        let file_str = fs::read_to_string(path).unwrap();
        let station_information: GBFSStationInformationV1 =
            serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            station_information,
            GBFSStationInformationV11 {
                last_updated: 1735301957,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSStationInformationDataV11 { stations: vec![] }
            }
        );
    }

    #[test]
    fn test_gbfs_station_status_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/station_status.json");
        let file_str = fs::read_to_string(path).unwrap();
        let station_status: GBFSStationStatusV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            station_status,
            GBFSStationStatusV11 {
                last_updated: 1735301974,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSStationStatusData { stations: vec![] }
            }
        );
    }

    #[test]
    fn test_gbfs_system_alerts_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_alerts.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_alerts: GBFSSystemAlertsV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_alerts,
            GBFSSystemAlertsV11 {
                last_updated: 1735302061,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemAlertsAlerts { alerts: vec![] }
            }
        );
    }

    #[test]
    fn test_gbfs_system_calendar_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_calendar.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_calendar: GBFSSystemCalendarV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_calendar,
            GBFSSystemCalendarV11 {
                last_updated: 1735302014,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemCalendarData { calendars: vec![] }
            }
        );
    }

    #[test]
    fn test_gbfs_system_hours_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_hours.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_hours: GBFSSystemHoursV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_hours,
            GBFSSystemHoursV11 {
                last_updated: 1735301997,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemHoursData {
                    rental_hours: vec![GBFSSystemHour {
                        user_types: vec![GBFSSystemHourType::Member],
                        days: vec![
                            GBFSSystemHourDay::Mon,
                            GBFSSystemHourDay::Tue,
                            GBFSSystemHourDay::Wed,
                            GBFSSystemHourDay::Thu,
                            GBFSSystemHourDay::Fri,
                            GBFSSystemHourDay::Sat,
                            GBFSSystemHourDay::Sun
                        ],
                        start_time: "00:00:00".into(),
                        end_time: "23:59:59".into()
                    }]
                }
            }
        );
    }

    #[test]
    fn test_gbfs_system_information_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_information.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_information: GBFSSystemInformationV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_information,
            GBFSSystemInformationV11 {
                last_updated: 1735301945,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemInformationDataV11 {
                    system_id: "HELBIZ-US".into(),
                    language: "en".into(),
                    name: "Helbiz USA".into(),
                    short_name: Some("Helbiz".into()),
                    operator: Some("Helbiz".into()),
                    url: Some("https://helbiz.com/".into()),
                    purchase_url: Some("https://helbiz.com/go".into()),
                    start_date: Some("2019-10-15".into()),
                    phone_number: Some("+1 (619) 313-5812".into()),
                    email: Some("support@helbiz.com".into()),
                    feed_contact_email: Some("support@helbiz.com".into()),
                    timezone: "-5".into(),
                    license_url: Some("https://helbiz.com/terms".into()),
                    rental_apps: Some(GBFSSystemInformationRentalApps {
                        android: Some(GBFSSystemInformationRentalApp {
                            store_uri:
                                "https://play.google.com/store/apps/details?id=com.helbiz.android"
                                    .into(),
                            discovery_uri: "helbiz://com.helbiz.android/scanQr".into()
                        }),
                        ios: Some(GBFSSystemInformationRentalApp {
                            store_uri: "https://apps.apple.com/us/app/helbiz-live/id1570423369"
                                .into(),
                            discovery_uri: "com.fabrika.helbiz.ios://scanQr".into()
                        })
                    })
                }
            }
        );
    }

    #[test]
    fn test_gbfs_system_pricing_plans_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_pricing_plans.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_pricing_plans: GBFSSystemPricingPlansV1 =
            serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_pricing_plans,
            GBFSSystemPricingPlansV11 {
                last_updated: 1735302051,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemPricingPlansData {
                    plans: vec![GBFSSystemPricingPlan {
                        plan_id: "scooter-unlock".into(),
                        url: None,
                        name: "Unlock".into(),
                        currency: "USD".into(),
                        price: 1.0,
                        is_taxable: true,
                        description: "Scooter unlock price".into()
                    }]
                }
            }
        );
    }

    #[test]
    fn test_gbfs_system_regions_v1_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/gbfs/fixtures/v1.1/system_regions.json");
        let file_str = fs::read_to_string(path).unwrap();
        let system_regions: GBFSSystemRegionsV1 = serde_json::from_str(&file_str).unwrap();
        assert_eq!(
            system_regions,
            GBFSSystemRegionsV11 {
                last_updated: 1735302039,
                ttl: 0,
                version: "1.1".into(),
                data: GBFSSystemRegionsData { regions: vec![] }
            }
        );
    }
}
