#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        data_structures::TileStoreOptions,
        parsers::{BufferWriter, FileReader},
        readers::json::JSONReader,
        util::CompressionFormat,
        writers::{
            BaseLayer, BuildGuide, ClusterLayerGuide, FormatOutput, GridLayerGuide, JSONBuildGuide,
            LayerGuide, LayerHandler, LocalTileWriter, OnFeature, PMTilesWriter, RasterLayerGuide,
            TileBuilder, VectorLayerGuide, WhichTileWriting,
        },
    };
    use open_vector_tile::Extent;
    use s2_tilejson::{
        Center, DrawType, Encoding, FaceBounds, LayerMetaData, Metadata, Scheme, SourceType,
        TileStatsMetadata, VectorLayer,
    };
    use s2json::{
        Attributions, BBox, MValue, MValueCompatible, PrimitiveShape, Projection, Properties,
        Shape, ShapeType,
    };
    use serde::{Deserialize, Serialize};
    use std::{collections::BTreeMap, path::PathBuf, vec};

    #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    struct Props {
        name: String,
    }

    #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    struct PointProps {
        id: i64,
    }

    #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    struct LineProps {
        linename: String,
    }

    #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    struct PolyProps {
        poly3d: bool,
    }

    #[test]
    fn test_setup_builder() {
        let local_tile_writer = LocalTileWriter::new();
        let mut build_guide = BuildGuide::default();
        let tmp_mvalue: MValue = Props::default().into();
        build_guide.layer_guides.push(LayerGuide::Vector(VectorLayerGuide {
            extent: Extent::Extent4096,
            draw_types: vec![DrawType::Points],
            shape: Some((&tmp_mvalue).into()),
            base: BaseLayer {
                description: Some("Test Vector Layer".into()),
                source_name: "test".into(),
                layer_name: "points".into(),
            },
            vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
            ..Default::default()
        }));
        build_guide.attributions = Attributions::from([("a".into(), "b".into())]);
        let mut tile_builder = TileBuilder::new(local_tile_writer, build_guide);

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/readers/json/fixtures/multipoint.geojson");

        let reader: JSONReader<FileReader, (), Props, Props> =
            JSONReader::new(FileReader::from(path));

        tile_builder.add_vector_source("test".into(), reader, None, None);
        tile_builder.build_tiles();

        let meta = tile_builder.tile_writer().metadata().unwrap();
        assert_eq!(
            meta,
            Metadata {
                s2tilejson: "1.0.0".into(),
                version: "1.0.0".into(),
                name: "auto generated".into(),
                scheme: Scheme::Fzxy,
                description: "generated via OpenS2 gis-tools".into(),
                r#type: SourceType::Vector,
                extension: "pbf".into(),
                encoding: Encoding::None,
                faces: vec![3.into()],
                bounds: BBox {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308
                },
                wmbounds: BTreeMap::default(),
                s2bounds: FaceBounds {
                    face0: BTreeMap::default(),
                    face1: BTreeMap::default(),
                    face2: BTreeMap::default(),
                    face3: BTreeMap::from([
                        (0, BBox::<u64> { left: 0, bottom: 0, right: 0, top: 0 }),
                        (1, BBox::<u64> { left: 1, bottom: 0, right: 1, top: 0 }),
                        (2, BBox::<u64> { left: 3, bottom: 0, right: 3, top: 0 }),
                        (3, BBox::<u64> { left: 7, bottom: 0, right: 7, top: 1 }),
                        (4, BBox::<u64> { left: 14, bottom: 1, right: 15, top: 2 })
                    ]),
                    face4: BTreeMap::default(),
                    face5: BTreeMap::default(),
                    wm: BTreeMap::default(),
                },
                minzoom: 0,
                maxzoom: 4,
                centerpoint: Center { lon: 0.0, lat: 0.0, zoom: 2 },
                attributions: Attributions::from([("a".into(), "b".into())]),
                layers: BTreeMap::from([(
                    "points".into(),
                    LayerMetaData {
                        description: Some("Test Vector Layer".into()),
                        minzoom: 0,
                        maxzoom: 4,
                        draw_types: vec![DrawType::Points],
                        shape: Shape::from([(
                            "name".into(),
                            ShapeType::Primitive(PrimitiveShape::String)
                        )]),
                        m_shape: None
                    }
                )]),
                tilestats: TileStatsMetadata {
                    total: 7,
                    total_0: 0,
                    total_1: 0,
                    total_2: 0,
                    total_3: 7,
                    total_4: 0,
                    total_5: 0,
                    total_wm: 0,
                },
                vector_layers: vec![VectorLayer {
                    id: "points".into(),
                    description: Some("Test Vector Layer".into()),
                    minzoom: Some(0),
                    maxzoom: Some(4),
                    fields: BTreeMap::default()
                }],
                tilejson: None,
                tiles: None,
                attribution: None,
                fillzoom: None,
                center: None,
                data: None,
                grids: None,
                legend: None,
                template: None,
                interval: None,
            }
        );
    }

    #[test]
    fn test_jsonbuild_guide() {
        let default = JSONBuildGuide::default();
        assert_eq!(
            default,
            JSONBuildGuide {
                name: "auto generated".into(),
                description: "generated via OpenS2 gis-tools".into(),
                version: "1.0.0".into(),
                extension: "pbf".into(),
                projection: Projection::S2,
                encoding: Encoding::None,
                attribution: Attributions::default(),
                format: FormatOutput::default(),
                vector_sources: vec![],
                raster_sources: vec![],
                grid_sources: vec![],
                build_indices: true,
                layer_guides: vec![],
                tile_writer: WhichTileWriting::Local,
                threads: 1,
            }
        );
    }

    #[test]
    fn test_format_output() {
        assert_eq!(FormatOutput::from("mapbox"), FormatOutput::Mapbox);
        assert_eq!(FormatOutput::from("flat-open-s2"), FormatOutput::FlatOpenS2);
        assert_eq!(FormatOutput::from("open-s2"), FormatOutput::OpenS2);
        assert_eq!(FormatOutput::from("raster"), FormatOutput::Raster);
        assert_eq!(FormatOutput::from("unknown"), FormatOutput::OpenS2);

        let source_type: SourceType = (&FormatOutput::from("mapbox")).into();
        assert_eq!(source_type, SourceType::Vector);

        let source_type: SourceType = (&FormatOutput::from("raster")).into();
        assert_eq!(source_type, SourceType::Raster);
    }

    #[test]
    fn wm_build_with_cluster() {
        let tmp_buffer_writer = BufferWriter::new(vec![]);
        let pm_writer = PMTilesWriter::new(tmp_buffer_writer, CompressionFormat::None);
        let build_guide = BuildGuide {
            projection: Projection::WG,
            build_indices: true,
            attributions: Attributions::from([(
                "Satellite Data".into(),
                "https://example.com".into(),
            )]),
            layer_guides: vec![
                // add points
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Points, DrawType::Points3D],
                    shape: Some((&MValue::from(PointProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Points Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "points".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
                // add lines
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Lines, DrawType::Lines3D],
                    shape: Some((&MValue::from(LineProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Lines Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "lines".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
                // add polys
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Polygons, DrawType::Polygons3D],
                    shape: Some((&MValue::from(PolyProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Polygons Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "polys".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        let mut tile_builder = TileBuilder::new(pm_writer, build_guide);

        // add json features
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/writers/fixtures/all-features.json");
        let reader: JSONReader<FileReader, (), Properties, Properties> =
            JSONReader::new(FileReader::from(path));
        tile_builder.add_vector_source("all_features".into(), reader, None, None);

        // build
        tile_builder.build_tiles();

        // let meta = tile_builder.tile_writer().metadata().unwrap();
        // assert_eq!(
        //     meta,
        //     Metadata { s2tilejson: "1.0.0".into(), version: "1.0.0".into(), name: "auto generated", scheme: Scheme::Xyz, description: "generated via OpenS2 gis-tools", r#type: Vector, extension: "pbf", encoding: None, faces: vec![Face::Face0], bounds: BBox { left: -180.0, bottom: -85.05112877980659, right: 180.0, top: 85.0511287798066 }, wmbounds: {0: BBox { left: 0, bottom: 0, right: 0, top: 0 }, 1: BBox { left: 0, bottom: 0, right: 1, top: 1 }, 2: BBox { left: 0, bottom: 0, right: 3, top: 3 }, 3: BBox { left: 0, bottom: 1, right: 7, top: 6 }, 4: BBox { left: 0, bottom: 3, right: 15, top: 12 }}, s2bounds: FaceBounds { face0: {}, face1: {}, face2: {}, face3: {}, face4: {}, face5: {} }, minzoom: 0, maxzoom: 4, centerpoint: Center { lon: 0.0, lat: 7.105427357601002e-15, zoom: 2 }, attributions: {"a": "b"}, layers: {"lines": LayerMetaData { description: Some("Lines Vector Layer"), minzoom: 0, maxzoom: 4, draw_types: [Points], shape: Map { map: {"linename": Primitive(String)} }, m_shape: None }, "points": LayerMetaData { description: Some("Points Vector Layer"), minzoom: 0, maxzoom: 4, draw_types: [Points], shape: Map { map: {"id": Primitive(I64)} }, m_shape: None }}, tilestats: TileStatsMetadata { total: 92, total_0: 0, total_1: 0, total_2: 0, total_3: 0, total_4: 0, total_5: 0 }, vector_layers: [VectorLayer { id: "points", description: Some("Points Vector Layer"), minzoom: Some(0), maxzoom: Some(4), fields: {} }, VectorLayer { id: "lines", description: Some("Lines Vector Layer"), minzoom: Some(0), maxzoom: Some(4), fields: {} }], tilejson: None, tiles: None, attribution: None, fillzoom: None, center: None, data: None, grids: None, legend: None, template: None }
        // );
    }

    #[test]
    fn wm_mapbox_build_with_cluster() {
        let local_tile_writer = LocalTileWriter::new();
        let build_guide = BuildGuide {
            projection: Projection::WG,
            build_indices: true,
            format: FormatOutput::Mapbox,
            attributions: Attributions::from([(
                "Satellite Data".into(),
                "https://example.com".into(),
            )]),
            layer_guides: vec![
                // add points
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Points, DrawType::Points3D],
                    shape: Some((&MValue::from(PointProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Points Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "points".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
                // add lines
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Lines, DrawType::Lines3D],
                    shape: Some((&MValue::from(LineProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Lines Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "lines".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
                // add polys
                LayerGuide::Vector(VectorLayerGuide {
                    extent: Extent::Extent4096,
                    draw_types: vec![DrawType::Polygons, DrawType::Polygons3D],
                    shape: Some((&MValue::from(PolyProps::default())).into()),
                    base: BaseLayer {
                        description: Some("Polygons Vector Layer".into()),
                        source_name: "all_features".into(),
                        layer_name: "polys".into(),
                    },
                    vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        };
        let mut tile_builder = TileBuilder::new(local_tile_writer, build_guide);

        // add json features
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/writers/fixtures/no_3d.json");
        let reader: JSONReader<FileReader, (), Properties, Properties> =
            JSONReader::new(FileReader::from(path));

        let on_source: OnFeature<_, _, _> = |feature| Some(feature);
        let on_layers: Vec<LayerHandler<_, _, _>> =
            vec![LayerHandler { layer_name: "points".into(), on_feature: |feature| Some(feature) }];
        tile_builder.add_vector_source(
            "all_features".into(),
            reader,
            Some(on_source),
            Some(&on_layers),
        );

        // build
        tile_builder.build_tiles();

        // let meta = tile_builder.tile_writer().metadata().unwrap();
        // assert_eq!(
        //     meta,
        //     Metadata { s2tilejson: "1.0.0".into(), version: "1.0.0".into(), name: "auto generated", scheme: Scheme::Xyz, description: "generated via OpenS2 gis-tools", r#type: Vector, extension: "pbf", encoding: None, faces: vec![Face::Face0], bounds: BBox { left: -180.0, bottom: -85.05112877980659, right: 180.0, top: 85.0511287798066 }, wmbounds: {0: BBox { left: 0, bottom: 0, right: 0, top: 0 }, 1: BBox { left: 0, bottom: 0, right: 1, top: 1 }, 2: BBox { left: 0, bottom: 0, right: 3, top: 3 }, 3: BBox { left: 0, bottom: 1, right: 7, top: 6 }, 4: BBox { left: 0, bottom: 3, right: 15, top: 12 }}, s2bounds: FaceBounds { face0: {}, face1: {}, face2: {}, face3: {}, face4: {}, face5: {} }, minzoom: 0, maxzoom: 4, centerpoint: Center { lon: 0.0, lat: 7.105427357601002e-15, zoom: 2 }, attributions: {"a": "b"}, layers: {"lines": LayerMetaData { description: Some("Lines Vector Layer"), minzoom: 0, maxzoom: 4, draw_types: [Points], shape: Map { map: {"linename": Primitive(String)} }, m_shape: None }, "points": LayerMetaData { description: Some("Points Vector Layer"), minzoom: 0, maxzoom: 4, draw_types: [Points], shape: Map { map: {"id": Primitive(I64)} }, m_shape: None }}, tilestats: TileStatsMetadata { total: 92, total_0: 0, total_1: 0, total_2: 0, total_3: 0, total_4: 0, total_5: 0 }, vector_layers: [VectorLayer { id: "points", description: Some("Points Vector Layer"), minzoom: Some(0), maxzoom: Some(4), fields: {} }, VectorLayer { id: "lines", description: Some("Lines Vector Layer"), minzoom: Some(0), maxzoom: Some(4), fields: {} }], tilejson: None, tiles: None, attribution: None, fillzoom: None, center: None, data: None, grids: None, legend: None, template: None }
        // );
    }

    #[test]
    fn test_raster_layer_guide() {
        let raster_guide = RasterLayerGuide::default();
        let layer_metadata: LayerMetaData = (&raster_guide).into();
        assert_eq!(
            layer_metadata,
            LayerMetaData {
                description: None,
                minzoom: 0,
                maxzoom: 16,
                draw_types: vec![DrawType::Raster],
                shape: Shape::default(),
                m_shape: None
            }
        );

        let layer_guide = LayerGuide::Raster(raster_guide);
        assert_eq!(layer_guide.zooms(), (0, 16));
        assert_eq!(layer_guide.has_source("test"), false);
        assert_eq!(layer_guide.layer_name(), "");
        assert_eq!(layer_guide.to_vector(), None);

        let layer_meta: LayerMetaData = (&layer_guide).into();
        assert_eq!(layer_meta, layer_metadata);
    }

    #[test]
    fn test_grid_layer_guide() {
        let grid_guide = GridLayerGuide::default();
        let layer_metadata: LayerMetaData = (&grid_guide).into();
        assert_eq!(
            layer_metadata,
            LayerMetaData {
                description: None,
                minzoom: 0,
                maxzoom: 16,
                draw_types: vec![DrawType::Grid],
                shape: Shape::default(),
                m_shape: None
            }
        );

        let layer_guide = LayerGuide::Grid(grid_guide);
        assert_eq!(layer_guide.zooms(), (0, 16));
        assert_eq!(layer_guide.has_source("test"), false);
        assert_eq!(layer_guide.layer_name(), "");
        assert_eq!(layer_guide.to_vector(), None);

        let layer_meta: LayerMetaData = (&layer_guide).into();
        assert_eq!(layer_meta, layer_metadata);
    }

    #[test]
    fn test_cluster_layer_guide() {
        let cluster_guide = ClusterLayerGuide::default();
        let layer_metadata: LayerMetaData = (&cluster_guide).into();
        assert_eq!(
            layer_metadata,
            LayerMetaData {
                description: None,
                minzoom: 0,
                maxzoom: 16,
                draw_types: vec![DrawType::Points],
                shape: Shape::default(),
                m_shape: None
            }
        );

        let layer_guide = LayerGuide::Cluster(cluster_guide);
        assert_eq!(layer_guide.zooms(), (0, 16));
        assert_eq!(layer_guide.has_source("test"), false);
        assert_eq!(layer_guide.layer_name(), "");
        assert_eq!(layer_guide.to_vector(), None);

        let layer_meta: LayerMetaData = (&layer_guide).into();
        assert_eq!(layer_meta, layer_metadata);
    }

    #[test]
    fn test_layer_guide() {
        let default = LayerGuide::default();
        assert_eq!(default, LayerGuide::Vector(VectorLayerGuide::default()));
    }
}
