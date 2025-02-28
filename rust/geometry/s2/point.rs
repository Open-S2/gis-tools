use core::cmp::Ordering;
use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Neg, Rem, RemAssign, Sub};

use libm::{atan2, fabs, sqrt};
use s2json::VectorPoint;

use crate::geometry::{xyz_to_face_st, xyz_to_face_uv, LonLat, S2CellId};

/// An S2Point represents a point on the unit sphere as a 3D vector. Usually
/// points are normalized to be unit length, but some methods do not require
/// this.  See util/math/vector.h for the methods available.  Among other
/// things, there are overloaded operators that make it convenient to write
/// arithmetic expressions (e.g. (1-x)*p1 + x*p2).
/// NOTE: asumes only f64 or greater is used.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
#[repr(C)]
pub struct S2Point {
    /// The x component.
    pub x: f64,
    /// The y component.
    pub y: f64,
    /// The z component.
    pub z: f64,
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

    /// Get the corss product of two XYZ Points
    pub fn cross(&self, b: &Self) -> Self {
        Self::new(
            self.y * b.z - self.z * b.y,
            self.z * b.x - self.x * b.z,
            self.x * b.y - self.y * b.x,
        )
    }

    /// Returns a Face-ST representation of this point
    pub fn to_face_st(&self) -> (u8, f64, f64) {
        xyz_to_face_st(self)
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

    /// norm returns the vector's norm.
    pub fn norm(&self) -> f64 {
        sqrt(self.norm2())
    }

    /// norm2 returns the vector's squared norm.
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
            if tmp.x > tmp.z {
                0
            } else {
                2
            }
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
}
impl From<&LonLat> for S2Point {
    fn from(lonlat: &LonLat) -> Self {
        lonlat.to_point()
    }
}
impl From<&VectorPoint> for S2Point {
    fn from(v: &VectorPoint) -> Self {
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
impl Add<f64> for S2Point {
    type Output = Self;
    fn add(self, other: f64) -> Self::Output {
        S2Point { x: self.x + other, y: self.y + other, z: self.z + other }
    }
}
// Implementing the Sub trait for S2Point
impl Sub<S2Point> for S2Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        S2Point { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}
impl Sub<f64> for S2Point {
    type Output = Self;
    fn sub(self, other: f64) -> Self::Output {
        S2Point { x: self.x - other, y: self.y - other, z: self.z - other }
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
impl Div<f64> for S2Point {
    type Output = Self;
    fn div(self, other: f64) -> Self::Output {
        S2Point { x: self.x / other, y: self.y / other, z: self.z / other }
    }
}
// Implementing the Mul trait for S2Point
impl Mul<S2Point> for S2Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        S2Point { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}
impl Mul<f64> for S2Point {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        S2Point { x: self.x * other, y: self.y * other, z: self.z * other }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let point = S2Point { x: 1.0, y: 2.0, z: 3.0 };
        assert!(!point.is_empty());
        assert!(S2Point::new(0.0, 0.0, 0.0).is_empty());
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn angle() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.angle(&point2), 1.5707963267948966);
    }

    #[test]
    fn cross() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.cross(&point2), S2Point { x: 0.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn to_face_st() {
        let point = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point.to_face_st(), (1, 0.5, 0.5));
    }

    #[test]
    fn get_face() {
        let point = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point.get_face(), 1);
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn distance() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.distance(&point2), 1.4142135623730951);
    }

    #[test]
    fn intermediate() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.intermediate(&point2, 0.5), S2Point { x: 0.5, y: 0.5, z: 0.0 });
    }

    #[test]
    fn add() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 + point2, S2Point { x: 1.0, y: 1.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 + f, S2Point { x: 1.5, y: 0.5, z: 0.5 });
    }

    #[test]
    fn sub() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 - point2, S2Point { x: 1.0, y: -1.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 - f, S2Point { x: 0.5, y: -0.5, z: -0.5 });
    }

    #[test]
    fn mul() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1 * point2, S2Point { x: 0.0, y: 0.0, z: 0.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 * f, S2Point { x: 0.5, y: 0.0, z: 0.0 });
    }

    #[test]
    fn div() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.5 };
        let point2 = S2Point { x: 1.0, y: 1.0, z: 0.1 };
        assert_eq!(point1 / point2, S2Point { x: 1.0, y: 0.0, z: 5.0 });
        let f: f64 = 0.5;
        assert_eq!(point1 / f, S2Point { x: 2.0, y: 0.0, z: 1.0 });
    }

    #[test]
    fn neg() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!(-point1, S2Point { x: -1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn dot() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!(point1.dot(&point2), 0.0);
    }

    #[test]
    fn rem() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let f: f64 = 0.5;
        assert_eq!(point1 % f, S2Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn rem_assign() {
        let mut point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let f: f64 = 0.5;
        point1 %= f;
        assert_eq!(point1, S2Point { x: 0.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn from_s2_cell_id() {
        let id: S2CellId = 1152921504606846977.into();
        assert_eq!(S2Point::from(id), S2Point { x: 1.0, y: 0.0, z: 0.0 });
    }

    #[test]
    fn from_vector_point() {
        let vp = VectorPoint::new(1., 2., None, None);
        assert_eq!(S2Point::from(&vp), S2Point { x: 1.0, y: 2.0, z: 0.0 });

        let vp = VectorPoint::new(1., 2., Some(3.), None);
        assert_eq!(S2Point::from(&vp), S2Point { x: 1.0, y: 2.0, z: 3.0 });
    }

    #[test]
    fn cmp() {
        let point1 = S2Point { x: 1.0, y: 0.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 1.0, z: 0.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 1.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 0.0, z: 5.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        assert!(point1 > point2);
        assert!(point2 < point1);
        assert!(point1 != point2);
        let point1 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        let point2 = S2Point { x: 0.0, y: 0.0, z: 2.0 };
        assert!(point1 == point2);

        let point1 = S2Point { x: f64::NAN, y: f64::NAN, z: f64::NAN };
        let point2 = S2Point { x: f64::NAN, y: f64::NAN, z: f64::NAN };
        assert!(point1 != point2);
    }
}
