use crate::{
    converter::OnFeature,
    data_structures::{ClusterOptions, GridOptions, HasLayer, TileStoreOptions},
    readers::{RGBA, ReaderType},
};
use alloc::{collections::BTreeMap, string::String, vec, vec::Vec};
use open_vector_tile::Extent;
use s2_tilejson::{Attributions, DrawType, Encoding, LayerMetaData, SourceType};
use s2json::{MValue, MValueCompatible, Projection, Properties, Shape, VectorFeature};
use serde::{Deserialize, Serialize};

/// This is how all vector features metadata will be stored
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToTileMetadata {
    /// Name of the layer
    pub layer_name: String,
}
impl ToTileMetadata {
    /// Create a new ToTileMetadata
    pub fn new(layer_name: String) -> ToTileMetadata {
        ToTileMetadata { layer_name }
    }
}
impl HasLayer for ToTileMetadata {
    fn get_layer(&self) -> Option<String> {
        Some(self.layer_name.clone())
    }
}

/// A Vector feature that uses the Open S2 spec to manage props and values
pub type MVectorFeature = VectorFeature<ToTileMetadata, Properties, MValue>;

/// This is a generic handler for any layer type
#[derive(Debug)]
pub struct LayerHandler<M: Clone + HasLayer, P: MValueCompatible, D: MValueCompatible> {
    /// Name of the layer
    pub layer_name: String,
    /// Manipulate the feature before storing it
    pub on_feature: OnFeature<M, P, D>,
}

/// No matter the type of layer you want to build, these are default properties to include
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseLayer {
    /// Explain what the layer is
    pub description: Option<String>,
    /// Name of the source
    pub source_name: String,
    /// Name of the layer
    pub layer_name: String,
}

/// Guide to building Raster layer data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RasterLayerGuide {
    /// describes how the image will be stored
    pub output_type: String,
    /// Raster clustering guide
    pub raster_guide: GridOptions<RGBA>,
    /// Common layer properties across all layer types
    #[serde(flatten)]
    pub base: BaseLayer,
}
impl From<&RasterLayerGuide> for LayerMetaData {
    fn from(lg: &RasterLayerGuide) -> Self {
        LayerMetaData {
            description: lg.base.description.clone(),
            minzoom: lg.raster_guide.minzoom.unwrap_or(0),
            maxzoom: lg.raster_guide.maxzoom.unwrap_or(16),
            draw_types: vec![DrawType::Raster],
            ..Default::default()
        }
    }
}

/// Guide to building Grid layer data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GridLayerGuide {
    /// Grid clustering guide
    pub grid_guide: GridOptions<f64>,
    /// Extent at which the layer is storing its data
    pub extent: Extent,
    /// Common layer properties across all layer types
    #[serde(flatten)]
    pub base: BaseLayer,
}
impl From<&GridLayerGuide> for LayerMetaData {
    fn from(lg: &GridLayerGuide) -> Self {
        LayerMetaData {
            description: lg.base.description.clone(),
            minzoom: lg.grid_guide.minzoom.unwrap_or(0),
            maxzoom: lg.grid_guide.maxzoom.unwrap_or(16),
            draw_types: vec![DrawType::Grid],
            ..Default::default()
        }
    }
}

/// Guide to building Cluster layer data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusterLayerGuide {
    /// If options are provided, the assumption is the point data is clustered
    pub cluster_guide: ClusterOptions,
    /// Extent at which the layer is storing its data
    pub extent: Extent,
    /// Common layer properties across all layer types
    #[serde(flatten)]
    pub base: BaseLayer,
}
impl From<&ClusterLayerGuide> for LayerMetaData {
    fn from(lg: &ClusterLayerGuide) -> Self {
        LayerMetaData {
            description: lg.base.description.clone(),
            minzoom: lg.cluster_guide.minzoom.unwrap_or(0),
            maxzoom: lg.cluster_guide.maxzoom.unwrap_or(16),
            draw_types: vec![DrawType::Points],
            ..Default::default()
        }
    }
}

