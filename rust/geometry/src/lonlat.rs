// const S1Angle = @import("s1/angle.zig").S1Angle;
// const S2Point = @import("point.zig").S2Point;

use libm::{fabs, cos, sin, atan2, sqrt, asin};
use core::f64::consts::PI;
use crate::s2::point::S2Point;
use crate::s1::angle::S1Angle;

/// This class represents a point on the unit sphere as a pair
/// of latitude-longitude coordinates.  Like the rest of the "geometry"
/// package, the intent is to represent spherical geometry as a mathematical
/// abstraction, so functions that are specifically related to the Earth's
/// geometry (e.g. easting/northing conversions) should be put elsewhere.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct LonLat {
    /// longitude in radians
    pub lon: f64,
    /// latitude in radians
    pub lat: f64,
}
impl LonLat {
    /// The default constructor sets the latitude and longitude to zero.  This is
    /// mainly useful when declaring arrays, STL containers, etc.
    pub fn new(lon: f64, lat: f64) -> Self {
        LonLat{ lon, lat }
    }

    /// Constructor.  The latitude and longitude are allowed to be outside
    /// the is_valid() range.  However, note that most methods that accept
    /// LngLats expect them to be normalized (see Normalized() below).
    pub fn from_angle(lon: &S1Angle, lat: &S1Angle) -> Self {
        LonLat::new(lat.radians, lon.radians)
    }

    // Convert a direction vector (not necessarily unit length) to an LonLat.
    pub fn from_s2_point(p: &S2Point) -> LonLat {
        let ll = LonLat::new(LonLat::latitude(p).radians, LonLat::longitude(p).radians);
        if !ll.is_valid() { unreachable!(); }
        ll
    }

    //   // Returns an LonLat for which is_valid() will return false.
    //   static constexpr LonLat Invalid();

    // Convenience functions -- shorter than calling S1Angle::Radians(), etc.
    //   static constexpr LonLat FromRadians(double lat_radians, double lon_radians);
    pub fn from_degrees(lon_deg: f64, lat_deg: f64) -> LonLat {
        LonLat::from_angle(&S1Angle::from_degrees(lat_deg), &S1Angle::from_degrees(lon_deg))
    }
    //   static constexpr LonLat FromE5(int32 lat_e5, int32 lon_e5);
    //   static constexpr LonLat FromE6(int32 lat_e6, int32 lon_e6);
    //   static constexpr LonLat FromE7(int32 lat_e7, int32 lon_e7);

    //   // Appends an encoded representation of the LonLat to "encoder".
    //   //
    //   // REQUIRES: "encoder" uses the default constructor, so that its buffer can be
    //   //           enlarged as necessary by calling Ensure(int).
    //   void Encode(Encoder* encoder) const;

    //   // Convenience functions -- to use when args have been fixed32s in protos.
    //   //
    //   // The arguments are static_cast into int32, so very large unsigned values
    //   // are treated as negative numbers.
    //   static constexpr LonLat FromUnsignedE6(uint32 lat_e6, uint32 lon_e6);
    //   static constexpr LonLat FromUnsignedE7(uint32 lat_e7, uint32 lon_e7);

    // Methods to compute the latitude and longitude of a point separately.
    pub fn latitude(p: &S2Point) -> S1Angle {
        // We use atan2 rather than asin because the input vector is not necessarily
        // unit length, and atan2 is much more accurate than asin near the poles.
        // The "+ 0.0" is to ensure that points with coordinates of -0.0 and +0.0
        // (which compare equal) are converted to identical LonLat values, since
        // even though -0.0 == +0.0 they can be formatted differently.
        S1Angle::new(atan2(p.z, sqrt(p.x * p.x + p.y * p.y)))
    }
    pub fn longitude(p: &S2Point) -> S1Angle {
        // The "+ 0.0" is to ensure that points with coordinates of -0.0 and +0.0
        // (which compare equal) are converted to identical LonLat values, since
        // even though -0.0 == +0.0 and -180 == 180 degrees, they can be formatted
        // differently.  Also note that atan2(0, 0) is defined to be zero.
        S1Angle::new(atan2(p.y + 0.0, p.x + 0.0))
    }

    // Accessor methods.
    pub fn lat_angle(&self) -> S1Angle {
        S1Angle::new(self.lat)
    }
    pub fn lon_angle(&self) -> S1Angle {
        S1Angle::new(self.lon)
    }
    pub fn lat_degrees(&self) -> f64 {
        self.lat_angle().degrees()
    }
    pub fn lon_degrees(&self) -> f64 {
        self.lon_angle().degrees()
    }
    pub fn from_axis(&self, axis: u8) -> f64 {
        if axis == 0 { self.lon } else { self.lat }
    }

