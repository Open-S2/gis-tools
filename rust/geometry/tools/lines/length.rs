use libm::{pow, sqrt};
use s2json::{
    Feature, Geometry, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// Get the total distance of a line or lines
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
pub trait LengthOfLines {
    /// Get the total distance of a line or lines
    fn line_length(&self) -> f64;
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> LengthOfLines for Feature<M, P, D> {
    fn line_length(&self) -> f64 {
        self.geometry.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for Geometry<M> {
    fn line_length(&self) -> f64 {
        match self {
            Geometry::Point(g) => g.line_length(),
            Geometry::MultiPoint(g) => g.line_length(),
            Geometry::LineString(g) => g.line_length(),
            Geometry::MultiLineString(g) => g.line_length(),
            Geometry::Polygon(g) => g.line_length(),
            Geometry::MultiPolygon(g) => g.line_length(),
            Geometry::Point3D(g) => g.line_length(),
            Geometry::MultiPoint3D(g) => g.line_length(),
            Geometry::LineString3D(g) => g.line_length(),
            Geometry::MultiLineString3D(g) => g.line_length(),
            Geometry::Polygon3D(g) => g.line_length(),
            Geometry::MultiPolygon3D(g) => g.line_length(),
        }
    }
}
impl<M: Clone + Default> LengthOfLines for PointGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiPointGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiLineStringGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiPolygonGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for Point3DGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiPoint3DGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiLineString3DGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for MultiPolygon3DGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}

// Feature Point types

impl LengthOfLines for Point {
    fn line_length(&self) -> f64 {
        0.
    }
}
impl LengthOfLines for MultiPoint {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        let mut prev: Option<&Point> = None;
        for p in self {
            if let Some(prev) = prev {
                res += sqrt(pow(p.0 - prev.0, 2.) + pow(p.1 - prev.1, 2.));
            }
            prev = Some(p);
        }
        res
    }
}
impl LengthOfLines for MultiLineString {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}
impl LengthOfLines for MultiPolygon {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}
impl LengthOfLines for Point3D {
    fn line_length(&self) -> f64 {
        0.
    }
}
impl LengthOfLines for MultiPoint3D {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        let mut prev: Option<&Point3D> = None;
        for p in self {
            if let Some(prev) = prev {
                res += sqrt(pow(p.0 - prev.0, 2.) + pow(p.1 - prev.1, 2.) + pow(p.2 - prev.2, 2.));
            }
            prev = Some(p);
        }
        res
    }
}
impl LengthOfLines for MultiLineString3D {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}
impl LengthOfLines for MultiPolygon3D {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> LengthOfLines for VectorFeature<M, P, D> {
    fn line_length(&self) -> f64 {
        self.geometry.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for VectorGeometry<M> {
    fn line_length(&self) -> f64 {
        match self {
            VectorGeometry::Point(g) => g.line_length(),
            VectorGeometry::MultiPoint(g) => g.line_length(),
            VectorGeometry::LineString(g) => g.line_length(),
            VectorGeometry::MultiLineString(g) => g.line_length(),
            VectorGeometry::Polygon(g) => g.line_length(),
            VectorGeometry::MultiPolygon(g) => g.line_length(),
        }
    }
}
impl<M: Clone + Default> LengthOfLines for VectorPointGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiPointGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiLineStringGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiPolygonGeometry<M> {
    fn line_length(&self) -> f64 {
        self.coordinates.line_length()
    }
}

// Vector Point Types

impl<M: Clone + Default> LengthOfLines for VectorPoint<M> {
    fn line_length(&self) -> f64 {
        0.
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiPoint<M> {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        let mut prev: Option<&VectorPoint<M>> = None;
        for p in self {
            if let Some(prev) = prev {
                res += prev.distance(p);
            }
            prev = Some(p);
        }
        res
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiLineString<M> {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}
impl<M: Clone + Default> LengthOfLines for VectorMultiPolygon<M> {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        for p in self {
            res += p.line_length();
        }
        res
    }
}
