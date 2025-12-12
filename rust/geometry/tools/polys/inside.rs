use crate::geometry::orient2d;
use alloc::vec::Vec;
use s2json::{
    Feature, Geometry, GetXY, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// The result of a point being inside, outside, or on the boundary
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InsideResult {
    /// The point is inside
    Inside,
    /// The point is outside
    Outside,
    /// The point is on the boundary
    Boundary,
}
impl InsideResult {
    /// Get the dominant result. Inside > Boundary > Outside
    ///
    /// Inside takes precedence over Boundary and Outside
    ///
    /// Boundary takes precedence over Outside
    pub fn get_dominant(&self, other: InsideResult) -> InsideResult {
        if *self == other {
            *self
        } else {
            match (self, other) {
                (InsideResult::Inside, _) => InsideResult::Inside,
                (_, InsideResult::Inside) => InsideResult::Inside,
                (InsideResult::Boundary, _) => InsideResult::Boundary,
                (_, InsideResult::Boundary) => InsideResult::Boundary,
                _ => InsideResult::Outside,
            }
        }
    }
}

/// Check if a point is inside a geometry.
///
/// If the geoemtry we check against is a Point, check if the points are equal
///
/// If the geometry we check against is a MultiPoint or LineString, treat as a polygon with no holes
///
/// If the geometry we check against is a Polygon or MultiPolygon, check if the point is inside
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
/// - `&Vec<Vec<P>>` or `&[Vec<P>]` where P implements [`GetXY`]
///
/// And all specific geometries of the above enums
pub trait Inside {
    /// Check if a point is inside a geometry.
    ///
    /// If the geoemtry we check against is a Point, check if the points are equal
    ///
    /// If the geometry we check against is a MultiPoint or LineString, treat as a polygon with no holes
    ///
    /// If the geometry we check against is a Polygon or MultiPolygon, check if the point is inside
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult;
}

// Feature and below

impl<P: GetXY> Inside for &Vec<Vec<P>> {
    fn inside<B: GetXY>(&self, b: &B) -> InsideResult {
        point_in_polygon(b, self)
    }
}
impl<P: GetXY> Inside for &[Vec<P>] {
    fn inside<B: GetXY>(&self, b: &B) -> InsideResult {
        point_in_polygon(b, self)
    }
}

impl<M, P: Clone + Default, D: Clone + Default> Inside for Feature<M, P, D> {
    fn inside<B: GetXY>(&self, b: &B) -> InsideResult {
        self.geometry.inside(b)
    }
}
impl<M: Clone + Default> Inside for Geometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        match self {
            Geometry::Point(g) => g.inside(b),
            Geometry::MultiPoint(g) => g.inside(b),
            Geometry::LineString(g) => g.inside(b),
            Geometry::MultiLineString(g) => g.inside(b),
            Geometry::Polygon(g) => g.inside(b),
            Geometry::MultiPolygon(g) => g.inside(b),
            Geometry::Point3D(g) => g.inside(b),
            Geometry::MultiPoint3D(g) => g.inside(b),
            Geometry::LineString3D(g) => g.inside(b),
            Geometry::MultiLineString3D(g) => g.inside(b),
            Geometry::Polygon3D(g) => g.inside(b),
            Geometry::MultiPolygon3D(g) => g.inside(b),
        }
    }
}
impl<M: Clone + Default> Inside for PointGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiPointGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiLineStringGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiPolygonGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for Point3DGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiPoint3DGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiLineString3DGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for MultiPolygon3DGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}

// Feature Point types

impl Inside for Point {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        if self.x() == b.x() && self.y() == b.y() {
            InsideResult::Inside
        } else {
            InsideResult::Outside
        }
    }
}
impl Inside for MultiPoint {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polyline(b, self)
    }
}
impl Inside for MultiLineString {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polygon(b, self)
    }
}
impl Inside for MultiPolygon {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        let mut res = InsideResult::Outside;
        for poly in self {
            res = res.get_dominant(poly.inside(b));
        }
        res
    }
}
impl Inside for Point3D {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        if self.x() == b.x() && self.y() == b.y() {
            InsideResult::Inside
        } else {
            InsideResult::Outside
        }
    }
}
impl Inside for MultiPoint3D {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polyline(b, self)
    }
}
impl Inside for MultiLineString3D {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polygon(b, self)
    }
}
impl Inside for MultiPolygon3D {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        let mut res = InsideResult::Outside;
        for poly in self {
            res = res.get_dominant(poly.inside(b));
        }
        res
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> Inside for VectorFeature<M, P, D> {
    fn inside<B: GetXY>(&self, b: &B) -> InsideResult {
        self.geometry.inside(b)
    }
}
impl<M: Clone + Default> Inside for VectorGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        match self {
            VectorGeometry::Point(g) => g.inside(b),
            VectorGeometry::MultiPoint(g) => g.inside(b),
            VectorGeometry::LineString(g) => g.inside(b),
            VectorGeometry::MultiLineString(g) => g.inside(b),
            VectorGeometry::Polygon(g) => g.inside(b),
            VectorGeometry::MultiPolygon(g) => g.inside(b),
        }
    }
}
impl<M: Clone + Default> Inside for VectorPointGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for VectorMultiPointGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for VectorMultiLineStringGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}
impl<M: Clone + Default> Inside for VectorMultiPolygonGeometry<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        self.coordinates.inside(b)
    }
}

