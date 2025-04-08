use crate::{
    geometry::{
        CellId, ConvertFeature, ConvertVectorFeatureS2, ConvertVectorFeatureWM, Face,
        JSONCollection, Projection, SimplifyVectorGeometry, TileChildren, VectorFeature,
        VectorGeometry, VectorPoint, convert,
    },
    readers::FeatureReader,
};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use s2json::{Feature, MValue, Properties};
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
    minzoom: u8,                            // min zoom to preserve detail on
    maxzoom: u8,                            // max zoom to preserve detail on
    faces: BTreeSet<Face>, // store which faces are active. 0 face could be entire WM projection
    index_maxzoom: u8,     // max zoom in the tile index
    tolerance: f64,        // simplification tolerance (higher means simpler)
    buffer: f64,           // tile buffer for lines and polygons
    tiles: BTreeMap<CellId, Tile<M, P, D>>, // stores both WM and S2 tiles
    projection: Projection, // projection to build tiles for
}
impl<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default> Default for TileStore<M, P, D> {
    fn default() -> Self {
        Self {
            minzoom: 0,
            maxzoom: 16,
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
            maxzoom: options.maxzoom.unwrap_or(16),
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
        self.x = (self.x * zoom - ti);
        self.y = (self.y * zoom - tj);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::S2CellId;
    use core::f64;
    use s2json::{
        BBox3D, Map, VectorLineStringGeometry, VectorMultiLineStringGeometry,
        VectorMultiPointGeometry, VectorPointGeometry,
    };

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
                maxzoom: 16,
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
                maxzoom: 16,
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

        assert_eq!(
            default_layer.features,
            vec![
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("a".into(), 1_u64.into())]),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.5,
                            top: 0.5,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("b".into(), 2_u64.into())]),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 0.625,
                            y: 0.35972503691520497,
                            z: Some(1.0),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.625,
                            bottom: 0.35972503691520497,
                            right: 0.625,
                            top: 0.35972503691520497,
                            near: 1.0,
                            far: 1.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("c".into(), 3_u64.into())]),
                    geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                        _type: "MultiPoint".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.375,
                                y: 0.640274963084795,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.375,
                                y: 0.35972503691520497,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.375,
                            bottom: 0.35972503691520497,
                            right: 0.375,
                            top: 0.640274963084795,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: MValue::from([("d".into(), 4_u64.into())]),
                    geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                        _type: "MultiPoint".into(),
                        is_3d: true,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.625,
                                y: 0.640274963084795,
                                z: Some(1.0),
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.0,
                                y: 0.4432805993614054,
                                z: Some(2.0),
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.0,
                            bottom: 0.4432805993614054,
                            right: 0.625,
                            top: 0.640274963084795,
                            near: 1.0,
                            far: 2.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn test_tile_store_s2_points() {
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
            TileStoreOptions { projection: Some(Projection::S2), ..Default::default() },
        );

        let face_0_tile = tile_store.get_tile(S2CellId::from_face(0)).unwrap();
        assert_eq!(face_0_tile.len(), 1);
        let default_layer = face_0_tile.layers.get("default").unwrap();
        assert_eq!(default_layer.features.len(), 1);

        assert_eq!(
            default_layer.features,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: None,
                face: 0.into(),
                properties: MValue::from([("a".into(), 1_u64.into())]),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    offset: None,
                    bbox: None,
                    vec_bbox: Some(BBox3D {
                        left: 0.5,
                        bottom: 0.5,
                        right: 0.5,
                        top: 0.5,
                        near: f64::MAX,
                        far: f64::MIN
                    }),
                    indices: None,
                    tessellation: None
                }),
                metadata: None
            }]
        );
    }

    #[test]
    fn test_tile_store_wg_lines() {
        let json_string = r#"{
            "type": "FeatureCollection",
            "features": [
                {
                    "type": "Feature",
                    "properties": {},
                    "geometry": {
                        "type": "LineString",
                        "coordinates": [
                            [-13.292352825505162, 54.34883408204476],
                            [36.83102287804303, 59.56941785818924],
                            [50.34083898563978, 16.040052775278994],
                            [76.38149901912357, 35.155968522292056]
                        ]
                    }
                },
                {
                    "type": "Feature",
                    "properties": {},
                    "geometry": {
                        "type": "MultiLineString3D",
                        "coordinates": [
                            [
                                [138.2192704758947, 53.37525605304839, -1.0],
                                [138.02907780308504, 45.48182328687463, 2.0],
                                [166.1775933788045, 52.68902110529311, 4.0],
                                [161.99335457700874, 40.765696887535825, -0.5]
                            ], [
                                [139.16452129458895, -69.38636090051318, 1.0],
                                [143.85299782010844, -63.55049044056966, 2.0],
                                [128.5373078367444, -51.22800042702269, -0.5],
                                [134.78860987076968, -45.63638565920266, 8.0]
                            ]
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
        assert_eq!(default_layer.features.len(), 2);

        // [], []], offset: None, bbox: None, vec_bbox: Some(BBox3D { left: 0.8570480773242899, bottom: 0.3240121995384903, right: 0.9616044260522347, top: 0.7712879476591746, near: -1.0, far: 8.0 }), indices: None, tessellation: None }), metadata: None }]

        assert_eq!(
            default_layer.features,
            vec![
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.4630767977069301,
                                y: 0.31942614957229354,
                                z: None,
                                m: None,
                                t: Some(1.),
                            },
                            VectorPoint {
                                x: 0.6023083968834528,
                                y: 0.29277635129241236,
                                z: None,
                                m: None,
                                t: Some(0.01120038734713082),
                            },
                            VectorPoint {
                                x: 0.6398356638489994,
                                y: 0.45485063470883236,
                                z: None,
                                m: None,
                                t: Some(0.00605876326361668)
                            },
                            VectorPoint {
                                x: 0.7121708306086766,
                                y: 0.3955684303719546,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.4630767977069301,
                            bottom: 0.29277635129241236,
                            right: 0.7121708306086766,
                            top: 0.45485063470883236,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "VectorFeature".into(),
                    id: None,
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                        _type: "MultiLineString".into(),
                        is_3d: true,
                        coordinates: vec![
                            vec![
                                VectorPoint {
                                    x: 0.8839424179885964,
                                    y: 0.3240121995384903,
                                    z: Some(-1.0),
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.8834141050085695,
                                    y: 0.3578242302600759,
                                    z: Some(2.0),
                                    m: None,
                                    t: Some(0.0011428082308213008)
                                },
                                VectorPoint {
                                    x: 0.9616044260522347,
                                    y: 0.32718207741863975,
                                    z: Some(4.0),
                                    m: None,
                                    t: Some(0.0020631440003536124)
                                },
                                VectorPoint {
                                    x: 0.9499815404916909,
                                    y: 0.37578687158091856,
                                    z: Some(-0.5),
                                    m: None,
                                    t: Some(1.0)
                                }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.8865681147071915,
                                    y: 0.7712879476591746,
                                    z: Some(1.0),
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.8995916606114123,
                                    y: 0.730480837159282,
                                    z: Some(2.0),
                                    m: None,
                                    t: Some(0.0005558708889643396)
                                },
                                VectorPoint {
                                    x: 0.8570480773242899,
                                    y: 0.6662313440317926,
                                    z: Some(-0.5),
                                    m: None,
                                    t: Some(0.0003800636747767906)
                                },
                                VectorPoint {
                                    x: 0.8744128051965825,
                                    y: 0.6427889614041957,
                                    z: Some(8.0),
                                    m: None,
                                    t: Some(1.0)
                                }
                            ]
                        ],
                        offset: None,
                        bbox: None,
                        vec_bbox: Some(BBox3D {
                            left: 0.8570480773242899,
                            bottom: 0.3240121995384903,
                            right: 0.9616044260522347,
                            top: 0.7712879476591746,
                            near: -1.0,
                            far: 8.0
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }
}
