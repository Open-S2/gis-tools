use s2json::{
    VectorFeature, VectorGeometry, VectorMultiLineString, VectorMultiLineStringGeometry,
    VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon, VectorMultiPolygonGeometry,
    VectorPoint, VectorPointGeometry,
};

/// Get the nearest of a collection of [`VectorPoint`].
///
/// This trait is implemented for:
/// - [`VectorFeature`]
/// - [`VectorGeometry`]
/// - [`VectorPointGeometry`]
/// - [`VectorMultiPointGeometry`]
/// - [`VectorMultiLineStringGeometry`]
/// - [`VectorMultiPolygonGeometry`]
/// - [`VectorMultiPoint`]
/// - [`VectorMultiLineString`]
/// - [`VectorMultiPolygon`]
///
/// And all specific geometries of the above enums
pub trait NearestPoint<M: Clone + Default> {
    /// Get the nearest of a collection of [`VectorPoint`]
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>>;
    /// Get the mutable nearest of a collection of [`VectorPoint`]
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>>;
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> NearestPoint<D> for VectorFeature<M, P, D> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<D>> {
        self.geometry.nearest_point(b)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<D>> {
        self.geometry.nearest_point_mut(b)
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorGeometry<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        match self {
            VectorGeometry::Point(g) => g.nearest_point(b),
            VectorGeometry::MultiPoint(g) => g.nearest_point(b),
            VectorGeometry::LineString(g) => g.nearest_point(b),
            VectorGeometry::MultiLineString(g) => g.nearest_point(b),
            VectorGeometry::Polygon(g) => g.nearest_point(b),
            VectorGeometry::MultiPolygon(g) => g.nearest_point(b),
        }
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        match self {
            VectorGeometry::Point(g) => g.nearest_point_mut(b),
            VectorGeometry::MultiPoint(g) => g.nearest_point_mut(b),
            VectorGeometry::LineString(g) => g.nearest_point_mut(b),
            VectorGeometry::MultiLineString(g) => g.nearest_point_mut(b),
            VectorGeometry::Polygon(g) => g.nearest_point_mut(b),
            VectorGeometry::MultiPolygon(g) => g.nearest_point_mut(b),
        }
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorPointGeometry<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        self.coordinates.nearest_point(b)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        self.coordinates.nearest_point_mut(b)
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiPointGeometry<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        self.coordinates.nearest_point(b)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        self.coordinates.nearest_point_mut(b)
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiLineStringGeometry<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        self.coordinates.nearest_point(b)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        self.coordinates.nearest_point_mut(b)
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiPolygonGeometry<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        self.coordinates.nearest_point(b)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        self.coordinates.nearest_point_mut(b)
    }
}

// Vector Point Types

impl<M: Clone + Default> NearestPoint<M> for VectorPoint<M> {
    fn nearest_point<M2: Clone + Default>(&self, _b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        Some(self)
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        _b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        Some(self)
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiPoint<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for p in self {
            let dist = p.distance(b);
            if dist < min_dist {
                min_dist = dist;
                res = Some(p);
            }
        }
        res
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for p in self {
            let dist = p.distance(b);
            if dist < min_dist {
                min_dist = dist;
                res = Some(p);
            }
        }
        res
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiLineString<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for line in self {
            for p in line {
                let dist = p.distance(b);
                if dist < min_dist {
                    min_dist = dist;
                    res = Some(p);
                }
            }
        }
        res
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for line in self {
            for p in line {
                let dist = p.distance(b);
                if dist < min_dist {
                    min_dist = dist;
                    res = Some(p);
                }
            }
        }
        res
    }
}
impl<M: Clone + Default> NearestPoint<M> for VectorMultiPolygon<M> {
    fn nearest_point<M2: Clone + Default>(&self, b: &VectorPoint<M2>) -> Option<&VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for poly in self {
            for line in poly {
                for p in line {
                    let dist = p.distance(b);
                    if dist < min_dist {
                        min_dist = dist;
                        res = Some(p);
                    }
                }
            }
        }
        res
    }
    fn nearest_point_mut<M2: Clone + Default>(
        &mut self,
        b: &VectorPoint<M2>,
    ) -> Option<&mut VectorPoint<M>> {
        let mut res = None;
        let mut min_dist = f64::MAX;
        for poly in self {
            for line in poly {
                for p in line {
                    let dist = p.distance(b);
                    if dist < min_dist {
                        min_dist = dist;
                        res = Some(p);
                    }
                }
            }
        }
        res
    }
}
