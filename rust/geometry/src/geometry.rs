use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use alloc::fmt;
use alloc::vec::Vec;

use core::f64::consts::PI;

use libm::{atan, log, pow, sin, sinh, sqrt};

/// Importing necessary types (equivalent to importing from 'values')
use crate::values::*;
use crate::Face;

/// The axis to apply an operation to
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    /// X axis
    X = 0,
    /// Y axis
    Y = 1,
}

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire line or polygon
/// The order is (left, bottom, right, top)
/// If WM, then the projection is lon-lat
/// If S2, then the projection is s-t
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct BBox<T = f64> {
    /// left most longitude (WM) or S (S2)
    pub left: T,
    /// bottom most latitude (WM) or T (S2)
    pub bottom: T,
    /// right most longitude (WM) or T (S2)
    pub right: T,
    /// top most latitude (WM) or S (S2)
    pub top: T,
}
impl<T> BBox<T> {
    /// Creates a new BBox
    pub fn new(left: T, bottom: T, right: T, top: T) -> Self
    where
        T: Copy,
    {
        BBox { left, bottom, right, top }
    }

    /// Checks if a point is within the BBox
    pub fn point_overlap(&self, point: VectorPoint) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        point.x >= self.left.into()
            && point.x <= self.right.into()
            && point.y >= self.bottom.into()
            && point.y <= self.top.into()
    }

    /// Merges another bounding box with this one
    pub fn merge(&self, other: &BBox<T>) -> BBox<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        new_bbox.left = if new_bbox.left < other.left { new_bbox.left } else { other.left };
        new_bbox.bottom =
            if new_bbox.bottom < other.bottom { new_bbox.bottom } else { other.bottom };
        new_bbox.right = if new_bbox.right > other.right { new_bbox.right } else { other.right };
        new_bbox.top = if new_bbox.top > other.top { new_bbox.top } else { other.top };

        new_bbox
    }

    /// Checks if another bounding box overlaps with this one and returns the overlap
    pub fn overlap(&self, other: &BBox<T>) -> Option<BBox<T>>
    where
        T: PartialOrd + Copy,
    {
        if self.left > other.right
            || self.right < other.left
            || self.bottom > other.top
            || self.top < other.bottom
        {
            None
        } else {
            let left = if self.left > other.left { self.left } else { other.left };
            let bottom = if self.bottom > other.bottom { self.bottom } else { other.bottom };
            let right = if self.right < other.right { self.right } else { other.right };
            let top = if self.top < other.top { self.top } else { other.top };

            Some(BBox { left, bottom, right, top })
        }
    }

    /// Clips the bounding box along an axis
    pub fn clip(&self, axis: Axis, k1: T, k2: T) -> BBox<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        if axis == Axis::X {
            new_bbox.left = if new_bbox.left > k1 { new_bbox.left } else { k1 };
            new_bbox.right = if new_bbox.right < k2 { new_bbox.right } else { k2 };
        } else {
            new_bbox.bottom = if new_bbox.bottom > k1 { new_bbox.bottom } else { k1 };
            new_bbox.top = if new_bbox.top < k2 { new_bbox.top } else { k2 };
        }

        new_bbox
    }
}
impl BBox<f64> {
    /// Creates a new BBox from a point
    pub fn from_point(point: &VectorPoint) -> Self {
        BBox::new(point.x, point.y, point.x, point.y)
    }

    /// Extends the bounding box with a point
    pub fn extend_from_point(&mut self, point: &VectorPoint) {
        *self = self.merge(&BBox::from_point(point));
    }

    /// Creates a new BBox from zoom-uv coordinates
    pub fn from_uv_zoom(u: f64, v: f64, zoom: u8) -> Self {
        let division_factor = 2. / (1 << zoom) as f64;

        BBox {
            left: division_factor * u - 1.0,
            bottom: division_factor * v - 1.0,
            right: division_factor * (u + 1.0) - 1.0,
            top: division_factor * (v + 1.0) - 1.0,
        }
    }

