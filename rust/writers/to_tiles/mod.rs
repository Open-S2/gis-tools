/// Types, Structs, and Enums for the to_tiles converter
pub mod types;
/// Worker
pub mod worker;

use super::OnFeature;
use crate::{
    data_structures::HasLayer,
    geometry::{wm, xyz_to_bbox},
    parsers::FeatureReader,
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
        on_source_feature: Option<OnFeature<M, P, D>>,
        on_layer_feature: Option<&Vec<LayerHandler<M, P, D>>>,
    ) {
        for mut feature in reader.iter() {
            if let Some(on_feature) = on_source_feature
                && let Some(new_feature) = (on_feature)(take(&mut feature))
            {
                feature = new_feature;
            }
            self.worker.store_feature(feature, source_name.clone(), on_layer_feature);
        }
    }

    /// Add vector points with RGBA attributes to build raster tiles
    #[cfg_attr(feature = "nightly", coverage(off))] // not implemented don't punish
    pub fn add_raster_source<
        M: Clone,
        P: MValueCompatible,
        D: MValueCompatible,
        T: FeatureReader<M, P, D>,
    >(
        _source_name: String,
        _reader: T,
        _on_feature: Option<OnFeature<M, P, D>>,
    ) {
        unimplemented!()
    }

    /// Add data that will be gridded like raster data but float precision points
    #[cfg_attr(feature = "nightly", coverage(off))] // not implemented don't punish
    pub fn add_grid_source<
        M: Clone,
        P: MValueCompatible,
        D: MValueCompatible,
        T: FeatureReader<M, P, D>,
    >(
        _source_name: String,
        _reader: T,
        _on_feature: Option<OnFeature<M, P, D>>,
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