/// Guide to building Vector layer data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorLayerGuide {
    /// Guide on how to splice the data into vector tiles
    pub vector_guide: TileStoreOptions,
    /// Extent at which the layer is storing its data
    pub extent: Extent,
    /// Shape guide for the vector layer. If not provided it will be built for you
    pub shape: Option<Shape>,
    /// M-Value Shape guide for the vector layer
    pub m_shape: Option<Shape>,
    /// Draw Types (points, lines, polygons, 3D points, 3D lines, 3D polygons).
    /// This is a filter mechanic. The source data may have multiple feature/draw types,
    /// but if the layer you"re building only wants to use the points, you can filter that here
    pub draw_types: Vec<DrawType>,
    /// Common layer properties across all layer types
    #[serde(flatten)]
    pub base: BaseLayer,
}
impl From<&VectorLayerGuide> for LayerMetaData {
    fn from(lg: &VectorLayerGuide) -> Self {
        LayerMetaData {
            description: lg.base.description.clone(),
            minzoom: lg.vector_guide.minzoom.unwrap_or(0),
            maxzoom: lg.vector_guide.maxzoom.unwrap_or(16),
            draw_types: vec![DrawType::Points],
            shape: lg.shape.clone().unwrap_or_default(),
            m_shape: lg.m_shape.clone(),
        }
    }
}

/// List of user defined guides to build layers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayerGuide {
    /// Raster guide
    Raster(RasterLayerGuide),
    /// Grid guide
    Grid(GridLayerGuide),
    /// Cluster guide
    Cluster(ClusterLayerGuide),
    /// Vector guide
    Vector(VectorLayerGuide),
}
impl LayerGuide {
    /// Get the minzoom and maxzoom of the active layer guide
    pub fn zooms(&self) -> (u8, u8) {
        match self {
            LayerGuide::Raster(r) => {
                (r.raster_guide.minzoom.unwrap_or(0), r.raster_guide.maxzoom.unwrap_or(16))
            }
            LayerGuide::Grid(g) => {
                (g.grid_guide.minzoom.unwrap_or(0), g.grid_guide.maxzoom.unwrap_or(16))
            }
            LayerGuide::Cluster(c) => {
                (c.cluster_guide.minzoom.unwrap_or(0), c.cluster_guide.maxzoom.unwrap_or(16))
            }
            LayerGuide::Vector(v) => {
                (v.vector_guide.minzoom.unwrap_or(0), v.vector_guide.maxzoom.unwrap_or(16))
            }
        }
    }

    /// Check the source name matches the layer"s source
    pub fn has_source(&self, source_name: &str) -> bool {
        match self {
            LayerGuide::Raster(r) => r.base.source_name == source_name,
            LayerGuide::Grid(g) => g.base.source_name == source_name,
            LayerGuide::Cluster(c) => c.base.source_name == source_name,
            LayerGuide::Vector(v) => v.base.source_name == source_name,
        }
    }

    /// Get the layer name of the active layer guide
    pub fn layer_name(&self) -> &str {
        match self {
            LayerGuide::Raster(r) => &r.base.layer_name,
            LayerGuide::Grid(g) => &g.base.layer_name,
            LayerGuide::Cluster(c) => &c.base.layer_name,
            LayerGuide::Vector(v) => &v.base.layer_name,
        }
    }

    /// Grab the vector guide
    pub fn to_vector(&self) -> Option<VectorLayerGuide> {
        match self {
            LayerGuide::Vector(v) => Some(v.clone()),
            _ => None,
        }
    }
}
impl Default for LayerGuide {
    fn default() -> Self {
        LayerGuide::Vector(VectorLayerGuide::default())
    }
}
impl From<&LayerGuide> for LayerMetaData {
    fn from(layer_guide: &LayerGuide) -> Self {
        match layer_guide {
            LayerGuide::Raster(r) => r.into(),
            LayerGuide::Grid(g) => g.into(),
            LayerGuide::Cluster(c) => c.into(),
            LayerGuide::Vector(v) => v.into(),
        }
    }
}

