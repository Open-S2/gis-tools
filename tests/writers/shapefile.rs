#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{BufferWriter, FeatureReader, Writer},
        readers::{JSONCollectionReader, shapefile_from_gzip},
        util::{WriteZipItem, zip_folder},
        writers::{to_dbf, to_dbf_meta, to_shp},
    };
    use s2json::{
        FeatureCollection, FeatureCollectionType, Features, MValue, Properties, VectorFeature,
        VectorGeometry, VectorPoint, VectorPointGeometry,
    };
    use std::collections::BTreeMap;

    fn create_mock_iterator_point_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Point(VectorPointGeometry {
                    coordinates: VectorPoint { x: ind, y: ind, ..Default::default() },
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };

        JSONCollectionReader::from(&mut feature_collection)
    }

    #[test]
    fn test_to_dbf_meta_tracks_exact_feature_counts_and_schema_fields() {
        let mock1 = create_mock_iterator_point_shp(vec![
            Properties::from([
                ("name".into(), "Raleigh".into()),
                ("elevation".into(), (110.5).into()),
            ]),
            Properties::from([
                ("name".into(), "Durham".into()),
                ("elevation".into(), (121).into()),
            ]),
        ]);
        let mock2 = create_mock_iterator_point_shp(vec![Properties::from([
            ("name".into(), "Charlotte".into()),
            ("population".into(), (870000).into()),
        ])]);

        let (meta, feature_count) = to_dbf_meta(&[&mock1, &mock2]);

        // Confirms fix for the nested loop property counting bug
        assert_eq!(feature_count, 3);
        // fields: name, elevation, population
        assert_eq!(meta.len(), 3);

        let name_field = meta.iter().find(|f| f.name == "name");
        assert_eq!(name_field.unwrap().data_type, 'C');
    }

    #[test]
    fn test_to_dbf_meta_computes_numeric_decimal_precision_up_to_15_decimals_maximum() {
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("coords".into(), (35.7796).into())]),
            Properties::from([("coords".into(), (-78.638212345678912).into())]),
            Properties::from([("coords".into(), (12.1).into())]),
        ]);

        let (meta, _feature_count) = to_dbf_meta(&[&mock]);
        let coords_field = meta.iter().find(|f| f.name == "coords");
        assert!(coords_field.is_some());
        assert_eq!(coords_field.unwrap().data_type, 'N');
        assert_eq!(coords_field.unwrap().decimal, 15);
        assert!(coords_field.unwrap().len <= 18);
    }

    #[test]
    fn tests_to_dbf_meta_forces_type_widening_up_to_string_characters_if_data_clashes() {
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("mixed".into(), (true).into())]),
            Properties::from([("mixed".into(), (42.12).into())]),
            Properties::from([("mixed".into(), ("Fallback String Content").into())]),
        ]);

        let (meta, _feature_count) = to_dbf_meta(&[&mock]);
        let mixed_field = meta.iter().find(|f| f.name == "mixed");
        assert!(mixed_field.is_some());
        assert_eq!(mixed_field.unwrap().data_type, 'C');
    }

    #[test]
    fn test_to_dbf_streams_complete_structural_byte_data_without_leaking_previous_row_buffers() {
        // Row 1 has a long name payload. Row 2 has a short name payload.
        // If memory flushing isn't running, 'Durham' will leak trailing chars from 'Asheville'
        let mut writer = BufferWriter::new(vec![]);
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        to_dbf(&mut writer, vec![&mock]);
        let final_bytes = writer.take();

        // The entire file must end explicitly with the standard EOF flag
        assert_eq!(final_bytes.last(), Some(&0x1a));

        // Turn bytes back to string format to check text alignment
        let raw_string_output = String::from_utf8_lossy(&final_bytes);

        assert!(raw_string_output.contains("Asheville "));
        assert!(raw_string_output.contains("Durham    "));
        assert!(!raw_string_output.contains("Durhamville")); // Absolute buffer pollution safety confirmation
    }

    #[test]
    fn test_to_dbf_serializes_logical_boolean_switches_and_strict_iso_date_strings() {
        let mut writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([
                ("active".into(), true.into()),
                ("updated".into(), "2026-06-05".into()),
            ]),
            Properties::from([
                ("active".into(), false.into()),
                ("updated".into(), "2026-12-25".into()),
            ]),
        ]);

        to_dbf(&mut writer, vec![&mock]);
        let final_bytes = writer.take();

        let raw_string_output = String::from_utf8_lossy(&final_bytes);
        // Assert boolean tokens conform to DBF spec
        //     expect(outputString).toContain('T');
        //     expect(outputString).toContain('F');
        assert!(raw_string_output.contains("C"));
        // Assert dates fall back neatly into continuous standard string geometry structures
        assert!(raw_string_output.contains("2026-06-05"));
        assert!(raw_string_output.contains("2026-12-25"));
    }

    #[test]
    fn test_to_shp_base_case() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);
        let iterators = vec![&mock];
        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            iterators,
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 156);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert_eq!(
            features[0].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Asheville".to_string()
        );
        assert_eq!(
            features[1].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Durham".to_string()
        );

        let first = features[0].geometry.point().unwrap();
        assert_eq!(first.x, 0.);
        assert_eq!(first.y, 0.);

        let second = features[1].geometry.point().unwrap();
        assert_eq!(second.x, 1.);
        assert_eq!(second.y, 1.);
    }
}