// Vector Point Types

impl<M: Clone + Default> Inside for VectorPoint<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        if self.x() == b.x() && self.y() == b.y() {
            InsideResult::Inside
        } else {
            InsideResult::Outside
        }
    }
}
impl<M: Clone + Default> Inside for VectorMultiPoint<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polyline(b, self)
    }
}
impl<M: Clone + Default> Inside for VectorMultiLineString<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        point_in_polygon(b, self)
    }
}
impl<M: Clone + Default> Inside for VectorMultiPolygon<M> {
    fn inside<P: GetXY>(&self, b: &P) -> InsideResult {
        let mut res = InsideResult::Outside;
        for poly in self {
            res = res.get_dominant(poly.inside(b));
        }
        res
    }
}

/// A Robust point in polygon test
///
/// ## Parameters
/// - `point`: the point
/// - `polygon`: the polygon
///
/// ## Returns
/// true if the point is in the polygon, 0 if on the boundary, false otherwise
pub fn point_in_polygon<P1: GetXY, P2: GetXY>(point: &P1, polygon: &[Vec<P2>]) -> InsideResult {
    let mut k = 0;
    let mut f;
    let mut u1;
    let mut v1;
    let mut u2;
    let mut v2;
    let mut current_p: &P2;
    let mut next_p: &P2;

    let x = point.x();
    let y = point.y();
    for contour in polygon {
        let contour_len = contour.len() - 1;

        current_p = &contour[0];
        if current_p.x() != contour[contour_len].x() && current_p.y() != contour[contour_len].y() {
            // since the first and last coordinates in a ring are not the same, assume it's not a polygon and return false
            return InsideResult::Outside;
        }

        u1 = current_p.x() - x;
        v1 = current_p.y() - y;

        for ii in 0..contour_len {
            next_p = &contour[ii + 1];

            u2 = next_p.x() - x;
            v2 = next_p.y() - y;

            if v1 == 0. && v2 == 0. {
                if (u2 <= 0. && u1 >= 0.) || (u1 <= 0. && u2 >= 0.) {
                    return InsideResult::Boundary;
                }
            } else if (v2 >= 0. && v1 <= 0.) || (v2 <= 0. && v1 >= 0.) {
                f = orient2d(u1, u2, v1, v2, 0., 0.);
                if f == 0. {
                    return InsideResult::Boundary;
                }
                if (f > 0. && v2 > 0. && v1 <= 0.) || (f < 0. && v2 <= 0. && v1 > 0.) {
                    k += 1;
                }
            }
            // current_p = next_p;
            v1 = v2;
            u1 = u2;
        }
    }

    if k % 2 == 0 { InsideResult::Outside } else { InsideResult::Inside }
}

/// A Robust point in polygon test
///
/// ## Parameters
/// - `point`: the point
/// - `polygon`: the polygon
///
/// ## Returns
/// true if the point is in the polygon, 0 if on the boundary, false otherwise
pub fn point_in_polyline<P1: GetXY, P2: GetXY>(point: &P1, contour: &[P2]) -> InsideResult {
    let mut k = 0;
    let mut f;
    let mut u1;
    let mut v1;
    let mut u2;
    let mut v2;
    let mut next_p: &P2;

    let x = point.x();
    let y = point.y();
    let contour_len = contour.len() - 1;

    let current_p = &contour[0];
    if current_p.x() != contour[contour_len].x() && current_p.y() != contour[contour_len].y() {
        // since the first and last coordinates in a ring are not the same, assume it's not a polygon and return false
        return InsideResult::Outside;
    }

    u1 = current_p.x() - x;
    v1 = current_p.y() - y;

    for ii in 0..contour_len {
        next_p = &contour[ii + 1];

        u2 = next_p.x() - x;
        v2 = next_p.y() - y;

        if v1 == 0. && v2 == 0. {
            if (u2 <= 0. && u1 >= 0.) || (u1 <= 0. && u2 >= 0.) {
                return InsideResult::Boundary;
            }
        } else if (v2 >= 0. && v1 <= 0.) || (v2 <= 0. && v1 >= 0.) {
            f = orient2d(u1, u2, v1, v2, 0., 0.);
            if f == 0. {
                return InsideResult::Boundary;
            }
            if (f > 0. && v2 > 0. && v1 <= 0.) || (f < 0. && v2 <= 0. && v1 > 0.) {
                k += 1;
            }
        }
        // current_p = next_p;
        v1 = v2;
        u1 = u2;
    }

    if k % 2 == 0 { InsideResult::Outside } else { InsideResult::Inside }
}

/// Check if a polyline/hole is inside another polyline/outer ring
///
/// ## Parameters
/// - `hole`: the hole to test if inside the outer
/// - `outer`: the outer to test against
///
/// ## Returns
/// true if the hole is inside the outer
pub fn polyline_in_polyline<P: GetXY>(hole: &[P], outer: &[P]) -> bool {
    for point in hole {
        match point_in_polyline(point, outer) {
            InsideResult::Inside => return true,
            InsideResult::Outside => return false,
            InsideResult::Boundary => continue,
        }
    }
    // If we make it means all points of the hole were on the boundary therefore its inside the outer
    true
}