    /// Creates a new BBox from zoom-st coordinates
    pub fn from_st_zoom(s: f64, t: f64, zoom: u8) -> Self {
        let division_factor = (2. / (1 << zoom) as f64) * 0.5;

        BBox {
            left: division_factor * s,
            bottom: division_factor * t,
            right: division_factor * (s + 1.),
            top: division_factor * (t + 1.),
        }
    }
}
impl<T> Serialize for BBox<T>
where
    T: Serialize + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(4)?;
        seq.serialize_element(&self.left)?;
        seq.serialize_element(&self.bottom)?;
        seq.serialize_element(&self.right)?;
        seq.serialize_element(&self.top)?;
        seq.end()
    }
}
impl<'de, T> Deserialize<'de> for BBox<T>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BBoxVisitor<T> {
            marker: core::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BBoxVisitor<T>
        where
            T: Deserialize<'de> + Copy,
        {
            type Value = BBox<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of four numbers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BBox<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(BBox { left, bottom, right, top })
            }
        }

        deserializer.deserialize_tuple(4, BBoxVisitor { marker: core::marker::PhantomData })
    }
}

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire 3D line or polygon
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct BBox3D<T = f64> {
    /// left most longitude (WM) or S (S2)
    pub left: T,
    /// bottom most latitude (WM) or T (S2)
    pub bottom: T,
    /// right most longitude (WM) or T (S2)
    pub right: T,
    /// top most latitude (WM) or S (S2)
    pub top: T,
    /// near most height (WM) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub near: T,
    /// far most height (WM) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub far: T,
}
impl<T> BBox3D<T> {
    /// Creates a new BBox3D
    pub fn new(left: T, bottom: T, right: T, top: T, near: T, far: T) -> Self
    where
        T: Copy,
    {
        BBox3D { left, bottom, right, top, near, far }
    }

    /// Checks if a point is within the BBox
    pub fn point_overlap(&self, point: VectorPoint) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        let z = point.z.unwrap_or_default();
        point.x >= self.left.into()
            && point.x <= self.right.into()
            && point.y >= self.bottom.into()
            && point.y <= self.top.into()
            && z >= self.near.into()
            && z <= self.far.into()
    }

    /// Merges another bounding box with this one
    pub fn merge(&self, other: &BBox3D<T>) -> BBox3D<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        new_bbox.left = if new_bbox.left < other.left { new_bbox.left } else { other.left };
        new_bbox.bottom =
            if new_bbox.bottom < other.bottom { new_bbox.bottom } else { other.bottom };
        new_bbox.right = if new_bbox.right > other.right { new_bbox.right } else { other.right };
        new_bbox.top = if new_bbox.top > other.top { new_bbox.top } else { other.top };
        new_bbox.near = if new_bbox.near < other.near { new_bbox.near } else { other.near };
        new_bbox.far = if new_bbox.far > other.far { new_bbox.far } else { other.far };

        new_bbox
    }

    /// Checks if another bounding box overlaps with this one and returns the overlap
    pub fn overlap(&self, other: &BBox3D<T>) -> Option<BBox3D<T>>
    where
        T: PartialOrd + Copy,
    {
        if self.left > other.right
            || self.right < other.left
            || self.bottom > other.top
            || self.top < other.bottom
            || self.near > other.far
            || self.far < other.near
        {
            None
        } else {
            let left = if self.left > other.left { self.left } else { other.left };
            let bottom = if self.bottom > other.bottom { self.bottom } else { other.bottom };
            let right = if self.right < other.right { self.right } else { other.right };
            let top = if self.top < other.top { self.top } else { other.top };

            let near = if self.near > other.near { self.near } else { other.near };
            let far = if self.far < other.far { self.far } else { other.far };

            Some(BBox3D { left, bottom, right, top, near, far })
        }
    }

    /// Clips the bounding box along an axis
    pub fn clip(&self, axis: Axis, k1: T, k2: T) -> BBox3D<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        if axis == Axis::X {
            new_bbox.left = if new_bbox.left > k1 { new_bbox.left } else { k1 };
            new_bbox.right = if new_bbox.right < k2 { new_bbox.right } else { k2 };
        } else {
            new_bbox.bottom = if new_bbox.bottom > k1 { new_bbox.bottom } else { k1 };
            new_bbox.top = if new_bbox.top < k2 { new_bbox.top } else { k2 };
        }

        new_bbox
    }
}
impl<T> Serialize for BBox3D<T>
where
    T: Serialize + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(6)?;
        seq.serialize_element(&self.left)?;
        seq.serialize_element(&self.bottom)?;
        seq.serialize_element(&self.right)?;
        seq.serialize_element(&self.top)?;
        seq.serialize_element(&self.near)?;
        seq.serialize_element(&self.far)?;
        seq.end()
    }
}
impl BBox3D<f64> {
    /// Creates a new BBox3D from a point
    pub fn from_point(point: &VectorPoint) -> Self {
        BBox3D::new(
            point.x,
            point.y,
            point.x,
            point.y,
            point.z.unwrap_or(0.),
            point.z.unwrap_or(1.),
        )
    }

