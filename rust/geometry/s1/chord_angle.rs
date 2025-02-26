use crate::geometry::{S1Angle, S2Point};

use libm::{asin, fabs, fmod, sin, sqrt};

use core::{
    cmp::Ordering,
    f64::consts::PI,
    ops::{Add, Deref, Div, Mul, Neg, Rem, RemAssign, Sub},
};

/// The Maximum allowed squared chord length.
pub const K_MAX_LENGTH_2: f64 = 4.0;

/// S1ChordAngle represents the angle subtended by a chord (i.e., the straight
/// line segment connecting two points on the sphere).  Its representation
/// makes it very efficient for computing and comparing distances, but unlike
/// S1Angle it is only capable of representing angles between 0 and Pi radians.
/// S1ChordAngle is intended for applications where many angles need to be
/// computed and compared, otherwise it is simpler to use S1Angle.
///
/// S1ChordAngle also loses some accuracy as the angle approaches Pi radians.
/// There are several different ways to measure this error, including the
/// representational error (i.e., how accurately S1ChordAngle can represent
/// angles near Pi radians), the conversion error (i.e., how much precision is
/// lost when an S1Angle is converted to an S1ChordAngle), and the measurement
/// error (i.e., how accurate the S1ChordAngle(a, b) constructor is when the
/// points A and B are separated by angles close to Pi radians).  All of these
/// errors differ by a small constant factor.
///
/// For the measurement error (which is the largest of these errors and also
/// the most important in practice), let the angle between A and B be (Pi - x)
/// radians, i.e. A and B are within "x" radians of being antipodal.  The
/// corresponding chord length is
/// ```latex
///    r = 2 * sin((Pi - x) / 2) = 2 * cos(x / 2) .
/// ```
/// For values of x not close to Pi the relative error in the squared chord
/// length is at most 4.5 * DBL_EPSILON (see GetS2PointConstructorMaxError).
/// The relative error in "r" is thus at most 2.25 * DBL_EPSILON ~= 5e-16.  To
/// convert this error into an equivalent angle, we have
/// ```latex
///    |dr / dx| = sin(x / 2)
/// ```
/// and therefore
/// ```latex
///    |dx| = dr / sin(x / 2)
///         = 5e-16 * (2 * cos(x / 2)) / sin(x / 2)
///         = 1e-15 / tan(x / 2)
/// ```
/// The maximum error is attained when
/// ```latext
///    x  = |dx|
///       = 1e-15 / tan(x / 2)
///      ~= 1e-15 / (x / 2)
///      ~= sqrt(2e-15)
/// ```
/// In summary, the measurement error for an angle (Pi - x) is at most
/// ```latex
///    dx  = min(1e-15 / tan(x / 2), sqrt(2e-15))
///      (~= min(2e-15 / x, sqrt(2e-15)) when x is small).
/// ```
/// On the Earth's surface (assuming a radius of 6371km), this corresponds to
/// the following worst-case measurement errors:
/// ```latex
///     Accuracy:             Unless antipodal to within:
///     ---------             ---------------------------
///     6.4 nanometers        10,000 km (90 degrees)
///     1 micrometer          81.2 kilometers
///     1 millimeter          81.2 meters
///     1 centimeter          8.12 meters
///     28.5 centimeters      28.5 centimeters
/// ```
/// The representational and conversion errors referred to earlier are somewhat
/// smaller than this.  For example, maximum distance between adjacent
/// representable S1ChordAngle values is only 13.5 cm rather than 28.5 cm.  To
/// see this, observe that the closest representable value to r^2 = 4 is
/// r^2 =  4 * (1 - DBL_EPSILON / 2).  Thus r = 2 * (1 - DBL_EPSILON / 4) and
/// the angle between these two representable values is
/// ```latex
///    x  = 2 * acos(r / 2)
///       = 2 * acos(1 - DBL_EPSILON / 4)
///      ~= 2 * asin(sqrt(DBL_EPSILON / 2)
///      ~= sqrt(2 * DBL_EPSILON)
///      ~= 2.1e-8
/// ```
/// which is 13.5 cm on the Earth's surface.
///
/// The worst case rounding error occurs when the value halfway between these
/// two representable values is rounded up to 4.  This halfway value is
/// r^2 = (4 * (1 - DBL_EPSILON / 4)), thus r = 2 * (1 - DBL_EPSILON / 8) and
/// the worst case rounding error is
/// ```latex
///    x  = 2 * acos(r / 2)
///       = 2 * acos(1 - DBL_EPSILON / 8)
///      ~= 2 * asin(sqrt(DBL_EPSILON / 4)
///      ~= sqrt(DBL_EPSILON)
///      ~= 1.5e-8
/// ```
/// which is 9.5 cm on the Earth's surface.
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Copy, Clone, Default, Debug)]
pub struct S1ChordAngle {
    /// The squared length of the corresponding S1Chord.
    pub length2: f64,
}
impl S1ChordAngle {
    /// Creates a new S1ChordAngle.
    pub fn new(length2: f64) -> Self {
        S1ChordAngle { length2 }
    }

