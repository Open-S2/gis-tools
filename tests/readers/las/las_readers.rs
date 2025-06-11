#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::collections::BTreeMap;
    use gistools::{
        parsers::RGBA,
        parsers::{FeatureReader, FileReader},
        readers::{LASHeader, LASPoint, LASReader},
    };
    use s2json::VectorPoint;
    use std::path::PathBuf;

    #[test]
    fn test_las_reader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/1.2-with-color.las");

        let las_reader = LASReader::new(FileReader::from(path.clone()), None);

        assert_eq!(
            las_reader.header,
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
                system_identifier: "".into(),
                generating_software: "TerraScan".into(),
                file_creation_day: 0,
                file_creation_year: 0,
                header_size: 227,
                offset_to_points: 229,
                num_variable_length_records: 0,
                point_data_format_id: 3,
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

        assert_eq!(las_reader.variable_length_records, BTreeMap::default());
        assert_eq!(las_reader.len(), 1_065);
        assert!(!las_reader.is_empty());

        let first_point = las_reader.get_point(0).unwrap();

        assert_eq!(
            first_point,
            VectorPoint { x: 637012.24, y: 849028.31, z: Some(431.66), m: None, t: None }
        );
        assert_eq!(
            first_point.m.clone().unwrap(),
            LASPoint {
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
            }
        );
        let first_point_rgba = first_point.m.unwrap().rgba.unwrap();
        assert_eq!(first_point_rgba.to_u16s(), (68, 77, 88, 65535));

        let all_points: Vec<_> = las_reader.iter().collect();
        assert_eq!(all_points.len(), 1_065);
    }
}
