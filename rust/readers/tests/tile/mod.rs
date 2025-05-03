#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use parsers::FeatureReader;
    use readers::{
        RasterTileFetcher, S2TileMetadata, TileFetcher, TileMetadata, TileReader, WMTileMetadata,
        convert_mapbox_elevation_data, convert_terrarium_elevation_data,
    };
    use s2_tilejson::{Metadata, Scheme};
    use s2json::{BBox, VectorGeometry, VectorPoint};
    use std::{collections::BTreeMap, path::PathBuf};
    use util::RGBA;

    #[test]
    fn test_convert_terrarium_elevation_data() {
        assert_eq!(convert_terrarium_elevation_data(0, 0, 0), -32768.0);
        assert_eq!(convert_terrarium_elevation_data(255, 255, 255), 32767.99609375);
        assert_eq!(convert_terrarium_elevation_data(0, 0, 255), -32767.00390625);
        assert_eq!(convert_terrarium_elevation_data(255, 0, 0), 32512.0);
        assert_eq!(convert_terrarium_elevation_data(0, 255, 0), -32513.0);
    }

    #[test]
    fn test_convert_mapbox_elevation_data() {
        assert_eq!(convert_mapbox_elevation_data(0, 0, 0), -10000.0);
        assert_eq!(convert_mapbox_elevation_data(255, 255, 255), 1667721.5);
        assert_eq!(convert_mapbox_elevation_data(0, 0, 255), -9974.5);
        assert_eq!(convert_mapbox_elevation_data(255, 0, 0), 1661168.0);
        assert_eq!(convert_mapbox_elevation_data(0, 255, 0), -3472.0);
    }

    #[test]
    fn default_tile_meta() {
        let meta: TileMetadata = TileMetadata::default();
        assert_eq!(meta, TileMetadata::WM(WMTileMetadata::default()));
    }

    #[test]
    fn read_in_wm_satellite() {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, Some(1));

        let metadata = reader.get_metadata();
        assert_eq!(
            metadata,
            &Metadata {
                name: "Mapbox Satellite".into(),
                scheme: Scheme::Xyz,
                minzoom: 0,
                maxzoom: 3,
                r#type: "raster".into(),
                extension: "webp".into(),
                faces: vec![],
                bounds: BBox::new(-180.0, -85.0, 180.0, 85.0),
                ..Default::default()
            }
        );

        assert!(reader.has_tile_wm(0, 0, 0));

        let data = reader.get_tile_wm(0, 0, 0);
        assert_eq!(data.image.width(), 512);
        assert_eq!(data.image.height(), 512);
        assert_eq!(data.image.len(), 512 * 512 * 4);
        assert_eq!(data.metadata, TileMetadata::WM(WMTileMetadata { zoom: 0, x: 0, y: 0 }));
        assert!(!data.tms_style);

        let feature = data.build_feature();
        assert_eq!(
            feature.metadata.unwrap(),
            TileMetadata::WM(WMTileMetadata { zoom: 0, x: 0, y: 0 })
        );
        if let VectorGeometry::MultiPoint(geo) = feature.geometry {
            let coords = geo.coordinates;
            assert_eq!(coords.len(), 512 * 512);
            let first_five: Vec<VectorPoint<RGBA>> = coords[0..5].to_vec();
            assert_eq!(
                first_five,
                vec![
                    VectorPoint::new_xy(
                        -179.6484375,
                        85.02070774312593,
                        Some(RGBA::from_u8s(8, 14, 28, 255))
                    ),
                    VectorPoint::new_xy(
                        -178.94531250000003,
                        85.02070774312593,
                        Some(RGBA::from_u8s(8, 14, 28, 255))
                    ),
                    VectorPoint::new_xy(
                        -178.24218750000003,
                        85.02070774312593,
                        Some(RGBA::from_u8s(8, 14, 28, 255))
                    ),
                    VectorPoint::new_xy(
                        -177.53906250000003,
                        85.02070774312593,
                        Some(RGBA::from_u8s(8, 14, 28, 255))
                    ),
                    VectorPoint::new_xy(
                        -176.8359375,
                        85.02070774312593,
                        Some(RGBA::from_u8s(8, 14, 28, 255))
                    ),
                ]
            );
        } else {
            panic!("Invalid geometry type: {:?}", feature.geometry);
        }

        let tiles: Vec<_> = reader.iter().collect();
        assert_eq!(tiles.len(), 4);
    }

    #[test]
    fn read_in_s2_modis_mini() {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/readers/tile/fixtures/s2/modis-mini");
        let reader = RasterTileFetcher::<RGBA>::new(path, Some(1));

        let metadata = reader.get_metadata();
        assert_eq!(
            metadata,
            &Metadata {
                name: "Modis Raster Dataset".into(),
                description: "NASA Modis Dataset Reprojected by S2 MAPS INC.".into(),
                // tilestats: TileStatsMetadata { total: 8190, total_0: 1365, total_1: 1365, total_2: 1365, total_3: 1365, total_4: 1365, total_5: 1365 }
                attributions: BTreeMap::from([(
                    "MODIS".into(),
                    "https://modis.gsfc.nasa.gov".into()
                )]),
                tilestats: s2_tilejson::TileStatsMetadata {
                    total: 8190,
                    total_0: 1365,
                    total_1: 1365,
                    total_2: 1365,
                    total_3: 1365,
                    total_4: 1365,
                    total_5: 1365
                },
                faces: vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into(), 5.into()],
                scheme: Scheme::Fzxy,
                minzoom: 0,
                maxzoom: 1,
                r#type: "raster".into(),
                extension: "webp".into(),
                bounds: BBox::new(-180.0, -90.0, 180.0, 90.0),
                ..Default::default()
            }
        );

        assert!(reader.has_tile_s2(0.into(), 0, 0, 0));

        let data = reader.get_tile_s2(0.into(), 0, 0, 0);
        assert_eq!(data.image.width(), 512);
        assert_eq!(data.image.height(), 512);
        assert_eq!(data.image.len(), 512 * 512 * 4);
        assert_eq!(
            data.metadata,
            TileMetadata::S2(S2TileMetadata { face: 0.into(), zoom: 0, x: 0, y: 0 })
        );
        assert!(!data.tms_style);

        let feature = data.build_feature();
        assert_eq!(
            feature.metadata.unwrap(),
            TileMetadata::S2(S2TileMetadata { face: 0.into(), zoom: 0, x: 0, y: 0 })
        );
        if let VectorGeometry::MultiPoint(geo) = feature.geometry {
            let coords = geo.coordinates;
            assert_eq!(coords.len(), 512 * 512);
            let first_five: Vec<VectorPoint<RGBA>> = coords[0..5].to_vec();
            assert_eq!(
                first_five,
                vec![
                    VectorPoint::new_xy(
                        0.0009765625,
                        0.0009765625,
                        Some(RGBA {
                            r: 0.13273681646785965,
                            g: 0.16742940664479233,
                            b: 0.314409290505088,
                            a: 1.0
                        })
                    ),
                    VectorPoint::new_xy(
                        0.0029296875,
                        0.0009765625,
                        Some(RGBA {
                            r: 0.13273681646785965,
                            g: 0.16742940664479233,
                            b: 0.314409290505088,
                            a: 1.0
                        })
                    ),
                    VectorPoint::new_xy(
                        0.0048828125,
                        0.0009765625,
                        Some(RGBA {
                            r: 0.13273681646785965,
                            g: 0.16742940664479233,
                            b: 0.314409290505088,
                            a: 1.0
                        })
                    ),
                    VectorPoint::new_xy(
                        0.0068359375,
                        0.0009765625,
                        Some(RGBA {
                            r: 0.13273681646785965,
                            g: 0.16742940664479233,
                            b: 0.314409290505088,
                            a: 1.0
                        })
                    ),
                    VectorPoint::new_xy(
                        0.0087890625,
                        0.0009765625,
                        Some(RGBA {
                            r: 0.13273681646785965,
                            g: 0.16742940664479233,
                            b: 0.314409290505088,
                            a: 1.0
                        })
                    ),
                ]
            );
        } else {
            panic!("Invalid geometry type: {:?}", feature.geometry);
        }

        // TODO: If we can optimize image crate speed, we can bring this back
        // let tiles: Vec<_> = reader.iter().collect();
        // assert_eq!(tiles.len(), 24);
    }
}