    /// Returns the zero S1ChordAngle.
    pub fn zero() -> Self {
        S1ChordAngle { length2: 0.0 }
    }

    /// Returns the infinite S1ChordAngle.
    pub fn infinity() -> Self {
        S1ChordAngle { length2: f64::INFINITY }
    }

    ///  Conversion from an S1Angle.  Angles outside the range [0, Pi] are handled
    ///  as follows: Infinity() is mapped to Infinity(), negative angles are
    ///  mapped to Negative(), and finite angles larger than Pi are mapped to
    ///  Straight().
    ///
    ///  Note that this operation is relatively expensive and should be avoided.
    ///  To use S1ChordAngle effectively, you should structure your code so that
    ///  input arguments are converted to S1ChordAngles at the beginning of your
    ///  algorithm, and results are converted back to S1Angles only at the end.
    ///
    ///  S1ChordAngles are represented by the squared chord length, which can
    ///  range from 0 to 4.  Infinity() uses an infinite squared length.
    ///  @param angle - An angle in radians.
    ///  @returns The corresponding ChordAngle.
    pub fn from_angle(angle: S1Angle) -> Self {
        let radians: f64 = angle.radians;
        if radians < 0. {
            (-1.).into()
        } else if radians == f64::INFINITY {
            f64::INFINITY.into()
        } else {
            // The chord length is 2 * sin(angle / 2).
            let length = 2.0 * sin(0.5 * f64::min(PI, radians));
            (length * length).into()
        }
    }

    /// Construct an S1ChordAngle from the squared chord length.  Note that the
    /// argument is automatically clamped to a maximum of 4.0 to handle possible
    /// roundoff errors.  The argument must be non-negative.
    pub fn from_length2(length2_: f64) -> Self {
        f64::min(K_MAX_LENGTH_2, length2_).into()
    }

    /// Construct the S1ChordAngle corresponding to the distance between the two
    /// given points.  The points must be unit length.
    pub fn from_s2_points(a: &S2Point, b: &S2Point) -> Self {
        // The squared distance may slightly exceed 4.0 due to roundoff errors.
        // The maximum error in the result is 2 * DBL_EPSILON * length2_.
        f64::min(K_MAX_LENGTH_2, (*a - *b).norm2()).into()
    }

    /// Return a chord angle of 90 degrees (a "right angle").
    pub fn right_angle() -> Self {
        2.0.into()
    }

    /// Return a chord angle of 180 degrees (a "straight angle").  This is the
    /// maximum finite chord angle.
    pub fn straight_angle() -> Self {
        K_MAX_LENGTH_2.into()
    }

    // /**
    /// Return a chord angle smaller than Zero().  The only valid operations on
    /// Negative() are comparisons, S1Angle conversions, and successor() / predecessor().
    pub fn negative_angle() -> Self {
        (-1.).into()
    }

    /// Construct an S1ChordAngle that is an upper bound on the given S1Angle.
    /// i.i. such that FastUpperBoundFrom(x).toAngle() >= x. Unlike the S1Angle
    /// constructor above, this method is very fast, and the bound is accurate to
    /// within 1% for distances up to about 3100km on the Earth's surface.
    pub fn fast_upper_bound_from(angle: S1Angle) -> Self {
        // This method uses the distance along the surface of the sphere as an upper
        // bound on the distance through the sphere's interior.
        Self::from_length2((*angle) * (*angle))
    }

    /// Convenience function to test if a ChordAngle is special.
    pub fn is_special(&self) -> bool {
        *self < 0. || *self == f64::INFINITY
    }

    /// Convert to an S1Angle.
    /// Infinity() is converted to S1Angle.Infinity(), and Negative() is
    /// converted to an unspecified negative S1Angle.
    ///
    /// Note that the conversion uses trigonometric functions and therefore
    /// should be avoided in inner loops.
    pub fn to_angle(&self) -> S1Angle {
        let length2 = self.length2;
        if length2 < 0. {
            (-1.0).into()
        } else if length2 == f64::INFINITY {
            f64::INFINITY.into()
        } else {
            (2. * asin(0.5 * sqrt(length2))).into()
        }
    }

    /// Convert to meters.
    /// If no radius is specified, the Earth's radius is used.
    pub fn to_meters(&self, radius: Option<f64>) -> f64 {
        self.to_angle().to_meters(radius)
    }

