use s2json::{
    BBox3D, Feature, Features, Geometry, GetXY, GetZ, MultiLineString, MultiLineString3D,
    MultiLineString3DGeometry, MultiLineStringGeometry, MultiPoint, MultiPoint3D,
    MultiPoint3DGeometry, MultiPointGeometry, MultiPolygon, MultiPolygon3D, MultiPolygon3DGeometry,
    MultiPolygonGeometry, Point, Point3D, Point3DGeometry, PointGeometry, VectorFeature,
    VectorGeometry, VectorMultiLineString, VectorMultiLineStringGeometry, VectorMultiPoint,
    VectorMultiPointGeometry, VectorMultiPolygon, VectorMultiPolygonGeometry, VectorPoint,
    VectorPointGeometry,
};

/// Get the center of a bounding box from a collection of [`VectorPoint`]
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
/// - [`s2json::VectorLineStringGeometry`]
/// - [`VectorMultiLineStringGeometry`]
/// - [`VectorMultiPolygonGeometry`]
/// - [`VectorMultiPoint`]
/// - [`VectorMultiLineString`]
/// - [`VectorMultiPolygon`]
/// - [`Features`]
/// - `&[P]` where P implements [`GetXY`] and [`GetZ`]
///
/// And all specific geometries of the above enums
pub trait CenterOfPoints {
    /// Get the center of a bounding box from a collection of [`VectorPoint`]
    fn center_of_points(&self) -> VectorPoint;
}

impl<P: GetXY + GetZ> CenterOfPoints for &[P] {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for p in self.iter() {
            bbox.extend_from_point(p)
        }
        bbox_center(bbox)
    }
}

// Feature and below

impl<M, P: Clone + Default, D: Clone + Default> CenterOfPoints for Feature<M, P, D> {
    fn center_of_points(&self) -> VectorPoint {
        self.geometry.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for Geometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        match self {
            Geometry::Point(g) => g.center_of_points(),
            Geometry::MultiPoint(g) => g.center_of_points(),
            Geometry::LineString(g) => g.center_of_points(),
            Geometry::MultiLineString(g) => g.center_of_points(),
            Geometry::Polygon(g) => g.center_of_points(),
            Geometry::MultiPolygon(g) => g.center_of_points(),
            Geometry::Point3D(g) => g.center_of_points(),
            Geometry::MultiPoint3D(g) => g.center_of_points(),
            Geometry::LineString3D(g) => g.center_of_points(),
            Geometry::MultiLineString3D(g) => g.center_of_points(),
            Geometry::Polygon3D(g) => g.center_of_points(),
            Geometry::MultiPolygon3D(g) => g.center_of_points(),
        }
    }
}
impl<M: Clone + Default> CenterOfPoints for PointGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiPointGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiLineStringGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiPolygonGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for Point3DGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiPoint3DGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiLineString3DGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for MultiPolygon3DGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}

// Feature Point types

impl CenterOfPoints for Point {
    fn center_of_points(&self) -> VectorPoint {
        VectorPoint::new_xy(self.0, self.1, None)
    }
}
impl CenterOfPoints for MultiPoint {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for p in self {
            bbox.extend_from_point(&VectorPoint::<()>::from(p))
        }
        bbox_center(bbox)
    }
}
impl CenterOfPoints for MultiLineString {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for line in self {
            for p in line {
                bbox.extend_from_point(&VectorPoint::<()>::from(p))
            }
        }
        bbox_center(bbox)
    }
}
impl CenterOfPoints for MultiPolygon {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for poly in self {
            for line in poly {
                for p in line {
                    bbox.extend_from_point(&VectorPoint::<()>::from(p))
                }
            }
        }
        bbox_center(bbox)
    }
}
impl CenterOfPoints for Point3D {
    fn center_of_points(&self) -> VectorPoint {
        VectorPoint::new_xyz(self.0, self.1, self.2, None)
    }
}
impl CenterOfPoints for MultiPoint3D {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for p in self {
            bbox.extend_from_point(&VectorPoint::<()>::from(p))
        }
        bbox_center(bbox)
    }
}
impl CenterOfPoints for MultiLineString3D {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for line in self {
            for p in line {
                bbox.extend_from_point(&VectorPoint::<()>::from(p))
            }
        }
        bbox_center(bbox)
    }
}
impl CenterOfPoints for MultiPolygon3D {
    fn center_of_points(&self) -> VectorPoint {
        let mut bbox = BBox3D::default();
        for poly in self {
            for line in poly {
                for p in line {
                    bbox.extend_from_point(&VectorPoint::<()>::from(p))
                }
            }
        }
        bbox_center(bbox)
    }
}

// Vector Feature and below

impl<M, P: Clone + Default, D: Clone + Default> CenterOfPoints for VectorFeature<M, P, D> {
    fn center_of_points(&self) -> VectorPoint {
        self.geometry.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        match self {
            VectorGeometry::Point(g) => g.center_of_points(),
            VectorGeometry::MultiPoint(g) => g.center_of_points(),
            VectorGeometry::LineString(g) => g.center_of_points(),
            VectorGeometry::MultiLineString(g) => g.center_of_points(),
            VectorGeometry::Polygon(g) => g.center_of_points(),
            VectorGeometry::MultiPolygon(g) => g.center_of_points(),
        }
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorPointGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiPointGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiLineStringGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiPolygonGeometry<M> {
    fn center_of_points(&self) -> VectorPoint {
        self.coordinates.center_of_points()
    }
}

// Vector Point Types

impl<M: Clone + Default> CenterOfPoints for VectorPoint<M> {
    fn center_of_points(&self) -> VectorPoint {
        VectorPoint::new(self.x, self.y, self.z, None)
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiPoint<M> {
    fn center_of_points(&self) -> VectorPoint {
        let bbox = BBox3D::from_linestring(self);
        bbox_center(bbox)
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiLineString<M> {
    fn center_of_points(&self) -> VectorPoint {
        let bbox = BBox3D::from_multi_linestring(self);
        bbox_center(bbox)
    }
}
impl<M: Clone + Default> CenterOfPoints for VectorMultiPolygon<M> {
    fn center_of_points(&self) -> VectorPoint {
        let bbox = BBox3D::from_multi_polygon(self);
        bbox_center(bbox)
    }
}

fn bbox_center(bbox: BBox3D) -> VectorPoint {
    if bbox.near != f64::MAX && bbox.far != f64::MIN {
        VectorPoint::new_xyz(
            (bbox.right + bbox.left) / 2.,
            (bbox.top + bbox.bottom) / 2.,
            (bbox.far + bbox.near) / 2.,
            None,
        )
    } else {
        VectorPoint::new_xy((bbox.right + bbox.left) / 2., (bbox.top + bbox.bottom) / 2., None)
    }
}

// Features

impl<M, P: Clone + Default, D: Clone + Default> CenterOfPoints for Features<M, P, D> {
    fn center_of_points(&self) -> VectorPoint {
        match self {
            Features::Feature(f) => f.center_of_points(),
            Features::VectorFeature(f) => f.center_of_points(),
        }
    }
}
