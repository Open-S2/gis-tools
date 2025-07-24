use alloc::vec;
use s2json::{
    Feature, Geometry, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// Convert any geometry shape to a [`VectorMultiPoint`]
///
/// This trait is implemented for:
/// - [`Feature`]
/// - [`Geometry`]
/// - [`PointGeometry`]
/// - [`MultiPointGeometry`]
/// - [`MultiLineStringGeometry`]
/// - [`MultiPolygonGeometry`]
/// - [`Point3DGeometry`]
/// - [`MultiPoint3DGeometry`]
/// - [`MultiLineString3DGeometry`]
/// - [`MultiPolygon3DGeometry`]
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
pub trait ToPoints<M: Clone + Default> {
    /// Convert any geometry shape to a [`VectorMultiPoint`]
    fn to_points(&self) -> VectorMultiPoint<M>;
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> ToPoints<D> for Feature<M, P, D> {
    fn to_points(&self) -> VectorMultiPoint<D> {
        self.geometry.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for Geometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        match self {
            Geometry::Point(g) => g.to_points(),
            Geometry::MultiPoint(g) => g.to_points(),
            Geometry::LineString(g) => g.to_points(),
            Geometry::MultiLineString(g) => g.to_points(),
            Geometry::Polygon(g) => g.to_points(),
            Geometry::MultiPolygon(g) => g.to_points(),
            Geometry::Point3D(g) => g.to_points(),
            Geometry::MultiPoint3D(g) => g.to_points(),
            Geometry::LineString3D(g) => g.to_points(),
            Geometry::MultiLineString3D(g) => g.to_points(),
            Geometry::Polygon3D(g) => g.to_points(),
            Geometry::MultiPolygon3D(g) => g.to_points(),
        }
    }
}
impl<M: Clone + Default> ToPoints<M> for PointGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPointGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiLineStringGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPolygonGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for Point3DGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPoint3DGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiLineString3DGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPolygon3DGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}

// Feature Point types

impl<M: Clone + Default> ToPoints<M> for Point {
    fn to_points(&self) -> VectorMultiPoint<M> {
        vec![VectorPoint::from(self)]
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPoint {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for p in self {
            points.push(VectorPoint::from(p));
        }
        points
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiLineString {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for line in self {
            points.extend(line.to_points());
        }
        points
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPolygon {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for poly in self {
            points.extend(poly.to_points());
        }
        points
    }
}
impl<M: Clone + Default> ToPoints<M> for Point3D {
    fn to_points(&self) -> VectorMultiPoint<M> {
        vec![VectorPoint::from(self)]
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPoint3D {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for p in self {
            points.push(VectorPoint::from(p));
        }
        points
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiLineString3D {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for line in self {
            points.extend(line.to_points());
        }
        points
    }
}
impl<M: Clone + Default> ToPoints<M> for MultiPolygon3D {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];
        for poly in self {
            points.extend(poly.to_points());
        }
        points
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> ToPoints<D> for VectorFeature<M, P, D> {
    fn to_points(&self) -> VectorMultiPoint<D> {
        self.geometry.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        match self {
            VectorGeometry::Point(g) => g.to_points(),
            VectorGeometry::MultiPoint(g) => g.to_points(),
            VectorGeometry::LineString(g) => g.to_points(),
            VectorGeometry::MultiLineString(g) => g.to_points(),
            VectorGeometry::Polygon(g) => g.to_points(),
            VectorGeometry::MultiPolygon(g) => g.to_points(),
        }
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorPointGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiPointGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiLineStringGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiPolygonGeometry<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.coordinates.to_points()
    }
}

// Vector Point Types

impl<M: Clone + Default> ToPoints<M> for VectorPoint<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        vec![self.clone()]
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiPoint<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        self.clone()
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiLineString<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];

        for line in self {
            points.extend(line.to_points());
        }

        points
    }
}
impl<M: Clone + Default> ToPoints<M> for VectorMultiPolygon<M> {
    fn to_points(&self) -> VectorMultiPoint<M> {
        let mut points = vec![];

        for polygon in self {
            points.extend(polygon.to_points());
        }

        points
    }
}