/// The vector format if applicable helps define how the vector data is stored.
/// - The more modern vector format is the "open-s2" which supports things like m-values
///   and 3D geometries.
/// - The new vector format is the "open-s2" which only supports 2D & 3D geometries, supports M-Values,
///   properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
/// - The basic vector format is the "flat-open-s2" which only supports 2D geometries and works on
///   older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
/// - The older vector format is the "mapbox" which is the legacy format used by Mapbox and slow to parse.
/// - The `raster` format is used speciially for raster ONLY data. Ensures the data is stored as a raster
///
/// Defaults to `"open-s2"`
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FormatOutput {
    /// Legacy Mapbox format
    Mapbox,
    /// Flat Open S2. Modified Legacy Mapbox format for better compression of polygons
    FlatOpenS2,
    /// Open Vector Tile format
    #[default]
    OpenS2,
    /// Raster data
    Raster,
}
impl From<&str> for FormatOutput {
    fn from(value: &str) -> Self {
        match value {
            "mapbox" => FormatOutput::Mapbox,
            "flat-open-s2" => FormatOutput::FlatOpenS2,
            "open-s2" => FormatOutput::OpenS2,
            "raster" => FormatOutput::Raster,
            _ => FormatOutput::OpenS2,
        }
    }
}
impl From<&FormatOutput> for SourceType {
    fn from(value: &FormatOutput) -> Self {
        match value {
            FormatOutput::Mapbox | FormatOutput::FlatOpenS2 | FormatOutput::OpenS2 => {
                SourceType::Vector
            }
            FormatOutput::Raster => SourceType::Raster,
        }
    }
}

/// The source input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// The name of the source
    pub source_name: String,
    /// The type of source data. E.g. "csv", "json", "pmtiles", "shapefile", etc.
    #[serde(rename = "inputType")]
    pub input_type: ReaderType,
}

/// The source input
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum WhichTileWriting {
    /// Use local memory
    #[default]
    Local,
    /// Filesystem
    File,
    /// Memory Mapped
    MMap,
}

/// A user defined guide on building the vector tiles
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JSONBuildGuide {
    /// The name of the data
    pub name: String,
    /// The description of the data
    pub description: String,
    /// User defined versioning for their data
    pub version: String,
    /// Specify the image type. e.g. "pbf", "png", "jpg", "webp", etc.
    /// [Default: "pbf"]
    pub extension: String,
    /// What kind of output format should be used. Used for describing either S2 or WM
    /// projections. [Default: "fzxy"]
    pub projection: Projection,
    /// The encoding format. Can be either "gz", "br", "zstd" or "none". [Default: "none"]
    pub encoding: Encoding,
    /// The attribution of the data. Store as `{ "presentation name": "href" }`.
    pub attribution: Attributions,
    /// The vector format if applicable helps define how the vector data is stored.
    /// - The more modern vector format is the "open-s2" which supports things like m-values
    ///   and 3D geometries.
    /// - The new vector format is the "open-s2" which only supports 2D & 3D geometries, supports M-Values,
    ///   properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
    /// - The basic vector format is the "flat-open-s2" which only supports 2D geometries and works on
    ///   older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
    /// - The older vector format is the "mapbox" which is the legacy format used by Mapbox and slow to parse.
    /// - The `raster` format is used speciially for raster ONLY data. Ensures the data is stored as a raster
    ///
    /// Defaults to `"open-s2"`
    pub format: FormatOutput,
    /// The vector sources that the tile is built from and how the layers are to be stored.
    /// Created using `{ [source_name: string]: FeatureIterator }`
    /// See: {@link FeatureIterator}
    #[serde(rename = "vectorSources")]
    pub vector_sources: Vec<Source>,
    /// The raster sources that will be conjoined into a single rgba pixel index for tile extraction
    #[serde(rename = "rasterSources")]
    pub raster_sources: Vec<Source>,
    /// The grid sources that will be conjoined into a single grid index for tile extraction
    #[serde(rename = "gridSources")]
    pub grid_sources: Vec<Source>,
    /// Should the indices be built for polygon data for faster rendering (file cost increases). [Default: true]
    #[serde(rename = "buildIndices")]
    pub build_indices: bool,
    /// The guides on how to build the various data
    /// See: {@link LayerGuide}
    #[serde(default, rename = "layerGuides")]
    pub layer_guides: Vec<LayerGuide>,
    /// Set the number of threads to use. [Default: 1]
    pub threads: usize,
    /// tileWriter
    #[serde(default, rename = "tileWriter")]
    pub tile_writer: WhichTileWriting,
}
impl Default for JSONBuildGuide {
    fn default() -> Self {
        JSONBuildGuide {
            name: "auto generated".into(),
            description: "generated via OpenS2 gis-tools".into(),
            version: "1.0.0".into(),
            extension: "pbf".into(),
            projection: Projection::S2,
            encoding: Encoding::None,
            attribution: BTreeMap::default(),
            format: FormatOutput::default(),
            vector_sources: vec![],
            raster_sources: vec![],
            grid_sources: vec![],
            build_indices: true,
            layer_guides: vec![],
            tile_writer: WhichTileWriting::Local,
            threads: 1,
        }
    }
}