    /// Convert from meters.
    /// If no radius is specified, the Earth's radius is used.
    pub fn from_meters(meters: f64, radius: Option<f64>) -> Self {
        S1Angle::from_meters(meters, radius).into()
    }

    /// Convert to kilometers.
    /// If no radius is specified, the Earth's radius is used.
    pub fn to_km(&self, radius: Option<f64>) -> f64 {
        self.to_angle().to_km(radius)
    }

    /// Convert from kilometers.
    /// If no radius is specified, the Earth's radius is used.
    pub fn from_km(km: f64, radius: Option<f64>) -> Self {
        S1Angle::from_km(km, radius).into()
    }

    // Trigonmetric functions.  It is more accurate and efficient to call these
    // rather than first converting to an S1Angle.

    /// apply a sine function on a ChordAngle
    pub fn chord_angle_sin(&self) -> f64 {
        sqrt(self.chord_angle_sin2())
    }

    /// apply a cosine function on a ChordAngle
    pub fn chord_angle_cos(&self) -> f64 {
        // cos(2*A) = cos^2(A) - sin^2(A) = 1 - 2*sin^2(A)
        1.0 - 0.5 * (**self)
    }

    /// apply a tangent function on a ChordAngle
    pub fn chord_angle_tan(&self) -> f64 {
        self.chord_angle_sin() / self.chord_angle_cos()
    }

    /// Returns sin(a)^2, but computed more efficiently.
    pub fn chord_angle_sin2(&self) -> f64 {
        // Let "a" be the (non-squared) chord length, and let A be the corresponding
        // half-angle (a = 2*sin(A)).  The formula below can be derived from:
        //   sin(2*A) = 2 * sin(A) * cos(A)
        //   cos^2(A) = 1 - sin^2(A)
        // This is much faster than converting to an angle and computing its sine.
        (**self) * (1.0 - 0.25 * (**self))
    }

    /// Returns the remainder when dividing by `modulus`
    pub fn modulo(&self, modulus: f64) -> Self {
        Self { length2: fmod(**self, fabs(modulus)) }
    }
}
impl From<f64> for S1ChordAngle {
    fn from(length2: f64) -> S1ChordAngle {
        S1ChordAngle { length2 }
    }
}
impl From<S1Angle> for S1ChordAngle {
    fn from(angle: S1Angle) -> S1ChordAngle {
        S1ChordAngle::from_angle(angle)
    }
}
impl From<S1ChordAngle> for S1Angle {
    fn from(c_angle: S1ChordAngle) -> S1Angle {
        c_angle.to_angle()
    }
}
impl Deref for S1ChordAngle {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.length2
    }
}

impl Add for S1ChordAngle {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        (self.length2 + rhs.length2).into()
    }
}
impl Add<f64> for S1ChordAngle {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        (self.length2 + rhs).into()
    }
}
impl Sub for S1ChordAngle {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        (self.length2 - rhs.length2).into()
    }
}
impl Sub<f64> for S1ChordAngle {
    type Output = Self;
    /// Subtracts a value from the length2 of the chord angle.
    fn sub(self, rhs: f64) -> Self {
        (self.length2 - rhs).into()
    }
}
impl Mul for S1ChordAngle {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        (self.length2 * rhs.length2).into()
    }
}
impl Mul<f64> for S1ChordAngle {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        (self.length2 * rhs).into()
    }
}
impl Div for S1ChordAngle {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        (self.length2 / rhs.length2).into()
    }
}
impl Div<f64> for S1ChordAngle {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        (self.length2 / rhs).into()
    }
}
impl Neg for S1ChordAngle {
    type Output = Self;
    fn neg(self) -> Self {
        (-self.length2).into()
    }
}
impl Rem<f64> for S1ChordAngle {
    type Output = Self;
    fn rem(self, modulus: f64) -> Self::Output {
        self.modulo(modulus)
    }
}
impl RemAssign<f64> for S1ChordAngle {
    fn rem_assign(&mut self, modulus: f64) {
        self.length2 = fmod(self.length2, fabs(modulus));
    }
}

// equalities
impl PartialOrd<S1ChordAngle> for S1ChordAngle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialOrd<f64> for S1ChordAngle {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.length2.partial_cmp(other)
    }
}
impl Ord for S1ChordAngle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length2.partial_cmp(&other.length2).unwrap_or(Ordering::Equal)
    }
}
impl PartialEq<S1ChordAngle> for S1ChordAngle {
    fn eq(&self, other: &Self) -> bool {
        self.length2 == other.length2
    }
}
impl PartialEq<f64> for S1ChordAngle {
    fn eq(&self, other: &f64) -> bool {
        self.length2 == *other
    }
}
impl Eq for S1ChordAngle {}
