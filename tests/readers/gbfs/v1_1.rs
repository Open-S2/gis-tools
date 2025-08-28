#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
#[rustfmt::skip]
mod tests {
    use crate::spawn_test_server;
    use gistools::{
        parsers::FeatureReader,
        readers::{
            GBFSFreeBikeDataV11, GBFSFreeBikeStatusV1, GBFSFreeBikeStatusV11, GBFSFreeBikeV11, GBFSReader, GBFSReaderV1, GBFSRentalUri, GBFSStationInformationDataV11, GBFSStationInformationV1, GBFSStationInformationV11, GBFSStationInformationV11Station, GBFSStationStatusDataV11, GBFSStationStatusV1, GBFSStationStatusV11, GBFSSystemAlertsAlertsV1, GBFSSystemAlertsV1, GBFSSystemAlertsV11, GBFSSystemCalendarDataV1, GBFSSystemCalendarV1, GBFSSystemCalendarV11, GBFSSystemHourDayV1, GBFSSystemHourTypeV1, GBFSSystemHourV1, GBFSSystemHoursDataV1, GBFSSystemHoursV1, GBFSSystemHoursV11, GBFSSystemInformationDataV11, GBFSSystemInformationRentalAppV11, GBFSSystemInformationRentalAppsV11, GBFSSystemInformationV1, GBFSSystemInformationV11, GBFSSystemPricingPlanV11, GBFSSystemPricingPlansDataV11, GBFSSystemPricingPlansV1, GBFSSystemPricingPlansV11, GBFSSystemRegionsDataV1, GBFSSystemRegionsV1, GBFSSystemRegionsV11, GBFSV11Feeds, GBFSV11FeedsData, GBFSV11FeedsName, GBFSVersion, GBFSVersionsDataV11, GBFSVersionsV1, GBFSVersionsV11, GBFSV1, GBFSV11
        },
    };
    use s2json::VectorFeature;
    use std::{collections::BTreeMap, format, fs, path::PathBuf};

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
                        GBFSVersion {
                            version: "1.1".into(),
                            url: "https://gbfs.helbiz.com/v1.1/durham/gbfs.json".into()
                        },
                        GBFSVersion {
                            version: "2.0".into(),
                            url: "https://gbfs.helbiz.com/v2.0/durham/gbfs.json".into()
                        },
                        GBFSVersion {
                            version: "2.1".into(),
                            url: "https://gbfs.helbiz.com/v2.1/durham/gbfs.json".into()
                        },
                        GBFSVersion {
                            version: "2.2".into(),
                            url: "https://gbfs.helbiz.com/v2.2/durham/gbfs.json".into()
                        },
                        GBFSVersion {
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
                data: GBFSStationInformationDataV11 { stations: vec![
                    GBFSStationInformationV11Station { station_id: "stn_XLN88vWNhaeHAMXa8jgxer".into(), name: "Hotel des Impts".into(), short_name: None, lat: 49.646536, lon: -1.643192, address: None, cross_street: None, region_id: None, post_code: None, rental_methods: None, capacity: Some(12), rental_uris: Some(GBFSRentalUri { android: None, ios: None, web: Some("https://capcotentin.ecovelo.mobi//#/station/stn_XLN88vWNhaeHAMXa8jgxer".into()) }) }
                ] }
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
                data: GBFSStationStatusDataV11 { stations: vec![] }
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
                data: GBFSSystemAlertsAlertsV1 { alerts: vec![] }
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
                data: GBFSSystemCalendarDataV1 { calendars: vec![] }
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
                data: GBFSSystemHoursDataV1 {
                    rental_hours: vec![GBFSSystemHourV1 {
                        user_types: vec![GBFSSystemHourTypeV1::Member],
                        days: vec![
                            GBFSSystemHourDayV1::Mon,
                            GBFSSystemHourDayV1::Tue,
                            GBFSSystemHourDayV1::Wed,
                            GBFSSystemHourDayV1::Thu,
                            GBFSSystemHourDayV1::Fri,
                            GBFSSystemHourDayV1::Sat,
                            GBFSSystemHourDayV1::Sun
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
                    rental_apps: Some(GBFSSystemInformationRentalAppsV11 {
                        android: Some(GBFSSystemInformationRentalAppV11 {
                            store_uri:
                                "https://play.google.com/store/apps/details?id=com.helbiz.android"
                                    .into(),
                            discovery_uri: "helbiz://com.helbiz.android/scanQr".into()
                        }),
                        ios: Some(GBFSSystemInformationRentalAppV11 {
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
                data: GBFSSystemPricingPlansDataV11 {
                    plans: vec![GBFSSystemPricingPlanV11 {
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
                data: GBFSSystemRegionsDataV1 { regions: vec![] }
            }
        );
    }

    #[test]
    fn test_gbfs_reader_v1() {
        // `http://localhost:${server.port}/readers/gbfs/fixtures/v1.1/gbfs.json`,
        smol::block_on(async {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let path_str: String = path.to_str().unwrap().into();
            let server = spawn_test_server(&path_str);

            let url = format!("{}/tests/readers/gbfs/fixtures/v1.1/gbfs.json", server);
            let reader_v1 = GBFSReader::from_url(&url, None).await;

            let features = reader_v1.iter().collect::<Vec<VectorFeature>>();
            assert_eq!(features.len(), 3);

            let features: Vec<_> = (0..3usize)
                .into_iter()
                .flat_map(|thread_id| {
                    let reader = reader_v1.clone();
                    let res: Vec<_> = reader.par_iter(3, thread_id).collect();
                    res
                })
                .collect();
            assert_eq!(features.len(), 3);

            if let GBFSReader::V1(v1) = reader_v1 {
                let features = v1.iter().collect::<Vec<VectorFeature>>();
                assert_eq!(features.len(), 3);

                let features: Vec<_> = (0..3usize)
                    .into_iter()
                    .flat_map(|thread_id| {
                        let reader = v1.clone();
                        let res: Vec<_> = reader.par_iter(3, thread_id).collect();
                        res
                    })
                    .collect();
                assert_eq!(features.len(), 3);

                assert_eq!(*v1.as_ref(), GBFSReaderV1 {
                    free_bike_status: Some(
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
                                        rental_uris: Some(
                                            GBFSRentalUri {
                                                android: Some(
                                                    "https://deeplink.helbiz.com/startRide?code=U3YXAT".into(),
                                                ),
                                                ios: Some(
                                                    "https://deeplink.helbiz.com/startRide?code=U3YXAT".into(),
                                                ),
                                                web: None,
                                            },
                                        ),
                                    },
                                    GBFSFreeBikeV11 {
                                        bike_id: "T60A2".into(),
                                        lat: 38.875526,
                                        lon: -77.234484,
                                        is_reserved: false,
                                        is_disabled: true,
                                        rental_uris: Some(
                                            GBFSRentalUri {
                                                android: Some(
                                                    "https://deeplink.helbiz.com/startRide?code=T60A2".into(),
                                                ),
                                                ios: Some(
                                                    "https://deeplink.helbiz.com/startRide?code=T60A2".into(),
                                                ),
                                                web: None,
                                            },
                                        ),
                                    },
                                ],
                            },
                        },
                    ),
                    gbfs: GBFSV11 {
                        last_updated: 1735292928,
                        ttl: 0,
                        version: "1.1".into(),
                        data: BTreeMap::from([
                            ("en".into(), GBFSV11FeedsData {
                                feeds: vec![
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::GbfsVersions,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/gbfs_versions.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemInformation,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_information.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::StationInformation,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/station_information.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::StationStatus,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/station_status.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::FreeBikeStatus,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/free_bike_status.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemHours,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_hours.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemCalendar,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_calendar.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemRegions,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_regions.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemPricingPlans,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_pricing_plans.json".into(),
                                    },
                                    GBFSV11Feeds {
                                        name: GBFSV11FeedsName::SystemAlerts,
                                        url: "https://gbfs.helbiz.com/v1.1/durham/system_alerts.json".into(),
                                    },
                                ],
                            }),                
                        ]),
                    },
                    gbfs_versions: Some(
                        GBFSVersionsV11 {
                            last_updated: 1735301932,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSVersionsDataV11 {
                                versions: vec![
                                    GBFSVersion {
                                        version: "1.1".into(),
                                        url: "https://gbfs.helbiz.com/v1.1/durham/gbfs.json".into(),
                                    },
                                    GBFSVersion {
                                        version: "2.0".into(),
                                        url: "https://gbfs.helbiz.com/v2.0/durham/gbfs.json".into(),
                                    },
                                    GBFSVersion {
                                        version: "2.1".into(),
                                        url: "https://gbfs.helbiz.com/v2.1/durham/gbfs.json".into(),
                                    },
                                    GBFSVersion {
                                        version: "2.2".into(),
                                        url: "https://gbfs.helbiz.com/v2.2/durham/gbfs.json".into(),
                                    },
                                    GBFSVersion {
                                        version: "2.2-google".into(),
                                        url: "https://gbfs.helbiz.com/v2.2-google/durham/gbfs.json".into(),
                                    },
                                ],
                            },
                        },
                    ),
                    station_information: Some(
                        GBFSStationInformationV11 {
                            last_updated: 1735301957,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSStationInformationDataV11 {
                                stations: vec![
                                    GBFSStationInformationV11Station { station_id: "stn_XLN88vWNhaeHAMXa8jgxer".into(), name: "Hotel des Impts".into(), short_name: None, lat: 49.646536, lon: -1.643192, address: None, cross_street: None, region_id: None, post_code: None, rental_methods: None, capacity: Some(12), rental_uris: Some(GBFSRentalUri { android: None, ios: None, web: Some("https://capcotentin.ecovelo.mobi//#/station/stn_XLN88vWNhaeHAMXa8jgxer".into()) }) }
                                ],
                            },
                        },
                    ),
                    station_status: Some(
                        GBFSStationStatusV11 {
                            last_updated: 1735301974,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSStationStatusDataV11 {
                                stations: vec![],
                            },
                        },
                    ),
                    system_alerts: Some(
                        GBFSSystemAlertsV11 {
                            last_updated: 1735302061,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSSystemAlertsAlertsV1 {
                                alerts: vec![],
                            },
                        },
                    ),
                    system_calendar: Some(
                        GBFSSystemCalendarV11 {
                            last_updated: 1735302014,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSSystemCalendarDataV1 {
                                calendars: vec![],
                            },
                        },
                    ),
                    system_hours: Some(
                        GBFSSystemHoursV11 {
                            last_updated: 1735301997,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSSystemHoursDataV1 {
                                rental_hours: vec![
                                    GBFSSystemHourV1 {
                                        user_types: vec![
                                            GBFSSystemHourTypeV1::Member,
                                        ],
                                        days: vec![
                                            GBFSSystemHourDayV1::Mon,
                                            GBFSSystemHourDayV1::Tue,
                                            GBFSSystemHourDayV1::Wed,
                                            GBFSSystemHourDayV1::Thu,
                                            GBFSSystemHourDayV1::Fri,
                                            GBFSSystemHourDayV1::Sat,
                                            GBFSSystemHourDayV1::Sun,
                                        ],
                                        start_time: "00:00:00".into(),
                                        end_time: "23:59:59".into(),
                                    },
                                ],
                            },
                        },
                    ),
                    system_information: GBFSSystemInformationV11 {
                        last_updated: 1735301945,
                        ttl: 0,
                        version: "1.1".into(),
                        data: GBFSSystemInformationDataV11 {
                            system_id: "HELBIZ-US".into(),
                            language: "en".into(),
                            name: "Helbiz USA".into(),
                            short_name: Some(
                                "Helbiz".into(),
                            ),
                            operator: Some(
                                "Helbiz".into(),
                            ),
                            url: Some(
                                "https://helbiz.com/".into(),
                            ),
                            purchase_url: Some(
                                "https://helbiz.com/go".into(),
                            ),
                            start_date: Some(
                                "2019-10-15".into(),
                            ),
                            phone_number: Some(
                                "+1 (619) 313-5812".into(),
                            ),
                            email: Some(
                                "support@helbiz.com".into(),
                            ),
                            feed_contact_email: Some(
                                "support@helbiz.com".into(),
                            ),
                            timezone: "-5".into(),
                            license_url: Some(
                                "https://helbiz.com/terms".into(),
                            ),
                            rental_apps: Some(
                                GBFSSystemInformationRentalAppsV11 {
                                    android: Some(
                                        GBFSSystemInformationRentalAppV11 {
                                            store_uri: "https://play.google.com/store/apps/details?id=com.helbiz.android".into(),
                                            discovery_uri: "helbiz://com.helbiz.android/scanQr".into(),
                                        },
                                    ),
                                    ios: Some(
                                        GBFSSystemInformationRentalAppV11 {
                                            store_uri: "https://apps.apple.com/us/app/helbiz-live/id1570423369".into(),
                                            discovery_uri: "com.fabrika.helbiz.ios://scanQr".into(),
                                        },
                                    ),
                                },
                            ),
                        },
                    },
                    system_pricing_plans: Some(
                        GBFSSystemPricingPlansV11 {
                            last_updated: 1735302051,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSSystemPricingPlansDataV11 {
                                plans: vec![
                                    GBFSSystemPricingPlanV11 {
                                        plan_id: "scooter-unlock".into(),
                                        url: None,
                                        name: "Unlock".into(),
                                        currency: "USD".into(),
                                        price: 1.0,
                                        is_taxable: true,
                                        description: "Scooter unlock price".into(),
                                    },
                                ],
                            },
                        },
                    ),
                    system_regions: Some(
                        GBFSSystemRegionsV11 {
                            last_updated: 1735302039,
                            ttl: 0,
                            version: "1.1".into(),
                            data: GBFSSystemRegionsDataV1 {
                                regions: vec![],
                            },
                        },
                    ),
                });
            } else {
                panic!("Expected GBFSReader::V1");
            }
        });
    }
}
