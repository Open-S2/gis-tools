#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::collections::BTreeMap;
    use gistools::{
        parsers::{FeatureReader, FileReader, RGBA},
        readers::{
            LASExtendedVariableLengthRecord, LASHeader, LASPoint, LASReader, LASReaderOptions,
        },
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

    #[test]
    fn test_las_reader_projection_1_2_0() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/las/fixtures/1.2_0.las");

        let las_reader = LASReader::new(
            FileReader::from(path.clone()),
            Some(LASReaderOptions {
                epsg_codes: BTreeMap::from([(
                    "26915".into(),
                    "PROJCRS[\"NAD83 / UTM zone 15N\",BASEGEOGCRS[\"NAD83\",DATUM[\"North \
                     American Datum 1983\",ELLIPSOID[\"GRS \
                     1980\",6378137,298.257222101,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"\
                     EPSG\",7019]],ID[\"EPSG\",6269]],ID[\"EPSG\",4269]],CONVERSION[\"UTM zone \
                     15N\",METHOD[\"Transverse Mercator\",ID[\"EPSG\",9807]],PARAMETER[\"Latitude \
                     of natural \
                     origin\",0,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"\
                     EPSG\",8801]],PARAMETER[\"Longitude of natural \
                     origin\",-93,ANGLEUNIT[\"degree\",0.0174532925199433,ID[\"EPSG\",9102]],ID[\"\
                     EPSG\",8802]],PARAMETER[\"Scale factor at natural \
                     origin\",0.9996,SCALEUNIT[\"unity\",1,ID[\"EPSG\",9201]],ID[\"EPSG\",8805]],\
                     PARAMETER[\"False \
                     easting\",500000,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",\
                     8806]],PARAMETER[\"False \
                     northing\",0,LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",8807]],\
                     ID[\"EPSG\",16015]],CS[Cartesian,2,ID[\"EPSG\",4400]],AXIS[\"Easting \
                     (E)\",east],AXIS[\"Northing \
                     (N)\",north],LENGTHUNIT[\"metre\",1,ID[\"EPSG\",9001]],ID[\"EPSG\",26915]]"
                        .into(),
                )]),
                dont_transform: false,
            }),
        );

        assert_eq!(
            las_reader.header,
            LASHeader {
                signature: "LASF".into(),
                source_id: 0,
                encoding: 0,
                project_id1: 2206790072,
                project_id2: 43547,
                project_id3: 16648,
                project_id4: "��kƎ{\u{6}.".into(),
                major_version: 1,
                minor_version: 2,
                system_identifier: "libLAS".into(),
                generating_software: "libLAS 1.2".into(),
                file_creation_day: 78,
                file_creation_year: 2008,
                header_size: 227,
                offset_to_points: 438,
                num_variable_length_records: 2,
                point_data_format_id: 0,
                point_data_record_length: 20,
                num_points: 1,
                num_points_by_return: [0, 1, 0, 0, 0],
                x_scale_factor: 0.01,
                y_scale_factor: 0.01,
                z_scale_factor: 0.01,
                x_offset: 0.0,
                y_offset: 0.0,
                z_offset: 0.0,
                max_x: 470692.447538,
                min_x: 470692.447538,
                max_y: 4602888.904642,
                min_y: 4602888.904642,
                max_z: 16.0,
                min_z: 16.0,
                waveform_data_packet_offset: 0,
                extended_variable_length_record_offset: 0,
                extended_variable_length_size: 0,
                num_points_by_return_ll: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            }
        );

        assert_eq!(
            las_reader.variable_length_records,
            BTreeMap::from([
                (
                    34735,
                    LASExtendedVariableLengthRecord {
                        reserved: 0,
                        user_id: "LASF_Projection".into(),
                        record_id: 34735,
                        record_length: 64,
                        description: "".into(),
                        data: Some(vec![
                            1, 0, 1, 0, 0, 0, 7, 0, 0, 4, 0, 0, 1, 0, 1, 0, 1, 4, 0, 0, 1, 0, 1, 0,
                            2, 4, 177, 135, 33, 0, 0, 0, 1, 8, 177, 135, 6, 0, 33, 0, 6, 8, 0, 0,
                            1, 0, 142, 35, 0, 12, 0, 0, 1, 0, 35, 105, 4, 12, 0, 0, 1, 0, 41, 35
                        ])
                    }
                ),
                (
                    34737,
                    LASExtendedVariableLengthRecord {
                        reserved: 0,
                        user_id: "LASF_Projection".into(),
                        record_id: 34737,
                        record_length: 39,
                        description: "".into(),
                        data: Some(vec![
                            85, 84, 77, 32, 90, 111, 110, 101, 32, 49, 53, 44, 32, 78, 111, 114,
                            116, 104, 101, 114, 110, 32, 72, 101, 109, 105, 115, 112, 104, 101,
                            114, 101, 124, 78, 65, 68, 56, 51, 124
                        ])
                    }
                ),
            ])
        );
        assert_eq!(las_reader.len(), 1);
        assert!(!las_reader.is_empty());

        let first_point = las_reader.get_point(0).unwrap();
        assert_eq!(
            first_point,
            VectorPoint {
                x: -93.35156259019989,
                y: 41.577148395419115,
                z: Some(16.),
                m: None,
                t: None
            }
        );
        assert_eq!(
            first_point.m.clone().unwrap(),
            LASPoint {
                x: 47069244,
                y: 460288890,
                z: 1600,
                intensity: 0,
                flags: 2,
                return_number: 2,
                number_of_returns: 0,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 2,
                is_synthetic: false,
                is_key_point: false,
                is_withheld: false,
                scan_angle_rank: -13,
                user_data: 0,
                point_source_id: 0,
                legacy_point_type: 0,
                legacy_classification: 0,
                legacy_return_number: 0,
                legacy_number_of_returns: 0,
                legacy_scan_angle_rank: 0,
                scanner_channel: 0,
                class_flag: 2,
                scan_angle: 0,
                gps_time_change: None,
                gps_time: None,
                rgba: None,
                wave_packet: None,
                nir: None
            }
        );
    }
}
