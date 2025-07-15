#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        data_structures::TileStoreOptions,
        parsers::FileReader,
        readers::json::JSONReader,
        writers::{
            BaseLayer, BuildGuide, LayerGuide, LocalTileWriter, TileBuilder, VectorLayerGuide,
        },
    };
    use open_vector_tile::Extent;
    use s2_tilejson::{
        Center, DrawType, Encoding, FaceBounds, LayerMetaData, Metadata, Scheme, SourceType,
        TileStatsMetadata, VectorLayer,
    };
    use s2json::{BBox, MValue, MValueCompatible, PrimitiveShape, Shape, ShapeType};
    use serde::{Deserialize, Serialize};
    use std::{collections::BTreeMap, path::PathBuf, vec};

    #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    #[serde(default)]
    struct Props {
        name: String,
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
        let mut tile_builder = TileBuilder::new(local_tile_writer, build_guide);

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path = path.join("tests/readers/json/fixtures/multipoint.geojson");

        let reader: JSONReader<FileReader, (), Props, Props> =
            JSONReader::new(FileReader::from(path), None);

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
                    face5: BTreeMap::default()
                },
                minzoom: 0,
                maxzoom: 4,
                centerpoint: Center { lon: 0.0, lat: 0.0, zoom: 2 },
                attributions: BTreeMap::default(),
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
                    total_5: 0
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
                template: None
            }
        );
    }
}
