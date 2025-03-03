use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

use libm::round;
use s2json::{Feature, MValue, MValueCompatible, Properties};

use crate::geometry::{
    convert, CellId, ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM, Face,
    JSONCollection, Projection, SimplifyVectorGeometry, TileChildren, VectorFeature,
    VectorGeometry, VectorPoint,
};
use crate::readers::FeatureIterator;

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
#[derive(Debug, Clone, PartialEq)]
pub struct Tile<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// the tile id
    pub id: CellId,
    /// the tile's layers
    pub layers: BTreeMap<String, Layer<M, P, D>>,
    /// whether the tile feature geometry has been transformed
    pub transformed: bool,
}
impl<M: HasLayer + Clone, P: MValueCompatible, D: MValueCompatible> Tile<M, P, D> {
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
        R: FeatureIterator<M, P, D>,
    {
        for feature in reader {
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
        let (_, zoom, i, j) = self.id.to_face_ij();

        for layer in self.layers.values_mut() {
            for feature in layer.features.iter_mut() {
                feature.geometry.simplify(tolerance, zoom, maxzoom);
                feature.geometry.transform(zoom.into(), i as f64, j as f64)
            }
        }

        self.transformed = true;
    }
}

/// Layer Class to contain the layer information for splitting or simplifying
#[derive(Debug, Clone, PartialEq)]
pub struct Layer<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// the layer name
    pub name: String,
    /// the layer's features
    pub features: Vec<VectorFeature<M, P, D>>,
}
impl<M, P: MValueCompatible, D: MValueCompatible> Layer<M, P, D> {
    /// Create a new Layer
    pub fn new(name: String) -> Self {
        Self { name, features: vec![] }
    }
}

/// Options for creating a TileStore
#[derive(Debug, Clone, PartialEq, Default)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct TileStore<
    M: HasLayer + Clone = (),
    P: MValueCompatible = Properties,
    D: MValueCompatible = MValue,
