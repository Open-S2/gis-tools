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
///
/// See [`TileBuilder`] for full documentation
pub type LocalTileBuilder = TileBuilder<LocalTileWriter>;
/// Use the Filesystem to store all file data
///
/// See [`TileBuilder`] for full documentation
pub type FileTileBuilder = TileBuilder<FileTileWriter>;

/// # The Tile Builder
///
/// ## Description
/// The TileBuilder creates tiles for the user given any source reader that implements [`FeatureReader`]
///
/// Create vector tiles, raster tiles, or gridded tiles.
///
/// Store as Mapbox Vector Tiles or Open Vector Tiles. Utilizes the [`TileWriter`] trait to write the data to.
///
/// Supports storing as a folder structure or a PMTiles (with gzip compression if enabled)
///
/// ## Usage
///
/// The methods you have access to:
/// - [`TileBuilder::new`]: Create a new TileBuilder
/// - [`TileBuilder::tile_writer`]: Get the tile writer
/// - [`TileBuilder::add_vector_source`]: Add a vector source to tile-ize
/// - [`TileBuilder::add_grid_source`]: Add data that will be gridded like raster data but float precision points
/// - [`TileBuilder::build_tiles`]: After adding all the source data, build all tiles into the tile writer
///
/// The Tile Writers this library supports:
/// - [`LocalTileWriter`]
/// - [`FileTileWriter`]
/// - [`PMTilesWriter`]
///
/// ### Writing WM projection tiles as Mapbox Vector Tiles to a PMTiles file
///
/// ```rust
/// use gistools::{
///     data_structures::TileStoreOptions,
///     parsers::{BufferWriter, FileReader},
///     readers::JSONReader,
///     util::CompressionFormat,
///     writers::{
///         BuildGuide, PMTilesWriter, TileBuilder, LayerGuide,
///         VectorLayerGuide, BaseLayer
///     }
/// };
/// use serde::{Deserialize, Serialize};
/// use s2_tilejson::{Metadata, MetadataBuilder, DrawType};
/// use open_vector_tile::Extent;
/// use s2json::{MValue, MValueCompatible, Properties, Projection};
/// use std::{path::PathBuf, collections::BTreeMap};
///
/// #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
/// #[serde(default)]
/// struct PointProps {
///     id: i64,
/// }
///
/// #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
/// #[serde(default)]
/// struct LineProps {
///     linename: String,
/// }
///
/// #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
/// #[serde(default)]
/// struct PolyProps {
///     poly3d: bool,
/// }
///
/// // using a buffer writer for example but ideally you are using a FileWriter
/// let tmp_buffer_writer = BufferWriter::new(vec![]);
/// let pm_writer = PMTilesWriter::new(tmp_buffer_writer, CompressionFormat::None);
/// let build_guide = BuildGuide {
///     projection: Projection::WG,
///     build_indices: true,
///     attributions: BTreeMap::from([("Satellite Data".into(), "https://example.com".into())]),
///     layer_guides: vec![
///         // add points
///         LayerGuide::Vector(VectorLayerGuide {
///             extent: Extent::Extent4096,
///             draw_types: vec![DrawType::Points, DrawType::Points3D],
///             shape: Some((&MValue::from(PointProps::default())).into()),
///             base: BaseLayer {
///                 description: Some("Points Vector Layer".into()),
///                 source_name: "all_features".into(),
///                 layer_name: "points".into(),
///             },
///             vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
///             ..Default::default()
///         }),
///         // add lines
///         LayerGuide::Vector(VectorLayerGuide {
///             extent: Extent::Extent4096,
///             draw_types: vec![DrawType::Lines, DrawType::Lines3D],
///             shape: Some((&MValue::from(LineProps::default())).into()),
///             base: BaseLayer {
///                 description: Some("Lines Vector Layer".into()),
///                 source_name: "all_features".into(),
///                 layer_name: "lines".into(),
///             },
///             vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
///             ..Default::default()
///         }),
///         // add polys
///         LayerGuide::Vector(VectorLayerGuide {
///             extent: Extent::Extent4096,
///             draw_types: vec![DrawType::Polygons, DrawType::Polygons3D],
///             shape: Some((&MValue::from(PolyProps::default())).into()),
///             base: BaseLayer {
///                 description: Some("Polygons Vector Layer".into()),
///                 source_name: "all_features".into(),
///                 layer_name: "polys".into(),
///             },
///             vector_guide: TileStoreOptions { maxzoom: Some(4), ..Default::default() },
///             ..Default::default()
///         }),
///     ],
///     ..Default::default()
/// };
/// let mut tile_builder = TileBuilder::new(pm_writer, build_guide);
///
/// // add json features
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path = path.join("tests/writers/fixtures/all-features.json");
/// let reader: JSONReader<FileReader, (), Properties, Properties> =
///     JSONReader::new(FileReader::from(path));
/// tile_builder.add_vector_source("all_features".into(), reader, None, None);
///
/// // build
/// tile_builder.build_tiles();
/// ```
///
/// ## Links
/// - https://github.com/Open-S2/s2-pmtiles
/// - https://github.com/protomaps/PMTiles
/// - https://github.com/mapbox/vector-tile-spec
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
