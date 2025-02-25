use crate::{
    geometry::{LonLat, S2Point},
    space::EARTH_RADIUS,
};

use libm::{fabs, fmod};

use core::f64::consts::PI;
use core::ops::{Add, Div, Mul, Neg, Rem, RemAssign, Sub};
use core::{cmp::Ordering, ops::Deref};

/// This class represents a one-dimensional angle (as opposed to a
/// two-dimensional solid angle).  It has methods for converting angles to
/// or from radians, degrees, and the E5/E6/E7 representations (i.e. degrees
/// multiplied by 1e5/1e6/1e7 and rounded to the nearest integer).
///
/// The internal representation is a double-precision value in radians, so
/// conversion to and from radians is exact.  Conversions between E5, E6, E7,
/// and Degrees are not always exact; for example, Degrees(3.1) is different
/// from E6(3100000) or E7(310000000).  However, the following properties are
/// guaranteed for any integer "n", provided that "n" is in the input range of
/// both functions:
/// ```latext
///     Degrees(n) == E6(1000000 * n)
///     Degrees(n) == E7(10000000 * n)
///          E6(n) == E7(10 * n)
/// ```
/// The corresponding properties are *not* true for E5, so if you use E5 then
/// don't test for exact equality when comparing to other formats such as
/// Degrees or E7.
///
/// The following conversions between degrees and radians are exact:
/// ```latext
///          Degrees(180) == Radians(M_PI);
///       Degrees(45 * k) == Radians(k * M_PI / 4)  for k == 0..8
/// ```
/// These identities also hold when the arguments are scaled up or down by any
/// power of 2.  Some similar identities are also true, for example,
/// Degrees(60) == Radians(M_PI / 3), but be aware that this type of identity
/// does not hold in general.  For example, Degrees(3) != Radians(M_PI / 60).
///
/// Similarly, the conversion to radians means that Angle::Degrees(x).degrees()
/// does not always equal "x".  For example,
/// ```latext
///         S1Angle::Degrees(45 * k).degrees() == 45 * k      for k == 0..8
///   but       S1Angle::Degrees(60).degrees() != 60.
/// ```
/// This means that when testing for equality, you should allow for numerical
/// errors (EXPECT_DOUBLE_EQ) or convert to discrete E5/E6/E7 values first.
///
/// CAVEAT: All of the above properties depend on "double" being the usual
/// 64-bit IEEE 754 type (which is true on almost all modern platforms).
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Copy, Clone, Default, Debug)]
pub struct S1Angle {
    /// Angle in radians
    pub radians: f64,
}
impl S1Angle {
    /// Creates an S1Angle from a value in radians.
    pub fn new(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an S1Angle with an infinite value.
    pub fn infinity() -> Self {
        Self { radians: f64::INFINITY }
    }

    /// Creates an S1Angle from a value in degrees, converting it to radians.
    pub fn from_degrees(degrees: f64) -> Self {
        Self { radians: degrees.to_radians() }
    }

    /// Returns the angle in degrees.
    pub fn to_degrees(&self) -> f64 {
        (**self).to_degrees()
    }

    /// build an angle in E5 format.
    pub fn to_e5(e5_: f64) -> Self {
        Self::from_degrees(e5_ * 1e-5)
    }

    /// build an angle in E6 format.
    pub fn to_e6(e6_: f64) -> Self {
        Self::from_degrees(e6_ * 1e-6)
    }

    /// build an angle in E7 format.
    pub fn to_e7(e7_: f64) -> Self {
        Self::from_degrees(e7_ * 1e-7)
    }

    /// Return the angle between two points, which is also equal to the distance
    /// between these points on the unit sphere.  The points do not need to be
    /// normalized.  This function has a maximum error of 3.25 * DBL_EPSILON (or
    /// 2.5 * DBL_EPSILON for angles up to 1 radian). If either point is
    /// zero-length (e.g. an uninitialized S2Point), or almost zero-length, the
    /// resulting angle will be zero.
    pub fn from_s2points(a: &S2Point, b: &S2Point) -> Self {
        a.angle(b).into()
    }

    /// Like the constructor above, but return the angle (i.e., distance) between
    /// two S2LatLng points.  This function has about 15 digits of accuracy for
    /// small distances but only about 8 digits of accuracy as the distance
    /// approaches 180 degrees (i.e., nearly-antipodal points).
    pub fn from_lon_lat(a: &LonLat, b: &LonLat) -> Self {
        a.get_distance(b).into()
    }

    /// Convert an angle in radians to an angle in meters
    /// If no radius is specified, the Earth's radius is used.
    pub fn to_meters(&self, radius: Option<f64>) -> f64 {
        let radius = radius.unwrap_or(EARTH_RADIUS);
        **self * radius
    }

    /// Convert an angle in meters to an angle in radians
    /// If no radius is specified, the Earth's radius is used.
    pub fn from_meters(angle: f64, radius: Option<f64>) -> Self {
        let radius = radius.unwrap_or(EARTH_RADIUS);
        (angle / radius).into()
    }

    /// Convert an angle in radians to an angle in kilometers
    /// If no radius is specified, the Earth's radius is used.
    pub fn to_km(&self, radius: Option<f64>) -> f64 {
        let radius = radius.unwrap_or(EARTH_RADIUS);
        **self * radius / 1_000.0
    }

    /// Convert an angle in kilometers to an angle in radians
    /// If no radius is specified, the Earth's radius is used.
    pub fn from_km(angle: f64, radius: Option<f64>) -> Self {
        let radius = radius.unwrap_or(EARTH_RADIUS);
        ((angle * 1_000.0) / radius).into()
    }

    // Note that the E5, E6, and E7 conversion involve two multiplications rather
    // than one.  This is mainly for backwards compatibility (changing this would
    // break many tests), but it does have the nice side effect that conversions
    // between Degrees, E6, and E7 are exact when the arguments are integers.

    /// Build an angle in E5 format.
    pub fn e5(&self) -> f64 {
        self.to_degrees() * 1e5
    }

    /// Build an angle in E6 format.
    pub fn e6(&self) -> f64 {
        self.to_degrees() * 1e6
    }

    /// Build an angle in E7 format.
    pub fn e7(&self) -> f64 {
        self.to_degrees() * 1e7
    }

    /// Normalize this angle to the range (-180, 180] degrees.
    pub fn normalize(&self) -> Self {
        *self % (2.0 * PI)
    }

    /// Returns the remainder when dividing by `modulus`
    pub fn modulo(&self, modulus: f64) -> Self {
        Self { radians: fmod(**self, fabs(modulus)) }
    }
}
impl From<f64> for S1Angle {
    fn from(radians: f64) -> S1Angle {
        S1Angle { radians }
    }
}
impl Deref for S1Angle {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.radians
    }
}

