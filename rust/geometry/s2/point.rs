use super::{ST_TO_UV, face_uv_to_xyz};
use crate::geometry::{
    LonLat, S2CellId, face_uv_to_xyz_gl, lon_lat_to_xyz, lon_lat_to_xyz_gl, st_to_ij,
    xyz_to_face_st, xyz_to_face_uv, xyz_to_lon_lat,
};
use core::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};
use libm::{atan2, fabs, sqrt};
use s2json::{GetXY, GetXYZ, GetZ, NewXY, NewXYZ, SetXY, SetZ, VectorPoint};
use serde::{Deserialize, Serialize};

/// An S2Point represents a point on the unit sphere as a 3D vector. Usually
/// points are normalized to be unit length, but some methods do not require
/// this.  See util/math/vector.h for the methods available.  Among other
/// things, there are overloaded operators that make it convenient to write
/// arithmetic expressions (e.g. p1 + p2).
///
/// NOTE: asumes only f64 or greater is used.
///
/// Implements the [`GetXY`] and [`GetZ`] traits
///
/// ## Usage
///
/// Methods that are available:
/// - [`S2Point::new`]: Create a new S2Point
/// - [`S2Point::is_empty`]: Check if the S2Point is empty
/// - [`S2Point::face`]: Returns the S2 face assocated with this point
/// - [`S2Point::angle`]: Returns the angle between this point and another
/// - [`S2Point::cross`]: Get the cross product of two XYZ Points
/// - [`S2Point::to_lon_lat`]: Returns a LonLat representation of this point
/// - [`S2Point::to_face_uv`]:  Returns a Face-UV representation of this point
/// - [`S2Point::to_face_st`]:  Returns a Face-ST representation of this point
/// - [`S2Point::to_face_ij`]:  Returns a Face-IJ representation of this point
/// - [`S2Point::get_face`]: Returns the S2 face assocated with this point
/// - [`S2Point::dot`]: Get the dot product of two XYZ Points
/// - [`S2Point::abs`]: Returns the absolute value of the point
/// - [`S2Point::invert`]: Inverts the point
/// - [`S2Point::len`]: Returns the length of the point
/// - [`S2Point::norm`]: Returns the vector's squared norm.
/// - [`S2Point::norm2`]: The dot product of the point with itself
/// - [`S2Point::normalize`]: Normalizes the point
/// - [`S2Point::distance`]: return the distance from this point to the other point
/// - [`S2Point::largest_abs_component`]: Returns the largest absolute component of the point
/// - [`S2Point::intermediate`]: Returns the intermediate point between this and the other point
/// - [`S2Point::from_face_uv`]: Convert an Face-U-V coordinate to an S2Point using the left-hand-rule
/// - [`S2Point::from_face_st`]: Convert an Face-S-T coordinate to an S2Point using the left-hand-rule
/// - [`S2Point::from_face_uv_gl`]: Convert an Face-U-V coordinate to an S2Point using the right-hand-rule
/// - [`S2Point::from_face_st_gl`]: Convert an Face-S-T coordinate to an S2Point using the right-hand-rule
/// - [`S2Point::from_lon_lat`]: Convert a lon-lat coord to an XYZ Point using the left-hand-rule
/// - [`S2Point::from_lon_lat_gl`]: Convert a lon-lat coord to an XYZ Point using the right-hand-rule
#[derive(Debug, Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
#[repr(C)]
pub struct S2Point {
    /// The x component.
    pub x: f64,
    /// The y component.
    pub y: f64,
    /// The z component.
    pub z: f64,
}
impl GetXY for S2Point {
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
}
impl GetZ for S2Point {
    fn z(&self) -> Option<f64> {
        Some(self.z)
    }
}
impl NewXY for S2Point {
    fn new_xy(x: f64, y: f64) -> Self {
        S2Point { x, y, z: 0.0 }
    }
}
impl NewXYZ for S2Point {
    fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        S2Point { x, y, z }
    }
}
impl SetXY for S2Point {
    fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    fn set_y(&mut self, y: f64) {
        self.y = y;
    }
}
impl SetZ for S2Point {
    fn set_z(&mut self, z: f64) {
        self.z = z;
    }
}
impl S2Point {
    /// Creates a new S2Point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        S2Point { x, y, z }
    }

    /// Returns true if the point is the zero vector.
    pub fn is_empty(&self) -> bool {
        let zero = f64::default();
        self.x == zero && self.y == zero && self.z == zero
    }

    /// Returns the S2 face assocated with this point
    pub fn face(&self, f: u8) -> f64 {
        if f == 0 {
            self.x
        } else if f == 1 {
            self.y
        } else {
            self.z
        }
    }

    /// Returns the angle between "this" and v in radians, in the range [0, pi]. If
    /// either vector is zero-length, or nearly zero-length, the result will be
    /// zero, regardless of the other value.
    pub fn angle(&self, b: &Self) -> f64 {
        atan2(self.cross(b).norm(), self.dot(b))
    }

    /// Get the cross product of two XYZ Points
    pub fn cross(&self, b: &Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }

    /// Return a Lon-lat representation of this point
    pub fn to_lon_lat<P: NewXY>(&self) -> P {
        xyz_to_lon_lat(self)
    }

    /// Returns a Face-UV representation of this point
    pub fn to_face_uv(&self) -> (u8, f64, f64) {
        xyz_to_face_uv(self)
    }

    /// Returns a Face-ST representation of this point
    pub fn to_face_st(&self) -> (u8, f64, f64) {
        xyz_to_face_st(self)
    }

    /// Returns a Face-IJ representation of this point
    pub fn to_face_ij(&self, level: Option<u8>) -> (u8, u32, u32) {
        // Convert the given XYZ Point to Face-S-T coordinates.
        let (face, s, t) = self.to_face_st();

        // Convert the S-T coordinates to I-J coordinates using the STtoIJ function.
        let mut i = st_to_ij(s);
        let mut j = st_to_ij(t);

        // If a level is provided, shift the I-J coordinates to the right by (30 - level) bits.
        if let Some(level) = level {
            i >>= 30 - level;
            j >>= 30 - level;
        }

        // Return the Face-I-J coordinates.
        (face, i, j)
    }

    /// Returns the S2 face assocated with this point
    pub fn get_face(&self) -> u8 {
        xyz_to_face_uv(self).0
    }

    /// dot returns the standard dot product of v and ov.
    pub fn dot(&self, b: &Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    /// Returns the absolute value of the point.
    pub fn abs(&self) -> Self {
        Self::new(fabs(self.x), fabs(self.y), fabs(self.z))
    }

    /// Returns the inverse of the point
    pub fn invert(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    /// Returns the length of the point.
    pub fn len(&self) -> f64 {
        self.norm()
    }

    /// Returns the vector's squared norm.
    pub fn norm(&self) -> f64 {
        sqrt(self.norm2())
    }

    /// The dot product of the point with itself
    pub fn norm2(&self) -> f64 {
        self.dot(self)
    }

    /// Normalize this point to unit length.
    pub fn normalize(&mut self) {
        let len = self.len();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    /// return the distance from this point to the other point in radians
    pub fn distance(&self, b: &Self) -> f64 {
        let d = *self - *b;
        d.len()
    }

    /// Returns the largest absolute component of the point.
    pub fn largest_abs_component(&self) -> u8 {
        let tmp = self.abs();
        if tmp.x > tmp.y {
            if tmp.x > tmp.z { 0 } else { 2 }
        } else if tmp.y > tmp.z {
            1
        } else {
            2
        }
    }

    /// Returns the intermediate point between this and the other point.
    pub fn intermediate(&self, b: &Self, t: f64) -> Self {
        Self::new(
            self.x + ((b.x - self.x) * (1.0 - t)),
            self.y + ((b.y - self.y) * (1.0 - t)),
            self.z + ((b.z - self.z) * (1.0 - t)),
        )
    }

    /// Convert a u-v coordinate to an XYZ Point.
    pub fn from_face_uv(face: u8, u: f64, v: f64) -> Self {
        let mut p: S2Point = face_uv_to_xyz(face, u, v);
        p.normalize();
        p
    }

    /// Convert an s-t coordinate to an XYZ Point.
    pub fn from_face_st(face: u8, s: f64, t: f64) -> Self {
        let u = ST_TO_UV(s);
        let v = ST_TO_UV(t);
        Self::from_face_uv(face, u, v)
    }

    /// Convert a u-v coordinate to an XYZ Point using the right-hand-rule
    pub fn from_face_uv_gl(face: u8, u: f64, v: f64) -> Self {
        let mut p: S2Point = face_uv_to_xyz_gl(face, u, v);
        p.normalize();
        p
    }

    /// Convert an s-t coordinate to an XYZ Point using the right-hand-rule
    pub fn from_face_st_gl(face: u8, s: f64, t: f64) -> Self {
        let u = ST_TO_UV(s);
        let v = ST_TO_UV(t);
        Self::from_face_uv_gl(face, u, v)
    }

    /// Convert a lon-lat coord to an XYZ Point using the left-hand-rule
    pub fn from_lon_lat<P: GetXYZ + NewXYZ>(ll: &P) -> Self {
        let res = lon_lat_to_xyz(ll);
        Self { x: res.x(), y: res.y(), z: res.z().unwrap() }
    }

    /// Convert a lon-lat coord to an XYZ Point using the right-hand-rule
    pub fn from_lon_lat_gl<P: GetXYZ + NewXYZ>(ll: &P) -> Self {
        let res = lon_lat_to_xyz_gl(ll);
        Self { x: res.x(), y: res.y(), z: res.z().unwrap() }
    }
}
impl<M: Clone + Default> From<&LonLat<M>> for S2Point {
    fn from(lonlat: &LonLat<M>) -> Self {
        lonlat.to_point()
    }
}
impl<M: Clone + Default> From<&VectorPoint<M>> for S2Point {
    fn from(v: &VectorPoint<M>) -> Self {
        Self { x: v.x, y: v.y, z: v.z.unwrap_or(0.0) }
    }
}
impl From<S2CellId> for S2Point {
    fn from(cellid: S2CellId) -> Self {
        cellid.to_point()
    }
}
// Implementing the Add trait for S2Point
impl Add<S2Point> for S2Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        S2Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}
impl AddAssign<S2Point> for S2Point {
    fn add_assign(&mut self, other: S2Point) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}
impl Add<f64> for S2Point {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        S2Point { x: self.x + other, y: self.y + other, z: self.z + other }
    }
}
impl AddAssign<f64> for S2Point {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}
// Implementing the Sub trait for S2Point
impl Sub<S2Point> for S2Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        S2Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}
impl SubAssign<S2Point> for S2Point {
    fn sub_assign(&mut self, other: S2Point) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}