    /// Creates a new BBox3D from a BBox
    pub fn from_bbox(bbox: &BBox) -> Self {
        BBox3D::new(bbox.left, bbox.bottom, bbox.right, bbox.top, 0., 1.)
    }

    /// Extends the bounding box with a point
    pub fn extend_from_point(&mut self, point: &VectorPoint) {
        *self = self.merge(&BBox3D::from_point(point));
    }

    /// Creates a new BBox3D from zoom-uv coordinates
    pub fn from_uv_zoom(u: f64, v: f64, zoom: u8) -> Self {
        let division_factor = 2. / (1 << zoom) as f64;

        BBox3D {
            left: division_factor * u - 1.0,
            bottom: division_factor * v - 1.0,
            right: division_factor * (u + 1.0) - 1.0,
            top: division_factor * (v + 1.0) - 1.0,
            near: 0.,
            far: 1.,
        }
    }

    /// Creates a new BBox from zoom-st coordinates
    pub fn from_st_zoom(s: f64, t: f64, zoom: u8) -> Self {
        let division_factor = (2. / (1 << zoom) as f64) * 0.5;

        BBox3D {
            left: division_factor * s,
            bottom: division_factor * t,
            right: division_factor * (s + 1.),
            top: division_factor * (t + 1.),
            near: 0.,
            far: 1.,
        }
    }
}
impl From<&BBox> for BBox3D<f64> {
    fn from(bbox: &BBox) -> Self {
        BBox3D::from_bbox(bbox)
    }
}
impl<'de, T> Deserialize<'de> for BBox3D<T>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BBox3DVisitor<T> {
            marker: core::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BBox3DVisitor<T>
        where
            T: Deserialize<'de> + Copy,
        {
            type Value = BBox3D<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of six numbers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BBox3D<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let near =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let far = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(5, &self))?;
                Ok(BBox3D { left, bottom, right, top, near, far })
            }
        }

        deserializer.deserialize_tuple(6, BBox3DVisitor { marker: core::marker::PhantomData })
    }
}

/// BBox or BBox3D
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum BBOX {
    /// 2D bounding box
    BBox(BBox),
    /// 3D bounding box
    BBox3D(BBox3D),
}
impl Default for BBOX {
    fn default() -> Self {
        BBOX::BBox(BBox::default())
    }
}

/// A Point in S2 Space with a Face
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct STPoint {
    /// The face of the point
    pub face: Face,
    /// The s coordinate
    pub s: f64,
    /// The t coordinate
    pub t: f64,
    /// The z coordinate
    pub z: Option<f64>,
    /// The m coordinate
    pub m: Option<MValue>,
}

/// Enum to represent specific geometry types as strings
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum GeometryType {
    /// Point
    Point,
    /// MultiPoint
    MultiPoint,
    /// LineString
    LineString,
    /// MultiLineString
    MultiLineString,
    /// Polygon
    Polygon,
    /// MultiPolygon
    MultiPolygon,
    /// 3D Point
    Point3D,
    /// 3D MultiPoint
    MultiPoint3D,
    /// 3D LineString
    LineString3D,
    /// 3D MultiLineString
    MultiLineString3D,
    /// 3D Polygon
    Polygon3D,
    /// 3D MultiPolygon
    MultiPolygon3D,
}
impl From<&str> for GeometryType {
    fn from(s: &str) -> Self {
        match s {
            "Point" => GeometryType::Point,
            "MultiPoint" => GeometryType::MultiPoint,
            "LineString" => GeometryType::LineString,
            "MultiLineString" => GeometryType::MultiLineString,
            "Polygon" => GeometryType::Polygon,
            "MultiPolygon" => GeometryType::MultiPolygon,
            "Point3D" => GeometryType::Point3D,
            "MultiPoint3D" => GeometryType::MultiPoint3D,
            "LineString3D" => GeometryType::LineString3D,
            "MultiLineString3D" => GeometryType::MultiLineString3D,
            "Polygon3D" => GeometryType::Polygon3D,
            "MultiPolygon3D" => GeometryType::MultiPolygon3D,
            _ => GeometryType::Point,
        }
    }
}

/// Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T
pub type Point = (f64, f64);
/// Definition of a MultiPoint
pub type MultiPoint = Vec<Point>;
/// Definition of a LineString
pub type LineString = Vec<Point>;
/// Definition of a MultiLineString
pub type MultiLineString = Vec<LineString>;
/// Definition of a Polygon
pub type Polygon = Vec<Vec<Point>>;
/// Definition of a MultiPolygon
pub type MultiPolygon = Vec<Polygon>;
/// Definition of a 3D Point. May represent WebMercator Lon-Lat or S2Geometry S-T with a z-value
pub type Point3D = (f64, f64, f64);
/// Definition of a 3D MultiPoint
pub type MultiPoint3D = Vec<Point3D>;
/// Definition of a 3D LineString
pub type LineString3D = Vec<Point3D>;
/// Definition of a 3D MultiLineString
pub type MultiLineString3D = Vec<LineString3D>;
/// Definition of a 3D Polygon
pub type Polygon3D = Vec<Vec<Point3D>>;
/// Definition of a 3D MultiPolygon
pub type MultiPolygon3D = Vec<Polygon3D>;

/// All possible geometry shapes
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Geometry {
    /// Point Shape
    Point(PointGeometry),
    /// MultiPoint Shape
    MultiPoint(MultiPointGeometry),
    /// LineString Shape
    LineString(LineStringGeometry),
    /// MultiLineString Shape
    MultiLineString(MultiLineStringGeometry),
    /// Polygon Shape
    Polygon(PolygonGeometry),
    /// MultiPolygon Shape
    MultiPolygon(MultiPolygonGeometry),
    /// Point3D Shape
    Point3D(Point3DGeometry),
    /// MultiPoint3D Shape
    MultiPoint3D(MultiPoint3DGeometry),
    /// LineString3D Shape
    LineString3D(LineString3DGeometry),
    /// MultiLineString3D Shape
    MultiLineString3D(MultiLineString3DGeometry),
    /// Polygon3D Shape
    Polygon3D(Polygon3DGeometry),
    /// MultiPolygon3D Shape
    MultiPolygon3D(MultiPolygon3DGeometry),
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct BaseGeometry<G = Geometry, M = MValues, B = BBOX> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: GeometryType,
    /// The geometry shape
    pub coordinates: G,
    /// The M-Values shape
    #[serde(rename = "mValues", skip_serializing_if = "Option::is_none")]
    pub m_values: Option<M>,
    /// The BBox shape
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<B>,
}

/// PointGeometry is a point
pub type PointGeometry = BaseGeometry<Point, MValue, BBox>;
/// MultiPointGeometry contains multiple points
pub type MultiPointGeometry = BaseGeometry<MultiPoint, LineStringMValues, BBox>;
/// LineStringGeometry is a line
pub type LineStringGeometry = BaseGeometry<LineString, LineStringMValues, BBox>;
/// MultiLineStringGeometry contains multiple lines
pub type MultiLineStringGeometry = BaseGeometry<MultiLineString, MultiLineStringMValues, BBox>;
/// PolygonGeometry is a polygon with potential holes
pub type PolygonGeometry = BaseGeometry<Polygon, PolygonMValues, BBox>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type MultiPolygonGeometry = BaseGeometry<MultiPolygon, MultiPolygonMValues, BBox>;
/// Point3DGeometry is a 3D point
pub type Point3DGeometry = BaseGeometry<Point3D, MValue, BBox3D>;
/// MultiPoint3DGeometry contains multiple 3D points
pub type MultiPoint3DGeometry = BaseGeometry<MultiPoint3D, LineStringMValues, BBox3D>;
/// LineString3DGeometry is a 3D line
pub type LineString3DGeometry = BaseGeometry<LineString3D, LineStringMValues, BBox3D>;
/// MultiLineString3DGeometry contains multiple 3D lines
pub type MultiLineString3DGeometry =
    BaseGeometry<MultiLineString3D, MultiLineStringMValues, BBox3D>;
