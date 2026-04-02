#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    extern crate alloc;

    use alloc::{vec, vec::Vec};
    use gistools::{
        geometry::{S2TileID, TileID, WMTileID},
        parsers::{FeatureReader, RGBA},
        readers::{RasterTileFetcher, TileFetcher, TileGridGuide, TileReader, build_tile_grid_wm},
        tools::{
            convert_mapbox_elevation_data, convert_terrarium_elevation_data,
            encode_mapbox_elevation_data, encode_terrarium_elevation_data,
        },
    };
    use s2_tilejson::{Metadata, Scheme};
    use s2json::{Attributions, BBox, VectorGeometry, VectorPoint};
    use std::path::PathBuf;

    #[test]
    fn test_convert_terrarium_elevation_data() {
        assert_eq!(convert_terrarium_elevation_data(0, 0, 0, None), -32768.0);
        assert_eq!(convert_terrarium_elevation_data(255, 255, 255, None), 32767.99609375);
        assert_eq!(convert_terrarium_elevation_data(0, 0, 255, None), -32767.00390625);
        assert_eq!(convert_terrarium_elevation_data(255, 0, 0, None), 32512.0);
        assert_eq!(convert_terrarium_elevation_data(0, 255, 0, None), -32513.0);
    }

    #[test]
    fn test_encode_terrarium_elevation_data() {
        assert_eq!(encode_terrarium_elevation_data(-32768.0), (0, 0, 0, None));
        assert_eq!(encode_terrarium_elevation_data(32767.99609375), (255, 255, 255, None));
        assert_eq!(encode_terrarium_elevation_data(-32767.00390625), (0, 0, 255, None));
        assert_eq!(encode_terrarium_elevation_data(32512.0), (255, 0, 0, None));
        assert_eq!(encode_terrarium_elevation_data(-32513.0), (0, 255, 0, None));
    }

    #[test]
    fn test_convert_mapbox_elevation_data() {
        assert_eq!(convert_mapbox_elevation_data(0, 0, 0, None), -10000.0);
        assert_eq!(convert_mapbox_elevation_data(255, 255, 255, None), 1667721.5);
        assert_eq!(convert_mapbox_elevation_data(0, 0, 255, None), -9974.5);
        assert_eq!(convert_mapbox_elevation_data(255, 0, 0, None), 1661168.0);
        assert_eq!(convert_mapbox_elevation_data(0, 255, 0, None), -3472.0);
    }

    #[test]
    fn test_encode_mapbox_elevation_data() {
        assert_eq!(encode_mapbox_elevation_data(-10000.0), (0, 0, 0, None));
        assert_eq!(encode_mapbox_elevation_data(1667721.5), (255, 255, 255, None));
        assert_eq!(encode_mapbox_elevation_data(-9974.5), (0, 0, 255, None));
        assert_eq!(encode_mapbox_elevation_data(1661168.0), (255, 0, 0, None));
        assert_eq!(encode_mapbox_elevation_data(-3472.0), (0, 255, 0, None));
    }

    #[test]
    fn default_tile_meta() {
        let meta: TileID = TileID::default();
        assert_eq!(meta, TileID::WM(WMTileID::default()));
    }

    #[test]
    fn read_in_wm_satellite() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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
        let data_image = data.image.as_ref().unwrap();
        assert_eq!(data_image.width(), 512);
        assert_eq!(data_image.height(), 512);
        assert_eq!(data_image.len(), 512 * 512 * 4);
        assert_eq!(data.metadata, TileID::WM(WMTileID { zoom: 0, x: 0, y: 0 }));
        assert!(!data.tms_style);

        let feature = data.build_feature();
        assert_eq!(feature.metadata.unwrap(), TileID::WM(WMTileID { zoom: 0, x: 0, y: 0 }));
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

        let lon_lat_value = reader.get_tile_value_wm(2, 20., -20., Some(512));
        assert_eq!(lon_lat_value, Some(RGBA::from_u8s(146, 123, 56, 255)));

        let tiles: Vec<_> = reader.iter().collect();
        assert_eq!(tiles.len(), 4);

        let tiles: Vec<_> = (0..3usize)
            .into_iter()
            .flat_map(|thread_id| {
                let read = reader.clone();
                let res: Vec<_> = read.par_iter(3, thread_id).collect();
                res
            })
            .collect();
        assert_eq!(tiles.len(), 4);
    }

    #[test]
    fn read_in_s2_modis_mini() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/s2/modis-mini");
        let reader = RasterTileFetcher::<RGBA>::new(path, Some(1));

        let metadata = reader.get_metadata();
        assert_eq!(
            metadata,
            &Metadata {
                name: "Modis Raster Dataset".into(),
                description: "NASA Modis Dataset Reprojected by S2 MAPS INC.".into(),
                // tilestats: TileStatsMetadata { total: 8190, total_0: 1365, total_1: 1365, total_2: 1365, total_3: 1365, total_4: 1365, total_5: 1365 }
                attributions: Attributions::from([(
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
                    total_5: 1365,
                    total_wm: 0,
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
        let data_image = data.image.as_ref().unwrap();
        assert_eq!(data_image.width(), 512);
        assert_eq!(data_image.height(), 512);
        assert_eq!(data_image.len(), 512 * 512 * 4);
        assert_eq!(data.metadata, TileID::S2(S2TileID { face: 0.into(), zoom: 0, x: 0, y: 0 }));
        assert!(!data.tms_style);

        let feature = data.build_feature();
        assert_eq!(
            feature.metadata.unwrap(),
            TileID::S2(S2TileID { face: 0.into(), zoom: 0, x: 0, y: 0 })
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

        let lon_lat_value = reader.get_tile_value_s2(2, 20., -20., Some(512));
        assert_eq!(lon_lat_value, Some(RGBA::from_u8s(93, 80, 46, 255)));

        // TODO: If we can optimize image crate speed, we can bring this back
        // let tiles: Vec<_> = reader.iter().collect();
        // assert_eq!(tiles.len(), 24);
    }

    #[test]
    fn build_tile_grid_wm_base_case() {
        let grid_guide =
            build_tile_grid_wm(&(WMTileID { zoom: 0, x: 0, y: 0 }).into(), 0, 512, 512, false);

        assert_eq!(
            grid_guide,
            vec![TileGridGuide {
                dest_offsets: (0, 0),
                tile: (WMTileID { zoom: 0, x: 0, y: 0 }).into(),
                src_offsets: (0, 0),
                write_size: (512, 512),
                image: None,
                clamp: None,
            }]
        );
    }

    #[test]
    fn build_tile_grid_wm_base_case_double_size() {
        let grid_guide =
            build_tile_grid_wm(&(WMTileID { zoom: 0, x: 0, y: 0 }).into(), 0, 256, 512, false);

        assert_eq!(
            grid_guide,
            vec![
                TileGridGuide {
                    dest_offsets: (0, 0),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 1 }).into(),
                    write_size: (256, 256),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    dest_offsets: (0, 256),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 1, zoom: 1 }).into(),
                    write_size: (256, 256),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    dest_offsets: (256, 0),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 1, y: 0, zoom: 1 }).into(),
                    write_size: (256, 256),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    dest_offsets: (256, 256),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 1, y: 1, zoom: 1 }).into(),
                    write_size: (256, 256),
                    image: None,
                    clamp: None,
                }
            ]
        );
    }

    #[test]
    fn build_tile_grid_wm_small_padding_zoom_0() {
        let grid_guide =
            build_tile_grid_wm(&(WMTileID { zoom: 0, x: 0, y: 0 }).into(), 2, 512, 512, false);

        assert_eq!(
            grid_guide,
            vec![
                TileGridGuide {
                    dest_offsets: (2, 2),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (512, 512),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (2, 0),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (512, 2),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (2, 514),
                    src_offsets: (0, 511),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (512, 2),
                    image: None,
                },
                TileGridGuide {
                    dest_offsets: (0, 2),
                    src_offsets: (510, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 512),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (0, 0),
                    src_offsets: (510, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 2),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (0, 514),
                    src_offsets: (510, 511),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 2),
                    image: None,
                },
                TileGridGuide {
                    dest_offsets: (514, 2),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 512),
                    image: None,
                    clamp: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (514, 0),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 2),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(true),
                    dest_offsets: (514, 514),
                    src_offsets: (0, 511),
                    tile: (WMTileID { x: 0, y: 0, zoom: 0 }).into(),
                    write_size: (2, 2),
                    image: None,
                },
            ],
        );
    }

    #[test]
    fn build_tile_grid_wm_higher_zoom_small_padding() {
        let grid_guide =
            build_tile_grid_wm(&(WMTileID { zoom: 3, x: 2, y: 2 }).into(), 4, 512, 512, false);

        assert_eq!(
            grid_guide,
            vec![
                TileGridGuide {
                    clamp: None,
                    dest_offsets: (4, 4),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 2, y: 2, zoom: 3 }).into(),
                    write_size: (512, 512),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (4, 0),
                    src_offsets: (0, 508),
                    tile: (WMTileID { x: 2, y: 1, zoom: 3 }).into(),
                    write_size: (512, 4),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (4, 516),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 2, y: 3, zoom: 3 }).into(),
                    write_size: (512, 4),
                    image: None,
                },
                TileGridGuide {
                    clamp: None,
                    dest_offsets: (0, 4),
                    src_offsets: (508, 0),
                    tile: (WMTileID { x: 1, y: 2, zoom: 3 }).into(),
                    write_size: (4, 512),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (0, 0),
                    src_offsets: (508, 508),
                    tile: (WMTileID { x: 1, y: 1, zoom: 3 }).into(),
                    write_size: (4, 4),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (0, 516),
                    src_offsets: (508, 0),
                    tile: (WMTileID { x: 1, y: 3, zoom: 3 }).into(),
                    write_size: (4, 4),
                    image: None,
                },
                TileGridGuide {
                    clamp: None,
                    dest_offsets: (516, 4),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 3, y: 2, zoom: 3 }).into(),
                    write_size: (4, 512),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (516, 0),
                    src_offsets: (0, 508),
                    tile: (WMTileID { x: 3, y: 1, zoom: 3 }).into(),
                    write_size: (4, 4),
                    image: None,
                },
                TileGridGuide {
                    clamp: Some(false),
                    dest_offsets: (516, 516),
                    src_offsets: (0, 0),
                    tile: (WMTileID { x: 3, y: 3, zoom: 3 }).into(),
                    write_size: (4, 4),
                    image: None,
                },
            ],
        );
    }

    #[test]
    fn get_tile_with_padding_wm_base_case() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, None);

        let tile = reader.get_tile_with_padding_wm(0, 0, 0, 1, Some(512), Some(512));
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert!(tile.image.is_some());
        let tile_image = tile.image.unwrap();
        assert_eq!(tile_image.width(), 514);
        assert_eq!(tile_image.height(), 514);
        assert_eq!(tile_image.len(), 514 * 514 * 4);
        assert_eq!(tile.metadata, TileID::WM(WMTileID { zoom: 0, x: 0, y: 0 }));

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite/baseCase.png");
        let expected_png_data = image::open(path);
        let expected = expected_png_data.unwrap().to_rgba8();
        assert_eq!(tile_image, expected);
    }

    #[test]
    fn get_tile_with_padding_wm_large_padding() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, None);

        let tile = reader.get_tile_with_padding_wm(0, 0, 0, 16, Some(512), Some(512));
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert!(tile.image.is_some());
        let tile_image = tile.image.unwrap();
        assert_eq!(tile_image.width(), 544);
        assert_eq!(tile_image.height(), 544);
        assert_eq!(tile_image.len(), 544 * 544 * 4);
        assert_eq!(tile.metadata, TileID::WM(WMTileID { zoom: 0, x: 0, y: 0 }));

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite/largerPadding.png");
        let expected_png_data = image::open(path);
        let expected = expected_png_data.unwrap().to_rgba8();
        assert_eq!(tile_image, expected);
    }

    #[test]
    fn get_tile_with_padding_wm_wrapping_lower_zoom() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, None);

        let tile = reader.get_tile_with_padding_wm(2, 0, 0, 16, Some(512), Some(512));
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert!(tile.image.is_some());
        let tile_image = tile.image.unwrap();
        assert_eq!(tile_image.width(), 544);
        assert_eq!(tile_image.height(), 544);
        assert_eq!(tile_image.len(), 544 * 544 * 4);
        assert_eq!(tile.metadata, TileID::WM(WMTileID { zoom: 2, x: 0, y: 0 }));

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite/wrappingZoom.png");
        let expected_png_data = image::open(path);
        let expected = expected_png_data.unwrap().to_rgba8();
        assert_eq!(tile_image, expected);
    }

    #[test]
    fn get_tile_with_padding_wm_wrapping_lower_zoom_other_end() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite");
        let reader = RasterTileFetcher::<RGBA>::new(path, None);

        let tile = reader.get_tile_with_padding_wm(2, 3, 3, 16, Some(512), Some(512));
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert!(tile.image.is_some());
        let tile_image = tile.image.unwrap();
        assert_eq!(tile_image.width(), 544);
        assert_eq!(tile_image.height(), 544);
        assert_eq!(tile_image.len(), 544 * 544 * 4);
        assert_eq!(tile.metadata, TileID::WM(WMTileID { zoom: 2, x: 3, y: 3 }));

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/satellite/wrappingZoom2.png");
        let expected_png_data = image::open(path);
        let expected = expected_png_data.unwrap().to_rgba8();
        assert_eq!(tile_image, expected);
    }

    #[test]
    fn get_tile_with_padding_wm_base_case_with_resizing() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/terrarium");
        let reader = RasterTileFetcher::<RGBA>::new(path, None);

        let tile = reader.get_tile_with_padding_wm(0, 0, 0, 1, Some(256), Some(512));
        assert!(tile.is_some());
        let tile = tile.unwrap();
        assert!(tile.image.is_some());
        let tile_image = tile.image.unwrap();
        assert_eq!(tile_image.width(), 514);
        assert_eq!(tile_image.height(), 514);
        assert_eq!(tile_image.len(), 514 * 514 * 4);
        assert_eq!(tile.metadata, TileID::WM(WMTileID { zoom: 0, x: 0, y: 0 }));

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/tile/fixtures/wm/terrarium/resize.png");
        let expected_png_data = image::open(path);
        let expected = expected_png_data.unwrap().to_rgba8();
        assert_eq!(tile_image, expected);
    }
}
