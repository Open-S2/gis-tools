#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::{
        parsers::Buffer,
        readers::{PMDirectory, PMEntry, PMTileType, S2PMEntries, S2PMHeader},
        util::CompressionFormat,
    };
    use s2json::Face;

    #[test]
    fn test_s2_entries() {
        let mut s2entries = S2PMEntries {
            face_0: PMDirectory {
                entries: vec![PMEntry::new(0, 0, 0, 0), PMEntry::new(1, 1, 1, 1)],
            },
            face_1: PMDirectory::default(),
            face_2: PMDirectory::default(),
            face_3: PMDirectory::default(),
            face_4: PMDirectory::default(),
            face_5: PMDirectory::default(),
        };

        // get
        assert_eq!(s2entries.get(Face::Face0), &s2entries.face_0);
        assert_eq!(s2entries.get(Face::Face1), &s2entries.face_1);
        assert_eq!(s2entries.get(Face::Face2), &s2entries.face_2);
        assert_eq!(s2entries.get(Face::Face3), &s2entries.face_3);
        assert_eq!(s2entries.get(Face::Face4), &s2entries.face_4);
        assert_eq!(s2entries.get(Face::Face5), &s2entries.face_5);

        // get mut
        let dir0 = s2entries.get_mut(Face::Face0).clone();
        assert_eq!(dir0, s2entries.face_0.clone());
        let dir1 = s2entries.get_mut(Face::Face1).clone();
        assert_eq!(dir1, s2entries.face_1.clone());
        let dir2 = s2entries.get_mut(Face::Face2).clone();
        assert_eq!(dir2, s2entries.face_2.clone());
        let dir3 = s2entries.get_mut(Face::Face3).clone();
        assert_eq!(dir3, s2entries.face_3.clone());
        let dir4 = s2entries.get_mut(Face::Face4).clone();
        assert_eq!(dir4, s2entries.face_4.clone());
        let dir5 = s2entries.get_mut(Face::Face5).clone();
        assert_eq!(dir5, s2entries.face_5.clone());

        // set
        s2entries.set_dir(
            Face::Face0,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );
        s2entries.set_dir(
            Face::Face1,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );
        s2entries.set_dir(
            Face::Face2,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );
        s2entries.set_dir(
            Face::Face3,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );
        s2entries.set_dir(
            Face::Face4,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );
        s2entries.set_dir(
            Face::Face5,
            PMDirectory { entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)] },
        );

        assert_eq!(
            s2entries,
            S2PMEntries {
                face_0: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
                face_1: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
                face_2: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
                face_3: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
                face_4: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
                face_5: PMDirectory {
                    entries: vec![PMEntry::new(0, 0, 3, 3), PMEntry::new(9, 8, 7, 6)]
                },
            }
        );
    }

    #[test]
    fn test_header() {
        let default_header = S2PMHeader { is_s2: true, version: 1, ..Default::default() };
        let mut buffer = default_header.to_bytes();
        let bytes = buffer.take();
        assert_eq!(
            bytes,
            vec![
                83, 50, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
        let from_bytes = S2PMHeader::from_bytes(&mut Buffer::new(bytes));
        assert_eq!(default_header, from_bytes);

        // set a complex header:
        let header = S2PMHeader {
            is_s2: true,
            version: 1,
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
            min_longitude: 0.0,
            min_latitude: 0.0,
            max_longitude: 0.0,
            max_latitude: 0.0,
            center_zoom: 0,
            center_longitude: 0.0,
            center_latitude: 0.0,
            root_directory_offset1: 17,
            root_directory_offset2: 18,
            root_directory_offset3: 19,
            root_directory_offset4: 20,
            root_directory_offset5: 21,
            root_directory_length1: 22,
            root_directory_length2: 23,
            root_directory_length3: 24,
            root_directory_length4: 25,
            root_directory_length5: 26,
            leaf_directory_offset1: 27,
            leaf_directory_offset2: 28,
            leaf_directory_offset3: 29,
            leaf_directory_offset4: 30,
            leaf_directory_offset5: 31,
            leaf_directory_length1: 32,
            leaf_directory_length2: 33,
            leaf_directory_length3: 34,
            leaf_directory_length4: 35,
            leaf_directory_length5: 36,
        };
        let mut bytes = header.to_bytes();
        let from_bytes = S2PMHeader::from_bytes(&mut bytes);
        assert_eq!(header, from_bytes);

        // get_root_offset
        assert_eq!(header.get_root_offset(0.into()), 1);
        assert_eq!(header.get_root_offset(1.into()), 17);
        assert_eq!(header.get_root_offset(2.into()), 18);
        assert_eq!(header.get_root_offset(3.into()), 19);
        assert_eq!(header.get_root_offset(4.into()), 20);
        assert_eq!(header.get_root_offset(5.into()), 21);

        // get_root_length
        assert_eq!(header.get_root_length(0.into()), 2);
        assert_eq!(header.get_root_length(1.into()), 22);
        assert_eq!(header.get_root_length(2.into()), 23);
        assert_eq!(header.get_root_length(3.into()), 24);
        assert_eq!(header.get_root_length(4.into()), 25);
        assert_eq!(header.get_root_length(5.into()), 26);
    }
}