/// Polygon3DGeometry is a 3D polygon with potential holes
pub type Polygon3DGeometry = BaseGeometry<Polygon3D, PolygonMValues, BBox3D>;
/// MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes
pub type MultiPolygon3DGeometry = BaseGeometry<MultiPolygon3D, MultiPolygonMValues, BBox3D>;

/// Enum to represent specific vector geometry types as strings
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Default)]
pub enum VectorGeometryType {
    /// Point
    #[default]
    Point,
    /// MultiPoint
    MultiPoint,
    /// LineString
    LineString,
    /// MultiLineString
    MultiLineString,
    /// Polygon
    Polygon,
    /// MultiPolygon
    MultiPolygon,
}
impl From<&str> for VectorGeometryType {
    fn from(s: &str) -> Self {
        match s {
            "Point" => VectorGeometryType::Point,
            "MultiPoint" => VectorGeometryType::MultiPoint,
            "LineString" => VectorGeometryType::LineString,
            "MultiLineString" => VectorGeometryType::MultiLineString,
            "Polygon" => VectorGeometryType::Polygon,
            "MultiPolygon" => VectorGeometryType::MultiPolygon,
            _ => VectorGeometryType::Point,
        }
    }
}

/// A Vector Point uses a structure for 2D or 3D points
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct VectorPoint {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate or "altitude". May be None
    pub z: Option<f64>,
    /// M-Value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m: Option<MValue>,
    /// T for tolerance. A tmp value used for simplification
    #[serde(skip)]
    pub t: Option<f64>,
}
impl VectorPoint {
    /// Create a new point
    pub fn new(x: f64, y: f64, z: Option<f64>) -> Self {
        Self { x, y, z, m: None, t: None }
    }

    /// Project the point into the 0->1 coordinate system
    pub fn project(&mut self, bbox: &mut Option<BBox3D>) {
        let y = self.y;
        let x = self.x;
        let sin = sin((y * PI) / 180.);
        let y2 = 0.5 - (0.25 * log((1. + sin) / (1. - sin))) / PI;
        self.x = x / 360. + 0.5;
        self.y = y2.clamp(0., 1.);

        match bbox {
            Some(bbox) => bbox.extend_from_point(self),
            None => *bbox = Some(BBox3D::from_point(self)),
        };
    }

    /// Unproject the point from the 0->1 coordinate system back to a lon-lat coordinate
    pub fn unproject(&mut self) {
        let lon = (self.x - 0.5) * 360.;
        let y2 = 0.5 - self.y;
        let lat = atan(sinh(PI * (y2 * 2.))).to_degrees();

        self.x = lon;
        self.y = lat;
    }

    /// Calculate the distance between two points
    pub fn distance(&self, other: &VectorPoint) -> f64 {
        sqrt(pow(other.x - self.x, 2.) + pow(other.y - self.y, 2.))
    }
}
impl From<&Point> for VectorPoint {
    fn from(p: &Point) -> Self {
        Self { x: p.0, y: p.1, z: None, m: None, t: None }
    }
}
impl From<&Point3D> for VectorPoint {
    fn from(p: &Point3D) -> Self {
        Self { x: p.0, y: p.1, z: Some(p.2), m: None, t: None }
    }
}
/// Definition of a Vector MultiPoint
pub type VectorMultiPoint = Vec<VectorPoint>;
/// Definition of a Vector LineString
pub type VectorLineString = Vec<VectorPoint>;
/// Definition of a Vector MultiLineString
pub type VectorMultiLineString = Vec<VectorLineString>;
/// Definition of a Vector Polygon
pub type VectorPolygon = Vec<VectorLineString>;
/// Definition of a Vector MultiPolygon
pub type VectorMultiPolygon = Vec<VectorPolygon>;

