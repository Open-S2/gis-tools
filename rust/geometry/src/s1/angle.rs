// const S2Point = @import("../point.zig").S2Point;
// const LonLat = @import("../lonlat.zig").LonLat;
// const s2earth = @import("../earth.zig");
// const radiusMeters = s2earth.radiusMeters;
// const radiusKm = s2earth.radiusKm;
// const FastIntRound = @import("../s2/coordsInternal.zig");

use core::f64::consts::PI;

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
///
///     Degrees(n) == E6(1000000 * n)
///     Degrees(n) == E7(10000000 * n)
///          E6(n) == E7(10 * n)
///
/// The corresponding properties are *not* true for E5, so if you use E5 then
/// don't test for exact equality when comparing to other formats such as
/// Degrees or E7.
///
/// The following conversions between degrees and radians are exact:
///
///          Degrees(180) == Radians(M_PI)
///       Degrees(45 * k) == Radians(k * M_PI / 4)  for k == 0..8
///
/// These identities also hold when the arguments are scaled up or down by any
/// power of 2.  Some similar identities are also true, for example,
/// Degrees(60) == Radians(M_PI / 3), but be aware that this type of identity
/// does not hold in general.  For example, Degrees(3) != Radians(M_PI / 60).
///
/// Similarly, the conversion to radians means that Angle::Degrees(x).degrees()
/// does not always equal "x".  For example,
///
///         S1Angle::Degrees(45 * k).degrees() == 45 * k      for k == 0..8
///   but       S1Angle::Degrees(60).degrees() != 60.
///
/// This means that when testing for equality, you should allow for numerical
/// errors (EXPECT_DOUBLE_EQ) or convert to discrete E5/E6/E7 values first.
///
/// CAVEAT: All of the above properties depend on "double" being the usual
/// 64-bit IEEE 754 type (which is true on almost all modern platforms).
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct S1Angle {
    pub radians: f64
}
impl S1Angle {
    pub fn new(radians: f64) -> Self {
        Self{ radians }
    }

    pub fn from_degrees(degrees: f64) -> Self {
        Self{ radians: degrees * (PI / 180.0) }
    }

    pub fn infinity() -> Self {
        Self{ radians: f64::INFINITY }
    }

    pub fn degrees(&self) -> f64 {
        (180. / PI) * self.radians
    }
}

//     /// Default Instance
//     pub fn Init(radians_: f64) S1Angle {
//         return .{ .radians = radians_ };
//     }

//     pub fn Infinity() S1Angle {
//         return .{ .radians = std.math.inf(f64) };
//     }

//     pub fn Zero() S1Angle {
//         return .{ .radians = 0.0 };
//     }

//     pub fn Degrees(degrees_: f64) S1Angle {
//         return .{ .radians = (std.math.pi / 180.0) * degrees_ };
//     }

//     pub fn E5(e5_: i32) S1Angle {
//         return Degrees(1e-5 * e5_);
//     }

//     pub fn E6(e6_: i32) S1Angle {
//         return Degrees(1e-6 * e6_);
//     }

//     pub fn E7(e7_: i32) S1Angle {
//         return Degrees(1e-7 * e7_);
//     }

//     pub fn UnsignedE6(e6_: u32) S1Angle {
//         return Degrees(1e-6 * e6_);
//     }

//     pub fn UnsignedE7(e7_: u32) S1Angle {
//         return Degrees(1e-7 * e7_);
//     }

//     // Return the angle between two points, which is also equal to the distance
//     // between these points on the unit sphere.  The points do not need to be
//     // normalized.  This function has a maximum error of 3.25 * DBL_EPSILON (or
//     // 2.5 * DBL_EPSILON for angles up to 1 radian). If either point is
//     // zero-length (e.g. an uninitialized S2Point), or almost zero-length, the
//     // resulting angle will be zero.
//     pub fn FromS2Points(x: *const S2Point, y: *const S2Point) S1Angle {
//         return .{ .radians = x.angle(y) };
//     }

//     // Like the constructor above, but return the angle (i.e., distance) between
//     // two S2LatLng points.  This function has about 15 digits of accuracy for
//     // small distances but only about 8 digits of accuracy as the distance
//     // approaches 180 degrees (i.e., nearly-antipodal points).
//     pub fn FromLonLats(x: *const LonLat, y: *const LonLat) !S1Angle {
//         return x.getDistance(y);
//     }

