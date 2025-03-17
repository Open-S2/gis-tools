use crate::{
    data_store::vector::{Vector, VectorStore},
    geometry::{convert, LonLat, S1ChordAngle, S2Cap, S2CellId, S2Point},
    readers::FeatureIterator,
};
use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use s2json::{
    Face, JSONCollection, MValue, MValueCompatible, Projection, VectorFeature, VectorFeatureType,
    VectorGeometry, VectorGeometryType, VectorPoint, VectorPointGeometry,
};
use serde::{de::DeserializeOwned, Serialize};

/// # Point Index
///
/// ## Description
/// An index of cells with radius queries
/// Assumes the data is compatible with S2JSON MValues with serde_json serialization
pub struct PointIndex<
    M: MValueCompatible + Serialize + DeserializeOwned = MValue,
    S: VectorStore<VectorPoint<M>> = Vector<M>,
> {
    store: S,
    unsorted: bool,
    projection: Projection,
    _marker: PhantomData<M>,
}
impl<M: MValueCompatible + Serialize + DeserializeOwned, S: VectorStore<VectorPoint<M>>>
    PointIndex<M, S>
{
    /// Create a new PointIndex
    pub fn new(store: Option<S>, projection: Option<Projection>) -> Self {
        let store = store.unwrap_or_else(|| S::new(None));
        let projection = projection.unwrap_or(Projection::S2);
        PointIndex { store, unsorted: false, projection, _marker: PhantomData }
    }

    /// Insert a cell with the point and its corresponding data to the index
    pub fn insert(&mut self, cell: S2CellId, point: VectorPoint<M>) {
        self.store.push(cell, point);
        self.unsorted = true;
    }

    /// Insert a point3D and its corresponding data to the index. NOTE: Assumes an S2 projection.
    /// This is a quick way to add points to the index, but it is recommended to use
    /// `insert_face_st`, `insert_lon_lat`, `insert_feature`, or `insert_reader`
    pub fn insert_point(&mut self, point: VectorPoint<M>) {
        self.insert((&point).into(), point);
    }

    /// Add a lon-lat pair to the cluster
    pub fn insert_lon_lat<T: Clone>(&mut self, mut ll: LonLat<M>) {
        self.insert_feature(JSONCollection::<T, M, M>::VectorFeature(VectorFeature {
            _type: VectorFeatureType::VectorFeature,
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                coordinates: ll.take(),
                is_3d: false,
                ..Default::default()
            }),
            ..Default::default()
        }));
    }

    /// Insert an STPoint to the index
    pub fn insert_face_st<T: Clone>(&mut self, face: Face, s: f64, t: f64, data: M) {
        self.insert_feature(JSONCollection::<T, M, M>::VectorFeature(VectorFeature {
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
    pub fn insert_reader<T: Clone, F: FeatureIterator<T, M, M>>(&mut self, reader: F) {
        for feature in reader {
            self.insert_feature(JSONCollection::<T, M, M>::VectorFeature(feature));
        }
    }

    /// Add a vector feature. It will try to use the M-value first, but if it doesn't exist
    /// it will use the feature properties data
    pub fn insert_feature<T: Clone>(&mut self, data: JSONCollection<T, M, M>) {
        let features = convert(self.projection, &data, None, None, Some(true));
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
        if !self.unsorted {
            return;
        }
        self.store.sort();
        self.unsorted = false;
    }

    /// Find the starting index of a search
    pub fn lower_bound(&mut self, id: S2CellId) -> usize {
        self.sort();
        // lower bound search
        let mut lo: usize = 0;
        let mut hi: usize = self.store.len();
        let mut mid: usize;

        while lo < hi {
            mid = (lo + hi) / 2;
            let (mid_cell, _) = self.store.get(mid).unwrap();
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
        &mut self,
        mut low: S2CellId,
        high: Option<S2CellId>,
        max_results: Option<usize>,
    ) -> Vec<(S2CellId, VectorPoint<M>)> {
        let max_results = max_results.unwrap_or(usize::MAX);
        self.sort();
        let mut res = vec![];
        let high = high.unwrap_or_else(|| {
            let (lo, hi) = low.range();
            low = lo;
            hi
        });
        let mut lo_idx = self.lower_bound(low);

        loop {
            if lo_idx >= self.store.len() {
                break;
            }
            let curr_lo = self.store.get(lo_idx).unwrap();
            if curr_lo.0 > high {
                break;
            }
            res.push(curr_lo.clone());
            if res.len() >= max_results {
                break;
            }
            lo_idx += 1;
        }

        res
    }

    /// TODO: Adjust the radius for the WM projection. Really not a massive issue though just adjust your calcuation for now
    /// Search for points within a given radius of a target point
    pub fn search_radius(
        &mut self,
        target: VectorPoint,
        radius: S1ChordAngle,
        max_results: Option<usize>,
    ) -> Vec<(S2CellId, VectorPoint<M>)> {
        let max_results = max_results.unwrap_or(usize::MAX);
        self.sort();
        let mut res = vec![];
        let target: S2Point = (&target).into();
        if radius < 0. {
            return res;
        }
        let cap = S2Cap::new(target, radius, ());
        for cell in cap.get_intersecting_cells() {
            // iterate each covering s2cell min-max range on store. check distance from found
            // store Cells to target and if within radius add to results
            let (min, max) = cell.range();
            for point in self.search_range(min, Some(max), Some(max_results)) {
                if S1ChordAngle::from_s2_points(&target, &(&point.1).into()) < radius {
                    res.push(point);
                }
                if res.len() >= max_results {
                    break;
                }
            }
        }

        res
    }

    /// Insert an STPoint to the index
    fn _insert_face_st(&mut self, face: u8, s: f64, t: f64, data: Option<M>) {
        let point = S2Point::from_face_st(face, s, t);
        let vp = VectorPoint::new(point.x, point.y, Some(point.z), data);
        self.insert(point.into(), vp);
    }
}