/// All possible geometry shapes
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum VectorGeometry {
    /// Point Shape
    Point(VectorPointGeometry),
    /// MultiPoint Shape
    MultiPoint(VectorMultiPointGeometry),
    /// LineString Shape
    LineString(VectorLineStringGeometry),
    /// MultiLineString Shape
    MultiLineString(VectorMultiLineStringGeometry),
    /// Polygon Shape
    Polygon(VectorPolygonGeometry),
    /// MultiPolygon Shape
    MultiPolygon(VectorMultiPolygonGeometry),
}
impl VectorGeometry {
    /// Get the vec_bbox of the geometry
    pub fn vec_bbox(&self) -> &Option<BBox3D> {
        match self {
            VectorGeometry::Point(g) => &g.vec_bbox,
            VectorGeometry::MultiPoint(g) => &g.vec_bbox,
            VectorGeometry::LineString(g) => &g.vec_bbox,
            VectorGeometry::MultiLineString(g) => &g.vec_bbox,
            VectorGeometry::Polygon(g) => &g.vec_bbox,
            VectorGeometry::MultiPolygon(g) => &g.vec_bbox,
        }
    }
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct VectorBaseGeometry<G = VectorGeometry, O = VectorOffsets> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: VectorGeometryType,
    /// Specifies if the geometry is 3D or 2D
    #[serde(rename = "is3D")]
    pub is_3d: bool,
    /// The geometry shape
    pub coordinates: G,
    /// The geometry offsets if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<O>,
    /// The BBox shape - always in lon-lat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox3D>,
    /// temporary bbox to track 0->1 clipping
    #[serde(skip)]
    pub vec_bbox: Option<BBox3D>,
    /// Polygon and MultiPolygon specific property
    pub indices: Option<Vec<u32>>,
    /// Polygon and MultiPolygon specific property
    pub tesselation: Option<f64>,
}

/** All possible geometry offsets */
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VectorOffsets {
    /// LineString offset
    LineOffset(VectorLineOffset),
    /// MultiLineString offset
    MultiLineOffset(VectorMultiLineOffset),
    /// Polygon offset
    PolygonOffset(VectorPolygonOffset),
    /// MultiPolygon offset
    MultiPolygonOffset(VectorMultiPolygonOffset),
}
/** An offset defines how far the starting line is from the original starting point pre-slice */
pub type VectorLineOffset = f64;
/** A collection of offsets */
pub type VectorMultiLineOffset = Vec<VectorLineOffset>;
/** A collection of offsets */
pub type VectorPolygonOffset = VectorMultiLineOffset;
/** A collection of collections of offsets */
pub type VectorMultiPolygonOffset = Vec<VectorPolygonOffset>;

/// PointGeometry is a point
pub type VectorPointGeometry = VectorBaseGeometry<VectorPoint>;
/// MultiPointGeometry contains multiple points
pub type VectorMultiPointGeometry = VectorBaseGeometry<VectorMultiPoint, VectorLineOffset>;
/// LineStringGeometry is a line
pub type VectorLineStringGeometry = VectorBaseGeometry<VectorLineString, VectorLineOffset>;
/// MultiLineStringGeometry contains multiple lines
pub type VectorMultiLineStringGeometry =
    VectorBaseGeometry<VectorMultiLineString, VectorMultiLineOffset>;
