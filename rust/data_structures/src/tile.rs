use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use geometry::{
    CellId, ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM, S2CellId,
    SimplifyVectorGeometry, build_sq_dists, clip_features, convert,
};
use parsers::FeatureReader;
use s2json::{
    Axis, Face, Feature, JSONCollection, MValue, Projection, Properties, VectorFeature,
    VectorGeometry, VectorPoint,
};
use serde::{Deserialize, Serialize};

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
impl HasLayer for MValue {
    fn get_layer(&self) -> Option<String> {
        let layer = self.get("layer");
        match layer {
            Some(l) => l.to_prim()?.to_string(),
            _ => None,
        }
    }
}

/// Tile Class to contain the tile information for splitting or simplifying
#[derive(Debug, Clone, PartialEq)]
pub struct Tile<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// the tile id
    pub id: CellId,
    /// the tile's layers
    pub layers: BTreeMap<String, Layer<M, P, D>>,
    /// whether the tile feature geometry has been transformed
    pub transformed: bool,
}
impl<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default> Tile<M, P, D> {
    /// Create a new Tile
    pub fn new(id: CellId) -> Self {
        Self { id, layers: BTreeMap::new(), transformed: false }
    }

    /// Returns the number of layers
    pub fn len(&self) -> usize {
        self.layers.len()
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

    /// Add any reader to the tile
    pub fn add_reader<R>(&mut self, reader: R, layer: Option<String>)
    where
        R: FeatureReader<M, P, D>,
    {
        for feature in reader.iter() {
            self.add_feature(feature, layer.clone());
        }
    }

    /// Add a feature to the tile
    pub fn add_feature(&mut self, feature: VectorFeature<M, P, D>, layer: Option<String>) {
        let layer_name = feature
            .metadata
            .as_ref()
            .and_then(|meta| meta.get_layer()) // Get the layer from metadata if it exists
            .or(layer) // Fall back to the provided layer
            .unwrap_or_else(|| "default".to_string()); // Fall back to "default" if none found

        let layer = self.layers.entry(layer_name.clone()).or_insert(Layer::new(layer_name));
        layer.features.push(feature);
    }

    /// Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
    /// NOTE: This should be called after the tile has been split into children if that functionality
    /// is needed.
    pub fn transform(&mut self, tolerance: f64, maxzoom: Option<u8>) {
        if self.transformed || self.id.is_face() {
            self.transformed = true;
            return;
        }

        let (_, zoom, i, j) = self.id.to_face_ij();
        for layer in self.layers.values_mut() {
            for feature in layer.features.iter_mut() {
                feature.geometry.simplify(tolerance, zoom, maxzoom);
                feature.geometry.transform(zoom.into(), i as f64, j as f64)
            }
        }

        self.transformed = true;
    }

    /// split tile into 4 children
    pub fn split(&mut self, buffer: Option<f64>) -> TileChildren<M, P, D> {
        let buffer = buffer.unwrap_or(0.0625);
        let (face, zoom, i, j) = self.id.to_face_ij();
        let [bl_id, br_id, tl_id, tr_id] = S2CellId::children_ij(face, zoom, i, j);
        let mut children = TileChildren {
            bottom_left: Tile::new(bl_id),
            bottom_right: Tile::new(br_id),
            top_left: Tile::new(tl_id),
            top_right: Tile::new(tr_id),
        };
        let scale = (1 << zoom) as f64;
        let k1 = 0.;
        let k2 = 0.5;
        let k3 = 0.5;
        let k4 = 1.;
        let i = i as f64;
        let j = j as f64;

        let mut tl: Option<Vec<VectorFeature<M, P, D>>>;
        let mut bl: Option<Vec<VectorFeature<M, P, D>>>;
        let mut tr: Option<Vec<VectorFeature<M, P, D>>>;
        let mut br: Option<Vec<VectorFeature<M, P, D>>>;

        for (name, layer) in self.layers.iter_mut() {
            let features = &layer.features;
            let left = clip_features(features, scale, i - k1, i + k3, Axis::X, buffer);
            let right = clip_features(features, scale, i + k2, i + k4, Axis::X, buffer);

            if let Some(left) = left {
                bl = clip_features(&left, scale, j - k1, j + k3, Axis::Y, buffer);
                tl = clip_features(&left, scale, j + k2, j + k4, Axis::Y, buffer);
                if let Some(bl) = bl {
                    for d in bl {
                        children.bottom_left.add_feature(d, Some(name.to_string()));
                    }
                }
                if let Some(tl) = tl {
                    for d in tl {
                        children.top_left.add_feature(d, Some(name.to_string()));
                    }
                }
            }

            if let Some(right) = right {
                br = clip_features(&right, scale, j - k1, j + k3, Axis::Y, buffer);
                tr = clip_features(&right, scale, j + k2, j + k4, Axis::Y, buffer);
                if let Some(br) = br {
                    for d in br {
                        children.bottom_right.add_feature(d, Some(name.to_string()));
                    }
                }
                if let Some(tr) = tr {
                    for d in tr {
                        children.top_right.add_feature(d, Some(name.to_string()));
                    }
                }
            }
        }

        children
    }
}

/// The children of a tile
#[derive(Debug)]
pub struct TileChildren<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// The bottom left child tile
    pub bottom_left: Tile<M, P, D>,
    /// The bottom right child tile
    pub bottom_right: Tile<M, P, D>,
    /// The top left child tile
    pub top_left: Tile<M, P, D>,
    /// The top right child tile
    pub top_right: Tile<M, P, D>,
}

