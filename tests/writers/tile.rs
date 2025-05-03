#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::{
        util::{CompressionFormat, Date},
        writers::{
            FileTileWriter, LocalTileWriter, S2TileKey, TemporalTileWriter, TileKey, TileWriter,
        },
    };
    use s2_tilejson::Metadata;
    use std::{env, fs};

    #[test]
    fn test_tile_writing() {
        let mut writer = LocalTileWriter::new();

        writer.write_tile_wm(16, 2, 3, vec![0, 1, 2]);
        writer.write_tile_s2(4.into(), 3, 2, 1, vec![3, 2, 1]);
        writer.write_temporal_tile_wm(Date::new(2024, 8, 2), 3, 0, 0, vec![1, 1, 1]);
        writer.write_temporal_tile_s2(Date::new(2002, 1, 1), 0.into(), 0, 0, 0, vec![2, 2, 2]);

        writer.commit(Metadata::default(), Some(CompressionFormat::Brotli));

        assert_eq!(
            writer.tiles.get(&TileKey::S2(S2TileKey { face: 4.into(), zoom: 3, x: 2, y: 1 })),
            Some(&vec![3, 2, 1])
        );
    }

    #[test]
    fn test_tile_writing_file() {
        let tmp_d = env::temp_dir().join("test_tile_writing");
        fs::create_dir_all(&tmp_d).unwrap();
        let tmp_dir: String = tmp_d.to_string_lossy().into();

        let mut writer = FileTileWriter::new(tmp_dir, Some("webp".into()));

        writer.write_tile_wm(16, 2, 3, vec![0, 1, 2]);
        writer.write_tile_s2(4.into(), 3, 2, 1, vec![3, 2, 1]);
        writer.write_temporal_tile_wm(Date::new(2024, 8, 2), 3, 0, 0, vec![1, 1, 1]);
        writer.write_temporal_tile_s2(Date::new(2002, 1, 1), 0.into(), 0, 0, 0, vec![2, 2, 2]);

        writer.commit(Metadata::default(), Some(CompressionFormat::Brotli));

        // let tile_reader = RasterTileReader::new(path);

        // assert_eq!(
        //     writer.tiles.get(&TileKey::S2(S2TileKey { face: 4.into(), zoom: 3, x: 2, y: 1 })),
        //     Some(&vec![3, 2, 1])
        // );
    }
}
