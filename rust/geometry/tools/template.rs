use s2json::{
    Feature, Geometry, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// Get the average of a collection of [`VectorPoint`].
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
pub trait AverageOfPoints {
    /// Get the average of a collection of [`VectorPoint`]
    fn average_of_points(&self) -> VectorPoint;
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> AverageOfPoints for Feature<M, P, D> {
    fn average_of_points(&self) -> VectorPoint {
        self.geometry.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for Geometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        match self {
            Geometry::Point(g) => g.average_of_points(),
            Geometry::MultiPoint(g) => g.average_of_points(),
            Geometry::LineString(g) => g.average_of_points(),
            Geometry::MultiLineString(g) => g.average_of_points(),
            Geometry::Polygon(g) => g.average_of_points(),
            Geometry::MultiPolygon(g) => g.average_of_points(),
            Geometry::Point3D(g) => g.average_of_points(),
            Geometry::MultiPoint3D(g) => g.average_of_points(),
            Geometry::LineString3D(g) => g.average_of_points(),
            Geometry::MultiLineString3D(g) => g.average_of_points(),
            Geometry::Polygon3D(g) => g.average_of_points(),
            Geometry::MultiPolygon3D(g) => g.average_of_points(),
        }
    }
}
impl<M: Clone + Default> AverageOfPoints for PointGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiPointGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiLineStringGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiPolygonGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for Point3DGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiPoint3DGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiLineString3DGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for MultiPolygon3DGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}

// Feature Point types

impl AverageOfPoints for Point {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiPoint {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiLineString {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiPolygon {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for Point3D {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiPoint3D {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiLineString3D {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl AverageOfPoints for MultiPolygon3D {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> AverageOfPoints for VectorFeature<M, P, D> {
    fn average_of_points(&self) -> VectorPoint {
        self.geometry.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        match self {
            VectorGeometry::Point(g) => g.average_of_points(),
            VectorGeometry::MultiPoint(g) => g.average_of_points(),
            VectorGeometry::LineString(g) => g.average_of_points(),
            VectorGeometry::MultiLineString(g) => g.average_of_points(),
            VectorGeometry::Polygon(g) => g.average_of_points(),
            VectorGeometry::MultiPolygon(g) => g.average_of_points(),
        }
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorPointGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiPointGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiLineStringGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiPolygonGeometry<M> {
    fn average_of_points(&self) -> VectorPoint {
        self.coordinates.average_of_points()
    }
}

// Vector Point Types

impl<M: Clone + Default> AverageOfPoints for VectorPoint<M> {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiPoint<M> {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiLineString<M> {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
impl<M: Clone + Default> AverageOfPoints for VectorMultiPolygon<M> {
    fn average_of_points(&self) -> VectorPoint {
        todo!()
    }
}