/// PolygonGeometry is a polygon with potential holes
pub type VectorPolygonGeometry = VectorBaseGeometry<VectorPolygon, VectorPolygonOffset>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type VectorMultiPolygonGeometry =
    VectorBaseGeometry<VectorMultiPolygon, VectorMultiPolygonOffset>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_bbox() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
        let str_bbox: BBox = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox::default();
        assert_eq!(default_bbox, BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 });

        let default_bbox_2 = BBOX::default();
        assert_eq!(
            default_bbox_2,
            BBOX::BBox(BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 })
        );
    }

    #[test]
    fn test_bbox_functions() {
        let bbox = BBox::new(0., 0., 1., 1.);
        assert!(bbox.point_overlap(VectorPoint::new(0.5, 0.5, None)));
        assert!(!bbox.point_overlap(VectorPoint::new(2.0, 2.0, None)));
        let bbox2 = BBox { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5 };
        assert_eq!(
            bbox.overlap(&bbox2),
            Some(BBox { left: 0.5, bottom: 0.5, right: 1.0, top: 1.0 })
        );
        let bbox3 = BBox { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0 };
        assert_eq!(bbox3.overlap(&bbox), None);
    }

    #[test]
    fn test_bbox_functions_2() {
        let bbox = BBox::from_st_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1., top: 1. });

        let bbox = BBox::from_st_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 1., top: 0.5 });

        let bbox = BBox::from_st_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 0.75, top: 0.25 });

        let bbox = BBox::from_uv_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: -1.0, bottom: -1.0, right: 1., top: 1. });

        let bbox = BBox::from_uv_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 1., top: 0. });

        let bbox = BBox::from_uv_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 0.5, top: -0.5 });
    }

    #[test]
    fn test_bbox3d() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 };
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 }
        );
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
        let str_bbox: BBox3D = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox3D::default();
        assert_eq!(
            default_bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0, near: 0.0, far: 0.0 }
        );
    }

    #[test]
    fn test_point_geometry() {
        let point = PointGeometry {
            _type: "Point".into(),
            coordinates: (0.0, 0.0),
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            point,
            PointGeometry {
                _type: "Point".into(),
                coordinates: (0.0, 0.0),
                m_values: None,
                bbox: None
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"type\":\"Point\",\"coordinates\":[0.0,0.0]}");
        let str_point: PointGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_point3d_geometry() {
        let point = Point3DGeometry {
            _type: "Point3D".into(),
            coordinates: (0.0, 0.0, 0.0),
            m_values: None,
            bbox: Some(BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.0,
                near: 0.0,
                far: 1.0,
            }),
        };
        assert_eq!(
            point,
            Point3DGeometry {
                _type: "Point3D".into(),
                coordinates: (0.0, 0.0, 0.0),
                m_values: None,
                bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 0.0,
                    right: 1.0,
                    top: 1.0,
                    near: 0.0,
                    far: 1.0
                })
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"type\":\"Point3D\",\"coordinates\":[0.0,0.0,0.0],\"bbox\":[0.0,0.0,1.0,1.0,0.0,1.0]}");
        let str_point: Point3DGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_line_string_geometry() {
        let line = LineStringGeometry {
            _type: "LineString".into(),
            coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(line_str, "{\"type\":\"LineString\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}");
        let str_line: LineStringGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_line_string3d_geometry() {
        let line = LineString3DGeometry {
            _type: "LineString3D".into(),
            coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineString3DGeometry {
                _type: "LineString3D".into(),
                coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(
            line_str,
            "{\"type\":\"LineString3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_line: LineString3DGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_multi_point_geometry() {
        let multi_point = MultiPointGeometry {
            _type: "MultiPoint".into(),
            coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}"
        );
        let str_multi_point: MultiPointGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_multi_point3d_geometry() {
        let multi_point = MultiPoint3DGeometry {
            _type: "MultiPoint3D".into(),
            coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPoint3DGeometry {
                _type: "MultiPoint3D".into(),
                coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_multi_point: MultiPoint3DGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_polygon_geometry() {
        let polygon = PolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            PolygonGeometry {
                _type: "Polygon".into(),
                coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(
            polygon_str,
            "{\"type\":\"Polygon\",\"coordinates\":[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]}"
        );
        let str_polygon: PolygonGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_polygon3d_geometry() {
        let polygon = Polygon3DGeometry {
            _type: "Polygon3D".into(),
            coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            Polygon3DGeometry {
                _type: "Polygon3D".into(),
                coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(polygon_str, "{\"type\":\"Polygon3D\",\"coordinates\":[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.0]]]}");
        let str_polygon: Polygon3DGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_multi_polygon_geometry() {
        let multi_polygon = MultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon\",\"coordinates\":[[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygonGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_multi_polygon3d_geometry() {
        let multi_polygon = MultiPolygon3DGeometry {
            _type: "MultiPolygon3D".into(),
            coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygon3DGeometry {
                _type: "MultiPolygon3D".into(),
                coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon3D\",\"coordinates\":[[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygon3DGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }
}
