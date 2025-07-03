#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::{
        parsers::{BufferReader, FileReader},
        readers::{PMTileType, PMTilesReader, S2PMHeader},
        util::{CompressionFormat, decompress_data},
    };
    use s2_tilejson::{Encoding, Metadata, Scheme, SourceType, VectorLayer};
    use std::path::PathBuf;

    #[test]
    fn test_fixture_1() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/readers/pmtiles/fixtures/test_fixture_1.pmtiles");
        let file_reader = FileReader::new(path).unwrap();
        let mut reader = PMTilesReader::new(file_reader, None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: false,
                version: 3,
                root_directory_offset: 127,
                root_directory_length: 25,
                metadata_offset: 152,
                metadata_length: 247,
                leaf_directory_offset: 0,
                leaf_directory_length: 0,
                data_offset: 399,
                data_length: 69,
                n_addressed_tiles: 1,
                n_tile_entries: 1,
                n_tile_contents: 1,
                clustered: false,
                internal_compression: CompressionFormat::Gzip,
                tile_compression: CompressionFormat::Gzip,
                tile_type: PMTileType::Pbf,
                min_zoom: 0,
                max_zoom: 0,
                min_longitude: 0.0,
                min_latitude: 0.0,
                max_longitude: 0.9999999,
                max_latitude: 1.0,
                center_zoom: 0,
                center_longitude: 0.0,
                center_latitude: 0.0,
                root_directory_offset1: 0,
                root_directory_length1: 0,
                root_directory_offset2: 0,
                root_directory_length2: 0,
                root_directory_offset3: 0,
                root_directory_length3: 0,
                root_directory_offset4: 0,
                root_directory_length4: 0,
                root_directory_offset5: 0,
                root_directory_length5: 0,
                leaf_directory_offset1: 0,
                leaf_directory_length1: 0,
                leaf_directory_offset2: 0,
                leaf_directory_length2: 0,
                leaf_directory_offset3: 0,
                leaf_directory_length3: 0,
                leaf_directory_offset4: 0,
                leaf_directory_length4: 0,
                leaf_directory_offset5: 0,
                leaf_directory_length5: 0,
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(
            *metadata,
            Metadata {
                s2tilejson: "1.0.0".into(),
                version: "2".into(),
                name: "test_fixture_1.pmtiles".into(),
                scheme: Scheme::Fzxy,
                description: "test_fixture_1.pmtiles".into(),
                r#type: SourceType::Unknown,
                extension: "pbf".into(),
                encoding: Encoding::None,
                minzoom: 0,
                maxzoom: 27,
                vector_layers: vec![VectorLayer {
                    id: "test_fixture_1pmtiles".into(),
                    description: Some("".into()),
                    minzoom: Some(0),
                    maxzoom: Some(0),
                    ..Default::default()
                }],
                ..Default::default()
            }
        );

        let tile = reader.get_tile(None, 0, 0, 0).unwrap();
        assert_eq!(
            tile,
            vec![
                26, 47, 120, 2, 10, 21, 116, 101, 115, 116, 95, 102, 105, 120, 116, 117, 114, 101,
                95, 49, 112, 109, 116, 105, 108, 101, 115, 40, 128, 32, 18, 17, 24, 3, 34, 13, 9,
                150, 32, 232, 31, 26, 0, 24, 21, 0, 0, 23, 15,
            ]
        );
    }

    #[test]
    fn test_fixture_1_local_manager() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/readers/pmtiles/fixtures/test_fixture_1.pmtiles");
        // println!("{}", path.to_str().unwrap());
        let data = std::fs::read(path).unwrap();
        let buf_reader = BufferReader::new(data);
        let mut reader = PMTilesReader::new(buf_reader, None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: false,
                version: 3,
                root_directory_offset: 127,
                root_directory_length: 25,
                metadata_offset: 152,
                metadata_length: 247,
                leaf_directory_offset: 0,
                leaf_directory_length: 0,
                data_offset: 399,
                data_length: 69,
                n_addressed_tiles: 1,
                n_tile_entries: 1,
                n_tile_contents: 1,
                clustered: false,
                internal_compression: CompressionFormat::Gzip,
                tile_compression: CompressionFormat::Gzip,
                tile_type: PMTileType::Pbf,
                min_zoom: 0,
                max_zoom: 0,
                min_longitude: 0.0,
                min_latitude: 0.0,
                max_longitude: 0.9999999,
                max_latitude: 1.0,
                center_zoom: 0,
                center_longitude: 0.0,
                center_latitude: 0.0,
                root_directory_offset1: 0,
                root_directory_length1: 0,
                root_directory_offset2: 0,
                root_directory_length2: 0,
                root_directory_offset3: 0,
                root_directory_length3: 0,
                root_directory_offset4: 0,
                root_directory_length4: 0,
                root_directory_offset5: 0,
                root_directory_length5: 0,
                leaf_directory_offset1: 0,
                leaf_directory_length1: 0,
                leaf_directory_offset2: 0,
                leaf_directory_length2: 0,
                leaf_directory_offset3: 0,
                leaf_directory_length3: 0,
                leaf_directory_offset4: 0,
                leaf_directory_length4: 0,
                leaf_directory_offset5: 0,
                leaf_directory_length5: 0,
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(
            *metadata,
            Metadata {
                s2tilejson: "1.0.0".into(),
                version: "2".into(),
                name: "test_fixture_1.pmtiles".into(),
                scheme: Scheme::Fzxy,
                description: "test_fixture_1.pmtiles".into(),
                r#type: SourceType::Unknown,
                extension: "pbf".into(),
                encoding: Encoding::None,
                minzoom: 0,
                maxzoom: 27,
                vector_layers: vec![VectorLayer {
                    id: "test_fixture_1pmtiles".into(),
                    description: Some("".into()),
                    minzoom: Some(0),
                    maxzoom: Some(0),
                    ..Default::default()
                }],
                ..Default::default()
            }
        );

        let tile = reader.get_tile(None, 0, 0, 0).unwrap();
        assert_eq!(
            tile,
            vec![
                26, 47, 120, 2, 10, 21, 116, 101, 115, 116, 95, 102, 105, 120, 116, 117, 114, 101,
                95, 49, 112, 109, 116, 105, 108, 101, 115, 40, 128, 32, 18, 17, 24, 3, 34, 13, 9,
                150, 32, 232, 31, 26, 0, 24, 21, 0, 0, 23, 15,
            ]
        );
    }

    #[test]
    fn decompress_test() {
        let data = vec![0, 1, 2, 3, 4];
        let decompressed = decompress_data(&data, CompressionFormat::None).unwrap();
        assert_eq!(decompressed, data);
    }
}