/// A user defined guide on building the vector tiles
#[derive(Debug, Clone)]
pub struct BuildGuide {
    /// The name of the data
    pub name: String,
    /// The description of the data
    pub description: String,
    /// User defined versioning for their data
    pub version: String,
    /// Specify the image type. e.g. "pbf", "png", "jpg", "webp", etc.
    /// [Default: `"pbf"`]
    pub extension: String,
    /// What kind of output format should be used. Used for describing either S2 or WM
    /// projections. [Default: `"fzxy"`]
    pub projection: Projection,
    /// The encoding format. Can be either "gz", "br", "zstd" or "none". [Default: `"none"`]
    pub encoding: Encoding,
    /// The attribution of the data. Store as `{ "presentation name": "href" }`.
    pub attributions: Attributions,
    /// The vector format if applicable helps define how the vector data is stored.
    /// - The more modern vector format is the "open-s2" which supports things like m-values
    ///   and 3D geometries.
    /// - The new vector format is the "open-s2" which only supports 2D & 3D geometries, supports M-Values,
    ///   properties and M-Values can have nested properties and/or arrays, and is decently fast to parse.
    /// - The basic vector format is the "flat-open-s2" which only supports 2D geometries and works on
    ///   older map engines like Mapbox-gl-js, is faster to parse and often lighter in size.
    /// - The older vector format is the "mapbox" which is the legacy format used by Mapbox and slow to parse.
    /// - The `raster` format is used speciially for raster ONLY data. Ensures the data is stored as a raster
    ///
    /// [Default: `"open-s2"`]
    pub format: FormatOutput,
    /// Should the indices be built for polygon data for faster rendering (file cost increases). [Default: `true`]
    pub build_indices: bool,
    /// The guides on how to build the various data
    pub layer_guides: Vec<LayerGuide>,
    /// Set the number of threads to use. [Default: `1`]
    pub threads: usize,
}
impl Default for BuildGuide {
    fn default() -> Self {
        BuildGuide {
            name: "auto generated".into(),
            description: "generated via OpenS2 gis-tools".into(),
            version: "1.0.0".into(),
            extension: "pbf".into(),
            projection: Projection::S2,
            encoding: Encoding::None,
            attributions: BTreeMap::default(),
            format: FormatOutput::default(),
            build_indices: true,
            layer_guides: vec![],
            threads: 1,
        }
    }
}