    /// Return the latitude or longitude coordinates in degrees.
    pub fn coords(self) -> [f64; 2] {
        [self.lon_degrees(), self.lat_degrees()]
    }

    /// Return true if the latitude is between -90 and 90 degrees inclusive
    /// and the longitude is between -180 and 180 degrees inclusive.
    pub fn is_valid(&self) -> bool {
        fabs(self.lat_degrees()) <= (PI / 2.0) &&
        fabs(self.lon_degrees()) <= PI
    }

    //   // Clamps the latitude to the range [-90, 90] degrees, and adds or subtracts
    //   // a multiple of 360 degrees to the longitude if necessary to reduce it to
    //   // the range [-180, 180].
    //   LonLat Normalized() const;

    // Converts an LonLat to the equivalent unit-length vector.  Unnormalized
    // values (see Normalize()) are wrapped around the sphere as would be expected
    // based on their definition as spherical angles.  So for example the
    // following pairs yield equivalent points (modulo numerical error):
    //     (90.5, 10) =~ (89.5, -170)
    //     (a, b) =~ (a + 360 * n, b)
    // The maximum error in the result is 1.5 * DBL_EPSILON.  (This does not
    // include the error of converting degrees, E5, E6, or E7 to radians.)
    //
    // Can be used just like an S2Point constructor.  For example:
    //   S2Cap cap;
    //   cap.AddPoint(S2Point(latlon));
    pub fn to_point(&self) -> S2Point {
        // TODO:
        if !self.is_valid() { unreachable!(); }
        // S2_DLOG_IF(ERROR, !is_valid())
        //     << "Invalid LonLat in LonLat::ToPoint: " << *this;
        let phi: f64 = self.lat;
        let theta: f64 = self.lon;
        let cosphi: f64 = cos(phi);
        S2Point::new(cos(theta) * cosphi, sin(theta) * cosphi, sin(phi))
    }

    // Returns the distance (measured along the surface of the sphere) to the
    // given LonLat, implemented using the Haversine formula.  This is
    // equivalent to
    //
    //   S1Angle(ToPoint(), o.ToPoint())
    //
    // except that this function is slightly faster, and is also somewhat less
    // accurate for distances approaching 180 degrees (see s1angle.h for
    // details).  Both LngLats must be normalized.
    pub fn get_distance(&self, b: &LonLat) -> S1Angle {
        // This implements the Haversine formula, which is numerically stable for
        // small distances but only gets about 8 digits of precision for very large
        // distances (e.g. antipodal points).  Note that 8 digits is still accurate
        // to within about 10cm for a sphere the size of the Earth.
        //
        // This could be fixed with another sin() and cos() below, but at that point
        // you might as well just convert both arguments to S2Points and compute the
        // distance that way (which gives about 15 digits of accuracy for all
        // distances).
        if !self.is_valid() || !b.is_valid() { unreachable!(); }

        let lat1 = self.lat;
        let lat2 = b.lat;
        let lon1 = self.lon;
        let lon2 = b.lon;
        let dlat = sin(0.5 * (lat2 - lat1));
        let dlon = sin(0.5 * (lon2 - lon1));
        let x = dlat * dlat + dlon * dlon * cos(lat1) * cos(lat2);
        S1Angle::new(2. * asin(sqrt(f64::min(1.0, x))))
    }

    //   // Simple arithmetic operations for manipulating latitude-longitude pairs.
    //   // The results are not normalized (see Normalized()).
    //   friend LonLat operator+(const LonLat& a, const LonLat& b);
    //   friend LonLat operator-(const LonLat& a, const LonLat& b);
    //   friend LonLat operator*(double m, const LonLat& a);
    //   friend LonLat operator*(const LonLat& a, double m);

    //   bool operator==(const LonLat& o) const { return coords_ == o.coords_; }
    //   bool operator!=(const LonLat& o) const { return coords_ != o.coords_; }
    //   bool operator<(const LonLat& o) const { return coords_ < o.coords_; }
    //   bool operator>(const LonLat& o) const { return coords_ > o.coords_; }
    //   bool operator<=(const LonLat& o) const { return coords_ <= o.coords_; }
    //   bool operator>=(const LonLat& o) const { return coords_ >= o.coords_; }

    //   // Returns true if the numerical coordinates of two LonLat objects are
    //   // close.  Note that since LonLat operates on a rectangular space, the
    //   // behavior of ApproxEquals will does not reflect closeness of points on a
    //   // sphere if the points are close to the poles.  For those comparisons,
    //   // consider using GetDistance() instead.
    //   bool ApproxEquals(const LonLat& o,
    //                     S1Angle max_error = S1Angle::Radians(1e-15)) const {
    //     return coords_.aequal(o.coords_, max_error.radians());
    //   }

    //   // Exports the latitude and longitude in degrees, separated by a comma.
    //   // e.g. "94.518000,150.300000"
    //   std::string ToStringInDegrees() const;

