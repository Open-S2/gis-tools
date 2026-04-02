#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate tempfile;

    use gistools::{
        parsers::{BufferReader, BufferWriter, FileWriter, Writer},
        readers::S2TilesReader,
        util::CompressionFormat,
        writers::{S2TilesWriter, TileWriter},
    };
    use s2_tilejson::Metadata;
    use std::format;
    use tempfile::NamedTempFile;

    #[test]
    fn test_s2tiles_buffer_writer_wm() {
        smol::block_on(async {
            let local_writer = BufferWriter::default();
            let mut s2tiles_writer = S2TilesWriter::new(local_writer, 9, CompressionFormat::None);

            // setup data
            let s = String::from("hello world");
            let buf = s.as_bytes().to_vec();
            let s2 = String::from("hello world 2");
            let buf2 = s2.as_bytes().to_vec();
            // write data in tile
            s2tiles_writer.write_tile_wm(0, 0, 0, buf.clone());
            s2tiles_writer.write_tile_wm(1, 0, 1, buf.clone());
            s2tiles_writer.write_tile_wm(9, 22, 9, buf2.clone());
            // finish
            s2tiles_writer.commit(Metadata::default(), None);

            let buf_data = s2tiles_writer.writer().take();
            assert_eq!(buf_data.len(), 230_057);
            assert!(!buf_data.is_empty());
            let mut reader = S2TilesReader::new(BufferReader::new(buf_data), None);
            let metadata = reader.get_metadata().await;
            assert_eq!(metadata, Metadata::default());

            assert!(reader.has_tile_wm(0, 0, 0).await);
            assert_eq!(reader.get_tile_wm(0, 0, 0).await.unwrap(), buf);
            assert_eq!(reader.get_tile_wm(1, 0, 1).await.unwrap(), buf);
            assert_eq!(reader.get_tile_wm(9, 22, 9).await.unwrap(), buf2);
            assert!(!reader.has_tile_wm(1, 1, 1).await);
        });
    }

    #[test]
    fn test_s2tiles_file_writer_s2() {
        smol::block_on(async {
            let temp_file = NamedTempFile::new().expect("Failed to create temporary file");
            let file_path = temp_file.path().to_string_lossy().into_owned();

            let file_writer = FileWriter::new(&file_path).unwrap();
            let mut s2tiles_writer = S2TilesWriter::new(file_writer, 8, CompressionFormat::None);

            // setup data
            let s = String::from("hello world");
            let buf = s.as_bytes().to_vec();
            let s2 = String::from("hello world 2");
            let buf2 = s2.as_bytes().to_vec();
            // write data in tile
            s2tiles_writer.write_tile_s2(0.into(), 0, 0, 0, buf.clone());
            s2tiles_writer.write_tile_s2(1.into(), 0, 0, 0, buf.clone());
            s2tiles_writer.write_tile_s2(2.into(), 8, 1, 1, buf2.clone());
            s2tiles_writer.write_tile_s2(3.into(), 2, 1, 1, buf2.clone());
            s2tiles_writer.write_tile_s2(4.into(), 5, 5, 5, buf2.clone());
            s2tiles_writer.write_tile_s2(5.into(), 5, 5, 5, buf.clone());
            // finish
            s2tiles_writer.commit(Metadata::default(), None);

            let end = s2tiles_writer.writer().len();
            let buf_data = s2tiles_writer.writer().slice(0, end);
            assert_eq!(buf_data.len(), 229_214);
            let mut reader = S2TilesReader::new(BufferReader::new(buf_data), None);
            let metadata = reader.get_metadata().await;
            assert_eq!(metadata, Metadata::default());

            assert!(reader.has_tile_s2(0.into(), 0, 0, 0).await);
            assert_eq!(reader.get_tile_s2(0.into(), 0, 0, 0).await.unwrap(), buf);
            assert_eq!(reader.get_tile_s2(1.into(), 0, 0, 0).await.unwrap(), buf);
            assert_eq!(reader.get_tile_s2(3.into(), 2, 1, 1).await.unwrap(), buf2);
            assert_eq!(reader.get_tile_s2(4.into(), 5, 5, 5).await.unwrap(), buf2);
            assert_eq!(reader.get_tile_s2(5.into(), 5, 5, 5).await.unwrap(), buf);
            assert_eq!(reader.get_tile_s2(2.into(), 8, 1, 1).await.unwrap(), buf2);
            assert!(!reader.has_tile_s2(1.into(), 1, 1, 1).await);
        });
    }

    #[test]
    fn test_file_writer_s2_wm_large() {
        smol::block_on(async {
            let local_writer = BufferWriter::default();
            let mut pmtiles_writer = S2TilesWriter::new(local_writer, 8, CompressionFormat::None);

            // write tiles
            for zoom in 0..8 {
                for x in 0..(1 << zoom) {
                    for y in 0..(1 << zoom) {
                        let tmp_str = format!("{zoom}-{x}-{y}").as_bytes().to_vec();
                        pmtiles_writer.write_tile_wm(zoom, x, y, tmp_str);
                    }
                }
            }
            // finish
            pmtiles_writer.commit(Metadata::default(), None);

            // test reading
            let pmtiles_data = pmtiles_writer.writer().take();
            let mut reader = S2TilesReader::new(BufferReader::new(pmtiles_data), None);

            // Random tile test 1
            let zoom = 5;
            let x = 12;
            let y = 30;
            let tile = reader.get_tile_wm(zoom, x, y).await.unwrap();
            assert_eq!(String::from_utf8_lossy(&tile), format!("{zoom}-{x}-{y}"));

            // Random tile test 2
            let zoom = 6;
            let x = 22;
            let y = 45;
            let tile = reader.get_tile_wm(zoom, x, y).await.unwrap();
            assert_eq!(String::from_utf8_lossy(&tile), format!("{zoom}-{x}-{y}"));
        });
    }
}
