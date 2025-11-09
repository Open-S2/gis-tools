use s2json::{
    Feature, Features, Geometry, GetXY, GetZ, MultiLineString, MultiLineString3D,
    MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint, MultiPoint3D,
    MultiPoint3DGeometry, MultiPointGeometry, MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry,
    MultiPolygonGeometry, NewXY, NewXYZ, Point, Point3D, Point3DGeometry, PointGeometry,
    VectorFeature, VectorGeometry, VectorMultiLineString, VectorMultiLineStringGeometry,
    VectorMultiPoint, VectorMultiPointGeometry, VectorMultiPolygon, VectorMultiPolygonGeometry,
    VectorPoint, VectorPointGeometry,
};

/// Get the average of a collection of [`VectorPoint`].
///
/// This trait is implemented for:
/// - [`Feature`]
/// - [`Geometry`]
/// - [`PointGeometry`]
/// - [`MultiPointGeometry`]
/// - [`s2json::LineStringGeometry`]
/// - [`MultiLineStringGeometry`]
/// - [`MultiPolygonGeometry`]
/// - [`Point3DGeometry`]
/// - [`MultiPoint3DGeometry`]
/// - [`s2json::LineString3DGeometry`]
/// - [`MultiLineString3DGeometry`]
/// - [`MultiPolygon3DGeometry`]
/// - [`VectorFeature`]
/// - [`VectorGeometry`]
/// - [`VectorPointGeometry`]
/// - [`VectorMultiPointGeometry`]
/// - [`VectorPoint`]
/// - [`VectorMultiLineStringGeometry`]
/// - [`VectorMultiPolygonGeometry`]
/// - [`VectorMultiPoint`]
/// - [`VectorMultiLineString`]
/// - [`VectorMultiPolygon`]
/// - [`Features`]
/// - `&[P]` where P implements [`GetXY`] and [`GetZ`]
///
/// And all specific geometries of the above enums
pub trait AverageOfPoints<Q: NewXY> {
    /// Get the average of a collection of [`VectorPoint`]
    fn average_of_points(&self) -> Q;
}

impl<Q: NewXY + NewXYZ, P: GetXY + GetZ> AverageOfPoints<Q> for &[P] {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        for p in self.iter() {
            x += p.x();
            y += p.y();
            z += p.z().unwrap_or(0.);
        }
        let len = self.len() as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q>
    for Feature<M, P, D>
{
    fn average_of_points(&self) -> Q {
        self.geometry.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for Geometry<M> {
    fn average_of_points(&self) -> Q {
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
impl<M: Clone + Default, Q: NewXY> AverageOfPoints<Q> for PointGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY> AverageOfPoints<Q> for MultiPointGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY> AverageOfPoints<Q> for MultiLineStringGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY> AverageOfPoints<Q> for MultiPolygonGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for Point3DGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiPoint3DGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiLineString3DGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiPolygon3DGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}

// Feature Point types

impl<Q: NewXY> AverageOfPoints<Q> for Point {
    fn average_of_points(&self) -> Q {
        Q::new_xy(self.0, self.1)
    }
}
impl<Q: NewXY> AverageOfPoints<Q> for MultiPoint {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        for p in self {
            x += p.0;
            y += p.1;
        }
        let len = self.len() as f64;
        x /= len;
        y /= len;
        Q::new_xy(x, y)
    }
}
impl<Q: NewXY> AverageOfPoints<Q> for MultiLineString {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut total = 0;
        for line in self {
            for p in line {
                x += p.0;
                y += p.1;
            }
            total += line.len();
        }
        let len = total as f64;
        x /= len;
        y /= len;
        Q::new_xy(x, y)
    }
}
impl<Q: NewXY> AverageOfPoints<Q> for MultiPolygon {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut total = 0;
        for poly in self {
            for line in poly {
                for p in line {
                    x += p.0;
                    y += p.1;
                }
                total += line.len();
            }
        }
        let len = total as f64;
        x /= len;
        y /= len;
        Q::new_xy(x, y)
    }
}
impl<Q: NewXY + NewXYZ> AverageOfPoints<Q> for Point3D {
    fn average_of_points(&self) -> Q {
        Q::new_xyz(self.0, self.1, self.2)
    }
}
impl<Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiPoint3D {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        for p in self {
            x += p.0;
            y += p.1;
            z += p.2;
        }
        let len = self.len() as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}
impl<Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiLineString3D {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        let mut total = 0;
        for line in self {
            for p in line {
                x += p.0;
                y += p.1;
                z += p.2;
            }
            total += line.len();
        }
        let len = total as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}
impl<Q: NewXY + NewXYZ> AverageOfPoints<Q> for MultiPolygon3D {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        let mut total = 0;
        for poly in self {
            for line in poly {
                for p in line {
                    x += p.0;
                    y += p.1;
                    z += p.2;
                }
                total += line.len();
            }
        }
        let len = total as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q>
    for VectorFeature<M, P, D>
{
    fn average_of_points(&self) -> Q {
        self.geometry.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorGeometry<M> {
    fn average_of_points(&self) -> Q {
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
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorPointGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorMultiPointGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q>
    for VectorMultiLineStringGeometry<M>
{
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorMultiPolygonGeometry<M> {
    fn average_of_points(&self) -> Q {
        self.coordinates.average_of_points()
    }
}

// Vector Point Types

impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorPoint<M> {
    fn average_of_points(&self) -> Q {
        Q::new_xyz(self.x, self.y, self.z.unwrap_or_default())
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorMultiPoint<M> {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        for p in self {
            x += p.x;
            y += p.y;
            z += p.z.unwrap_or_default();
        }
        let len = self.len() as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorMultiLineString<M> {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        let mut total = 0;

        for line in self {
            for point in line {
                x += point.x;
                y += point.y;
                z += point.z.unwrap_or_default();
            }
            total += line.len();
        }

        let len = total as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}
impl<M: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q> for VectorMultiPolygon<M> {
    fn average_of_points(&self) -> Q {
        let mut x = 0.;
        let mut y = 0.;
        let mut z = 0.;
        let mut total = 0;

        for poly in self {
            for line in poly {
                for point in line {
                    x += point.x;
                    y += point.y;
                    z += point.z.unwrap_or_default();
                }
                total += line.len();
            }
        }

        let len = total as f64;
        x /= len;
        y /= len;
        z /= len;
        Q::new_xyz(x, y, z)
    }
}

// Features

impl<M, P: Clone + Default, D: Clone + Default, Q: NewXY + NewXYZ> AverageOfPoints<Q>
    for Features<M, P, D>
{
    fn average_of_points(&self) -> Q {
        match self {
            Features::Feature(f) => f.average_of_points(),
            Features::VectorFeature(f) => f.average_of_points(),
        }
    }
}