    //  private:
    //   // Internal constructor.
    //   explicit LonLat(const R2Point& coords) : coords_(coords) {}

    //   // This is internal to avoid ambiguity about which units are expected.
    //   constexpr LonLat(double lat_radians, double lon_radians)
    //       : coords_(lat_radians, lon_radians) {}

    //   R2Point coords_;
}

// // Hasher for LonLat.
// // Does *not* need to be specified explicitly; this will be used by default for
// // absl::flat_hash_map/set.
// template <typename H>
// H AbslHashValue(H h, const LonLat& lat_lon) {
//   return H::combine(std::move(h), lat_lon.coords().x(), lat_lon.coords().y());
// }

// // Legacy hash functor for LonLat. This only exists for backwards
// // compatibility with old hash types like std::unordered_map that don't use
// // absl::Hash natively.
// #ifndef SWIG
// using LngLatHash = absl::Hash<LonLat>;
// #endif

// //////////////////   Implementation details follow   ////////////////////

// constexpr LonLat::LonLat(S1Angle lat, S1Angle lon)
//     : coords_(lat.radians(), lon.radians()) {}

// constexpr LonLat::LonLat() : coords_(0, 0) {}

// inline constexpr LonLat LonLat::FromRadians(double lat_radians,
//                                                 double lon_radians) {
//   return LonLat(lat_radians, lon_radians);
// }

// inline constexpr LonLat LonLat::FromDegrees(double lat_degrees,
//                                                 double lon_degrees) {
//   return LonLat(S1Angle::Degrees(lat_degrees), S1Angle::Degrees(lon_degrees));
// }

// inline constexpr LonLat LonLat::FromE5(int32 lat_e5, int32 lon_e5) {
//   return LonLat(S1Angle::E5(lat_e5), S1Angle::E5(lon_e5));
// }

// inline constexpr LonLat LonLat::FromE6(int32 lat_e6, int32 lon_e6) {
//   return LonLat(S1Angle::E6(lat_e6), S1Angle::E6(lon_e6));
// }

// inline constexpr LonLat LonLat::FromE7(int32 lat_e7, int32 lon_e7) {
//   return LonLat(S1Angle::E7(lat_e7), S1Angle::E7(lon_e7));
// }

// inline constexpr LonLat LonLat::FromUnsignedE6(uint32 lat_e6,
//                                                    uint32 lon_e6) {
//   return LonLat(S1Angle::UnsignedE6(lat_e6), S1Angle::UnsignedE6(lon_e6));
// }

// inline constexpr LonLat LonLat::FromUnsignedE7(uint32 lat_e7,
//                                                    uint32 lon_e7) {
//   return LonLat(S1Angle::UnsignedE7(lat_e7), S1Angle::UnsignedE7(lon_e7));
// }

// inline constexpr LonLat LonLat::Invalid() {
//   // These coordinates are outside the bounds allowed by is_valid().
//   return LonLat(M_PI, 2 * M_PI);
// }

// inline S1Angle LonLat::Latitude(const S2Point& p) {
//   // We use atan2 rather than asin because the input vector is not necessarily
//   // unit length, and atan2 is much more accurate than asin near the poles.
//   // The "+ 0.0" is to ensure that points with coordinates of -0.0 and +0.0
//   // (which compare equal) are converted to identical LonLat values, since
//   // even though -0.0 == +0.0 they can be formatted differently.
//   return S1Angle::Radians(atan2(p[2] + 0.0, sqrt(p[0]*p[0] + p[1]*p[1])));
// }

// inline S1Angle LonLat::Longitude(const S2Point& p) {
//   // The "+ 0.0" is to ensure that points with coordinates of -0.0 and +0.0
//   // (which compare equal) are converted to identical LonLat values, since
//   // even though -0.0 == +0.0 and -180 == 180 degrees, they can be formatted
//   // differently.  Also note that atan2(0, 0) is defined to be zero.
//   return S1Angle::Radians(atan2(p[1] + 0.0, p[0] + 0.0));
// }

// inline bool LonLat::is_valid() const {
//   return (std::fabs(lat().radians()) <= M_PI_2 &&
//           std::fabs(lon().radians()) <= M_PI);
// }

// inline LonLat::operator S2Point() const {
//   return ToPoint();
// }

// inline LonLat operator+(const LonLat& a, const LonLat& b) {
//   return LonLat(a.coords_ + b.coords_);
// }

// inline LonLat operator-(const LonLat& a, const LonLat& b) {
//   return LonLat(a.coords_ - b.coords_);
// }

// inline LonLat operator*(double m, const LonLat& a) {
//   return LonLat(m * a.coords_);
// }

// inline LonLat operator*(const LonLat& a, double m) {
//   return LonLat(m * a.coords_);
// }
