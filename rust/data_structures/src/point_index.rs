use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use data_store::vector::{Vector, VectorStore};
use geometry::{LonLat, S1ChordAngle, S2Cap, S2CellId, S2Point, convert};
use parsers::FeatureReader;
use s2json::{
    Face, GetM, GetXY, GetZ, JSONCollection, MValue, Projection, VectorFeature, VectorFeatureType,
    VectorGeometry, VectorPoint,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// Locally allocated Point Index
pub type LocalPointIndex<M = MValue> = PointIndex<M, Vector<S2CellId, IndexPoint<M>>>;

/// An Indexed Point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexPoint<M> {
    /// S2Point (3D vector)
    pub s2point: S2Point,
    /// Data associated with the point
    pub data: M,
}
impl<M> IndexPoint<M> {
    /// Create a new Indexed Point
    pub fn new(s2point: S2Point, data: M) -> Self {
        IndexPoint { s2point, data }
    }
}
impl<M> GetXY for IndexPoint<M> {
    fn x(&self) -> f64 {
        self.s2point.x()
    }
    fn y(&self) -> f64 {
        self.s2point.y()
    }
}
impl<M> GetZ for IndexPoint<M> {
    fn z(&self) -> Option<f64> {
        self.s2point.z()
    }
}
impl<M> GetM<M> for IndexPoint<M> {
    fn m(&self) -> Option<&M> {
        Some(&self.data)
    }
}

/// # Point Index
///
/// ## Description
/// An index of cells with radius queries
/// Assumes the data is compatible with S2JSON MValues with serde_json serialization
#[derive(Debug, Default)]
pub struct PointIndex<
    M: Clone + Default + Serialize + DeserializeOwned = MValue,
    S: VectorStore<S2CellId, IndexPoint<M>> = Vector<S2CellId, IndexPoint<M>>,
