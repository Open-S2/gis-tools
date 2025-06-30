#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate tempfile;

    use gistools::{
        parsers::{BufferReader, BufferWriter, FileReader, FileWriter},
        readers::{PMTileType, PMTilesReader, S2PMHeader},
        util::CompressionFormat,
        writers::PMTilesWriter,
    };
    use s2_tilejson::Metadata;
    use s2json::Face;
    use std::format;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_writer_wm() {
        let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
        let file_path = temp_file.path().to_string_lossy().into_owned();

        let file_writer = FileWriter::new(&file_path).unwrap();
        let mut pmtiles_writer = PMTilesWriter::new(file_writer, CompressionFormat::None);

        // setup data
        let tmp_str = "hello world";
        // write data in tile
        pmtiles_writer.write_tile_xyz(0, 0, 0, tmp_str.as_bytes());
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let mut reader = PMTilesReader::new(FileReader::from(file_path), None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: false,
                version: 3,
                root_directory_offset: 262,
                root_directory_length: 5,
                metadata_offset: 267,
                metadata_length: 473,
                leaf_directory_offset: 98315,
                leaf_directory_length: 0,
                data_offset: 98304,
                data_length: 11,
                n_addressed_tiles: 1,
                n_tile_entries: 1,
                n_tile_contents: 0,
                tile_type: PMTileType::Unknown,
                ..Default::default()
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(*metadata, Metadata::default());

        let tile = reader.get_tile_zxy(0, 0, 0).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());

        temp_file.close().unwrap();
    }

    #[test]
    fn test_file_writer_s2() {
        let local_writer = BufferWriter::default();
        let mut pmtiles_writer = PMTilesWriter::new(local_writer, CompressionFormat::None);

        // setup data
        let tmp_str = "hello world";
        // write data in tile
        pmtiles_writer.write_tile_s2(Face::Face0, 0, 0, 0, tmp_str.as_bytes());
        pmtiles_writer.write_tile_s2(Face::Face3, 2, 1, 1, tmp_str.as_bytes());
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let pmtiles_data = pmtiles_writer.take();

        let mut reader = PMTilesReader::new(BufferReader::new(pmtiles_data), None);

        let header = reader.get_header();
        assert_eq!(
            header,
            S2PMHeader {
                is_s2: true,
                version: 1,
                root_directory_offset: 262,
                root_directory_length: 5,
                metadata_offset: 276,
                metadata_length: 473,
                leaf_directory_offset: 98326,
                leaf_directory_length: 0,
                data_offset: 98304,
                data_length: 22,
                n_addressed_tiles: 2,
                n_tile_entries: 0,
                n_tile_contents: 0,
                clustered: false,
                internal_compression: CompressionFormat::None,
                tile_compression: CompressionFormat::None,
                tile_type: PMTileType::Unknown,
                min_zoom: 0,
                max_zoom: 0,
                min_longitude: 0.0,
                min_latitude: 0.0,
                max_longitude: 0.0,
                max_latitude: 0.0,
                center_zoom: 0,
                center_longitude: 0.0,
                center_latitude: 0.0,
                root_directory_offset1: 267,
                root_directory_length1: 1,
                root_directory_offset2: 268,
                root_directory_length2: 1,
                root_directory_offset3: 269,
                root_directory_length3: 5,
                root_directory_offset4: 274,
                root_directory_length4: 1,
                root_directory_offset5: 275,
                root_directory_length5: 1,
                leaf_directory_offset1: 98326,
                leaf_directory_length1: 0,
                leaf_directory_offset2: 98326,
                leaf_directory_length2: 0,
                leaf_directory_offset3: 98326,
                leaf_directory_length3: 0,
                leaf_directory_offset4: 98326,
                leaf_directory_length4: 0,
                leaf_directory_offset5: 98326,
                leaf_directory_length5: 0,
            }
        );

        let metadata = reader.get_metadata();
        assert_eq!(*metadata, Metadata::default());

        let tile = reader.get_tile_s2(Face::Face0, 0, 0, 0).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());

        let tile = reader.get_tile_s2(Face::Face3, 2, 1, 1).unwrap();
        assert_eq!(tile, tmp_str.as_bytes());
    }

    #[test]
    fn test_file_writer_wm_large() {
        let local_writer = BufferWriter::default();
        let mut pmtiles_writer = PMTilesWriter::new(local_writer, CompressionFormat::None);

        // write tiles
        for zoom in 0..8 {
            for x in 0..(1 << zoom) {
                for y in 0..(1 << zoom) {
                    let tmp_str = format!("{zoom}-{x}-{y}");
                    pmtiles_writer.write_tile_xyz(zoom, x, y, tmp_str.as_bytes());
                }
            }
        }
        // finish
        pmtiles_writer.commit(&Metadata::default());

        let pmtiles_data = pmtiles_writer.take();

        let mut reader = PMTilesReader::new(BufferReader::new(pmtiles_data), None);

        let zoom = 5;
        let x = 12;
        let y = 30;

        let tile = reader.get_tile_zxy(zoom, x, y).unwrap();
        let tmp_str = format!("{zoom}-{x}-{y}");
        assert_eq!(tile, tmp_str.as_bytes());
    }
}
