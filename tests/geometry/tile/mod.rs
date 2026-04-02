#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use gistools::geometry::{GetTileID, S2CellId, S2TileID, TileID, WMTileID};
    use s2json::{BBox, Feature, Geometry, VectorPoint};
    use std::{fs, panic, path::PathBuf};

    #[test]
    fn tile_id() {
        // x
        assert_eq!(TileID::WM(WMTileID::new(0, 0, 0)).x(), 0);
        assert_eq!(TileID::WM(WMTileID::new(0, 1, 0)).x(), 1);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).x(), 0);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 0, 1, 0)).x(), 1);
        // y
        assert_eq!(TileID::WM(WMTileID::new(0, 0, 0)).y(), 0);
        assert_eq!(TileID::WM(WMTileID::new(0, 0, 1)).y(), 1);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).y(), 0);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 0, 0, 1)).y(), 1);
        // zoom
        assert_eq!(TileID::WM(WMTileID::new(0, 0, 0)).zoom(), 0);
        assert_eq!(TileID::WM(WMTileID::new(1, 0, 0)).zoom(), 1);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).zoom(), 0);
        assert_eq!(TileID::S2(S2TileID::new(0.into(), 1, 0, 0)).zoom(), 1);

        // from->to WMTileID
        let wm_tile_id = WMTileID::new(0, 0, 0);
        let tile_id: TileID = wm_tile_id.into();
        let wm_tile_id2: WMTileID = tile_id.into();
        assert_eq!(wm_tile_id, wm_tile_id2);
        let panic_test = panic::catch_unwind(|| S2TileID::from(tile_id));
        assert!(panic_test.is_err());
        // from->to S2TileID
        let s2_tile_id = S2TileID::new(0.into(), 0, 0, 0);
        let tile_id: TileID = s2_tile_id.into();
        let s2_tile_id2: S2TileID = tile_id.into();
        assert_eq!(s2_tile_id, s2_tile_id2);
        let panic_test = panic::catch_unwind(|| WMTileID::from(tile_id));
        assert!(panic_test.is_err());
    }

    #[test]
    fn to_id() {
        // wm
        assert_eq!(WMTileID::new(0, 0, 0).to_id(), 1152921504606846976.into());
        assert_eq!(WMTileID::new(1, 1, 0).to_id(), 2017612633061982208.into());
        assert_eq!(TileID::WM(WMTileID::new(0, 0, 0)).to_id(), 1152921504606846976.into());
        assert_eq!(S2CellId::from(WMTileID::new(0, 0, 0)), 1152921504606846976.into());
        assert_eq!(S2CellId::from(TileID::WM(WMTileID::new(0, 0, 0))), 1152921504606846976.into());
        // s2
        assert_eq!(S2TileID::new(0.into(), 0, 0, 0).to_id(), 1152921504606846976.into());
        assert_eq!(S2TileID::new(0.into(), 1, 1, 0).to_id(), 2017612633061982208.into());
        assert_eq!(S2TileID::new(1.into(), 1, 1, 0).to_id(), 3170534137668829184.into());
        assert_eq!(
            TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).to_id(),
            1152921504606846976.into()
        );
        assert_eq!(S2CellId::from(S2TileID::new(0.into(), 0, 0, 0)), 1152921504606846976.into());
        assert_eq!(
            S2CellId::from(TileID::S2(S2TileID::new(0.into(), 0, 0, 0))),
            1152921504606846976.into()
        );
    }

    #[test]
    fn from_id() {
        // wm
        assert_eq!(WMTileID::from_id(1152921504606846976.into()), WMTileID::new(0, 0, 0));
        assert_eq!(WMTileID::from_id(2017612633061982208.into()), WMTileID::new(1, 1, 0));
        assert_eq!(
            TileID::from_id(1152921504606846976.into(), true),
            TileID::WM(WMTileID::new(0, 0, 0))
        );
        assert_eq!(WMTileID::from(S2CellId::from(1152921504606846976)), WMTileID::new(0, 0, 0));
        // s2
        assert_eq!(S2TileID::from_id(1152921504606846976.into()), S2TileID::new(0.into(), 0, 0, 0));
        assert_eq!(S2TileID::from_id(2017612633061982208.into()), S2TileID::new(0.into(), 1, 1, 0));
        assert_eq!(S2TileID::from_id(3170534137668829184.into()), S2TileID::new(1.into(), 1, 1, 0));
        assert_eq!(
            TileID::from_id(1152921504606846976.into(), false),
            TileID::S2(S2TileID::new(0.into(), 0, 0, 0))
        );
        assert_eq!(
            S2TileID::from(S2CellId::from(1152921504606846976)),
            S2TileID::new(0.into(), 0, 0, 0)
        );
    }

    #[test]
    fn to_bbox() {
        // wm
        assert_eq!(
            WMTileID::new(0, 0, 0).to_bbox(None),
            BBox::new(-180., -85.05112877980659, 180., 85.0511287798066)
        );
        assert_eq!(WMTileID::new(1, 1, 0).to_bbox(None), BBox::new(0., 0., 180., 85.0511287798066));
        assert_eq!(
            WMTileID::new(1, 1, 0).to_bbox(Some(true)),
            BBox::new(0., -85.05112877980659, 180., 0.)
        );
        assert_eq!(
            TileID::WM(WMTileID::new(0, 0, 0)).to_bbox(None),
            BBox::new(-180., -85.05112877980659, 180., 85.0511287798066)
        );
        assert_eq!(
            TileID::WM(WMTileID::new(1, 1, 0)).to_bbox(Some(true)),
            BBox::new(0., -85.05112877980659, 180., 0.)
        );
        // s2
        assert_eq!(
            S2TileID::new(0.into(), 0, 0, 0).to_bbox(None),
            BBox::new(-45., -35.264389682754654, 45., 35.264389682754654)
        );
        assert_eq!(S2TileID::new(0.into(), 1, 1, 0).to_bbox(None), BBox::new(0., -45., 45., 0.));
        assert_eq!(S2TileID::new(1.into(), 1, 1, 0).to_bbox(None), BBox::new(90., -45., 135., 0.));
        assert_eq!(
            TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).to_bbox(None),
            BBox::new(-45., -35.264389682754654, 45., 35.264389682754654)
        );
    }

    #[test]
    fn tile_to_center_lon_lat() {
        // wm
        assert_eq!(
            WMTileID::new(0, 0, 0).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(0., 7.105427357601002e-15)
        );
        assert_eq!(
            WMTileID::new(1, 1, 0).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(90., 42.5255643899033)
        );
        assert_eq!(
            WMTileID::new(1, 1, 0).to_center_lon_lat::<VectorPoint>(Some(true)),
            VectorPoint::from_xy(90., -42.525564389903295)
        );
        assert_eq!(
            TileID::WM(WMTileID::new(0, 0, 0)).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(0., 7.105427357601002e-15)
        );
        assert_eq!(
            TileID::WM(WMTileID::new(1, 1, 0)).to_center_lon_lat::<VectorPoint>(Some(true)),
            VectorPoint::from_xy(90., -42.525564389903295)
        );
        // s2
        assert_eq!(
            S2TileID::new(0.into(), 0, 0, 0).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(0., 0.)
        );
        assert_eq!(
            S2TileID::new(0.into(), 1, 1, 0).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(22.5, -22.5)
        );
        assert_eq!(
            S2TileID::new(1.into(), 1, 1, 0).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(112.5, -22.5)
        );
        assert_eq!(
            TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).to_center_lon_lat::<VectorPoint>(None),
            VectorPoint::from_xy(0., 0.)
        );
    }

    #[test]
    fn children() {
        // wm
        assert_eq!(
            WMTileID::new(0, 0, 0).children(),
            vec![
                WMTileID::new(1, 0, 0),
                WMTileID::new(1, 0, 1),
                WMTileID::new(1, 1, 0),
                WMTileID::new(1, 1, 1)
            ]
        );
        assert_eq!(
            TileID::WM(WMTileID::new(0, 0, 0)).children(),
            vec![
                TileID::WM(WMTileID::new(1, 0, 0)),
                TileID::WM(WMTileID::new(1, 0, 1)),
                TileID::WM(WMTileID::new(1, 1, 0)),
                TileID::WM(WMTileID::new(1, 1, 1))
            ]
        );
        // s2
        assert_eq!(
            S2TileID::new(0.into(), 0, 0, 0).children(),
            vec![
                S2TileID::new(0.into(), 1, 0, 0),
                S2TileID::new(0.into(), 1, 1, 0),
                S2TileID::new(0.into(), 1, 1, 1),
                S2TileID::new(0.into(), 1, 0, 1)
            ]
        );
        assert_eq!(
            S2TileID::new(2.into(), 1, 0, 0).children(),
            vec![
                S2TileID::new(2.into(), 2, 0, 0),
                S2TileID::new(2.into(), 2, 0, 1),
                S2TileID::new(2.into(), 2, 1, 1),
                S2TileID::new(2.into(), 2, 1, 0)
            ]
        );
        assert_eq!(
            TileID::S2(S2TileID::new(0.into(), 0, 0, 0)).children(),
            vec![
                TileID::S2(S2TileID::new(0.into(), 1, 0, 0)),
                TileID::S2(S2TileID::new(0.into(), 1, 1, 0)),
                TileID::S2(S2TileID::new(0.into(), 1, 1, 1)),
                TileID::S2(S2TileID::new(0.into(), 1, 0, 1))
            ]
        );
    }

    #[test]
    fn parent() {
        // wm
        assert_eq!(WMTileID::new(0, 0, 0).parent(), None);
        assert_eq!(WMTileID::new(1, 1, 0).parent(), Some(WMTileID::new(0, 0, 0)));
        assert_eq!(
            TileID::WM(WMTileID::new(1, 1, 0)).parent(),
            Some(TileID::WM(WMTileID::new(0, 0, 0)))
        );
        // s2
        assert_eq!(S2TileID::new(0.into(), 0, 0, 0).parent(), None);
        assert_eq!(
            S2TileID::new(0.into(), 1, 1, 0).parent(),
            Some(S2TileID::new(0.into(), 0, 0, 0))
        );
        assert_eq!(
            S2TileID::new(1.into(), 1, 1, 0).parent(),
            Some(S2TileID::new(1.into(), 0, 0, 0))
        );
        assert_eq!(
            TileID::S2(S2TileID::new(1.into(), 1, 1, 0)).parent(),
            Some(TileID::S2(S2TileID::new(1.into(), 0, 0, 0)))
        );
    }

    #[test]
    fn neighbors() {
        // wm
        assert_eq!(
            WMTileID::new(1, 1, 0).neighbors(),
            vec![WMTileID::new(1, 0, 0), WMTileID::new(1, 1, 1)]
        );
        assert_eq!(
            TileID::WM(WMTileID::new(1, 1, 0)).neighbors(),
            vec![TileID::WM(WMTileID::new(1, 0, 0)), TileID::WM(WMTileID::new(1, 1, 1))]
        );
        // s2
        assert_eq!(
            S2TileID::new(0.into(), 1, 1, 0).neighbors(),
            vec![
                S2TileID::new(5.into(), 1, 1, 1),
                S2TileID::new(1.into(), 1, 0, 0),
                S2TileID::new(0.into(), 1, 1, 1),
                S2TileID::new(0.into(), 1, 0, 0),
            ]
        );
        assert_eq!(
            TileID::S2(S2TileID::new(0.into(), 1, 1, 0)).neighbors(),
            vec![
                TileID::S2(S2TileID::new(5.into(), 1, 1, 1)),
                TileID::S2(S2TileID::new(1.into(), 1, 0, 0)),
                TileID::S2(S2TileID::new(0.into(), 1, 1, 1)),
                TileID::S2(S2TileID::new(0.into(), 1, 0, 0)),
            ]
        );
    }

    #[test]
    fn tiles_from_vector_point() {
        let point = VectorPoint::from_xy(79.0809631347656, 21.135184856708992);
        let tile = WMTileID::from_point(&point, 1);
        assert_eq!(tile, WMTileID::new(1, 1, 0));
    }

    #[test]
    fn tiles_from_vector_multipoint() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Feature = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/multipoint.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_feature(&feature, 12);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 1086, y: 1498, zoom: 12 },
            WMTileID { x: 1014, y: 1552, zoom: 12 },
            WMTileID { x: 1086, y: 1497, zoom: 12 },
            WMTileID { x: 1014, y: 1551, zoom: 12 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_line() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Feature = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/line.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_feature(&feature, 12);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 839, y: 1708, zoom: 12 },
            WMTileID { x: 839, y: 1707, zoom: 12 },
            WMTileID { x: 840, y: 1707, zoom: 12 },
            WMTileID { x: 840, y: 1706, zoom: 12 },
            WMTileID { x: 840, y: 1705, zoom: 12 },
            WMTileID { x: 841, y: 1705, zoom: 12 },
            WMTileID { x: 843, y: 1706, zoom: 12 },
            WMTileID { x: 843, y: 1707, zoom: 12 },
            WMTileID { x: 843, y: 1708, zoom: 12 },
            WMTileID { x: 421, y: 852, zoom: 11 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_multiline() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Feature = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/multiline.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_feature(&feature, 8);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 136, y: 85, zoom: 8 },
            WMTileID { x: 135, y: 85, zoom: 8 },
            WMTileID { x: 133, y: 86, zoom: 8 },
            WMTileID { x: 135, y: 88, zoom: 8 },
            WMTileID { x: 134, y: 88, zoom: 8 },
            WMTileID { x: 132, y: 90, zoom: 8 },
            WMTileID { x: 131, y: 90, zoom: 8 },
            WMTileID { x: 128, y: 87, zoom: 8 },
            WMTileID { x: 129, y: 87, zoom: 8 },
            WMTileID { x: 129, y: 86, zoom: 8 },
            WMTileID { x: 130, y: 86, zoom: 8 },
            WMTileID { x: 130, y: 87, zoom: 8 },
            WMTileID { x: 130, y: 88, zoom: 8 },
            WMTileID { x: 131, y: 88, zoom: 8 },
            WMTileID { x: 131, y: 87, zoom: 8 },
            WMTileID { x: 135, y: 89, zoom: 8 },
            WMTileID { x: 136, y: 89, zoom: 8 },
            WMTileID { x: 136, y: 90, zoom: 8 },
            WMTileID { x: 67, y: 43, zoom: 7 },
            WMTileID { x: 66, y: 44, zoom: 7 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_edgeline() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Feature = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/edgeline.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_feature(&feature, 14);
        tile.sort();
        let mut expected =
            vec![WMTileID { x: 4543, y: 6612, zoom: 14 }, WMTileID { x: 4544, y: 6612, zoom: 14 }];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_polygon() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Geometry = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/polygon.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_geometry(&feature, 15);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 16850, y: 14480, zoom: 15 },
            WMTileID { x: 16850, y: 14451, zoom: 15 },
            WMTileID { x: 16851, y: 14451, zoom: 15 },
            WMTileID { x: 16852, y: 14451, zoom: 15 },
            WMTileID { x: 16853, y: 14451, zoom: 15 },
            WMTileID { x: 16854, y: 14451, zoom: 15 },
            WMTileID { x: 16855, y: 14451, zoom: 15 },
            WMTileID { x: 16856, y: 14451, zoom: 15 },
            WMTileID { x: 16857, y: 14451, zoom: 15 },
            WMTileID { x: 16858, y: 14451, zoom: 15 },
            WMTileID { x: 16859, y: 14451, zoom: 15 },
            WMTileID { x: 16860, y: 14451, zoom: 15 },
            WMTileID { x: 16861, y: 14451, zoom: 15 },
            WMTileID { x: 16862, y: 14451, zoom: 15 },
            WMTileID { x: 16863, y: 14451, zoom: 15 },
            WMTileID { x: 16864, y: 14451, zoom: 15 },
            WMTileID { x: 16865, y: 14451, zoom: 15 },
            WMTileID { x: 16866, y: 14451, zoom: 15 },
            WMTileID { x: 16867, y: 14451, zoom: 15 },
            WMTileID { x: 16868, y: 14451, zoom: 15 },
            WMTileID { x: 16869, y: 14451, zoom: 15 },
            WMTileID { x: 16870, y: 14451, zoom: 15 },
            WMTileID { x: 16871, y: 14451, zoom: 15 },
            WMTileID { x: 16872, y: 14451, zoom: 15 },
            WMTileID { x: 16873, y: 14451, zoom: 15 },
            WMTileID { x: 16874, y: 14451, zoom: 15 },
            WMTileID { x: 16875, y: 14451, zoom: 15 },
            WMTileID { x: 16876, y: 14451, zoom: 15 },
            WMTileID { x: 16877, y: 14451, zoom: 15 },
            WMTileID { x: 16878, y: 14451, zoom: 15 },
            WMTileID { x: 16879, y: 14451, zoom: 15 },
            WMTileID { x: 16880, y: 14451, zoom: 15 },
            WMTileID { x: 16881, y: 14451, zoom: 15 },
            WMTileID { x: 16882, y: 14451, zoom: 15 },
            WMTileID { x: 16883, y: 14451, zoom: 15 },
            WMTileID { x: 16884, y: 14451, zoom: 15 },
            WMTileID { x: 16885, y: 14451, zoom: 15 },
            WMTileID { x: 16885, y: 14480, zoom: 15 },
            WMTileID { x: 16884, y: 14480, zoom: 15 },
            WMTileID { x: 16883, y: 14480, zoom: 15 },
            WMTileID { x: 16882, y: 14480, zoom: 15 },
            WMTileID { x: 16881, y: 14480, zoom: 15 },
            WMTileID { x: 16880, y: 14480, zoom: 15 },
            WMTileID { x: 16879, y: 14480, zoom: 15 },
            WMTileID { x: 16878, y: 14480, zoom: 15 },
            WMTileID { x: 16877, y: 14480, zoom: 15 },
            WMTileID { x: 16876, y: 14480, zoom: 15 },
            WMTileID { x: 16875, y: 14480, zoom: 15 },
            WMTileID { x: 16874, y: 14480, zoom: 15 },
            WMTileID { x: 16873, y: 14480, zoom: 15 },
            WMTileID { x: 16872, y: 14480, zoom: 15 },
            WMTileID { x: 16871, y: 14480, zoom: 15 },
            WMTileID { x: 16870, y: 14480, zoom: 15 },
            WMTileID { x: 16869, y: 14480, zoom: 15 },
            WMTileID { x: 16868, y: 14480, zoom: 15 },
            WMTileID { x: 16867, y: 14480, zoom: 15 },
            WMTileID { x: 16866, y: 14480, zoom: 15 },
            WMTileID { x: 16865, y: 14480, zoom: 15 },
            WMTileID { x: 16864, y: 14480, zoom: 15 },
            WMTileID { x: 16863, y: 14480, zoom: 15 },
            WMTileID { x: 16862, y: 14480, zoom: 15 },
            WMTileID { x: 16861, y: 14480, zoom: 15 },
            WMTileID { x: 16860, y: 14480, zoom: 15 },
            WMTileID { x: 16859, y: 14480, zoom: 15 },
            WMTileID { x: 16858, y: 14480, zoom: 15 },
            WMTileID { x: 16857, y: 14480, zoom: 15 },
            WMTileID { x: 16856, y: 14480, zoom: 15 },
            WMTileID { x: 16855, y: 14480, zoom: 15 },
            WMTileID { x: 16854, y: 14480, zoom: 15 },
            WMTileID { x: 16853, y: 14480, zoom: 15 },
            WMTileID { x: 16852, y: 14480, zoom: 15 },
            WMTileID { x: 16851, y: 14480, zoom: 15 },
            WMTileID { x: 8425, y: 7239, zoom: 14 },
            WMTileID { x: 8425, y: 7238, zoom: 14 },
            WMTileID { x: 8425, y: 7237, zoom: 14 },
            WMTileID { x: 8425, y: 7236, zoom: 14 },
            WMTileID { x: 8425, y: 7235, zoom: 14 },
            WMTileID { x: 8425, y: 7234, zoom: 14 },
            WMTileID { x: 8425, y: 7233, zoom: 14 },
            WMTileID { x: 8425, y: 7232, zoom: 14 },
            WMTileID { x: 8425, y: 7231, zoom: 14 },
            WMTileID { x: 8425, y: 7230, zoom: 14 },
            WMTileID { x: 8425, y: 7229, zoom: 14 },
            WMTileID { x: 8425, y: 7228, zoom: 14 },
            WMTileID { x: 8425, y: 7227, zoom: 14 },
            WMTileID { x: 8425, y: 7226, zoom: 14 },
            WMTileID { x: 8442, y: 7226, zoom: 14 },
            WMTileID { x: 8442, y: 7227, zoom: 14 },
            WMTileID { x: 8442, y: 7228, zoom: 14 },
            WMTileID { x: 8442, y: 7229, zoom: 14 },
            WMTileID { x: 8442, y: 7230, zoom: 14 },
            WMTileID { x: 8442, y: 7231, zoom: 14 },
            WMTileID { x: 8442, y: 7232, zoom: 14 },
            WMTileID { x: 8442, y: 7233, zoom: 14 },
            WMTileID { x: 8442, y: 7234, zoom: 14 },
            WMTileID { x: 8442, y: 7235, zoom: 14 },
            WMTileID { x: 8442, y: 7236, zoom: 14 },
            WMTileID { x: 8442, y: 7237, zoom: 14 },
            WMTileID { x: 8442, y: 7238, zoom: 14 },
            WMTileID { x: 8442, y: 7239, zoom: 14 },
            WMTileID { x: 4213, y: 3613, zoom: 13 },
            WMTileID { x: 4214, y: 3613, zoom: 13 },
            WMTileID { x: 4215, y: 3613, zoom: 13 },
            WMTileID { x: 4216, y: 3613, zoom: 13 },
            WMTileID { x: 4217, y: 3613, zoom: 13 },
            WMTileID { x: 4218, y: 3613, zoom: 13 },
            WMTileID { x: 4219, y: 3613, zoom: 13 },
            WMTileID { x: 4220, y: 3613, zoom: 13 },
            WMTileID { x: 4213, y: 3614, zoom: 13 },
            WMTileID { x: 4220, y: 3614, zoom: 13 },
            WMTileID { x: 4213, y: 3615, zoom: 13 },
            WMTileID { x: 4220, y: 3615, zoom: 13 },
            WMTileID { x: 4213, y: 3616, zoom: 13 },
            WMTileID { x: 4220, y: 3616, zoom: 13 },
            WMTileID { x: 4213, y: 3617, zoom: 13 },
            WMTileID { x: 4220, y: 3617, zoom: 13 },
            WMTileID { x: 4213, y: 3618, zoom: 13 },
            WMTileID { x: 4220, y: 3618, zoom: 13 },
            WMTileID { x: 4213, y: 3619, zoom: 13 },
            WMTileID { x: 4220, y: 3619, zoom: 13 },
            WMTileID { x: 2107, y: 1807, zoom: 12 },
            WMTileID { x: 2108, y: 1807, zoom: 12 },
            WMTileID { x: 2109, y: 1807, zoom: 12 },
            WMTileID { x: 2107, y: 1808, zoom: 12 },
            WMTileID { x: 2107, y: 1809, zoom: 12 },
            WMTileID { x: 1054, y: 904, zoom: 11 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_uk() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Feature = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/uk.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_feature(&feature, 9);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 247, y: 164, zoom: 9 },
            WMTileID { x: 246, y: 164, zoom: 9 },
            WMTileID { x: 245, y: 164, zoom: 9 },
            WMTileID { x: 245, y: 163, zoom: 9 },
            WMTileID { x: 245, y: 162, zoom: 9 },
            WMTileID { x: 245, y: 161, zoom: 9 },
            WMTileID { x: 246, y: 161, zoom: 9 },
            WMTileID { x: 250, y: 154, zoom: 9 },
            WMTileID { x: 250, y: 155, zoom: 9 },
            WMTileID { x: 251, y: 155, zoom: 9 },
            WMTileID { x: 252, y: 155, zoom: 9 },
            WMTileID { x: 253, y: 155, zoom: 9 },
            WMTileID { x: 253, y: 156, zoom: 9 },
            WMTileID { x: 252, y: 156, zoom: 9 },
            WMTileID { x: 252, y: 157, zoom: 9 },
            WMTileID { x: 252, y: 158, zoom: 9 },
            WMTileID { x: 252, y: 159, zoom: 9 },
            WMTileID { x: 253, y: 159, zoom: 9 },
            WMTileID { x: 254, y: 161, zoom: 9 },
            WMTileID { x: 254, y: 162, zoom: 9 },
            WMTileID { x: 254, y: 163, zoom: 9 },
            WMTileID { x: 255, y: 163, zoom: 9 },
            WMTileID { x: 256, y: 165, zoom: 9 },
            WMTileID { x: 256, y: 166, zoom: 9 },
            WMTileID { x: 256, y: 167, zoom: 9 },
            WMTileID { x: 257, y: 167, zoom: 9 },
            WMTileID { x: 258, y: 167, zoom: 9 },
            WMTileID { x: 258, y: 168, zoom: 9 },
            WMTileID { x: 258, y: 169, zoom: 9 },
            WMTileID { x: 258, y: 170, zoom: 9 },
            WMTileID { x: 254, y: 172, zoom: 9 },
            WMTileID { x: 253, y: 172, zoom: 9 },
            WMTileID { x: 252, y: 172, zoom: 9 },
            WMTileID { x: 251, y: 172, zoom: 9 },
            WMTileID { x: 250, y: 172, zoom: 9 },
            WMTileID { x: 250, y: 173, zoom: 9 },
            WMTileID { x: 247, y: 173, zoom: 9 },
            WMTileID { x: 249, y: 167, zoom: 9 },
            WMTileID { x: 249, y: 166, zoom: 9 },
            WMTileID { x: 249, y: 165, zoom: 9 },
            WMTileID { x: 250, y: 165, zoom: 9 },
            WMTileID { x: 251, y: 165, zoom: 9 },
            WMTileID { x: 251, y: 164, zoom: 9 },
            WMTileID { x: 249, y: 162, zoom: 9 },
            WMTileID { x: 248, y: 162, zoom: 9 },
            WMTileID { x: 247, y: 159, zoom: 9 },
            WMTileID { x: 247, y: 158, zoom: 9 },
            WMTileID { x: 247, y: 157, zoom: 9 },
            WMTileID { x: 247, y: 156, zoom: 9 },
            WMTileID { x: 247, y: 155, zoom: 9 },
            WMTileID { x: 247, y: 154, zoom: 9 },
            WMTileID { x: 123, y: 81, zoom: 8 },
            WMTileID { x: 125, y: 76, zoom: 8 },
            WMTileID { x: 126, y: 80, zoom: 8 },
            WMTileID { x: 128, y: 84, zoom: 8 },
            WMTileID { x: 128, y: 85, zoom: 8 },
            WMTileID { x: 124, y: 86, zoom: 8 },
            WMTileID { x: 125, y: 81, zoom: 8 },
            WMTileID { x: 124, y: 80, zoom: 8 },
            WMTileID { x: 124, y: 77, zoom: 8 },
            WMTileID { x: 124, y: 76, zoom: 8 },
            WMTileID { x: 125, y: 80, zoom: 8 },
            WMTileID { x: 126, y: 81, zoom: 8 },
            WMTileID { x: 125, y: 83, zoom: 8 },
            WMTileID { x: 62, y: 39, zoom: 7 },
            WMTileID { x: 63, y: 41, zoom: 7 },
            WMTileID { x: 63, y: 42, zoom: 7 },
            WMTileID { x: 62, y: 42, zoom: 7 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }

    #[test]
    fn tiles_from_vector_blocky() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let feature: Geometry = serde_json::from_str(
            fs::read_to_string(format!(
                "{}{}",
                path.to_str().unwrap(),
                "/tests/geometry/tile/fixtures/blocky.geojson"
            ))
            .unwrap()
            .as_str(),
        )
        .unwrap();

        let mut tile = WMTileID::from_geometry(&feature, 6);
        tile.sort();
        let mut expected = vec![
            WMTileID { x: 10, y: 26, zoom: 6 },
            WMTileID { x: 11, y: 26, zoom: 6 },
            WMTileID { x: 11, y: 25, zoom: 6 },
            WMTileID { x: 12, y: 25, zoom: 6 },
            WMTileID { x: 13, y: 25, zoom: 6 },
            WMTileID { x: 14, y: 25, zoom: 6 },
            WMTileID { x: 15, y: 25, zoom: 6 },
            WMTileID { x: 16, y: 25, zoom: 6 },
            WMTileID { x: 16, y: 26, zoom: 6 },
            WMTileID { x: 16, y: 27, zoom: 6 },
            WMTileID { x: 16, y: 28, zoom: 6 },
            WMTileID { x: 16, y: 29, zoom: 6 },
            WMTileID { x: 11, y: 29, zoom: 6 },
            WMTileID { x: 11, y: 28, zoom: 6 },
            WMTileID { x: 11, y: 27, zoom: 6 },
            WMTileID { x: 7, y: 14, zoom: 5 },
            WMTileID { x: 6, y: 14, zoom: 5 },
            WMTileID { x: 6, y: 13, zoom: 5 },
            WMTileID { x: 7, y: 13, zoom: 5 },
        ];
        expected.sort();
        assert_eq!(tile, expected);
    }
}