> {
    minzoom: u8,                            // min zoom to preserve detail on
    maxzoom: u8,                            // max zoom to preserve detail on
    faces: BTreeSet<Face>, // store which faces are active. 0 face could be entire WM projection
    index_maxzoom: u8,     // max zoom in the tile index
    tolerance: f64,        // simplification tolerance (higher means simpler)
    buffer: f64,           // tile buffer for lines and polygons
    tiles: BTreeMap<CellId, Tile<M, P, D>>, // stores both WM and S2 tiles
    projection: Projection, // projection to build tiles for
}
impl<M: HasLayer + Clone, P: MValueCompatible, D: MValueCompatible> Default for TileStore<M, P, D> {
    fn default() -> Self {
        Self {
            minzoom: 0,
            maxzoom: 18,
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: 4,
            tolerance: 3.,
            buffer: 0.0625,
            tiles: BTreeMap::<CellId, Tile<M, P, D>>::new(),
            projection: Projection::S2,
        }
    }
}
impl<M: HasLayer + Clone, P: MValueCompatible, D: MValueCompatible> TileStore<M, P, D>
where
    VectorFeature<M, P, D>: ConvertVectorFeatureWM<M, P, D> + ConvertVectorFeatureS2<M, P, D>,
    Feature<M, P, D>: ConvertFeature<M, P, D>,
{
    /// Create a new TileStore
    pub fn new(data: JSONCollection<M, P, D>, options: TileStoreOptions) -> Self {
        let mut tile_store = Self {
            minzoom: options.minzoom.unwrap_or(0),
            maxzoom: options.maxzoom.unwrap_or(20),
            faces: BTreeSet::<Face>::new(),
            index_maxzoom: options.index_maxzoom.unwrap_or(4),
            tolerance: options.tolerance.unwrap_or(3.),
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
        let features: Vec<VectorFeature<M, P, D>> = convert(
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
    fn add_feature(&mut self, feature: VectorFeature<M, P, D>) {
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
pub trait TransformVectorGeometry<M: MValueCompatible = MValue> {
    /// Transform the geometry from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64);
}
impl<M: MValueCompatible> TransformVectorGeometry<M> for VectorGeometry<M> {
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
impl<M: MValueCompatible> TransformVectorGeometry<M> for VectorPoint<M> {
    /// Transform the point from the 0->1 coordinate system to a tile coordinate system
    fn transform(&mut self, zoom: f64, ti: f64, tj: f64) {
        self.x = round(self.x * zoom - ti);
        self.y = round(self.y * zoom - tj);
    }
}

#[cfg(test)]
mod tests {
    use s2json::{Map, VectorPointGeometry};

    use crate::geometry::S2CellId;

    use super::*;

    const SIMPLIFY_MAXZOOM: u8 = 16;

    #[test]
    fn test_transform() {
        let mut p: VectorPoint = VectorPoint { x: 0.5, y: 0.5, z: Some(0.0), m: None, t: None };
        p.transform(10.0, 0.0, 0.0);
        assert_eq!(p.x, 5.0);
        assert_eq!(p.y, 5.0);

        let mut p: VectorPoint = VectorPoint { x: 0., y: 0., z: Some(0.0), m: None, t: None };
        p.transform(1., 0., 0.);
        assert_eq!(p.x, 0.);
        assert_eq!(p.y, 0.);

        let mut p: VectorPoint = VectorPoint { x: 0., y: 0., z: Some(0.0), m: None, t: None };
        p.transform(1., 1., 0.);
        assert_eq!(p.x, -1.);
        assert_eq!(p.y, -0.);
    }

    #[test]
    fn test_tile() {
        let mut tile: Tile = Tile::new(S2CellId::from_face(0));
        assert_eq!(
            tile,
            Tile { id: 1152921504606846976.into(), layers: BTreeMap::new(), transformed: false }
        );
        assert!(tile.is_empty());
        assert_eq!(tile.len(), 0);

        tile.add_feature(
            VectorFeature::new_wm(
                None,
                Map::new(),
                VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    ..Default::default()
                }),
                None,
            ),
            Some("default".into()),
        );

        assert!(!tile.is_empty());
        assert_eq!(tile.len(), 1);

        tile.transform(3., Some(SIMPLIFY_MAXZOOM));
        // call it again (it will fail)
        tile.transform(3., Some(SIMPLIFY_MAXZOOM));

        // grab the feature
        let layer = tile.layers.get("default").unwrap();
        let first_feature = layer.features.first().unwrap();
        assert_eq!(
            first_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_tile_store() {
        let tile_store: TileStore = TileStore::default();
        assert_eq!(
            tile_store,
            TileStore {
                minzoom: 0,
                maxzoom: 18,
                faces: BTreeSet::<Face>::new(),
                index_maxzoom: 4,
                tolerance: 3.,
                buffer: 0.0625,
                tiles: BTreeMap::new(),
                projection: Projection::S2,
            }
        );

        let tile_store: TileStore = Default::default();
        assert_eq!(
            tile_store,
            TileStore {
                minzoom: 0,
                maxzoom: 18,
                faces: BTreeSet::<Face>::new(),
                index_maxzoom: 4,
                tolerance: 3.,
                buffer: 0.0625,
                tiles: BTreeMap::new(),
                projection: Projection::S2,
            }
        );
    }

    #[test]
    fn test_tile_store_wg_points() {
        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": { "a": 1 },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "b": 2 },
                    "geometry": {
                        "type": "Point3D",
                        "coordinates": [45, 45, 1]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "c": 3 },
                    "geometry": {
                        "type": "MultiPoint",
                        "coordinates": [
                            [-45, -45],
                            [-45, 45]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": { "d": 4 },
                    "geometry": {
                        "type": "MultiPoint3D",
                        "coordinates": [
                            [45, -45, 1],
                            [-180, 20, 2]
                        ]
                    }
                }
            ]
        }"#;
        let data: JSONCollection = serde_json::from_str(json_string).unwrap();
        let mut tile_store: TileStore = TileStore::<_, _, _>::new(
            data,
            TileStoreOptions { projection: Some(Projection::WG), ..Default::default() },
        );

        let face_0_tile = tile_store.get_tile(S2CellId::from_face(0)).unwrap();
        assert_eq!(face_0_tile.len(), 1);
        let default_layer = face_0_tile.layers.get("default").unwrap();
        assert_eq!(default_layer.features.len(), 4);
    }
}
