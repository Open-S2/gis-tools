use libm::fabs;
use core::fmt::Debug;
use core::ops::{Add, Sub, Neg, Div, Mul};

/// An S2Point represents a point on the unit sphere as a 3D vector. Usually
/// points are normalized to be unit length, but some methods do not require
/// this.  See util/math/vector.h for the methods available.  Among other
/// things, there are overloaded operators that make it convenient to write
/// arithmetic expressions (e.g. (1-x)*p1 + x*p2).
/// NOTE: asumes only f64 or greater is used.
#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct S2Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl S2Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        S2Point { x, y, z }
    }

    pub fn is_empty(&self) -> bool {
        let zero = f64::default();
        self.x == zero && self.y == zero && self.z == zero
    }

    pub fn face(&self, f: u8) -> f64 {
        if f == 0 {
            self.x
        } else if f == 1 {
            self.y
        } else {
            self.z
        }
    }

    /// dot returns the standard dot product of v and ov.
    pub fn dot(&self, b: &Self) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn abs(&self) -> Self {
        Self::new(fabs(self.x), fabs(self.y), fabs(self.z))
    }
    
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
}
// Implementing the Add trait for S2Point
impl Add for S2Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        S2Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// Implementing the Sub trait for S2Point
impl Sub for S2Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        S2Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// Implementing the Neg trait for S2Point
impl Neg for S2Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        S2Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Implementing the Div trait for S2Point
impl Div for S2Point {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        S2Point {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

// Implementing the Mul trait for S2Point
impl Mul for S2Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        S2Point {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

    //     pub fn addMut(self: *Self, b: *const S2PointType(T)) void {
    //         self.x += b.x;
    //         self.y += b.y;
    //         self.z += b.z;
    //     }
    //     pub fn addScalar(&self, b: T) S2PointType(T) {
    //         return .{
    //             .x = self.x + b,
    //             .y = self.y + b,
    //             .z = self.z + b,
    //         };
    //     }
    //     pub fn addMutScalar(self: *Self, b: T) void {
    //         self.x += b;
    //         self.y += b;
    //         self.z += b;
    //     }

    //     pub fn sub(&self, b: *const S2PointType(T)) S2PointType(T) {
    //         return .{
    //             .x = self.x - b.x,
    //             .y = self.y - b.y,
    //             .z = self.z - b.z,
    //         };
    //     }
    //     pub fn subMut(self: *Self, b: *const S2PointType(T)) void {
    //         self.x -= b.x;
    //         self.y -= b.y;
    //         self.z -= b.z;
    //     }
    //     pub fn subScalar(&self, b: T) S2PointType(T) {
    //         return .{
    //             .x = self.x - b,
    //             .y = self.y - b,
    //             .z = self.z - b,
    //         };
    //     }
    //     pub fn subMutScalar(self: *Self, b: T) void {
    //         self.x -= b;
    //         self.y -= b;
    //         self.z -= b;
    //     }

    //     pub fn mul(&self, b: *const S2PointType(T)) S2PointType(T) {
    //         return .{
    //             .x = self.x * b.x,
    //             .y = self.y * b.y,
    //             .z = self.z * b.z,
    //         };
    //     }
    //     pub fn mulMut(self: *Self, b: *const S2PointType(T)) void {
    //         self.x *= b.x;
    //         self.y *= b.y;
    //         self.z *= b.z;
    //     }
    //     pub fn mulScalar(&self, b: T) S2PointType(T) {
    //         return .{
    //             .x = self.x * b,
    //             .y = self.y * b,
    //             .z = self.z * b,
    //         };
    //     }
    //     pub fn mulMutScalar(self: *Self, b: T) void {
    //         self.x *= b;
    //         self.y *= b;
    //         self.z *= b;
    //     }

    //     pub fn div(&self, b: *const S2PointType(T)) S2PointType(T) {
    //         return .{
    //             .x = self.x / b.x,
    //             .y = self.y / b.y,
    //             .z = self.z / b.z,
    //         };
    //     }
    //     pub fn divMut(self: *Self, b: *const S2PointType(T)) void {
    //         self.x /= b.x;
    //         self.y /= b.y;
    //         self.z /= b.z;
    //     }
    //     pub fn divScalar(&self, b: T) S2PointType(T) {
    //         return .{
    //             .x = self.x / b,
    //             .y = self.y / b,
    //             .z = self.z / b,
    //         };
    //     }
    //     pub fn divMutScalar(self: *Self, b: T) void {
    //         self.x /= b;
    //         self.y /= b;
    //         self.z /= b;
    //     }

    //     pub fn eq(a: *const Self, b: *const S2PointType(T)) bool {
    //         return a.x == b.x and a.y == b.y and a.z == b.z;
    //     }

    //     pub fn neq(a: *const Self, b: *const S2PointType(T)) bool {
    //         return a.x != b.x or a.y != b.y or a.z != b.z;
    //     }

    //     pub fn max(a: *const Self, b: *const S2PointType(T)) S2PointType(T) {
    //         return Init(@max(a.x, b.x), @max(a.y, b.y), @max(a.z, b.z));
    //     }

    //     pub fn min(a: *const Self, b: *const S2PointType(T)) S2PointType(T) {
    //         return Init(@min(a.x, b.x), @min(a.y, b.y), @min(a.z, b.z));
    //     }

    //     pub fn round(&self) S2PointType(T) {
    //         return Init(@round(self.x), @round(self.y), @round(self.z));
    //     }

    //     pub fn ceil(&self) S2PointType(T) {
    //         return Init(@ceil(self.x), @ceil(self.y), @ceil(self.z));
    //     }

    //     pub fn floor(&self) S2PointType(T) {
    //         return Init(@floor(self.x), @floor(self.y), @floor(self.z));
    //     }

    //     pub fn sqrt(&self) S2PointType(T) {
    //         return Init(@sqrt(self.x), @sqrt(self.y), @sqrt(self.z));
    //     }

    //     pub fn lt(a: *const Self, b: *const S2PointType(T)) bool {
    //         return a.x < b.x and a.y < b.y and a.z < b.z;
    //     }

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