impl Sub<f64> for S2Point {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        S2Point { x: self.x - other, y: self.y - other, z: self.z - other }
    }
}
impl SubAssign<f64> for S2Point {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}
// Implementing the Neg trait for S2Point
impl Neg for S2Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        S2Point { x: -self.x, y: -self.y, z: -self.z }
    }
}
// Implementing the Div trait for S2Point
impl Div<S2Point> for S2Point {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        S2Point { x: self.x / other.x, y: self.y / other.y, z: self.z / other.z }
    }
}
impl DivAssign<S2Point> for S2Point {
    fn div_assign(&mut self, other: S2Point) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}
impl Div<f64> for S2Point {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        S2Point { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}
impl DivAssign<f64> for S2Point {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}
// Implementing the Mul trait for S2Point
impl Mul<S2Point> for S2Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        S2Point { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}
impl MulAssign<S2Point> for S2Point {
    fn mul_assign(&mut self, other: S2Point) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}
impl Mul<f64> for S2Point {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        S2Point { x: self.x * other, y: self.y * other, z: self.z * other }
    }
}
impl MulAssign<f64> for S2Point {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}
impl Rem<f64> for S2Point {
    type Output = Self;
    fn rem(self, other: f64) -> Self::Output {
        S2Point { x: self.x % other, y: self.y % other, z: self.z % other }
    }
}
impl RemAssign<f64> for S2Point {
    fn rem_assign(&mut self, other: f64) {
        self.x %= other;
        self.y %= other;
        self.z %= other;
    }
}
impl Eq for S2Point {}
impl Ord for S2Point {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap(), // Handle cases where `x` comparison is not equal
        }
        match self.y.partial_cmp(&other.y) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap(), // Handle cases where `y` comparison is not equal
        }
        match self.z.partial_cmp(&other.z) {
            Some(order) => order,
            None => Ordering::Equal, // This handles the NaN case safely
        }
    }
}
impl PartialOrd for S2Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
