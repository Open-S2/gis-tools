use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use libm::round;

use crate::geometry::{
    convert, CellId, ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM, Face,
    JSONCollection, Projection, SimplifyVectorGeometry, TileChildren, VectorFeature,
    VectorGeometry, VectorPoint,
};

/// If a user creates metadata for a VectorFeature, it needs to define a get_layer function
pub trait HasLayer {
    /// Get the layer from metadata if it exists
    fn get_layer(&self) -> Option<String>;
}
impl HasLayer for () {
    fn get_layer(&self) -> Option<String> {
        None
    }
}

/// Tile Class to contain the tile information for splitting or simplifying
pub struct Tile<M> {
    /// the tile id
    pub id: CellId,
    /// the tile's layers
    pub layers: BTreeMap<String, Layer<M>>,
    /// whether the tile feature geometry has been transformed
    pub transformed: bool,
}
impl<M: HasLayer + Clone> Tile<M> {
    /// Create a new Tile
    pub fn new(id: CellId) -> Self {
        Self { id, layers: BTreeMap::new(), transformed: false }
    }

    /// Returns true if the tile is empty of features
    pub fn is_empty(&self) -> bool {
        for layer in self.layers.values() {
            if !layer.features.is_empty() {
                return false;
            }
        }

        true
    }

    /// Add a feature to the tile
    pub fn add_feature(&mut self, feature: VectorFeature<M>, layer: Option<String>) {
        let layer_name = feature
            .metadata
            .as_ref()
            .and_then(|meta| meta.get_layer()) // Get the layer from metadata if it exists
            .or(layer) // Fall back to the provided layer
            .unwrap_or_else(|| "default".to_string()); // Fall back to "default" if none found

        if !self.layers.contains_key(&layer_name) {
            self.layers.insert(layer_name.clone(), Layer::new(layer_name.clone()));
        }
        self.layers.get_mut(&layer_name).unwrap().features.push(feature);
    }

    /// Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
    /// NOTE: This should be called after the tile has been split into children if that functionality
    /// is needed.
    pub fn transform(&mut self, tolerance: f64, maxzoom: Option<u8>) {
        if self.transformed {
            return;
        }
        let (zoom, i, j) = self.id.to_zoom_ij(None);

        for layer in self.layers.values_mut() {
            for feature in layer.features.iter_mut() {
                feature.geometry.simplify(tolerance, zoom, maxzoom);
                feature.geometry.transform(zoom, i as f64, j as f64)
            }
        }

        self.transformed = true;
    }
}

/// Layer Class to contain the layer information for splitting or simplifying
pub struct Layer<M> {
    /// the layer name
    pub name: String,
    /// the layer's features
    pub features: Vec<VectorFeature<M>>,
}
impl<M> Layer<M> {
    /// Create a new Layer
    pub fn new(name: String) -> Self {
        Self { name, features: vec![] }
    }
}

/// Options for creating a TileStore
pub struct TileStoreOptions {
    /// manually set the projection, otherwise it defaults to whatever the data type is
    pub projection: Option<Projection>,
    /// min zoom to generate data on
    pub minzoom: Option<u8>,
    /// max zoom level to cluster the points on
    pub maxzoom: Option<u8>,
    /// tile buffer on each side in pixels
    pub index_maxzoom: Option<u8>,
    /// simplification tolerance (higher means simpler)
    pub tolerance: Option<f64>,
    /// tile buffer on each side so lines and polygons don't get clipped
    pub buffer: Option<f64>,
}

