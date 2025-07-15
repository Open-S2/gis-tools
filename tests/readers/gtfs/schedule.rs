#[cfg(test)]
// #[coverage(off)]
mod tests {
    use gistools::{
        parsers::FeatureReader,
        readers::{
            GTFSArea, GTFSAttribution, GTFSBookingRule, GTFSFareLegJoinRule, GTFSFareLegRule,
            GTFSFareMedia, GTFSFareProduct, GTFSFareTransferRule, GTFSFeedInfo, GTFSFrequency,
            GTFSLevel, GTFSLocationGroup, GTFSLocationGroupStop, GTFSNetwork, GTFSPathway,
            GTFSRouteNetwork, GTFSScheduleReader, GTFSStopArea, GTFSTimeframe, GTFSTransfer,
            GTFSTranslation,
        },
    };
    use s2json::{
        BBox3D, Properties, VectorBaseGeometry, VectorFeature, VectorGeometry, VectorGeometryType,
        VectorPoint,
    };
    use std::{collections::BTreeMap, fs, path::PathBuf};

    #[test]
    fn gtfs_schedule_transfers() {
        let transfers = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/transfers.csv"),
        )
        .unwrap();

        let transfers = GTFSTransfer::new(&transfers);
        assert_eq!(
            transfers,
            vec![
                GTFSTransfer {
                    from_stop_id: Some("STOP1".into()),
                    to_stop_id: Some("STOP2".into()),
                    from_route_id: Some("ROUTE1".into()),
                    to_route_id: Some("ROUTE2".into()),
                    from_trip_id: None,
                    to_trip_id: None,
                    transfer_type: 0,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP3".into()),
                    to_stop_id: Some("STOP4".into()),
                    from_route_id: Some("ROUTE3".into()),
                    to_route_id: Some("ROUTE4".into()),
                    from_trip_id: Some("TRIP3".into()),
                    to_trip_id: Some("TRIP4".into()),
                    transfer_type: 1,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP5".into()),
                    to_stop_id: Some("STOP6".into()),
                    from_route_id: Some("ROUTE5".into()),
                    to_route_id: Some("ROUTE6".into()),
                    from_trip_id: None,
                    to_trip_id: None,
                    transfer_type: 2,
                    min_transfer_time: Some(300)
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP7".into()),
                    to_stop_id: Some("STOP8".into()),
                    from_route_id: Some("ROUTE7".into()),
                    to_route_id: Some("ROUTE8".into()),
                    from_trip_id: Some("TRIP7".into()),
                    to_trip_id: Some("TRIP8".into()),
                    transfer_type: 3,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP9".into()),
                    to_stop_id: Some("STOP10".into()),
                    from_route_id: Some("ROUTE9".into()),
                    to_route_id: Some("ROUTE10".into()),
                    from_trip_id: Some("TRIP9".into()),
                    to_trip_id: Some("TRIP10".into()),
                    transfer_type: 4,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP11".into()),
                    to_stop_id: Some("STOP12".into()),
                    from_route_id: Some("ROUTE11".into()),
                    to_route_id: Some("ROUTE12".into()),
                    from_trip_id: Some("TRIP11".into()),
                    to_trip_id: Some("TRIP12".into()),
                    transfer_type: 5,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP13".into()),
                    to_stop_id: Some("STOP14".into()),
                    from_route_id: None,
                    to_route_id: None,
                    from_trip_id: None,
                    to_trip_id: None,
                    transfer_type: 0,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: None,
                    to_stop_id: Some("STOP15".into()),
                    from_route_id: Some("ROUTE13".into()),
                    to_route_id: Some("ROUTE14".into()),
                    from_trip_id: Some("TRIP13".into()),
                    to_trip_id: Some("TRIP14".into()),
                    transfer_type: 1,
                    min_transfer_time: None
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP16".into()),
                    to_stop_id: None,
                    from_route_id: Some("ROUTE15".into()),
                    to_route_id: Some("ROUTE16".into()),
                    from_trip_id: Some("TRIP15".into()),
                    to_trip_id: Some("TRIP16".into()),
                    transfer_type: 2,
                    min_transfer_time: Some(180)
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP17".into()),
                    to_stop_id: Some("STOP18".into()),
                    from_route_id: Some("ROUTE17".into()),
                    to_route_id: Some("ROUTE18".into()),
                    from_trip_id: Some("TRIP17".into()),
                    to_trip_id: Some("TRIP18".into()),
                    transfer_type: 0,
                    min_transfer_time: Some(0)
                },
                GTFSTransfer {
                    from_stop_id: Some("STOP19".into()),
                    to_stop_id: Some("STOP20".into()),
                    from_route_id: Some("ROUTE19".into()),
                    to_route_id: Some("ROUTE20".into()),
                    from_trip_id: Some("TRIP19".into()),
                    to_trip_id: Some("TRIP20".into()),
                    transfer_type: 2,
                    min_transfer_time: Some(0)
                }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_timeframes() {
        let timeframes = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/timeframes.csv"),
        )
        .unwrap();

        let timeframes = GTFSTimeframe::new(&timeframes);
        assert_eq!(
            timeframes,
            BTreeMap::from([
                (
                    "TF1".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF1".into(),
                        service_id: "SVC1".into(),
                        start_time: Some("10:00:00".into()),
                        end_time: Some("14:00:00".into()),
                    }
                ),
                (
                    "TF2".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF2".into(),
                        service_id: "SVC2".into(),
                        start_time: None,
                        end_time: None,
                    }
                ),
                (
                    "TF3".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF3".into(),
                        service_id: "SVC3".into(),
                        start_time: Some("18:00:00".into()),
                        end_time: None,
                    }
                ),
                (
                    "TF4".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF4".into(),
                        service_id: "SVC4".into(),
                        start_time: Some("22:00:00".into()),
                        end_time: Some("24:00:00".into()),
                    }
                ),
                (
                    "TF5".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF5".into(),
                        service_id: "SVC5".into(),
                        start_time: Some("00:00:00".into()),
                        end_time: Some("24:00:00".into()),
                    }
                ),
                (
                    "TF6".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF6".into(),
                        service_id: "SVC6".into(),
                        start_time: Some("12:30:00".into()),
                        end_time: Some("15:45:00".into()),
                    }
                ),
                (
                    "TF7".into(),
                    GTFSTimeframe {
                        timeframe_group_id: "TF7".into(),
                        service_id: "SVC7".into(),
                        start_time: Some("09:00:00".into()),
                        end_time: Some("09:00:00".into()),
                    }
                ),
            ])
        );
    }

    #[test]
    fn gtfs_schedule_levels() {
        let levels = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/levels.csv"),
        )
        .unwrap();

        let levels = GTFSLevel::new(&levels);
        assert_eq!(
            levels,
            BTreeMap::from([
                (
                    "L1".into(),
                    GTFSLevel {
                        level_id: "L1".into(),
                        level_index: 0,
                        level_name: Some("Ground".into())
                    }
                ),
                (
                    "L2".into(),
                    GTFSLevel {
                        level_id: "L2".into(),
                        level_index: 1,
                        level_name: Some("Mezzanine".into())
                    }
                ),
                (
                    "L3".into(),
                    GTFSLevel {
                        level_id: "L3".into(),
                        level_index: -1,
                        level_name: Some("Basement".into())
                    }
                ),
                (
                    "L4".into(),
                    GTFSLevel { level_id: "L4".into(), level_index: 2, level_name: None }
                ),
                (
                    "L5".into(),
                    GTFSLevel {
                        level_id: "L5".into(),
                        level_index: 3,
                        level_name: Some("Upper Level, East".into())
                    }
                ),
                (
                    "L6".into(),
                    GTFSLevel {
                        level_id: "L6".into(),
                        level_index: 4,
                        level_name: Some("Observation Deck, West".into())
                    }
                ),
                (
                    "L7".into(),
                    GTFSLevel { level_id: "L7".into(), level_index: 5, level_name: None }
                )
            ])
        );
    }

    #[test]
    fn gtfs_schedule_fare_leg_join_rules() {
        let fare_leg_join_rules = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/fare_leg_join_rules.csv"),
        )
        .unwrap();

        let fare_leg_join_rules = GTFSFareLegJoinRule::new(&fare_leg_join_rules);
        assert_eq!(
            fare_leg_join_rules,
            vec![
                GTFSFareLegJoinRule {
                    from_network_id: "networkA".into(),
                    to_network_id: "networkB".into(),
                    from_stop_id: Some("stop1".into()),
                    to_stop_id: Some("stop2".into())
                },
                GTFSFareLegJoinRule {
                    from_network_id: "networkC".into(),
                    to_network_id: "networkD".into(),
                    from_stop_id: None,
                    to_stop_id: None
                },
                GTFSFareLegJoinRule {
                    from_network_id: "networkE".into(),
                    to_network_id: "networkF".into(),
                    from_stop_id: Some("stop3".into()),
                    to_stop_id: Some("stop4".into())
                },
                GTFSFareLegJoinRule {
                    from_network_id: "networkG".into(),
                    to_network_id: "networkH".into(),
                    from_stop_id: Some("stop5".into()),
                    to_stop_id: Some("stop6".into())
                },
                GTFSFareLegJoinRule {
                    from_network_id: "networkI".into(),
                    to_network_id: "networkJ".into(),
                    from_stop_id: None,
                    to_stop_id: None
                }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_location_groups() {
        let location_groups = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/location_groups.csv"),
        )
        .unwrap();

        let location_groups = GTFSLocationGroup::new(&location_groups);
        assert_eq!(
            location_groups,
            BTreeMap::from([(
                "476_stops".into(),
                GTFSLocationGroup {
                    location_group_id: "476_stops".into(),
                    location_group_name: Some(
                        "durch den RufBus 476 bedientes Gebiet im Raum Angermünde".into()
                    )
                }
            )])
        );
    }

    #[test]
    fn gtfs_schedule_networks() {
        let networks = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/networks.csv"),
        )
        .unwrap();

        let networks = GTFSNetwork::new(&networks);
        assert_eq!(
            networks,
            BTreeMap::from([(
                "mnr_hudson".into(),
                GTFSNetwork {
                    network_id: "mnr_hudson".into(),
                    network_name: Some("MNR Hudson Line".into())
                }
            )])
        );
    }

    #[test]
    fn gtfs_schedule_route_networks() {
        let route_networks = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/route_networks.csv"),
        )
        .unwrap();

        let route_networks = GTFSRouteNetwork::new(&route_networks);
        assert_eq!(
            route_networks,
            vec![GTFSRouteNetwork { network_id: "mnr_hudson".into(), route_id: "669".into() }]
        );
    }

    #[test]
    fn gtfs_schedule_stop_areas() {
        let stop_areas = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/stop_areas.csv"),
        )
        .unwrap();

        let stop_areas = GTFSStopArea::new(&stop_areas);
        assert_eq!(
            stop_areas,
            vec![
                GTFSStopArea { area_id: "mnr_1".into(), stop_id: "ITO1887".into() },
                GTFSStopArea { area_id: "mnr_1".into(), stop_id: "ITO2383".into() },
                GTFSStopArea { area_id: "mnr_HUD-5".into(), stop_id: "ITO1804".into() },
                GTFSStopArea { area_id: "mnr_HUD-6".into(), stop_id: "ITO1669".into() },
                GTFSStopArea { area_id: "mnr_HUD-6".into(), stop_id: "ITO1824".into() },
                GTFSStopArea { area_id: "mnr_HUD-7".into(), stop_id: "ITO1856".into() },
                GTFSStopArea { area_id: "mnr_HUD-7".into(), stop_id: "ITO1897".into() },
                GTFSStopArea { area_id: "mnr_HUD-8".into(), stop_id: "ITO1777".into() },
                GTFSStopArea { area_id: "mnr_HUD-8".into(), stop_id: "ITO1789".into() },
                GTFSStopArea { area_id: "mnr_HUD-9".into(), stop_id: "ITO2096".into() }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_translations() {
        let translations = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/translations.csv"),
        )
        .unwrap();

        let translations = GTFSTranslation::new(&translations);
        assert_eq!(
            translations,
            vec![GTFSTranslation {
                table_name: "stops".into(),
                field_name: "stop_name".into(),
                language: "nl".into(),
                translation: "Brussel-West".into(),
                record_id: Some("S8815040".into()),
                record_sub_id: None,
                field_value: None
            }]
        );
    }

    #[test]
    fn gtfs_schedule_fare_transfer_rules() {
        let fare_transfer_rules = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/fare_transfer_rules.csv"),
        )
        .unwrap();

        let fare_transfer_rules = GTFSFareTransferRule::new(&fare_transfer_rules);
        assert_eq!(
            fare_transfer_rules,
            vec![GTFSFareTransferRule {
                from_leg_group_id: Some("core_local_one_way_trip".into()),
                to_leg_group_id: Some("core_local_one_way_trip".into()),
                transfer_count: Some(-1),
                duration_limit: Some(5400),
                duration_limit_type: Some(1),
                fare_transfer_type: 0,
                fare_product_id: None
            }]
        );
    }

    #[test]
    fn gtfs_schedule_feed_info() {
        let feed_info = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/feed_info.csv"),
        )
        .unwrap();

        let feed_info = GTFSFeedInfo::new(&feed_info);
        assert_eq!(
            feed_info,
            BTreeMap::from([(
                "Transport for Cairo".into(),
                GTFSFeedInfo {
                    feed_publisher_name: "Transport for Cairo".into(),
                    feed_publisher_url: "http://transportforcairo.com/".into(),
                    feed_lang: "en".into(),
                    default_lang: None,
                    feed_start_date: Some("20160101".into()),
                    feed_end_date: Some("20161201".into()),
                    feed_version: Some("0.5".into()),
                    feed_contact_email: None,
                    feed_contact_url: None
                }
            )])
        );
    }

    #[test]
    fn gtfs_schedule_frequencies() {
        let frequencies = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/frequencies.csv"),
        )
        .unwrap();

        let frequencies = GTFSFrequency::new(&frequencies);
        assert_eq!(
            frequencies,
            vec![
                GTFSFrequency {
                    trip_id: "22M-GLOBAUX-00-S_1_2".into(),
                    start_time: "16:01:25".into(),
                    end_time: "16:19:25".into(),
                    headway_secs: 180,
                    exact_times: None
                },
                GTFSFrequency {
                    trip_id: "22M-GLOBAUX-00-S_1_2".into(),
                    start_time: "16:19:25".into(),
                    end_time: "17:03:25".into(),
                    headway_secs: 165,
                    exact_times: None
                }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_areas() {
        let areas = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/readers/gtfs/fixtures/areas.csv"),
        )
        .unwrap();

        let areas = GTFSArea::new(&areas);
        assert_eq!(
            areas,
            BTreeMap::from([
                ("ASHB".into(), GTFSArea { area_id: "ASHB".into(), area_name: None }),
                ("GLEN".into(), GTFSArea { area_id: "GLEN".into(), area_name: None }),
                ("OAKL".into(), GTFSArea { area_id: "OAKL".into(), area_name: None })
            ])
        );
    }

    #[test]
    fn gtfs_schedule_fare_media() {
        let fare_media = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/fare_media.csv"),
        )
        .unwrap();

        let fare_media = GTFSFareMedia::new(&fare_media);
        assert_eq!(
            fare_media,
            BTreeMap::from([
                (
                    "cash".into(),
                    GTFSFareMedia {
                        fare_media_id: "cash".into(),
                        fare_media_name: Some("Cash".into()),
                        fare_media_type: 0
                    }
                ),
                (
                    "clipper".into(),
                    GTFSFareMedia {
                        fare_media_id: "clipper".into(),
                        fare_media_name: Some("Clipper".into()),
                        fare_media_type: 2
                    }
                ),
                (
                    "munimobile".into(),
                    GTFSFareMedia {
                        fare_media_id: "munimobile".into(),
                        fare_media_name: Some("SFMTA MuniMobile".into()),
                        fare_media_type: 4
                    }
                )
            ])
        );
    }

    #[test]
    fn gtfs_schedule_fare_leg_rules() {
        let fare_leg_rules = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/fare_leg_rules.csv"),
        )
        .unwrap();

        let fare_leg_rules = GTFSFareLegRule::new(&fare_leg_rules);
        assert_eq!(
            fare_leg_rules,
            vec![
                GTFSFareLegRule {
                    leg_group_id: Some("core_local_one_way_trip".into()),
                    network_id: Some("core".into()),
                    from_area_id: None,
                    to_area_id: None,
                    from_timeframe_group_id: None,
                    to_timeframe_group_id: None,
                    fare_product_id: "core_local_oneway_fare".into(),
                    rule_priority: 0
                },
                GTFSFareLegRule {
                    leg_group_id: Some("core_local_one_way_trip".into()),
                    network_id: Some("core".into()),
                    from_area_id: None,
                    to_area_id: None,
                    from_timeframe_group_id: None,
                    to_timeframe_group_id: None,
                    fare_product_id: "core_local_1_day_fare".into(),
                    rule_priority: 0
                },
                GTFSFareLegRule {
                    leg_group_id: Some("core_local_one_way_trip".into()),
                    network_id: Some("core".into()),
                    from_area_id: None,
                    to_area_id: None,
                    from_timeframe_group_id: None,
                    to_timeframe_group_id: None,
                    fare_product_id: "core_local_31_day_fare".into(),
                    rule_priority: 0
                },
                GTFSFareLegRule {
                    leg_group_id: Some("core_local_one_way_trip".into()),
                    network_id: Some("core".into()),
                    from_area_id: None,
                    to_area_id: None,
                    from_timeframe_group_id: None,
                    to_timeframe_group_id: None,
                    fare_product_id: "core_local_7_day_fare".into(),
                    rule_priority: 0
                }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_fare_products() {
        let fare_products = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/fare_products.csv"),
        )
        .unwrap();

        let fare_products = GTFSFareProduct::new(&fare_products);
        assert_eq!(
            fare_products,
            BTreeMap::from([
                (
                    "core_local_1_day_fare".into(),
                    GTFSFareProduct {
                        fare_product_id: "core_local_1_day_fare".into(),
                        fare_product_name: Some("1-Day Pass - Core Service".into()),
                        amount: 4.6,
                        currency: "USD".into(),
                        fare_media_id: None,
                        rider_category_id: None,
                    }
                ),
                (
                    "core_local_31_day_fare".into(),
                    GTFSFareProduct {
                        fare_product_id: "core_local_31_day_fare".into(),
                        fare_product_name: Some("31-Day Pass - Core Service".into()),
                        amount: 77.0,
                        currency: "USD".into(),
                        fare_media_id: None,
                        rider_category_id: None,
                    }
                ),
                (
                    "core_local_7_day_fare".into(),
                    GTFSFareProduct {
                        fare_product_id: "core_local_7_day_fare".into(),
                        fare_product_name: Some("7-Day Pass - Core Service".into()),
                        amount: 22.0,
                        currency: "USD".into(),
                        fare_media_id: None,
                        rider_category_id: None,
                    }
                ),
                (
                    "core_local_oneway_fare".into(),
                    GTFSFareProduct {
                        fare_product_id: "core_local_oneway_fare".into(),
                        fare_product_name: Some("One Way Full Fare".into()),
                        amount: 2.0,
                        currency: "USD".into(),
                        fare_media_id: None,
                        rider_category_id: None,
                    }
                )
            ])
        );
    }

    #[test]
    fn gtfs_schedule_booking_rules() {
        let booking_rules = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/booking_rules.csv"),
        )
        .unwrap();

        let booking_rules = GTFSBookingRule::new(&booking_rules);
        assert_eq!(
            booking_rules,
            BTreeMap::from([
                (
                    "flächenrufbus_angermünde_weekdays".into(),
                    GTFSBookingRule {
                        booking_rule_id: "flächenrufbus_angermünde_weekdays".into(),
                        booking_type: 1,
                        prior_notice_duration_min: Some(60),
                        prior_notice_duration_max: None,
                        prior_notice_last_day: None,
                        prior_notice_last_time: None,
                        prior_notice_start_day: None,
                        prior_notice_start_time: None,
                        prior_notice_service_id: None,
                        message: Some(
                            "Anmeldung mind. 60min vorher erforderlich, per Anruf zwischen 08:00 und 24:00 möglich, oder online rund um die Uhr".into()
                        ),
                        pickup_message: None,
                        drop_off_message: None,
                        phone_number: Some("+49 3332 442 755".into()),
                        info_url: Some("https://uvg-online.com/rufbus-angermuende/".into()),
                        booking_url: Some("https://uvg.tdimo.net/bapp/#/astBuchungenView".into())
                    }
                ),
                (
                    "flächenrufbus_angermünde_weekends".into(),
                    GTFSBookingRule {
                        booking_rule_id: "flächenrufbus_angermünde_weekends".into(),
                        booking_type: 1,
                        prior_notice_duration_min: Some(60),
                        prior_notice_duration_max: None,
                        prior_notice_last_day: None,
                        prior_notice_last_time: None,
                        prior_notice_start_day: None,
                        prior_notice_start_time: None,
                        prior_notice_service_id: None,
                        message: Some(
                            "1€ Komfortzuschlag pro Person; Anmeldung mind. 60min vorher erforderlich, per Anruf zwischen 08:00 und 24:00 möglich, oder online rund um die Uhr".into()
                        ),
                        pickup_message: None,
                        drop_off_message: None,
                        phone_number: Some("+49 3332 442 755".into()),
                        info_url: Some("https://uvg-online.com/rufbus-angermuende/".into()),
                        booking_url: Some("https://uvg.tdimo.net/bapp/#/astBuchungenView".into())
                    }
                )
            ])
        );
    }

    #[test]
    fn gtfs_schedule_location_group_stops() {
        let location_group_stops = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/location_group_stops.csv"),
        )
        .unwrap();

        let location_group_stops = GTFSLocationGroupStop::new(&location_group_stops);
        assert_eq!(
            location_group_stops,
            vec![
                GTFSLocationGroupStop {
                    location_group_id: "476_stops".into(),
                    stop_id: "de:12073:900340004::1".into()
                },
                GTFSLocationGroupStop {
                    location_group_id: "476_stops".into(),
                    stop_id: "de:12073:900340004::2".into()
                }
            ]
        );
    }

    #[test]
    fn gtfs_schedule_attributions() {
        let attributions = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/attributions.csv"),
        )
        .unwrap();

        let attributions = GTFSAttribution::new(&attributions);
        assert_eq!(
            attributions,
            BTreeMap::from([(
                "Rejseplanen".into(),
                GTFSAttribution {
                    attribution_id: Some("rp".into()),
                    agency_id: None,
                    route_id: None,
                    trip_id: None,
                    organization_name: "Rejseplanen".into(),
                    is_producer: Some("1".into()),
                    is_operator: None,
                    is_authority: None,
                    attribution_url: Some("https://www.rejseplanen.dk".into()),
                    attribution_email: None,
                    attribution_phone: None
                }
            )])
        );
    }

    #[test]
    fn gtfs_schedule_pathways() {
        let pathways = fs::read_to_string(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/pathways.csv"),
        )
        .unwrap();

        let pathways = GTFSPathway::new(&pathways);
        assert_eq!(
            pathways,
            BTreeMap::from([
                (
                    "escalatorA".into(),
                    GTFSPathway {
                        pathway_id: "escalatorA".into(),
                        from_stop_id: "96".into(),
                        to_stop_id: "".into(),
                        pathway_mode: 0,
                        is_bidirectional: 4,
                        length: None,
                        traversal_time: None,
                        stair_count: None,
                        max_slope: None,
                        min_width: None,
                        signposted_as: None,
                        reversed_signposted_as: None
                    }
                ),
                (
                    "stairsA".into(),
                    GTFSPathway {
                        pathway_id: "stairsA".into(),
                        from_stop_id: "90".into(),
                        to_stop_id: "".into(),
                        pathway_mode: 0,
                        is_bidirectional: 2,
                        length: None,
                        traversal_time: None,
                        stair_count: None,
                        max_slope: None,
                        min_width: None,
                        signposted_as: None,
                        reversed_signposted_as: None
                    }
                )
            ])
        );
    }

    #[test]
    fn gtfs_schedule_caltrain_20160406() {
        let gzip_data = fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/readers/gtfs/fixtures/caltrain_20160406.zip"),
        )
        .unwrap();

        let reader = GTFSScheduleReader::from_gzip(&gzip_data);

        assert_eq!(reader.stops.len(), 95);

        let features: Vec<VectorFeature> = reader.iter().collect();

        assert_eq!(features.len(), 103);

        // First
        assert_eq!(
            features[0],
            VectorFeature {
                properties: Properties::from([
                    ("level_id".into(), "".into()),
                    ("location_type".into(), 0_i64.into()),
                    ("parent_station".into(), "ctsf".into()),
                    ("platform_code".into(), "NB".into()),
                    ("stop_code".into(), "70011".into()),
                    ("stop_desc".into(), "".into()),
                    ("stop_id".into(), "70011".into()),
                    ("stop_name".into(), "San Francisco Caltrain".into()),
                    ("stop_timezone".into(), "".into()),
                    (
                        "stop_url".into(),
                        "http://www.caltrain.com/stations/sanfranciscostation.html".into()
                    ),
                    ("tts_stop_name".into(), "".into()),
                    ("wheelchair_boarding".into(), 1_i64.into()),
                    ("zone_id".into(), "1".into()),
                ]),
                geometry: VectorGeometry::Point(VectorBaseGeometry {
                    _type: VectorGeometryType::Point,
                    is_3d: false,
                    coordinates: VectorPoint {
                        x: -122.394992,
                        y: 37.77639,
                        z: None,
                        m: None,
                        t: None
                    },
                    ..Default::default()
                }),
                ..Default::default()
            },
        );

        // Last
        assert_eq!(
            features[features.len() - 1],
            VectorFeature {
                properties: Properties::from([("shape_id".into(), "cal_tam_sj".into()),]),
                geometry: VectorGeometry::LineString(VectorBaseGeometry {
                    _type: VectorGeometryType::LineString,
                    is_3d: false,
                    coordinates: vec![
                        VectorPoint { x: -121.884277, y: 37.311441, z: None, m: None, t: None },
                        VectorPoint { x: -121.884295, y: 37.311464, z: None, m: None, t: None },
                        VectorPoint { x: -121.884329, y: 37.311504, z: None, m: None, t: None },
                        VectorPoint { x: -121.88448, y: 37.311703, z: None, m: None, t: None },
                        VectorPoint { x: -121.884713, y: 37.312007, z: None, m: None, t: None },
                        VectorPoint { x: -121.884939, y: 37.312273, z: None, m: None, t: None },
                        VectorPoint { x: -121.885016, y: 37.312362, z: None, m: None, t: None },
                        VectorPoint { x: -121.885193, y: 37.312567, z: None, m: None, t: None },
                        VectorPoint { x: -121.88545, y: 37.312859, z: None, m: None, t: None },
                        VectorPoint { x: -121.885725, y: 37.313147, z: None, m: None, t: None },
                        VectorPoint { x: -121.886171, y: 37.313593, z: None, m: None, t: None },
                        VectorPoint { x: -121.886634, y: 37.314053, z: None, m: None, t: None },
                        VectorPoint { x: -121.886917, y: 37.314351, z: None, m: None, t: None },
                        VectorPoint { x: -121.887058, y: 37.314494, z: None, m: None, t: None },
                        VectorPoint { x: -121.88729, y: 37.314734, z: None, m: None, t: None },
                        VectorPoint { x: -121.887501, y: 37.314955, z: None, m: None, t: None },
                        VectorPoint { x: -121.887809, y: 37.315285, z: None, m: None, t: None },
                        VectorPoint { x: -121.887864, y: 37.315349, z: None, m: None, t: None },
                        VectorPoint { x: -121.887915, y: 37.315407, z: None, m: None, t: None },
                        VectorPoint { x: -121.888033, y: 37.315542, z: None, m: None, t: None },
                        VectorPoint { x: -121.888133, y: 37.315665, z: None, m: None, t: None },
                        VectorPoint { x: -121.888285, y: 37.315857, z: None, m: None, t: None },
                        VectorPoint { x: -121.888435, y: 37.316054, z: None, m: None, t: None },
                        VectorPoint { x: -121.888515, y: 37.31616, z: None, m: None, t: None },
                        VectorPoint { x: -121.88867, y: 37.316364, z: None, m: None, t: None },
                        VectorPoint { x: -121.888848, y: 37.316626, z: None, m: None, t: None },
                        VectorPoint { x: -121.888903, y: 37.316724, z: None, m: None, t: None },
                        VectorPoint { x: -121.888967, y: 37.316838, z: None, m: None, t: None },
                        VectorPoint { x: -121.889092, y: 37.31708, z: None, m: None, t: None },
                        VectorPoint { x: -121.889203, y: 37.317305, z: None, m: None, t: None },
                        VectorPoint { x: -121.889298, y: 37.31752, z: None, m: None, t: None },
                        VectorPoint { x: -121.889438, y: 37.317876, z: None, m: None, t: None },
                        VectorPoint { x: -121.889522, y: 37.318104, z: None, m: None, t: None },
                        VectorPoint { x: -121.889637, y: 37.318472, z: None, m: None, t: None },
                        VectorPoint { x: -121.889758, y: 37.318909, z: None, m: None, t: None },
                        VectorPoint { x: -121.889986, y: 37.319712, z: None, m: None, t: None },
                        VectorPoint { x: -121.890025, y: 37.319862, z: None, m: None, t: None },
                        VectorPoint { x: -121.890141, y: 37.320299, z: None, m: None, t: None },
                        VectorPoint { x: -121.890211, y: 37.320532, z: None, m: None, t: None },
                        VectorPoint { x: -121.890255, y: 37.320662, z: None, m: None, t: None },
                        VectorPoint { x: -121.890321, y: 37.320838, z: None, m: None, t: None },
                        VectorPoint { x: -121.890444, y: 37.321136, z: None, m: None, t: None },
                        VectorPoint { x: -121.890563, y: 37.321399, z: None, m: None, t: None },
                        VectorPoint { x: -121.890687, y: 37.321652, z: None, m: None, t: None },
                        VectorPoint { x: -121.890832, y: 37.321905, z: None, m: None, t: None },
                        VectorPoint { x: -121.890974, y: 37.322139, z: None, m: None, t: None },
                        VectorPoint { x: -121.891154, y: 37.322421, z: None, m: None, t: None },
                        VectorPoint { x: -121.891454, y: 37.322872, z: None, m: None, t: None },
                        VectorPoint { x: -121.891587, y: 37.323072, z: None, m: None, t: None },
                        VectorPoint { x: -121.891651, y: 37.323167, z: None, m: None, t: None },
                        VectorPoint { x: -121.8919, y: 37.323558, z: None, m: None, t: None },
                        VectorPoint { x: -121.891934, y: 37.323611, z: None, m: None, t: None },
                        VectorPoint { x: -121.892002, y: 37.323719, z: None, m: None, t: None },
                        VectorPoint { x: -121.892027, y: 37.323758, z: None, m: None, t: None },
                        VectorPoint { x: -121.892049, y: 37.32379, z: None, m: None, t: None },
                        VectorPoint { x: -121.892156, y: 37.323918, z: None, m: None, t: None },
                        VectorPoint { x: -121.892198, y: 37.323991, z: None, m: None, t: None },
                        VectorPoint { x: -121.892289, y: 37.324149, z: None, m: None, t: None },
                        VectorPoint { x: -121.892513, y: 37.324519, z: None, m: None, t: None },
                        VectorPoint { x: -121.892705, y: 37.3248, z: None, m: None, t: None },
                        VectorPoint { x: -121.892751, y: 37.32487, z: None, m: None, t: None },
                        VectorPoint { x: -121.893054, y: 37.325554, z: None, m: None, t: None },
                        VectorPoint { x: -121.893242, y: 37.3259, z: None, m: None, t: None },
                        VectorPoint { x: -121.893272, y: 37.325962, z: None, m: None, t: None },
                        VectorPoint { x: -121.893512, y: 37.326406, z: None, m: None, t: None },
                        VectorPoint { x: -121.893567, y: 37.32651, z: None, m: None, t: None },
                        VectorPoint { x: -121.893708, y: 37.326772, z: None, m: None, t: None },
                        VectorPoint { x: -121.893756, y: 37.326856, z: None, m: None, t: None },
                        VectorPoint { x: -121.89381, y: 37.326939, z: None, m: None, t: None },
                        VectorPoint { x: -121.894251, y: 37.327528, z: None, m: None, t: None },
                        VectorPoint { x: -121.894305, y: 37.327587, z: None, m: None, t: None },
                        VectorPoint { x: -121.894401, y: 37.327703, z: None, m: None, t: None },
                        VectorPoint { x: -121.894533, y: 37.327866, z: None, m: None, t: None },
                        VectorPoint { x: -121.894556, y: 37.327898, z: None, m: None, t: None },
                        VectorPoint { x: -121.894629, y: 37.328, z: None, m: None, t: None },
                        VectorPoint { x: -121.89477, y: 37.328233, z: None, m: None, t: None },
                        VectorPoint { x: -121.894946, y: 37.328551, z: None, m: None, t: None },
                        VectorPoint { x: -121.895107, y: 37.328868, z: None, m: None, t: None },
                        VectorPoint { x: -121.895259, y: 37.32917, z: None, m: None, t: None },
                        VectorPoint { x: -121.895332, y: 37.329317, z: None, m: None, t: None },
                        VectorPoint { x: -121.895412, y: 37.329486, z: None, m: None, t: None },
                        VectorPoint { x: -121.895656, y: 37.330005, z: None, m: None, t: None },
                        VectorPoint { x: -121.895737, y: 37.330182, z: None, m: None, t: None },
                        VectorPoint { x: -121.895888, y: 37.330512, z: None, m: None, t: None },
                        VectorPoint { x: -121.895952, y: 37.330767, z: None, m: None, t: None },
                        VectorPoint { x: -121.896012, y: 37.331031, z: None, m: None, t: None },
                        VectorPoint { x: -121.896038, y: 37.331158, z: None, m: None, t: None },
                        VectorPoint { x: -121.896096, y: 37.331442, z: None, m: None, t: None },
                        VectorPoint { x: -121.896115, y: 37.331535, z: None, m: None, t: None },
                        VectorPoint { x: -121.896132, y: 37.331602, z: None, m: None, t: None },
                        VectorPoint { x: -121.896229, y: 37.331966, z: None, m: None, t: None },
                        VectorPoint { x: -121.896361, y: 37.332442, z: None, m: None, t: None },
                        VectorPoint { x: -121.896459, y: 37.332846, z: None, m: None, t: None },
                        VectorPoint { x: -121.896561, y: 37.333265, z: None, m: None, t: None },
                        VectorPoint { x: -121.896585, y: 37.33336, z: None, m: None, t: None },
                        VectorPoint { x: -121.896712, y: 37.3333, z: None, m: None, t: None },
                        VectorPoint { x: -121.896828, y: 37.333245, z: None, m: None, t: None },
                        VectorPoint { x: -121.896948, y: 37.333188, z: None, m: None, t: None },
                        VectorPoint { x: -121.897094, y: 37.333119, z: None, m: None, t: None },
                        VectorPoint { x: -121.897243, y: 37.333048, z: None, m: None, t: None },
                        VectorPoint { x: -121.897631, y: 37.332859, z: None, m: None, t: None },
                        VectorPoint { x: -121.897893, y: 37.33273, z: None, m: None, t: None },
                        VectorPoint { x: -121.898835, y: 37.332279, z: None, m: None, t: None },
                        VectorPoint { x: -121.898922, y: 37.33224, z: None, m: None, t: None },
                        VectorPoint { x: -121.899155, y: 37.332152, z: None, m: None, t: None },
                        VectorPoint { x: -121.899207, y: 37.332139, z: None, m: None, t: None },
                        VectorPoint { x: -121.899326, y: 37.332112, z: None, m: None, t: None },
                        VectorPoint { x: -121.899486, y: 37.332092, z: None, m: None, t: None },
                        VectorPoint { x: -121.900244, y: 37.332076, z: None, m: None, t: None },
                        VectorPoint { x: -121.90124, y: 37.332027, z: None, m: None, t: None },
                        VectorPoint { x: -121.901229, y: 37.331898, z: None, m: None, t: None },
                        VectorPoint { x: -121.901146, y: 37.330973, z: None, m: None, t: None },
                        VectorPoint { x: -121.901089, y: 37.330199, z: None, m: None, t: None },
                        VectorPoint { x: -121.901983, y: 37.330163, z: None, m: None, t: None }
                    ],
                    bbox: Some(BBox3D {
                        left: -121.901983,
                        bottom: 37.311441,
                        right: -121.884277,
                        top: 37.33336,
                        near: 1.7976931348623157e308,
                        far: -1.7976931348623157e308
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
        );
    }
}