//     pub fn Clone(self: *const S1Angle) S1Angle {
//         return .{ .radians = self.radians };
//     }

//     // * FUNCTIONS

//     pub fn radians(self: *const S1Angle) f64 {
//         return self.radians;
//     }

//     pub fn degrees(self: *const S1Angle) f64 {
//         return (180.0 / std.math.pi) * self.radians;
//     }

//     pub fn toMeters(self: *const S1Angle) f64 {
//         return self.radians() * radiusMeters;
//     }

//     pub fn toKm(self: *const S1Angle) f64 {
//         return self.radians() * radiusKm;
//     }

//     // Note that the E5, E6, and E7 conversion involve two multiplications rather
//     // than one.  This is mainly for backwards compatibility (changing this would
//     // break many tests), but it does have the nice side effect that conversions
//     // between Degrees, E6, and E7 are exact when the arguments are integers.
//     pub fn e5(self: *const S1Angle) i32 {
//         return FastIntRound(1e5 * self.degrees());
//     }

//     pub fn e6(self: *const S1Angle) i32 {
//         return FastIntRound(1e6 * self.degrees());
//     }

//     pub fn e7(self: *const S1Angle) i32 {
//         return FastIntRound(1e7 * self.degrees());
//     }

//     pub fn uE6(self: *const S1Angle) u32 {
//         return @as(u32, @intFromFloat(1e6 * self.degrees()));
//     }

//     pub fn uE7(self: *const S1Angle) u32 {
//         return @as(u32, @intFromFloat(1e7 * self.degrees()));
//     }

//     // Trigonmetric functions (not necessary but slightly more convenient).
//     pub fn sin(self: *const S1Angle) f64 {
//         return @sin(self.radians);
//     }
//     pub fn cos(self: *const S1Angle) f64 {
//         return @cos(self.radians);
//     }
//     pub fn tan(self: *const S1Angle) f64 {
//         return @tan(self.radians);
//     }
//     pub fn abs(self: *const S1Angle) S1Angle {
//         return .{ .radians = @abs(self.radians) };
//     }

//     // Normalize this angle to the range (-180, 180] degrees.
//     pub fn normalized(self: *const S1Angle) S1Angle {
//         return .{ .radians = @rem(self.radians, 2.0 * std.math.pi) };
//     }
//     pub fn normalize(self: *const S1Angle) void {
//         self.radians = @rem(self.radians, 2.0 * std.math.pi);
//     }

//     // Comparison operators.
//     pub fn eq(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians == b.radians;
//     }
//     pub fn eqScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians == b;
//     }
//     pub fn neq(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians != b.radians;
//     }
//     pub fn neqScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians != b;
//     }
//     pub fn lt(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians < b.radians;
//     }
//     pub fn ltScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians < b;
//     }
//     pub fn gt(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians > b.radians;
//     }
//     pub fn gtScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians > b;
//     }
//     pub fn le(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians <= b.radians;
//     }
//     pub fn leScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians <= b;
//     }
//     pub fn ge(self: *const S1Angle, b: *const S1Angle) bool {
//         return self.radians >= b.radians;
//     }
//     pub fn geScalar(self: *const S1Angle, b: f64) bool {
//         return self.radians >= b;
//     }

//     // Simple arithmetic operators for manipulating S1Angles.
//     pub fn add(self: *S1Angle, b: *const S1Angle) void {
//         self.radians += b.radians;
//     }
//     pub fn addScalar(self: *S1Angle, b: f64) void {
//         self.radians += b;
//     }
//     pub fn sub(self: *S1Angle, b: *const S1Angle) void {
//         self.radians -= b.radians;
//     }
//     pub fn subScalar(self: *S1Angle, b: f64) void {
//         self.radians -= b;
//     }
//     pub fn mul(self: *S1Angle, b: *const S1Angle) void {
//         self.radians *= b.radians;
//     }
//     pub fn mulScalar(self: *S1Angle, b: f64) void {
//         self.radians *= b;
//     }
//     pub fn div(self: *S1Angle, b: *const S1Angle) void {
//         self.radians /= b.radians;
//     }
//     pub fn divScalar(self: *S1Angle, b: f64) void {
//         self.radians /= b;
//     }
// };