> {
    store: S,
    sorted: bool,
    projection: Projection,
    _marker: PhantomData<M>,
}
impl<M: Clone + Default + Serialize + DeserializeOwned, S: VectorStore<S2CellId, IndexPoint<M>>>
    PointIndex<M, S>
{
    /// Create a new PointIndex
    pub fn new(store: Option<S>, projection: Option<Projection>) -> Self {
        let store = store.unwrap_or_else(|| S::new(None));
        let projection = projection.unwrap_or(Projection::S2);
        PointIndex { store, sorted: true, projection, _marker: PhantomData }
    }

    /// Returns the number of points in the index
    pub fn len(&self) -> u64 {
        self.store.len()
    }

    /// Returns true if the index is empty
    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    /// Get a reference to the point store
    pub fn get_index(&self, index: u64) -> Option<&(S2CellId, IndexPoint<M>)> {
        self.store.get_index(index)
    }

    /// Get a mutable reference to the point store
    pub fn get_index_mut(&mut self, index: u64) -> Option<&mut (S2CellId, IndexPoint<M>)> {
        self.store.get_index_mut(index)
    }

    /// Insert a cell with the point and its corresponding data to the index
    pub fn insert(&mut self, cell: S2CellId, point: S2Point, data: Option<M>) {
        self.store.push(cell, IndexPoint::new(point, data.unwrap_or_default()));
        self.sorted = false;
    }

    /// Add a lon-lat pair as with any shape
    pub fn insert_point<T: GetXY>(&mut self, point: T, data: Option<M>) {
        self.insert_vector_point(VectorPoint::new(point.x(), point.y(), None, data));
    }

    /// Add a lon-lat pair to the cluster
    pub fn insert_lon_lat(&mut self, mut vp: LonLat<M>) {
        self.insert_vector_point(vp.take());
    }

    /// Add a lon-lat pair via a VectorPoint to the cluster
    pub fn insert_vector_point(&mut self, vp: VectorPoint<M>) {
        self.insert_feature(JSONCollection::<(), M, M>::VectorFeature(VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::new_point(vp, None),
            ..Default::default()
        }));
    }

    /// Insert an STPoint to the index
    pub fn insert_face_st(&mut self, face: Face, s: f64, t: f64, data: M) {
        self.insert_feature(JSONCollection::<(), M, M>::VectorFeature(VectorFeature {
            _type: VectorFeatureType::S2Feature,
            face,
            geometry: VectorGeometry::new_point(VectorPoint::new(s, t, None, Some(data)), None),
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

    /// Sort the index
    pub fn sort(&mut self) {
        if self.sorted {
            return;
        }
        self.store.sort();
        self.sorted = true;
    }

    /// Find the starting index of a search
    pub fn lower_bound(&self, id: S2CellId) -> u64 {
        assert!(self.sorted);
        // lower bound search
        let mut lo: u64 = 0;
        let mut hi: u64 = self.store.len();
        let mut mid: u64;

        while lo < hi {
            mid = (lo + hi) / 2;
            let (mid_cell, _) = self.store.get_index(mid).unwrap();
            if *mid_cell < id {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }

    /// Search for points given a range of low and high ids
    pub fn search_range(
        &self,
        mut low: S2CellId,
        high: Option<S2CellId>,
        max_results: Option<usize>,
    ) -> Vec<&(S2CellId, IndexPoint<M>)> {
        assert!(self.sorted);
        let max_results = max_results.unwrap_or(usize::MAX);
        let mut res = vec![];
        let high = high.unwrap_or_else(|| {
            let (lo, hi) = low.range();
            low = lo;
            hi
        });
        let mut lo_idx = self.lower_bound(low);

        let len = self.store.len();
        loop {
            if lo_idx >= len {
                break;
            }
            let curr_lo = self.store.get_index(lo_idx).unwrap();
            if curr_lo.0 > high {
                break;
            }
            res.push(curr_lo);
            if res.len() >= max_results {
                break;
            }
            lo_idx += 1;
        }

        res
    }

    /// Search for points given a range of low and high ids
    pub fn search_range_mut(
        &mut self,
        mut low: S2CellId,
        high: Option<S2CellId>,
        max_results: Option<usize>,
    ) -> Vec<&mut (S2CellId, IndexPoint<M>)> {
        // setup
        let max_results = max_results.unwrap_or(usize::MAX);
        self.sort();
        let high = high.unwrap_or_else(|| {
            let (lo, hi) = low.range();
            low = lo;
            hi
        });
        let mut lo_idx = self.lower_bound(low);

        let mut res = vec![];
        let len = self.store.len();
        while lo_idx < len {
            let entry_ptr: *mut (S2CellId, IndexPoint<M>) =
                self.store.get_index_mut(lo_idx).unwrap() as *mut _; // Convert to raw ptr
            if unsafe { (*entry_ptr).0 } > high {
                break;
            }
            // store
            unsafe {
                res.push(&mut *entry_ptr);
            }
            // if were at max results, break
            if res.len() >= max_results {
                break;
            }
            lo_idx += 1;
        }

        res
    }

    /// TODO: WG Proj case: is the radius correct?
    /// Search for points within a given radius of a target point
    pub fn search_radius<M2: Clone + Default>(
        &self,
        target: &LonLat<M2>,
        radius: S1ChordAngle,
        max_results: Option<usize>,
    ) -> Vec<&(S2CellId, IndexPoint<M>)> {
        self.s2_search_radius(self.adjust_lon_lat(target), radius, max_results)
    }

    /// Search for points within a given radius of a target point in S2 space
    pub fn s2_search_radius(
        &self,
        target: S2Point,
        radius: S1ChordAngle,
        max_results: Option<usize>,
    ) -> Vec<&(S2CellId, IndexPoint<M>)> {
        assert!(self.sorted);
        let max_results = max_results.unwrap_or(usize::MAX);
        if radius < 0. {
            return vec![];
        }
        let cap = S2Cap::new(target, radius, ());
        let mut res = vec![];
        for cell in cap.get_intersecting_cells() {
            // iterate each covering s2cell min-max range on store. check distance from found
            // store Cells to target and if within radius add to results
            let (min, max) = cell.range();
            for point in self.search_range(min, Some(max), Some(max_results)) {
                if S1ChordAngle::from_s2_points(&target, &(point.1.s2point)) < radius {
                    res.push(point);
                }
                if res.len() >= max_results {
                    break;
                }
            }
        }

        res
    }

    /// TODO: WG Proj case: is the radius correct?
    /// Search for points within a given radius of a target point
    pub fn search_radius_mut<M2: Clone + Default>(
        &mut self,
        target: &LonLat<M2>,
        radius: S1ChordAngle,
        max_results: Option<usize>,
    ) -> Vec<&mut (S2CellId, IndexPoint<M>)> {
        self.s2_search_radius_mut(self.adjust_lon_lat(target), radius, max_results)
    }

    /// Search for points within a given radius of a target point in S2 space
    pub fn s2_search_radius_mut(
        &mut self,
        target: S2Point,
        radius: S1ChordAngle,
        max_results: Option<usize>,
    ) -> Vec<&mut (S2CellId, IndexPoint<M>)> {
        let max_results = max_results.unwrap_or(usize::MAX);
        self.sort();
        if radius < 0. {
            return vec![];
        }
        // first pass is to store the raw pointers
        let cap = S2Cap::new(target, radius, ());
        let mut raw_ptrs = vec![];
        for cell in cap.get_intersecting_cells() {
            // iterate each covering s2cell min-max range on store. check distance from found
            // store Cells to target and if within radius add to results
            let (min, max) = cell.range();
            let search_results = self.search_range_mut(min, Some(max), Some(max_results));
            for point in search_results {
                let s2point = &point.1.s2point;
                let angle = S1ChordAngle::from_s2_points(&target, s2point);
                let ptr: *mut (S2CellId, IndexPoint<M>) = point as *mut _; // Convert to raw pointer
                if angle < radius {
                    raw_ptrs.push(ptr);
                }
                if raw_ptrs.len() >= max_results {
                    break;
                }
            }
        }

        // Second pass: Convert raw pointers back to &mut references safely
        let mut res = vec![];
        unsafe {
            for ptr in raw_ptrs {
                res.push(&mut *ptr);
            }
        }
        res
    }

    fn adjust_lon_lat<M2: Clone + Default>(&self, lonlat: &LonLat<M2>) -> S2Point {
        if self.projection == Projection::WG {
            let mut projected = lonlat.0.clone();
            projected.project(None);
            return S2Point::from_face_st(0, projected.x, projected.y);
        }
        lonlat.into()
    }

    /// Insert an STPoint to the index
    fn _insert_face_st(&mut self, face: u8, s: f64, t: f64, data: Option<M>) {
        let point = S2Point::from_face_st(face, s, t);
        self.insert((&point).into(), point, data);
    }

    /// Get a reference iterator over the point store
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (S2CellId, IndexPoint<M>)>
    where
        IndexPoint<M>: 'a,
    {
        self.store.iter()
    }

    /// Get a mutable iterator over the point store
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (S2CellId, IndexPoint<M>)>
    where
        IndexPoint<M>: 'a,
    {
        self.store.iter_mut()
    }
}