impl Add for S1Angle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        (self.radians + rhs.radians).into()
    }
}
impl Add<f64> for S1Angle {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        (self.radians + rhs).into()
    }
}
impl Sub for S1Angle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        (self.radians - rhs.radians).into()
    }
}
impl Sub<f64> for S1Angle {
    type Output = Self;
    /// Subtracts a value from the radians of the chord angle.
    fn sub(self, rhs: f64) -> Self {
        (self.radians - rhs).into()
    }
}
impl Mul for S1Angle {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        (self.radians * rhs.radians).into()
    }
}
impl Mul<f64> for S1Angle {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        (self.radians * rhs).into()
    }
}
impl Div for S1Angle {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        (self.radians / rhs.radians).into()
    }
}
impl Div<f64> for S1Angle {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        (self.radians / rhs).into()
    }
}
impl Neg for S1Angle {
    type Output = Self;
    fn neg(self) -> Self {
        (-self.radians).into()
    }
}
impl Rem<f64> for S1Angle {
    type Output = Self;
    fn rem(self, modulus: f64) -> Self::Output {
        self.modulo(modulus)
    }
}
impl RemAssign<f64> for S1Angle {
    fn rem_assign(&mut self, modulus: f64) {
        self.radians = fmod(self.radians, fabs(modulus));
    }
}

impl PartialOrd<S1Angle> for S1Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
// Implement PartialOrd for comparison between S1Angle and f64
impl PartialOrd<f64> for S1Angle {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.radians.partial_cmp(other)
    }
}
impl Ord for S1Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.radians.partial_cmp(&other.radians).unwrap_or(Ordering::Equal)
    }
}
impl PartialEq<S1Angle> for S1Angle {
    fn eq(&self, other: &Self) -> bool {
        self.radians == other.radians
    }
}
impl PartialEq<f64> for S1Angle {
    fn eq(&self, other: &f64) -> bool {
        self.radians == *other
    }
}
impl Eq for S1Angle {}