/// TileStore Class is a tile-lookup system that splits and simplifies as needed for each tile request */
pub struct TileStore<M: HasLayer + Clone> {
    minzoom: u8,                      // min zoom to preserve detail on
    maxzoom: u8,                      // max zoom to preserve detail on
    faces: BTreeSet<Face>, // store which faces are active. 0 face could be entire WM projection
    index_maxzoom: u8,     // max zoom in the tile index
    tolerance: f64,        // simplification tolerance (higher means simpler)
    buffer: f64,           // tile buffer for lines and polygons
    tiles: BTreeMap<CellId, Tile<M>>, // stores both WM and S2 tiles
    projection: Projection, // projection to build tiles for
}
impl<M: HasLayer + Clone> Default for TileStore<M> {
    fn default() -> Self {
        Self {
            minzoom: 0,
            maxzoom: 18,
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: 4,
            tolerance: 3.,
            buffer: 0.0625,
            tiles: BTreeMap::<CellId, Tile<M>>::new(),
            projection: Projection::S2,
        }
    }
}
impl<
        M: HasLayer
            + Clone
            + ConvertFeature<M>
            + ConvertVectorFeatureWM<M>
            + ConvertVectorFeatureS2<M>,
    > TileStore<M>
{
    /// Create a new TileStore
    pub fn new(data: JSONCollection<M>, options: TileStoreOptions) -> Self {
        let mut tile_store = Self {
            minzoom: options.minzoom.unwrap_or(0),
            maxzoom: options.maxzoom.unwrap_or(20),
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: options.index_maxzoom.unwrap_or(4),
            tolerance: options.tolerance.unwrap_or(3.),
            buffer: options.buffer.unwrap_or(64.),
            tiles: BTreeMap::<CellId, Tile<M>>::new(),
            projection: options.projection.unwrap_or(Projection::S2),
        };
        // sanity check
        debug_assert!(
            tile_store.minzoom <= tile_store.maxzoom
                && tile_store.maxzoom > 0
                && tile_store.maxzoom <= 20,
            "maxzoom should be in the 0-20 range"
        );
        // convert features
        let features: Vec<VectorFeature<M>> = convert(
            tile_store.projection,
            &data,
            Some(tile_store.tolerance),
            Some(tile_store.maxzoom),
            None,
        );
        features.into_iter().for_each(|feature| tile_store.add_feature(feature));
        for i in 0..6 {
            tile_store.split_tile(CellId::from_face(i), None, None);
        }

        tile_store
    }

    /// Add a feature to the tile store
    pub fn add_feature(&mut self, feature: VectorFeature<M>) {
        let face: u8 = feature.face.into();
        let tile = self.tiles.entry(CellId::from_face(face)).or_insert_with(|| {
            self.faces.insert(feature.face);
            Tile::new(CellId::from_face(face))
        });

        tile.add_feature(feature, None);
    }

    /// Split tiles given a range
    fn split_tile(&mut self, start_id: CellId, end_id: Option<CellId>, end_zoom: Option<u8>) {
        let TileStore { buffer, tiles, tolerance, maxzoom, index_maxzoom, .. } = self;
        let end_zoom = end_zoom.unwrap_or(*maxzoom);
        let mut stack: Vec<CellId> = vec![start_id];
        // avoid recursion by using a processing queue
        while !stack.is_empty() {
            // find our next tile to split
            let stack_id = stack.pop();
            if stack_id.is_none() {
                break;
            }
            let tile = tiles.get_mut(&stack_id.unwrap());
            // if the tile we need does not exist, is empty, or already transformed, skip it
            if tile.is_none() {
                continue;
            }
            let tile = tile.unwrap();
            if tile.is_empty() || tile.transformed {
                continue;
            }
            let tile_zoom = tile.id.level();
            // 1: stop tiling if we reached a defined end
            // 2: stop tiling if it's the first-pass tiling, and we reached max zoom for indexing
            // 3: stop at currently needed maxzoom OR current tile does not include child
            if tile_zoom >= *maxzoom || // 1
                (end_id.is_none() && tile_zoom >= *index_maxzoom) || // 2
                (end_id.is_some() && (tile_zoom > end_zoom || !tile.id.contains(&end_id.unwrap())))
            {
                continue;
            }

            // split the tile
            let TileChildren {
                bottom_left: bl_id,
                bottom_right: br_id,
                top_left: tl_id,
                top_right: tr_id,
            } = tile.split(Some(*buffer));
            // now that the tile has been split, we can transform it
            tile.transform(*tolerance, Some(*maxzoom));
            // push the new features to the stack
            stack.extend(vec![bl_id.id, br_id.id, tl_id.id, tr_id.id]);
            // store the children
            tiles.insert(bl_id.id, bl_id);
            tiles.insert(br_id.id, br_id);
            tiles.insert(tl_id.id, tl_id);
            tiles.insert(tr_id.id, tr_id);
        }
    }

    /// Get a tile
    pub fn get_tile(&mut self, id: CellId) -> Option<&Tile<M>> {
        let zoom = id.level();
        let face = id.face();
        // If the zoom is out of bounds, return nothing
        if !(0..=20).contains(&zoom) || !self.faces.contains(&face.into()) {
            return None;
        }

        // we want to find the closest tile to the data.
        let mut p_id = id;
        while !self.tiles.contains_key(&p_id) && !p_id.is_face() {
            p_id = p_id.parent(None);
        }
        // split as necessary, the algorithm will know if the tile is already split
        self.split_tile(p_id, Some(id), Some(zoom));

        // grab the tile and split it if necessary
        self.tiles.get(&id)
    }
}

/// A trait for transforming a geometry from the 0->1 coordinate system to a tile coordinate system
pub trait TransformVectorGeometry {
    /// Transform the geometry from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: u8, ti: f64, tj: f64);
}

impl TransformVectorGeometry for VectorGeometry {
    /// Transform the geometry from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: u8, ti: f64, tj: f64) {
        let zoom = (1 << (zoom as u64)) as f64;
        match self {
            VectorGeometry::Point(p) => p.coordinates.transform(zoom, ti, tj),
            VectorGeometry::LineString(l) | VectorGeometry::MultiPoint(l) => {
                l.coordinates.iter_mut().for_each(|p| p.transform(zoom, ti, tj))
            }
            VectorGeometry::MultiLineString(l) | VectorGeometry::Polygon(l) => l
                .coordinates
                .iter_mut()
                .for_each(|l| l.iter_mut().for_each(|p| p.transform(zoom, ti, tj))),
            VectorGeometry::MultiPolygon(l) => l.coordinates.iter_mut().for_each(|p| {
                p.iter_mut().for_each(|l| l.iter_mut().for_each(|p| p.transform(zoom, ti, tj)))
            }),
        }
    }
}

/// A trait for transforming a point from the 0->1 coordinate system to a tile coordinate system
pub trait TransformVectorPoint {
    /// Transform the point from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64);
}

impl TransformVectorPoint for VectorPoint {
    /// Transform the point from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64) {
        self.x = round(self.x * zoom - ti);
        self.y = round(self.y * zoom - tj);
    }
}
