use crate::space::EARTH_RADIUS;
use libm::sin;
use s2json::{
    Feature, Geometry, GetXY, MultiLineString, MultiLineString3D, MultiLineString3DGeometry,
    MultiLineStringGeometry, MultiPoint, MultiPoint3D, MultiPoint3DGeometry, MultiPointGeometry,
    MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry, MultiPolygonGeometry, Point, Point3D,
    Point3DGeometry, PointGeometry, VectorFeature, VectorGeometry, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
};

/// Get the area of the polygon. Lines return 0 if not closed. Other geometries return 0.
///
/// Assumes geometry is in Lon-Lat space
///
/// If no radius is provided, the Earth's radius is used
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
pub trait Area {
    /// Get the area of the polygon. Lines return 0 if not closed. Other geometries return 0.
    ///
    /// Assumes geometry is in Lon-Lat space
    ///
    /// If no radius is provided, the Earth's radius is used
    fn area(&self, radius: Option<f64>) -> f64;
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> Area for Feature<M, P, D> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.geometry.area(radius)
    }
}
impl<M: Clone + Default> Area for Geometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        match self {
            Geometry::Point(g) => g.area(radius),
            Geometry::MultiPoint(g) => g.area(radius),
            Geometry::LineString(g) => g.area(radius),
            Geometry::MultiLineString(g) => g.area(radius),
            Geometry::Polygon(g) => g.area(radius),
            Geometry::MultiPolygon(g) => g.area(radius),
            Geometry::Point3D(g) => g.area(radius),
            Geometry::MultiPoint3D(g) => g.area(radius),
            Geometry::LineString3D(g) => g.area(radius),
            Geometry::MultiLineString3D(g) => g.area(radius),
            Geometry::Polygon3D(g) => g.area(radius),
            Geometry::MultiPolygon3D(g) => g.area(radius),
        }
    }
}
impl<M: Clone + Default> Area for PointGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiPointGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiLineStringGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiPolygonGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for Point3DGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiPoint3DGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiLineString3DGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for MultiPolygon3DGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}

// Feature Point types

impl Area for Point {
    fn area(&self, _radius: Option<f64>) -> f64 {
        0.
    }
}
impl Area for MultiPoint {
    fn area(&self, radius: Option<f64>) -> f64 {
        if self.first() != self.last() {
            0.
        } else {
            ring_area(self, radius.unwrap_or(EARTH_RADIUS))
        }
    }
}
impl Area for MultiLineString {
    fn area(&self, radius: Option<f64>) -> f64 {
        // first line adds, all others subtract
        let mut total = 0.;
        for (i, line) in self.iter().enumerate() {
            if i == 0 {
                total += line.area(radius);
            } else {
                total -= line.area(radius);
            }
        }
        total
    }
}
impl Area for MultiPolygon {
    fn area(&self, radius: Option<f64>) -> f64 {
        let mut total = 0.;
        for poly in self {
            total += poly.area(radius);
        }
        total
    }
}
impl Area for Point3D {
    fn area(&self, _radius: Option<f64>) -> f64 {
        0.
    }
}
impl Area for MultiPoint3D {
    fn area(&self, radius: Option<f64>) -> f64 {
        if self.first() != self.last() {
            0.
        } else {
            ring_area(self, radius.unwrap_or(EARTH_RADIUS))
        }
    }
}
impl Area for MultiLineString3D {
    fn area(&self, radius: Option<f64>) -> f64 {
        // first line adds, all others subtract
        let mut total = 0.;
        for (i, line) in self.iter().enumerate() {
            if i == 0 {
                total += line.area(radius);
            } else {
                total -= line.area(radius);
            }
        }
        total
    }
}
impl Area for MultiPolygon3D {
    fn area(&self, radius: Option<f64>) -> f64 {
        let mut total = 0.;
        for poly in self {
            total += poly.area(radius);
        }
        total
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> Area for VectorFeature<M, P, D> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.geometry.area(radius)
    }
}
impl<M: Clone + Default> Area for VectorGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        match self {
            VectorGeometry::Point(g) => g.area(radius),
            VectorGeometry::MultiPoint(g) => g.area(radius),
            VectorGeometry::LineString(g) => g.area(radius),
            VectorGeometry::MultiLineString(g) => g.area(radius),
            VectorGeometry::Polygon(g) => g.area(radius),
            VectorGeometry::MultiPolygon(g) => g.area(radius),
        }
    }
}
impl<M: Clone + Default> Area for VectorPointGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for VectorMultiPointGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for VectorMultiLineStringGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}
impl<M: Clone + Default> Area for VectorMultiPolygonGeometry<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        self.coordinates.area(radius)
    }
}

// Vector Point Types

impl<M: Clone + Default> Area for VectorPoint<M> {
    fn area(&self, _radius: Option<f64>) -> f64 {
        0.
    }
}
impl<M: Clone + Default> Area for VectorMultiPoint<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        if self.first() != self.last() {
            0.
        } else {
            ring_area(self, radius.unwrap_or(EARTH_RADIUS))
        }
    }
}
impl<M: Clone + Default> Area for VectorMultiLineString<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        // first line adds, all others subtract
        let mut total = 0.;
        for (i, line) in self.iter().enumerate() {
            if i == 0 {
                total += line.area(radius);
            } else {
                total -= line.area(radius);
            }
        }
        total
    }
}
impl<M: Clone + Default> Area for VectorMultiPolygon<M> {
    fn area(&self, radius: Option<f64>) -> f64 {
        let mut total = 0.;
        for poly in self {
            total += poly.area(radius);
        }
        total
    }
}

/// Calculate the approximate area of the polygon were it projected onto the planet.
/// Note that this area will be positive if ring is oriented counter-clockwise,
/// otherwise it will be negative.
///
/// Reference:
/// Robert. G. Chamberlain and William H. Duquette, "Some Algorithms for Polygons on a Sphere",
/// JPL Publication 07-03, Jet Propulsion
/// Laboratory, Pasadena, CA, June 2007 `https://trs.jpl.nasa.gov/handle/2014/40409`
///
/// ## Parameters
/// - `coords`: ring Coordinates in lon-lat space
/// - `planetRadius`: the radius of the planet (Earth by default)
///
/// ## Returns
/// The approximate signed geodesic area of the polygon in square meters.
pub fn ring_area<P: GetXY>(coords: &[P], radius: f64) -> f64 {
    let coords_length = coords.len() - 1;
    let factor = (radius * radius) / 2.;

    if coords_length <= 2 {
        return 0.;
    }

    let mut total = 0.;
    let mut i = 0;
    while i < coords_length {
        let lower = &coords[i];
        let middle = &coords[if i + 1 == coords_length { 0 } else { i + 1 }];
        let upper = &coords[if i + 2 >= coords_length { (i + 2) % coords_length } else { i + 2 }];

        let lower_x = lower.x().to_radians();
        let middle_y = middle.y().to_radians();
        let upper_x = upper.x().to_radians();

        total += (upper_x - lower_x) * sin(middle_y);

        i += 1;
    }

    -(total * factor)
}
