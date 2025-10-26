use libm::{asin, cos, fmin, pow, sin, sqrt};
use s2json::{
    Feature, Geometry, GetXY, GetZ, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// Get the total euclidean distance of a line or lines
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
/// - `&[P]` where P implements [`GetXY`] and [`GetZ`]
///
/// And all specific geometries of the above enums
pub trait LengthOfLines {
    /// Get the total euclidean distance of a line or lines
    fn line_length(&self) -> f64;
}

// Feature and below

impl<P: GetXY + GetZ> LengthOfLines for &[P] {
    fn line_length(&self) -> f64 {
        let mut res = 0.;
        let mut prev: Option<&P> = None;
        for p in self.iter() {
            if let Some(prev) = prev {
                res += euclidean_distance(p, prev);
            }
            prev = Some(p);
        }
        res
    }
}

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
                res += euclidean_distance(p, prev);
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
                res += euclidean_distance(p, prev);
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

/// Assuming two points are on the surface of the earth as Lon-Lat coordinates, find the distance
/// between them using the haversine formula.  Returns the distance in degrees.
///
/// # Parameters
/// Both points require the [`GetXY`] trait to be implemented
/// - `a`: The first point
/// - `b`: The second point
///
/// # Returns
/// - `f64`: The distance between the two points
pub fn haversine_distance<P: GetXY, Q: GetXY>(a: &P, b: &Q) -> f64 {
    let lat1 = a.y().to_radians();
    let lat2 = b.y().to_radians();
    let lon1 = a.x().to_radians();
    let lon2 = b.x().to_radians();
    let dlat = sin(0.5 * (lat2 - lat1));
    let dlon = sin(0.5 * (lon2 - lon1));
    let x = dlat * dlat + dlon * dlon * cos(lat1) * cos(lat2);
    (2. * asin(sqrt(fmin(1., x)))).to_degrees()
}

/// Find the euclidean distance between two points
///
/// # Parameters
/// Both points require the [`GetXY`] trait to be implemented
/// - `a`: The first point
/// - `b`: The second point
///
/// # Returns
/// - `f64`: The distance between the two points
pub fn euclidean_distance<P: GetXY + GetZ, Q: GetXY + GetZ>(a: &P, b: &Q) -> f64 {
    if let Some(z1) = a.z()
        && let Some(z2) = b.z()
    {
        return sqrt(pow(b.x() - a.x(), 2.) + pow(b.y() - a.y(), 2.) + pow(z2 - z1, 2.));
    }
    sqrt(pow(b.x() - a.x(), 2.) + pow(b.y() - a.y(), 2.))
}
