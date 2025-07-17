use super::{IndexPoint, PointIndex, Tile};
use crate::{
    data_store::vector::{Vector, VectorStore},
    geometry::{K_AVG_ANGLE_SPAN, LonLat, S1ChordAngle, S2CellId, S2Point, convert},
    parsers::FeatureReader,
};
use alloc::{collections::BTreeMap, string::String};
use core::cell::RefCell;
use libm::{floor, log2};
use s2json::{
    Face, GetXY, JSONCollection, MValue, Projection, VectorFeature, VectorFeatureType,
    VectorGeometry, VectorGeometryType, VectorPoint, VectorPointGeometry,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Options for point clustering
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterOptions {
    /// projection to use
    pub projection: Option<Projection>,
    /// Name of the layer to build when requesting a tile
    pub layer_name: Option<String>,
    /// min zoom to generate clusters on
    pub minzoom: Option<u8>,
    /// max zoom level to cluster the points on
    pub maxzoom: Option<u8>,
    /// cluster radius in pixels. Assuming a tile size of 512x512, radius is a multiplier
    /// of a singular pixel "cell"'s average length
    pub radius: Option<f64>,
}

/// A cluster is a storage device to maintain groups of information in a cluster
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Cluster<M: Clone + Default = MValue> {
    /// The data of the cluster
    pub data: Option<M>,
    /// The number of points in the cluster
    pub count: usize,
    /// Whether the cluster has been visited. 0 for not visited, 1 for visited
    visited: u8,
}
impl<M: Clone + Default> Cluster<M> {
    /// Create a cluster given the value (cluster size) and data
    pub fn new(data: Option<M>, count: usize) -> Self {
        Cluster { data, count, visited: 0 }
    }
}

/// Used for storing features.
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClusterData {
    /// The number of points that were added into the cluster
    pub count: usize,
}

/// Compare two data items, return true to merge data
pub type ClusterDataComparitor<M> = fn(a: &Option<M>, b: &Option<M>) -> bool;

/// A cluster point
pub type ClusterPoint<M> = IndexPoint<Cluster<M>>;

/// A locally allocated Point Cluster Store
pub type LocalClusterStore<M = MValue> = PointCluster<M, Vector<S2CellId, ClusterPoint<M>>>;

/// # Point Cluster
///
/// ## Description
/// A cluster store to index points at each zoom level
#[derive(Debug)]
pub struct PointCluster<
    M: Clone + Default + Serialize + DeserializeOwned,
    S: VectorStore<S2CellId, ClusterPoint<M>> = Vector<S2CellId, ClusterPoint<M>>,
> {
    projection: Projection,
    layer_name: String,
    minzoom: u8,
    maxzoom: u8,
    radius: f64,
    grid_size: u32, // a default is a 512x512 pixel tile
    indexes: BTreeMap<u8, RefCell<PointIndex<Cluster<M>, S>>>, // zoom => index
}
impl<M: Clone + Default + Serialize + DeserializeOwned, S: VectorStore<S2CellId, ClusterPoint<M>>>
    PointCluster<M, S>
{
    /// Create a new point cluster
    pub fn new(
        options: Option<ClusterOptions>,
        max_zoom_store: Option<PointIndex<Cluster<M>, S>>,
    ) -> PointCluster<M, S> {
        let options = options.unwrap_or_default();
        let mut cluster_store = PointCluster {
            projection: options.projection.unwrap_or(Projection::S2),
            layer_name: options.layer_name.unwrap_or(String::from("default")),
            minzoom: options.minzoom.unwrap_or(0),
            maxzoom: options.maxzoom.unwrap_or(16),
            radius: options.radius.unwrap_or(40.0),
            grid_size: 512,
            indexes: BTreeMap::new(),
        };
        // one extra zoom incase its a cell search system (bottom zoom isn't clustered to a cell)
        for zoom in cluster_store.minzoom..=cluster_store.maxzoom + 1 {
            cluster_store
                .indexes
                .insert(zoom, PointIndex::new(None, Some(cluster_store.projection)).into());
        }
        // inject a store if provided
        if let Some(store) = max_zoom_store {
            cluster_store.indexes.insert(cluster_store.maxzoom + 1, store.into());
        }

        cluster_store
    }

    /// Add a point to the maxzoom index. The point is a Point3D
    pub fn insert(&mut self, id: S2CellId, point: S2Point, data: Option<M>) {
        let mut maxzoom_index = self.indexes.get(&(self.maxzoom + 1)).unwrap().borrow_mut();
        let cluster = Cluster::new(data, 1);
        maxzoom_index.insert(id, point, Some(cluster));
    }

    /// Add a lon-lat pair as with any shape
    pub fn insert_point<P: GetXY>(&mut self, point: P, data: Option<M>) {
        self.insert_vector_point(VectorPoint::new(point.x(), point.y(), None, data));
    }

    /// Add a lon-lat pair to the cluster
    pub fn insert_lon_lat(&mut self, mut ll: LonLat<M>) {
        self.insert_vector_point(ll.take());
    }

    /// Add a lon-lat pair from a vectorPoint to the cluster
    pub fn insert_vector_point(&mut self, ll: VectorPoint<M>) {
        self.insert_feature(JSONCollection::<(), M, M>::VectorFeature(VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                coordinates: ll,
                is_3d: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }

    /// Insert an STPoint to the index
    pub fn insert_face_st(&mut self, face: Face, s: f64, t: f64, data: M) {
        self.insert_feature(JSONCollection::<(), M, M>::VectorFeature(VectorFeature {
            _type: VectorFeatureType::S2Feature,
            face,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                coordinates: VectorPoint::new(s, t, None, Some(data)),
                is_3d: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }

    /// Add all points from a reader. It will try to use the M-value first, but if it doesn't exist
    /// it will use the feature properties data
    pub fn insert_reader<T: Clone, F: FeatureReader<T, M, M>>(&mut self, reader: &F) {
        for feature in reader.iter() {
            self.insert_feature(JSONCollection::<T, M, M>::VectorFeature(feature))
        }
    }

    /// Add a vector feature. It will try to use the M-value first, but if it doesn't exist
    /// it will use the feature properties data
    pub fn insert_feature<T: Clone>(&mut self, data: JSONCollection<T, M, M>) {
        let features = convert(self.projection, &data, Some(true), Some(true));
        for feature in features {
            match feature.geometry {
                VectorGeometry::Point(geometry) => {
                    let coordinates = geometry.coordinates;
                    self._insert_face_st(
                        feature.face.into(),
                        coordinates.x,
                        coordinates.y,
                        coordinates.m.or(Some(feature.properties)),
                    );
                }
                VectorGeometry::MultiPoint(geometry) => {
                    for point in geometry.coordinates {
                        self._insert_face_st(
                            feature.face.into(),
                            point.x,
                            point.y,
                            point.m.or(Some(feature.properties.clone())),
                        );
                    }
                }
                _ => {}
            }
        }
    }

    /// Build the clusters when done adding points
    pub fn build_clusters(&mut self, cmp_: Option<ClusterDataComparitor<M>>) {
        let cmp = cmp_.unwrap_or(|_a: &Option<M>, _b: &Option<M>| true);
        for zoom in (self.minzoom..=self.maxzoom).rev() {
            self.cluster_radius(zoom, &cmp);
        }
        // ensure minzoom is sorted:
        let mut minzoom_index = self.indexes.get(&self.minzoom).unwrap().borrow_mut();
        minzoom_index.sort();
    }

    /// Radial clustering
    fn cluster_radius(&mut self, zoom: u8, cmp: &ClusterDataComparitor<M>) {
        let radius = self.get_level_radius(zoom);
        let mut curr_index = self.indexes.get(&zoom).unwrap().borrow_mut();
        let mut query_index = self.indexes.get(&(zoom + 1)).unwrap().borrow_mut();
        query_index.sort();
        let len = query_index.len();
        for i in 0..len {
            let (_, point) = query_index.get_index_mut(i).unwrap();
            let cluster_data = &mut point.data;
            if cluster_data.visited == 1 {
                continue;
            }
            cluster_data.visited = 1;
            let point = point.clone();
            let cluster_data = &point.data;
            // setup a new weighted cluster point
            let mut new_num_points = cluster_data.count;
            let mut new_cluster_point: S2Point = point.s2point;
            new_cluster_point *= new_num_points as f64;
            // joining all points found within radius
            for (_, found_point) in query_index.s2_search_radius_mut(point.s2point, radius, None) {
                let found_data = &mut found_point.data;
                let count = found_data.count;
                // only add points that match or have not been visited already
                if !cmp(&cluster_data.data, &found_data.data) || found_data.visited == 1 {
                    continue;
                }
                found_data.visited = 1;
                // weighted add to new_cluster_point position
                let fp = found_point.s2point;
                new_cluster_point += fp * count as f64;
                new_num_points += count;
            }
            // finish position average
            new_cluster_point /= new_num_points as f64;
            new_cluster_point.normalize();
            // store the new cluster point
            curr_index.insert(
                (&new_cluster_point).into(),
                new_cluster_point,
                Some(Cluster::new(cluster_data.data.clone(), new_num_points)),
            );
        }
    }

    /// Given a tile ID, return the vector tile of all the clustered points
    pub fn get_tile(&mut self, id: S2CellId) -> Tile<(), M, ClusterData> {
        let mut tile: Tile<(), M, ClusterData> = Tile::new(id);
        let zoom = id.level();
        if zoom < self.minzoom {
            return tile;
        }
        let (min, max) = id.range();
        // get the index
        let index = self.indexes.get(&u8::min(zoom, self.maxzoom));
        if index.is_none() {
            return tile;
        }

        for (_, point) in index.unwrap().borrow().search_range(min, Some(max), None) {
            let (face, s, t) = point.s2point.to_face_st();
            let cluster = &point.data;
            tile.add_feature(
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    face: face.into(),
                    properties: cluster.data.clone().unwrap_or_default(),
                    geometry: VectorGeometry::new_point(
                        VectorPoint {
                            x: s,
                            y: t,
                            z: None,
                            m: Some(ClusterData { count: cluster.count }),
                            t: None,
                        },
                        None,
                    ),
                    ..Default::default()
                },
                Some(self.layer_name.clone()),
            );
        }
        // transform the geometry to be relative to the tile
        tile.transform(0., Some(self.maxzoom));

        tile
    }

    /// Insert an STPoint to the index
    fn _insert_face_st(&mut self, face: u8, s: f64, t: f64, data: Option<M>) {
        let point = S2Point::from_face_st(face, s, t);
        self.insert((&point).into(), point, data);
    }

    /// Get a S1ChordAngle relative to a tile zoom level
    fn get_level_radius(&self, zoom: u8) -> S1ChordAngle {
        let zoom: i32 = zoom as i32;
        let grid_size = self.grid_size as f64;

        // if the grid is 512 x 512, log2 is 9, meaning the quadtree must split 9 times to analyze
        // each individual pixel. Make sure we don't dive past 30 levels as that's the limit of the spec.
        let zoom_grid_cell_level = i32::min(zoom + (floor(log2(grid_size)) as i32), 30);
        // get the average angle span for all cells at the zoom's cell level
        // divide by 2 to get half length to act more like a radius instead of a diameter
        let angle_span = K_AVG_ANGLE_SPAN.get_value(zoom_grid_cell_level) / 2.;

        S1ChordAngle::from_length2(angle_span * self.radius)
    }
}