/// Layer Class to contain the layer information for splitting or simplifying
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// the layer name
    pub name: String,
    /// the layer's features
    pub features: Vec<VectorFeature<M, P, D>>,
}
impl<M, P: Clone + Default, D: Clone + Default> Layer<M, P, D> {
    /// Create a new Layer
    pub fn new(name: String) -> Self {
        Self { name, features: vec![] }
    }
}

/// Options for creating a TileStore
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TileStoreOptions {
    /// manually set the projection, otherwise it defaults to whatever the data type is
    pub projection: Option<Projection>,
    /// min zoom to generate data on
    pub minzoom: Option<u8>,
    /// max zoom level to cluster the points on
    pub maxzoom: Option<u8>,
    /// tile buffer on each side in pixels
    pub index_maxzoom: Option<u8>,
    /// simplification tolerance (higher means simpler). Default is 3.
    /// Note: this tolerance is measured against a 4_096x4_096 unit grid when applying simplification.
    pub tolerance: Option<f64>,
    /// tile buffer on each side so lines and polygons don't get clipped
    pub buffer: Option<f64>,
}

/// TileStore Class is a tile-lookup system that splits and simplifies as needed for each tile request */
#[derive(Debug, Clone, PartialEq)]
pub struct TileStore<
    M: HasLayer + Clone = (),
    P: Clone + Default = Properties,
    D: Clone + Default = MValue,
> {
    /// min zoom to preserve detail on
    pub minzoom: u8,
    /// max zoom to preserve detail on
    pub maxzoom: u8,
    /// store which faces are active. 0 face could be entire WM projection
    pub faces: BTreeSet<Face>,
    /// max zoom in the tile index
    pub index_maxzoom: u8,
    /// simplification tolerance (higher means simpler)
    pub tolerance: f64,
    /// tile buffer for lines and polygons
    pub buffer: f64,
    /// stores both WM and S2 tiles
    pub tiles: BTreeMap<CellId, Tile<M, P, D>>,
    /// projection to build tiles for
    pub projection: Projection,
}
impl<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default> Default for TileStore<M, P, D> {
    fn default() -> Self {
        Self {
            minzoom: 0,
            maxzoom: 14,
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: 4,
            tolerance: 3. / 4_096.,
            buffer: 0.0625,
            tiles: BTreeMap::<CellId, Tile<M, P, D>>::new(),
            projection: Projection::S2,
        }
    }
}
impl<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default> TileStore<M, P, D>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    /// Create a new TileStore
    pub fn new(data: JSONCollection<M, P, D>, options: TileStoreOptions) -> Self {
        let mut tile_store = Self {
            minzoom: options.minzoom.unwrap_or(0),
            maxzoom: options.maxzoom.unwrap_or(14),
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: options.index_maxzoom.unwrap_or(4),
            tolerance: options.tolerance.unwrap_or(3.) / 4_096.,
            buffer: options.buffer.unwrap_or(64.),
            tiles: BTreeMap::<CellId, Tile<M, P, D>>::new(),
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
        let features: Vec<VectorFeature<M, P, D>> =
            convert(tile_store.projection, &data, Some(true), Some(true));
        features.into_iter().for_each(|feature| tile_store.add_feature(feature));
        for i in 0..6 {
            tile_store.split_tile(CellId::from_face(i), None, None);
        }

        tile_store
    }

    /// Add a feature to the tile store
    fn add_feature(&mut self, mut feature: VectorFeature<M, P, D>) {
        let face: u8 = feature.face.into();
        let tile = self.tiles.entry(CellId::from_face(face)).or_insert_with(|| {
            self.faces.insert(feature.face);
            Tile::new(CellId::from_face(face))
        });
        // Prep Douglas-Peucker simplification by setting t-values.
        build_sq_dists(&mut feature.geometry, self.tolerance, Some(self.maxzoom));

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
                (end_id.is_some() && (tile_zoom > end_zoom || !tile.id.contains(end_id.unwrap())))
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
    pub fn get_tile(&mut self, id: CellId) -> Option<&Tile<M, P, D>> {
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
pub trait TransformVectorGeometry<M: Clone + Default = MValue> {
    /// Transform the geometry from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64);
}
impl<M: Clone + Default> TransformVectorGeometry<M> for VectorGeometry<M> {
    /// Transform the geometry from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64) {
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
impl<M: Clone + Default> TransformVectorGeometry<M> for VectorPoint<M> {
    /// Transform the point from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64) {
        self.x = self.x * zoom - ti;
        self.y = self.y * zoom - tj;
    }
}
