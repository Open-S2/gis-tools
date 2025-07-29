use alloc::vec;
use s2json::{
    Feature, Geometry, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorLineString,
    VectorMultiLineString, VectorMultiLineStringGeometry, VectorMultiPoint,
    VectorMultiPointGeometry, VectorMultiPolygon, VectorMultiPolygonGeometry, VectorPoint,
    VectorPointGeometry,
};

/// Given a Geometry, attempt to Return a VectorLineString.
///
/// Smaller geometries return an empty line
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
pub trait ToLines<M: Clone + Default> {
    /// Given a Geometry, attempt to Return a VectorLineString.
    ///
    /// Smaller geometries return an empty line
    fn to_lines(&self) -> VectorLineString<M>;
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> ToLines<D> for Feature<M, P, D> {
    fn to_lines(&self) -> VectorLineString<D> {
        self.geometry.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for Geometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        match self {
            Geometry::Point(g) => g.to_lines(),
            Geometry::MultiPoint(g) => g.to_lines(),
            Geometry::LineString(g) => g.to_lines(),
            Geometry::MultiLineString(g) => g.to_lines(),
            Geometry::Polygon(g) => g.to_lines(),
            Geometry::MultiPolygon(g) => g.to_lines(),
            Geometry::Point3D(g) => g.to_lines(),
            Geometry::MultiPoint3D(g) => g.to_lines(),
            Geometry::LineString3D(g) => g.to_lines(),
            Geometry::MultiLineString3D(g) => g.to_lines(),
            Geometry::Polygon3D(g) => g.to_lines(),
            Geometry::MultiPolygon3D(g) => g.to_lines(),
        }
    }
}
impl<M: Clone + Default> ToLines<M> for PointGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPointGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiLineStringGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPolygonGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for Point3DGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPoint3DGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiLineString3DGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPolygon3DGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}

// Feature Point types

impl<M: Clone + Default> ToLines<M> for Point {
    fn to_lines(&self) -> VectorLineString<M> {
        vec![]
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPoint {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().map(VectorPoint::from).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiLineString {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|l| l.to_lines()).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPolygon {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|p| p.to_lines()).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for Point3D {
    fn to_lines(&self) -> VectorLineString<M> {
        vec![]
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPoint3D {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().map(VectorPoint::from).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiLineString3D {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|l| l.to_lines()).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for MultiPolygon3D {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|p| p.to_lines()).collect()
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> ToLines<D> for VectorFeature<M, P, D> {
    fn to_lines(&self) -> VectorLineString<D> {
        self.geometry.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        match self {
            VectorGeometry::Point(g) => g.to_lines(),
            VectorGeometry::MultiPoint(g) => g.to_lines(),
            VectorGeometry::LineString(g) => g.to_lines(),
            VectorGeometry::MultiLineString(g) => g.to_lines(),
            VectorGeometry::Polygon(g) => g.to_lines(),
            VectorGeometry::MultiPolygon(g) => g.to_lines(),
        }
    }
}
impl<M: Clone + Default> ToLines<M> for VectorPointGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiPointGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiLineStringGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiPolygonGeometry<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.coordinates.to_lines()
    }
}

// Vector Point Types

impl<M: Clone + Default> ToLines<M> for VectorPoint<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        vec![]
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiPoint<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.clone()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiLineString<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|l| l.to_lines()).collect()
    }
}
impl<M: Clone + Default> ToLines<M> for VectorMultiPolygon<M> {
    fn to_lines(&self) -> VectorLineString<M> {
        self.iter().flat_map(|p| p.to_lines()).collect()
    }
}
