#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::collections::BTreeMap;
    use gistools::{
        parsers::{FeatureReader, FileReader, RGBA},
        readers::{
            LASExtendedVariableLengthRecord, LASHeader, LASPoint, LASReaderOptions, LAZCompressor,
            LAZHeader, LAZHeaderItem, LAZHeaderItemType, LAZReader,
        },
    };
    use s2json::{GetXY, GetZ, VectorPoint};
    use std::path::PathBuf;

    #[test]
    fn test_laz_reader_v2() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/simple.laz");
        let laz_reader = LAZReader::new(FileReader::from(path.clone()), None);

        assert_eq!(
            laz_reader.header,
            LASHeader {
                signature: "LASF".into(),
                source_id: 0,
                encoding: 0,
                project_id1: 0,
                project_id2: 0,
                project_id3: 0,
                project_id4: "".into(),
                major_version: 1,
                minor_version: 2,
                system_identifier: "LAStools (c) by rapidlasso GmbH".into(),
                generating_software: "las2las (version 221128)".into(),
                file_creation_day: 0,
                file_creation_year: 0,
                header_size: 227,
                offset_to_points: 333,
                num_variable_length_records: 1,
                point_data_format_id: 131,
                point_data_record_length: 34,
                num_points: 1065,
                num_points_by_return: [925, 114, 21, 5, 0,],
                x_scale_factor: 0.01,
                y_scale_factor: 0.01,
                z_scale_factor: 0.01,
                x_offset: -0.0,
                y_offset: -0.0,
                z_offset: -0.0,
                max_x: 638982.55,
                min_x: 635619.85,
                max_y: 853535.43,
                min_y: 848899.7000000001,
                max_z: 586.38,
                min_z: 406.59000000000003,
                waveform_data_packet_offset: 0,
                extended_variable_length_record_offset: 0,
                extended_variable_length_size: 0,
                num_points_by_return_ll: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,],
            }
        );

        assert_eq!(
            laz_reader.variable_length_records,
            BTreeMap::from([(
                22204,
                LASExtendedVariableLengthRecord {
                    reserved: 43707,
                    user_id: "laszip encoded".into(),
                    record_id: 22204,
                    record_length: 52,
                    description: "by laszip of LAStools (221128)".into(),
                    data: Some(vec![
                        2, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 3, 0, 6, 0, 20, 0,
                        2, 0, 7, 0, 8, 0, 2, 0, 8, 0, 6, 0, 2, 0
                    ])
                }
            )])
        );
        assert_eq!(laz_reader.len(), 1_065);
        assert!(!laz_reader.is_empty());

        assert_eq!(
            laz_reader.laz_header,
            LAZHeader {
                compressor: LAZCompressor::PointwiseAndChunked,
                coder: 0,
                version_major: 3,
                version_minor: 4,
                version_revision: 3,
                options: 0,
                chunk_size: 50000,
                num_special_evlrs: -1,
                offset_special_evlrs: -1,
                num_items: 3,
                items: vec![
                    LAZHeaderItem { r#type: LAZHeaderItemType::Point10, size: 20, version: 2 },
                    LAZHeaderItem { r#type: LAZHeaderItemType::GpsTime11, size: 8, version: 2 },
                    LAZHeaderItem { r#type: LAZHeaderItemType::Rgb12, size: 6, version: 2 }
                ]
            }
        );

        let first_point = laz_reader.get_point().unwrap();
        assert_eq!(first_point, VectorPoint::new_xyz(637012.2400, 849028.3100, 431.6600, None));
        assert_eq!(
            first_point.m,
            Some(LASPoint {
                x: 63701224,
                y: 84902831,
                z: 43166,
                intensity: 143,
                flags: 73,
                return_number: 1,
                legacy_return_number: 0,
                number_of_returns: 1,
                legacy_number_of_returns: 0,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                legacy_classification: 0,
                classification: 1,
                legacy_point_type: 0,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: -9,
                legacy_scan_angle_rank: 0,
                user_data: 132,
                point_source_id: 7326,
                scanner_channel: 0,
                class_flag: 1,
                scan_angle: 0,
                gps_time_change: None,
                gps_time: Some(245380.78254962614),
                rgba: Some(RGBA {
                    r: 0.04402025454572302,
                    g: 0.04657895458200714,
                    b: 0.04949367621919935,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );

        let second_point = laz_reader.get_point().unwrap();
        assert_eq!(
            second_point,
            VectorPoint::new_xyz(636896.3300, 849087.7000000001, 446.3900, None)
        );
        assert_eq!(
            second_point.m,
            Some(LASPoint {
                x: 63689633,
                y: 84908770,
                z: 44639,
                intensity: 18,
                flags: 81,
                return_number: 1,
                legacy_return_number: 0,
                number_of_returns: 2,
                legacy_number_of_returns: 0,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                legacy_classification: 0,
                classification: 1,
                legacy_point_type: 0,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: -11,
                legacy_scan_angle_rank: 0,
                user_data: 128,
                point_source_id: 7326,
                scanner_channel: 0,
                class_flag: 1,
                scan_angle: 0,
                gps_time_change: None,
                gps_time: Some(245381.45279923646),
                rgba: Some(RGBA {
                    r: 0.03964109677208718,
                    g: 0.04342695493987016,
                    b: 0.04402025454572302,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );

        let third_point = laz_reader.get_point().unwrap();
        assert_eq!(
            third_point,
            VectorPoint::new_xyz(636784.7400, 849106.6600, 426.71000000000004, None)
        );

        let fourth_point = laz_reader.get_point().unwrap();
        assert_eq!(fourth_point, VectorPoint::new_xyz(636699.3800, 848991.0100, 425.3900, None));

        // iterate
        let all_points: Vec<_> = laz_reader.iter().collect();
        assert_eq!(all_points.len(), 1_065);

        let last_point = all_points.last().unwrap();
        let last_p = last_point.geometry.point().unwrap();
        assert_eq!(last_p, &VectorPoint::new_xyz(637342.8500, 853240.3200000001, 423.9200, None));
        assert_eq!(
            last_p.m,
            Some(LASPoint {
                x: 63734285,
                y: 85324032,
                z: 42392,
                intensity: 116,
                flags: 73,
                return_number: 1,
                legacy_return_number: 0,
                number_of_returns: 1,
                legacy_number_of_returns: 0,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                legacy_classification: 0,
                classification: 1,
                legacy_point_type: 0,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: 9,
                legacy_scan_angle_rank: 0,
                user_data: 124,
                point_source_id: 7334,
                scanner_channel: 0,
                class_flag: 1,
                scan_angle: 0,
                gps_time_change: None,
                gps_time: Some(249773.20172406783),
                rgba: Some(RGBA {
                    r: 0.06072482401120425,
                    g: 0.05409300702598807,
                    b: 0.06032319916436203,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );
    }

    #[test]
    fn test_laz_reader_v3() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/simpleV3.laz");
        let laz_reader = LAZReader::new(FileReader::from(path.clone()), None);

        assert_eq!(
            laz_reader.header,
            LASHeader {
                signature: "LASF".into(),
                source_id: 0,
                encoding: 16,
                project_id1: 0,
                project_id2: 0,
                project_id3: 0,
                project_id4: "".into(),
                major_version: 1,
                minor_version: 4,
                system_identifier: "PDAL".into(),
                generating_software: "PDAL 2.8.3 (Releas)".into(),
                file_creation_day: 41,
                file_creation_year: 2025,
                header_size: 375,
                offset_to_points: 475,
                num_variable_length_records: 1,
                point_data_format_id: 135,
                point_data_record_length: 36,
                num_points: 1065,
                num_points_by_return: [0, 0, 0, 0, 0],
                x_scale_factor: 0.01,
                y_scale_factor: 0.01,
                z_scale_factor: 0.01,
                x_offset: 0.0,
                y_offset: 0.0,
                z_offset: 0.0,
                max_x: 638982.55,
                min_x: 635619.85,
                max_y: 853535.43,
                min_y: 848899.7000000001,
                max_z: 586.38,
                min_z: 406.59000000000003,
                waveform_data_packet_offset: 0,
                extended_variable_length_record_offset: 0,
                extended_variable_length_size: 0,
                num_points_by_return_ll: [
                    3972844748800,
                    489626271744,
                    90194313216,
                    21474836480,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0
                ]
            }
        );

        assert_eq!(
            laz_reader.variable_length_records,
            BTreeMap::from([(
                22204,
                LASExtendedVariableLengthRecord {
                    reserved: 0,
                    user_id: "laszip encoded".into(),
                    record_id: 22204,
                    record_length: 46,
                    description: "http://laszip.org".into(),
                    data: Some(vec![
                        3, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 80, 195, 0, 0, 255, 255, 255, 255, 255,
                        255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 2, 0, 10, 0, 30, 0,
                        3, 0, 11, 0, 6, 0, 3, 0
                    ])
                }
            )])
        );
        assert_eq!(laz_reader.len(), 1_065);
        assert!(!laz_reader.is_empty());

        assert_eq!(
            laz_reader.laz_header,
            LAZHeader {
                compressor: LAZCompressor::LayeredAndChunked,
                coder: 0,
                version_major: 3,
                version_minor: 4,
                version_revision: 3,
                options: 0,
                chunk_size: 50000,
                num_special_evlrs: -1,
                offset_special_evlrs: -1,
                num_items: 2,
                items: vec![
                    LAZHeaderItem { r#type: LAZHeaderItemType::Point14, size: 30, version: 3 },
                    LAZHeaderItem { r#type: LAZHeaderItemType::Rgb14, size: 6, version: 3 }
                ]
            }
        );

        let first_point = laz_reader.get_point().unwrap();
        assert_eq!(first_point, VectorPoint::new_xyz(637012.2400, 849028.3100, 431.6600, None));
        assert_eq!(
            first_point.m,
            Some(LASPoint {
                x: 63701224,
                y: 84902831,
                z: 43166,
                intensity: 143,
                flags: 73,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                classification: 1,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: 0,
                user_data: 132,
                point_source_id: 7326,
                legacy_point_type: 0,
                legacy_classification: 1,
                legacy_return_number: 1,
                legacy_number_of_returns: 2,
                legacy_scan_angle_rank: -9,
                scanner_channel: 0,
                class_flag: 0,
                scan_angle: -1500,
                gps_time_change: Some(0),
                gps_time: Some(245380.78254962614),
                rgba: Some(RGBA {
                    r: 0.04402025454572302,
                    g: 0.04657895458200714,
                    b: 0.04949367621919935,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );

        let second_point = laz_reader.get_point().unwrap();
        assert_eq!(
            second_point,
            VectorPoint::new_xyz(636896.3300, 849087.7000000001, 446.3900, None)
        );
        assert_eq!(
            second_point.m,
            Some(LASPoint {
                x: 63689633,
                y: 84908770,
                z: 44639,
                intensity: 18,
                flags: 73,
                return_number: 1,
                number_of_returns: 2,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                classification: 1,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: 0,
                user_data: 128,
                point_source_id: 7326,
                legacy_point_type: 0,
                legacy_classification: 1,
                legacy_return_number: 1,
                legacy_number_of_returns: 2,
                legacy_scan_angle_rank: -11,
                scanner_channel: 0,
                class_flag: 0,
                scan_angle: -1833,
                gps_time_change: Some(0),
                gps_time: Some(245381.45279923646),
                rgba: Some(RGBA {
                    r: 0.03964109677208718,
                    g: 0.04342695493987016,
                    b: 0.04402025454572302,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );

        let third_point = laz_reader.get_point().unwrap();
        assert_eq!(
            third_point,
            VectorPoint::new_xyz(636784.7400, 849106.6600, 426.71000000000004, None)
        );
        assert_eq!(
            third_point.m,
            Some(LASPoint {
                x: 63678474,
                y: 84910666,
                z: 42671,
                intensity: 118,
                flags: 73,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 1,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: 0,
                user_data: 122,
                point_source_id: 7326,
                legacy_point_type: 0,
                legacy_classification: 1,
                legacy_return_number: 1,
                legacy_number_of_returns: 1,
                legacy_scan_angle_rank: -10,
                scanner_channel: 0,
                class_flag: 0,
                scan_angle: -1667,
                gps_time_change: Some(1),
                gps_time: Some(245382.13595006886),
                rgba: Some(RGBA {
                    r: 0.05522766608959828,
                    g: 0.051733518168215115,
                    b: 0.05567377929310104,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );

        // iterate
        let all_points: Vec<_> = laz_reader.iter().collect();
        assert_eq!(all_points.len(), 1_065);

        let last_point = all_points.last().unwrap();
        let last_p = last_point.geometry.point().unwrap();
        assert_eq!(last_p, &VectorPoint::new_xyz(637342.8500, 853240.3200000001, 423.9200, None));
        assert_eq!(
            last_p.m,
            Some(LASPoint {
                x: 63734285,
                y: 85324032,
                z: 42392,
                intensity: 116,
                flags: 73,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: true,
                edge_of_flight_line: false,
                classification: 1,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: 0,
                user_data: 124,
                point_source_id: 7334,
                legacy_point_type: 0,
                legacy_classification: 1,
                legacy_return_number: 1,
                legacy_number_of_returns: 1,
                legacy_scan_angle_rank: 9,
                scanner_channel: 0,
                class_flag: 0,
                scan_angle: 1500,
                gps_time_change: Some(1),
                gps_time: Some(249773.20172406783),
                rgba: Some(RGBA {
                    r: 0.06072482401120425,
                    g: 0.05409300702598807,
                    b: 0.06032319916436203,
                    a: 1.0
                }),
                wave_packet: None,
                nir: None
            })
        );
    }

    #[test]
    fn test_laz_autzen() {
        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("tests/readers/las/fixtures/simple.jsonld");
        // let expected_reader: NewLineDelimitedJSONReader<FileReader, (), Properties, MValue> =
        //     NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        // let expected_features: Vec<_> = expected_reader.iter().collect();
        // assert_eq!(expected_features.len(), 110_000);
        // let expected_features: Vec<_> = expected_features
        //     .iter()
        //     .map(|f| {
        //         let point = f.geometry.point().unwrap();
        //         VectorPoint::from_xyz(point.x(), point.y(), point.z().unwrap_or(0.))
        //     })
        //     .collect();
        // assert_eq!(expected_features.len(), 110_000);

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/autzen_trim.laz");
        let laz_reader = LAZReader::new(
            FileReader::from(path.clone()),
            Some(LASReaderOptions { dont_transform: true, ..Default::default() }),
        );
        let features: Vec<_> = laz_reader.iter().collect();
        let features: Vec<_> = features
            .iter()
            .map(|f| {
                let point = f.geometry.point().unwrap();
                VectorPoint::from_xyz(point.x(), point.y(), point.z().unwrap_or(0.))
            })
            .collect();

        assert_eq!(features.len(), 110_000);

        // Map the features each to ONLY points
        // assert_eq!(features[0..10], expected_features[0..10]);
    }

    #[test]
    fn test_laz_autzen_v3() {
        // let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // path.push("tests/readers/las/fixtures/simple.jsonld");
        // let expected_reader: NewLineDelimitedJSONReader<FileReader, (), Properties, MValue> =
        //     NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        // let expected_features: Vec<_> = expected_reader.iter().collect();
        // assert_eq!(expected_features.len(), 110_000);
        // let expected_features: Vec<_> = expected_features
        //     .iter()
        //     .map(|f| {
        //         let point = f.geometry.point().unwrap();
        //         VectorPoint::from_xyz(point.x(), point.y(), point.z().unwrap_or(0.))
        //     })
        //     .collect();
        // assert_eq!(expected_features.len(), 110_000);

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/autzen_trim_v3.laz");
        let laz_reader = LAZReader::new(
            FileReader::from(path.clone()),
            Some(LASReaderOptions { dont_transform: true, ..Default::default() }),
        );
        let features: Vec<_> = laz_reader.iter().collect();
        let features: Vec<_> = features
            .iter()
            .map(|f| {
                let point = f.geometry.point().unwrap();
                VectorPoint::from_xyz(point.x(), point.y(), point.z().unwrap_or(0.))
            })
            .collect();

        assert_eq!(features.len(), 110_000);

        // Map the features each to ONLY points
        // assert_eq!(features[0..10], expected_features[0..10]);
    }
}
