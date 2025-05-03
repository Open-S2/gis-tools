#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::{string::ToString, vec};
    use parsers::Buffer;
    use readers::{PMDirectory, PMEntry, PMHeader, PMTilePos, PMTileType, find_tile};
    use util::CompressionFormat;

    #[test]
    fn test_tile() {
        let tile = PMTilePos { x: 0, y: 0, zoom: 0 };
        assert_eq!(tile, PMTilePos::new(0, 0, 0));

        // from_id
        let tile = PMTilePos::from_id(0);
        assert_eq!(tile, PMTilePos { x: 0, y: 0, zoom: 0 });

        // from_zoom_pos
        let tile = PMTilePos::from_zoom_pos(0, 0);
        assert_eq!(tile, PMTilePos { x: 0, y: 0, zoom: 0 });

        // to_id
        let tile = PMTilePos { x: 0, y: 0, zoom: 0 };
        let id = tile.to_id();
        assert_eq!(id, 0);

        let tile = PMTilePos { x: 1_002, y: 6_969, zoom: 20 };
        let id = tile.to_id();
        assert_eq!(id, 366567509724);
        assert_eq!(PMTilePos::from_id(id), tile);
    }

    // PMEntry
    #[test]
    fn test_entry() {
        let entry = PMEntry { tile_id: 1, offset: 2, length: 3, run_length: 4 };
        assert_eq!(entry, PMEntry::new(1, 2, 3, 4));
    }

    // Directory
    #[test]
    fn test_directory() {
        // new
        let directory = PMDirectory::new(vec![
            PMEntry::new(1, 2, 3, 4),
            PMEntry::new(5, 6, 7, 8),
            PMEntry::new(9, 10, 11, 12),
        ]);
        assert_eq!(
            directory,
            PMDirectory {
                entries: vec![
                    PMEntry::new(1, 2, 3, 4),
                    PMEntry::new(5, 6, 7, 8),
                    PMEntry::new(9, 10, 11, 12),
                ]
            }
        );

        // serialize
        let data = directory.serialize();
        assert_eq!(data, vec![3, 1, 4, 4, 4, 8, 12, 3, 7, 11, 3, 7, 11]);
        let mut buf = Buffer::new(data);
        // from_buffer
        let d2 = PMDirectory::from_buffer(&mut buf);
        assert_eq!(d2, directory);

        // is_empty
        let directory = PMDirectory::new(vec![]);
        assert!(directory.is_empty());
        let directory = PMDirectory::new(vec![PMEntry::new(1, 2, 3, 4)]);
        assert!(!directory.is_empty());

        // len
        let directory = PMDirectory::new(vec![
            PMEntry::new(1, 2, 3, 4),
            PMEntry::new(5, 6, 7, 8),
            PMEntry::new(9, 10, 11, 12),
        ]);
        assert_eq!(directory.len(), 3);

        // first
        let mut directory = PMDirectory::new(vec![
            PMEntry::new(1, 2, 3, 4),
            PMEntry::new(5, 6, 7, 8),
            PMEntry::new(9, 10, 11, 12),
        ]);
        assert_eq!(directory.first(), Some(&PMEntry::new(1, 2, 3, 4)));
        assert_eq!(directory.first_mut(), Some(&mut PMEntry::new(1, 2, 3, 4)));

        // last
        let mut directory = PMDirectory::new(vec![
            PMEntry::new(1, 2, 3, 4),
            PMEntry::new(5, 6, 7, 8),
            PMEntry::new(9, 10, 11, 12),
        ]);
        assert_eq!(directory.last(), Some(&PMEntry::new(9, 10, 11, 12)));
        assert_eq!(directory.last_mut(), Some(&mut PMEntry::new(9, 10, 11, 12)));

        // get
        let mut directory = PMDirectory::new(vec![
            PMEntry::new(0, 2, 3, 4),
            PMEntry::new(1, 6, 7, 8),
            PMEntry::new(9, 10, 11, 12),
        ]);
        assert_eq!(directory.get(0), Some(&PMEntry::new(0, 2, 3, 4)));
        assert_eq!(directory.get_mut(1), Some(&mut PMEntry::new(1, 6, 7, 8)));

        // set
        let mut directory = PMDirectory::new(vec![]);
        directory.set(0, PMEntry::new(1, 2, 3, 4));
        directory.insert(PMEntry::new(5, 6, 7, 8));
    }

    // TileType
    #[test]
    fn test_tile_type() {
        // default
        assert_eq!(PMTileType::Pbf, PMTileType::default());
        // from_u8
        assert_eq!(PMTileType::Unknown, 0_u8.into());
        assert_eq!(PMTileType::Pbf, 1_u8.into());
        assert_eq!(PMTileType::Png, 2_u8.into());
        assert_eq!(PMTileType::Jpeg, 3_u8.into());
        assert_eq!(PMTileType::Webp, 4_u8.into());
        assert_eq!(PMTileType::Avif, 5_u8.into());
        // into_u8
        assert_eq!(0_u8, u8::from(PMTileType::Unknown));
        assert_eq!(1_u8, u8::from(PMTileType::Pbf));
        assert_eq!(2_u8, u8::from(PMTileType::Png));
        assert_eq!(3_u8, u8::from(PMTileType::Jpeg));
        assert_eq!(4_u8, u8::from(PMTileType::Webp));
        assert_eq!(5_u8, u8::from(PMTileType::Avif));
        // to_string
        assert_eq!("unknown".to_string(), String::from(PMTileType::Unknown));
        assert_eq!("pbf".to_string(), String::from(PMTileType::Pbf));
        assert_eq!("png".to_string(), String::from(PMTileType::Png));
        assert_eq!("jpeg".to_string(), String::from(PMTileType::Jpeg));
        assert_eq!("webp".to_string(), String::from(PMTileType::Webp));
        assert_eq!("avif".to_string(), String::from(PMTileType::Avif));
    }

    // Header, from_bytes, to_bytes
    #[test]
    fn test_header() {
        let default_header = PMHeader { version: 3, ..Default::default() };
        let mut buffer = default_header.to_bytes();
        let bytes = buffer.take();
        assert_eq!(
            bytes,
            vec![
                80, 77, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        let from_bytes = PMHeader::from_bytes(&mut Buffer::new(bytes));
        assert_eq!(default_header, from_bytes);

        // set a complex header:
        let header = PMHeader {
            version: 3,
            root_directory_offset: 1,
            root_directory_length: 2,
            metadata_offset: 3,
            metadata_length: 4,
            leaf_directory_offset: 5,
            leaf_directory_length: 6,
            data_offset: 7,
            data_length: 8,
            n_addressed_tiles: 9,
            n_tile_entries: 10,
            n_tile_contents: 11,
            clustered: true,
            internal_compression: CompressionFormat::Brotli,
            tile_compression: CompressionFormat::Zstd,
            tile_type: PMTileType::Jpeg,
            min_zoom: 12,
            max_zoom: 13,
            min_longitude: -180.0,
            min_latitude: -90.0,
            max_longitude: 180.0,
            max_latitude: 90.0,
            center_zoom: 14,
            center_longitude: 15.0,
            center_latitude: 16.0,
        };
        let mut bytes = header.to_bytes();
        let from_bytes = PMHeader::from_bytes(&mut bytes);
        assert_eq!(header, from_bytes);
    }

    // find_tile
    #[test]
    fn test_find_tile() {
        let entries: Vec<PMEntry> = vec![
            PMEntry {
                tile_id: PMTilePos::new(1, 0, 0).to_id(),
                run_length: 0,
                length: 0,
                offset: 0,
            },
            PMEntry {
                tile_id: PMTilePos::new(1, 1, 0).to_id(),
                run_length: 0,
                length: 0,
                offset: 0,
            },
            PMEntry {
                tile_id: PMTilePos::new(1, 0, 1).to_id(),
                run_length: 0,
                length: 0,
                offset: 0,
            },
            PMEntry {
                tile_id: PMTilePos::new(1, 1, 1).to_id(),
                run_length: 0,
                length: 0,
                offset: 0,
            },
        ];
        let none = find_tile(&entries, 10);
        assert_eq!(none, Some(PMEntry { tile_id: 3, run_length: 0, length: 0, offset: 0 }));
        let tile = find_tile(&entries, 4);
        assert_eq!(tile, Some(PMEntry { tile_id: 4, run_length: 0, length: 0, offset: 0 }));

        let entries: Vec<PMEntry> = vec![];
        let none = find_tile(&entries, 10);
        assert_eq!(none, None);
    }
}
