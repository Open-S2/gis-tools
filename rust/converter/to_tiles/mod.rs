/// Types, Structs, and Enums for the to_tiles converter
pub mod types;
/// Worker
pub mod worker;

use super::OnFeature;
use crate::{
    data_structures::HasLayer,
    geometry::{wm, xyz_to_bbox},
    readers::FeatureReader,
    writers::{FileTileWriter, LocalTileWriter, TileWriter},
};
use alloc::{string::String, vec::Vec};
use core::mem::take;
use s2_tilejson::{LonLatBounds, MetadataBuilder, Scheme};
use s2json::{MValueCompatible, Projection};
pub use types::*;
use worker::{BuiltTile, TileWorker};

/// Use a Local Tile Builder via memory
pub type LocalTileBuilder = TileBuilder<LocalTileWriter>;
/// Use the Filesystem to store all file data
pub type FileTileBuilder = TileBuilder<FileTileWriter>;

/// The TileBuilder creates tiles for the user given any source reader that implements FeatureReader
#[derive(Debug)]
pub struct TileBuilder<W: TileWriter = LocalTileWriter> {
    /// The data created will be stored in either a folder structure or a pmtiles file
    /// Folder structure is either '{face}/{zoom}/{x}/{y}.pbf' or '{zoom}/{x}/{y}.pbf'.
    /// PMTiles store all data in a single data file.
    tile_writer: W,
    /// Explains how to build data
    build_guide: BuildGuide,
    /// TileWorker collects data then builds tiles
    worker: TileWorker,
    /// The metadata that will be stored with the tile data
    meta_builder: MetadataBuilder,
}
impl<W: TileWriter> TileBuilder<W> {
    /// Create a new Tile builder
    pub fn new(tile_writer: W, build_guide: BuildGuide) -> Self {
        let meta_builder = setup_builder(&build_guide);
        let worker = TileWorker::new(0, build_guide.clone());
        Self { tile_writer, build_guide, worker, meta_builder }
    }

    /// Get the tile writer
    pub fn tile_writer(&self) -> &W {
        &self.tile_writer
    }

    /// Add a vector source to tile-ize
    pub fn add_vector_source<
        M: Clone + HasLayer,
        P: MValueCompatible,
        D: MValueCompatible,
        T: FeatureReader<M, P, D>,
    >(
        &mut self,
        source_name: String,
        reader: T,
        on_source_feature: Option<&OnFeature<M, P, D>>,
        on_layer_feature: Option<&Vec<LayerHandler<M, P, D>>>,
    ) {
        for mut feature in reader.iter() {
            if let Some(on_feature) = on_source_feature {
                if let Some(new_feature) = (on_feature)(take(&mut feature)) {
                    feature = new_feature;
                }
            }
            self.worker.store_feature(feature, source_name.clone(), on_layer_feature);
        }
    }

    /// Add vector points with RGBA attributes to build raster tiles
    pub fn add_raster_source<
        M: Clone,
        P: MValueCompatible,
        D: MValueCompatible,
        T: FeatureReader<M, P, D>,
    >(
        _source_name: String,
        _reader: T,
        _on_feature: Option<&OnFeature<M, P, D>>,
    ) {
        unimplemented!()
    }

    /// Add data that will be gridded like raster data but float precision points
    pub fn add_grid_source<
        M: Clone,
        P: MValueCompatible,
        D: MValueCompatible,
        T: FeatureReader<M, P, D>,
    >(
        _source_name: String,
        _reader: T,
        _on_feature: Option<&OnFeature<M, P, D>>,
    ) {
        unimplemented!()
    }

    /// After adding all the source data, build all tiles into the tile writer
    pub fn build_tiles(&mut self) {
        let BuildGuide { projection, .. } = &self.build_guide;
        // ensure all data is sorted
        self.worker.sort();
        // collect all tiles
        for BuiltTile { face, zoom, x, y, data } in self.worker.build_tiles() {
            if *projection == Projection::S2 {
                self.tile_writer.write_tile_s2(face, zoom, x, y, data);
                // TODO: Get correct ll-bounds
                self.meta_builder.add_tile_s2(face, zoom, x, y, &LonLatBounds::default());
            } else {
                self.tile_writer.write_tile_wm(zoom, x, y, data);
                let (w, s, e, n) =
                    xyz_to_bbox(x, y, zoom, Some(false), Some(wm::Source::WGS84), None);
                self.meta_builder.add_tile_wm(zoom, x, y, &LonLatBounds::new(w, s, e, n));
            }
        }
        // finally commit the metadata
        self.tile_writer
            .commit(self.meta_builder.commit(), Some((&self.build_guide.encoding).into()));
    }
}

/// Setup a metadata builder
fn setup_builder(build_guide: &BuildGuide) -> MetadataBuilder {
    let mut meta_builder = MetadataBuilder::default();
    let BuildGuide { name, description, version, projection, attributions, .. } = build_guide;

    meta_builder.set_name(name.into());
    meta_builder.set_extension(build_guide.extension.clone());
    meta_builder.set_description(description.into());
    meta_builder.set_version(version.into());
    // NOTE: For now we only support xyz and fzxy
    let scheme = if *projection == Projection::S2 { Scheme::Fzxy } else { Scheme::Xyz };
    meta_builder.set_scheme(scheme);
    meta_builder.set_type((&build_guide.format).into());

    meta_builder.set_encoding(build_guide.encoding.clone());
    // Add attribution
    for (display_name, href) in attributions.iter() {
        meta_builder.add_attribution(display_name, href);
    }
    // add layer guides
    for layer in build_guide.layer_guides.iter() {
        meta_builder.add_layer(layer.layer_name(), &layer.into());
    }

    meta_builder
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use crate::{
        data_structures::TileStoreOptions,
        readers::{FileReader, json::JSONReader},
    };
    use open_vector_tile::Extent;
    use s2_tilejson::{
        Center, DrawType, Encoding, FaceBounds, LayerMetaData, Metadata, SourceType,
        TileStatsMetadata, VectorLayer,
    };
    use s2json::{BBox, MValue, PrimitiveShape, Shape, ShapeType};
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
