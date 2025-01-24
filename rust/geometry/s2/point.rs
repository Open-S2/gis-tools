use core::cmp::Ordering;
use core::fmt::Debug;
use core::ops::{Add, Div, Mul, Neg, Sub};

use libm::{fabs, sqrt};

use crate::{s2::xyz_to_face_uv, wm::LonLat, xyz_to_face_st};

use super::S2CellId;

/// An S2Point represents a point on the unit sphere as a 3D vector. Usually
/// points are normalized to be unit length, but some methods do not require
/// this.  See util/math/vector.h for the methods available.  Among other
/// things, there are overloaded operators that make it convenient to write
/// arithmetic expressions (e.g. (1-x)*p1 + x*p2).
/// NOTE: asumes only f64 or greater is used.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
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

    /// Returns the length of the point.
    pub fn len(&self) -> f64 {
        sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    /// return the distance from this point to the other point
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

    /// Normalize this point to unit length.
    pub fn normalize(&mut self) {
        let mut len = self.x * self.x + self.y * self.y + self.z * self.z;
        if len > 0.0 {
            len = sqrt(len);
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }
}
impl From<&LonLat> for S2Point {
    fn from(lonlat: &LonLat) -> Self {
        lonlat.to_point()
    }
}
impl From<&S2CellId> for S2Point {
    fn from(cellid: &S2CellId) -> Self {
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

//     /// norm returns the vector's norm.
//     pub fn norm(&self) T {
//         return @sqrt(self.norm2());
//     }

//     /// norm2 returns the square of the norm.
//     pub fn norm2(&self) T {
//         return self.dot(self);
//     }

//     pub fn normalize(&self) S2PointType(T) {
//         const len = self.norm();
//         return Init(self.x / len, self.y / len, self.z / len);
//     }

//     pub fn distance(a: *const Self, b: *const S2PointType(T)) T {
//         return @sqrt(pow(@abs(b.x - a.x), 2) + pow(@abs(b.y - a.y), 2) + pow(@abs(b.z - a.z), 2));
//     }

//     pub fn cross(&self, b: *const S2PointType(T)) S2PointType(T) {
//         return Init(
//             self.y * b.z - self.z * b.y,
//             self.z * b.x - self.x * b.z,
//             self.x * b.y - self.y * b.x,
//         );
//     }

//     pub fn intermediate(&self, b: *const S2PointType(T), t: T) S2PointType(T) {
//         var c = .{
//             .x = (self.x) + ((b.x - self.x) * (1 - t)),
//             .y = (self.y) + ((b.y - self.y) * (1 - t)),
//             .z = (self.z) + ((b.z - self.z) * (1 - t)),
//         };
//         return c.normalize();
//     }

//     /// Returns the angle between "this" and v in radians, in the range [0, pi]. If
//     /// either vector is zero-length, or nearly zero-length, the result will be
//     /// zero, regardless of the other value.
//     pub fn angle(&self, b: *const S2PointType(T)) T {
//         return atan2(T, self.cross(b).norm(), self.dot(b));
//     }
// };
// }
