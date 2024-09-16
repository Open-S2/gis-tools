// const S1Angle = @import("s1/angle.zig").S1Angle;
// const S1ChordAngle = @import("s1/chordAngle.zig").S1ChordAngle;
// const LonLat = @import("lonlat.zig").LonLat;
// const S2Point = @import("point.zig").S2Point;
// use core::cmp::min;

// use crate::lonlat::LonLat;
use crate::s1::angle::S1Angle;
// use crate::s2::s2point::S2Point;

// The earth modeled as a sphere.  There are lots of convenience functions so
// that it doesn't take 2 lines of code just to do a single conversion.

// These functions convert between distances on the unit sphere
// (expressed as angles subtended from the sphere's center) and
// distances on the Earth's surface.  This is possible only because
// the Earth is modeled as a sphere; otherwise a given angle would
// correspond to a range of distances depending on where the
// corresponding line segment was located.

/// http://en.wikipedia.org/wiki/Haversine_formula
/// Haversine(x) has very good numerical stability around zero.
/// Haversine(x) == (1-cos(x))/2 == sin(x/2)^2; must be implemented with the
/// second form to reap the numerical benefits.
// pub fn haversine(radians: f64) -> f64 {
//     let sin_half = f64::cos(radians / 2.);
//     sin_half * sin_half
// }

// Functions for converting distances to angles:
pub fn meters_to_angle(meters: f64) -> S1Angle {
    S1Angle::new(meters_to_radians(meters))
}
// pub fn meters_to_chord_angle(meters: f64) -> S1ChordAngle {
//     let angle = metersToAngle(meters);
//     S1ChordAngle.FromS1Angle(&angle)
// }
pub fn meters_to_radians(meters: f64) -> f64 {
    meters / RADIUS_METERS
}

// Functions for converting angles to distances:
// NOTE: S1Angle has a "toMeters" function
pub fn to_meters_s1_angle(angle: &S1Angle) -> f64 {
    angle.radians * RADIUS_METERS
}
// NOTE: S1ChordAngle has a "toMeters" function
// pub fn to_meters_s1_chord_angle(cangle: &S1ChordAngle) -> f64 {
//     let angle = cangle.toAngle();
//     to_meters_s1_angle(&angle)
// }
pub fn radians_to_meters(radians: f64) -> f64 {
    radians * RADIUS_METERS
}

// Like the above, but where distances are expressed in kilometers:
pub fn km_to_angle(km: f64) -> S1Angle {
    S1Angle::new(km_to_radians(km))
}
// pub fn km_to_chord_angle(km: f64) -> S1ChordAngle {
//     return S1ChordAngle.Init(km_to_angle(km));
// }
pub fn km_to_radians(km: f64) -> f64 {
    km / RADIUS_KM
}
// NOTE: S1Angle has a "toKm" function
pub fn to_km_s1_angle(angle: &S1Angle) -> f64 {
    angle.radians * RADIUS_KM
}
// NOTE: S1ChordAngle has a "toKm" function
// pub fn to_km_s1_chord_angle(cangle: &S1ChordAngle) -> f64 {
//     return to_km_s1_angle(cangle.toAngle());
// }
pub fn radians_to_km(radians: f64) -> f64 {
    radians * RADIUS_KM
}

// These functions convert between areas on the unit sphere
// (as returned by the S2 library) and areas on the Earth's surface.
// Note that the area of a region on the unit sphere is equal to the
// solid angle it subtends from the sphere's center (measured in steradians).
pub fn square_km_to_steradians(km2: f64) -> f64 {
    km2 / (RADIUS_KM * RADIUS_KM)
}
pub fn square_meters_to_steradians(m2: f64) -> f64 {
    m2 / (RADIUS_METERS * RADIUS_METERS)
}
pub fn steradians_to_square_km(steradians: f64) -> f64 {
    steradians * RADIUS_KM * RADIUS_KM
}
pub fn steradians_to_square_meters(steradians: f64) -> f64 {
    steradians * RADIUS_METERS * RADIUS_METERS
}

/// Convenience functions for the frequent case where you need to call
/// ToRadians in order to convert an east-west distance on the globe to
/// radians. The output is a function of how close to the poles you are
/// (i.e. at the bulge at the equator, one unit of longitude represents a
/// much farther distance). The function will never return more than 2*PI
/// radians, even if you're trying to go 100 million miles west at the north
/// pole.
// pub fn meters_to_longitude_radians(meters: f64, latitude_radians: f64) -> f64 {
//     let scalar = cos(latitude_radians);
//     if (scalar == 0) { return std.math.pi * 2; }

//     min(metersToRadians(meters) / scalar, std.math.pi * 2)
// }
// pub fn km_to_longitude_radians(km: f64, latitude_radians: f64) -> f64 {
//     metersToLongitudeRadians(1000 * km, latitude_radians)
// }

/// Computes the initial bearing from a to b. This is the bearing an observer
/// at point a has when facing point b. A bearing of 0 degrees is north, and it
/// increases clockwise (90 degrees is east, etc).
///
/// If a == b, a == -b, or a is one of the Earths' poles, the return value is
/// undefined.
/// Sourced from http://www.movable-type.co.uk/scripts/latlong.html.
// pub fn get_initial_bearing(a: &LonLat, b: &LonLat) -> S1Angle {
//     let lat1 = a.lat().radians;
//     let cos_lat2 = cos(b.lat().radians);
//     let lat_diff = b.lat().radians - a.lat().radians;
//     let lon_diff = b.lng().radians - a.lng().radians;

//     let x = sin(lat_diff) + sin(lat1) * cos_lat2 * 2 * haversine(lon_diff);
//     let y = sin(lon_diff) * cos_lat2;
//     return S1Angle.Init(std.math.atan2(y, x));
// }

/// Returns the distance between two points.
// pub fn get_distance_meters_s2_point(a: &S2Point, b: &S2Point) -> f64 {
//     return a.angle(b).toMeters();
// }
// pub fn get_distance_meters_s2_lat_lng(a: &LonLat, b: &LonLat) -> f64 {
//     return a.getDistance(b).toMeters();
// }
// pub fn get_distance_km_s2_point(a: &S2Point, b: &S2Point) -> f64 {
//     return a.angle(b).toKm();
// }
// pub fn get_distance_km_s2_lat_lng(a: &LonLat, b: &LonLat) -> f64 {
//     return a.getDistance(b).toKm();
// }

/// Returns the Earth's mean radius, which is the radius of the equivalent
/// sphere with the same surface area.  According to NASA, this value is
/// 6371.01 +/- 0.02 km.  The equatorial radius is 6378.136 km, and the polar
/// radius is 6356.752 km.  They differ by one part in 298.257.
///
/// Reference: http://ssd.jpl.nasa.gov/phys_props_earth.html, which quotes
/// Yoder, C.F. 1995. "Astrometric and Geodetic Properties of Earth and the
/// Solar System" in Global Earth Physics, A Handbook of Physical Constants,
/// AGU Reference Shelf 1, American Geophysical Union, Table 2.
pub const RADIUS_METERS: f64 = 6371010.0;
pub const RADIUS_KM: f64 = 0.001 * RADIUS_METERS;

// Convenience functions.

/// Returns the altitude of the lowest known point on Earth. The lowest known
/// point on Earth is the Challenger Deep with an altitude of -10898 meters
/// above the surface of the spherical earth.
pub const LOWEST_ALTITUDE_METERS: f64 = -10898.0;

/// Returns the altitude of the highest known point on Earth. The highest
/// known point on Earth is Mount Everest with an altitude of 8846 meters
/// above the surface of the spherical earth.
pub const HIGHEST_ALTITUDE_METERS: f64 = 8846.0;

// test "TestAngleConversion" {
//     // Functions that use meters:
//     try testing.expectEqual(meters_to_angle(RADIUS_METERS).radians, 1.0);
//     const ca = meters_to_chord_angle(RADIUS_METERS);
//     try testing.expectEqual(ca.radians(), 1.0);
//     try testing.expectEqual(meters_to_radians(radians_to_km(0.3) * 1000.0), 0.3);
//     try testing.expectEqual(to_meters_s1_angle(&S1Angle.Degrees(180.0)), RADIUS_METERS * std.math.pi);
//     const ca2 = S1ChordAngle.Degrees(180.0);
//     try testing.expectEqual(to_meters_s1_chord_angle(&ca2), RADIUS_METERS * std.math.pi);
//     try testing.expectEqual(radians_to_meters(km_to_radians(2.5)), 2500.0);
// }

// TEST(S2EarthTest, TestAngleConversion) {

//   // Functions that use kilometers:
//   EXPECT_DOUBLE_EQ(kmToAngle(RADIUS_KM()).radians(), 1);
//   EXPECT_DOUBLE_EQ(kmToChordAngle(RADIUS_KM()).radians(), 1);
//   EXPECT_DOUBLE_EQ(kmToRadians(RADIUS_METERS / 1000), 1);
//   EXPECT_DOUBLE_EQ(ToKm(S1Angle.Init(0.5)),
//                    0.5 * RADIUS_KM());
//   EXPECT_DOUBLE_EQ(ToKm(S1ChordAngle.Radians(0.5)),
//                    0.5 * RADIUS_KM());
//   EXPECT_DOUBLE_EQ(radiansToKm(0.5), 0.5 * RADIUS_KM());

//   // Functions that use util.units.Meters (which only has "float" precision,
//   // but fortunately Radius() is exactly representable as "float"):
//   EXPECT_DOUBLE_EQ(ToAngle(Radius()).radians(), 1);
//   EXPECT_DOUBLE_EQ(ToChordAngle(Radius()).radians(), 1);
//   EXPECT_DOUBLE_EQ(ToRadians(Radius()), 1);
//   EXPECT_FLOAT_EQ(ToDistance(S1Angle.Init(2)).value(),
//                   2 * RADIUS_METERS);
//   EXPECT_FLOAT_EQ(ToDistance(S1ChordAngle.Radians(0.5)).value(),
//                   0.5 * RADIUS_METERS);
//   EXPECT_FLOAT_EQ(RadiansToDistance(1.5).value(),
//                   1.5 * RADIUS_METERS);
// }

// TEST(S2EarthTest, TestSolidAngleConversion) {
//   EXPECT_DOUBLE_EQ(1, SquareKmToSteradians(
//                           std.pow(RADIUS_METERS / 1000, 2)));
//   EXPECT_DOUBLE_EQ(std.pow(0.5 * RADIUS_KM(), 2),
//                    SteradiansToSquareKm(std.pow(0.5, 2)));
//   EXPECT_DOUBLE_EQ(std.pow(0.3, 2), SquareMetersToSteradians(std.pow(
//                                          radiansToKm(0.3) * 1000, 2)));
//   EXPECT_DOUBLE_EQ(std.pow(2500, 2),
//                    SteradiansToSquareMeters(
//                        std.pow(KmToRadians(2.5), 2)));
// }

// TEST(S2EarthTest, TestToLongitudeRadians) {
//   const util.units.Meters earth_radius = Radius();

//   // At the equator, ToLongitudeRadians behaves exactly like ToRadians.
//   EXPECT_DOUBLE_EQ(ToLongitudeRadians(earth_radius, 0), 1);

//   // The closer we get to the poles, the more radians we need to go the same
//   // distance.
//   EXPECT_GT(ToLongitudeRadians(earth_radius, 0.5),
//             ToLongitudeRadians(earth_radius, 0.4));

//   // At the poles, we should return 2PI radians instead of dividing by 0.
//   EXPECT_DOUBLE_EQ(ToLongitudeRadians(earth_radius, M_PI_2), M_PI * 2);

//   // Within epsilon of the poles, we should still return 2PI radians instead
//   // of directing the caller to take thousands of radians around.
//   EXPECT_DOUBLE_EQ(ToLongitudeRadians(earth_radius, M_PI_2 - 1e-4),
//                    M_PI * 2);

//   // Check that the "meters" and "kilometer" versions are compatible.
//   EXPECT_EQ(ToLongitudeRadians(earth_radius, 0.5),
//             MetersToLongitudeRadians(earth_radius.value(), 0.5));
//   EXPECT_DOUBLE_EQ(
//       ToLongitudeRadians(earth_radius, 0.5),
//       KmToLongitudeRadians(earth_radius.value() / 1000.0, 0.5));
// }

// TEST(S2EarthTest, TestGetInitialBearing) {
//   struct TestConfig {
//     std.string description;
//     S2LatLng a;
//     S2LatLng b;
//     S1Angle bearing;
//   } test_configs[] = {
//       {"Westward on equator", S2LatLng.FromDegrees(0, 50),
//        S2LatLng.FromDegrees(0, 100), S1Angle.Degrees(90)},
//       {"Eastward on equator", S2LatLng.FromDegrees(0, 50),
//        S2LatLng.FromDegrees(0, 0), S1Angle.Degrees(-90)},
//       {"Northward on meridian", S2LatLng.FromDegrees(16, 28),
//        S2LatLng.FromDegrees(81, 28), S1Angle.Degrees(0)},
//       {"Southward on meridian", S2LatLng.FromDegrees(24, 64),
//        S2LatLng.FromDegrees(-27, 64), S1Angle.Degrees(180)},
//       {"Towards north pole", S2LatLng.FromDegrees(12, 76),
//        S2LatLng.FromDegrees(90, 50), S1Angle.Degrees(0)},
//       {"Towards south pole", S2LatLng.FromDegrees(-35, 105),
//        S2LatLng.FromDegrees(-90, -120), S1Angle.Degrees(180)},
//       {"Spain to Japan", S2LatLng.FromDegrees(40.4379332, -3.749576),
//        S2LatLng.FromDegrees(35.6733227, 139.6403486), S1Angle.Degrees(29.2)},
//       {"Japan to Spain", S2LatLng.FromDegrees(35.6733227, 139.6403486),
//        S2LatLng.FromDegrees(40.4379332, -3.749576), S1Angle.Degrees(-27.2)},
//   };

//   for (const TestConfig& config : test_configs) {
//     const S1Angle bearing = GetInitialBearing(config.a, config.b);
//     const S1Angle angle_diff = (bearing - config.bearing).Normalized().abs();
//     EXPECT_LE(angle_diff.degrees(), 1e-2)
//         << "GetInitialBearing() test failed on: " << config.description
//         << ". Expected " << config.bearing << ", got " << bearing;
//   }
// }

// TEST(S2EarthTest, TestGetDistance) {
//   S2Point north(0, 0, 1);
//   S2Point south(0, 0, -1);
//   S2Point west(0, -1, 0);

//   EXPECT_FLOAT_EQ(
//       util.units.Miles(GetDistance(north, south)).value(),
//       util.units.Miles(M_PI * Radius()).value());
//   EXPECT_DOUBLE_EQ(GetDistanceKm(west, west), 0);
//   EXPECT_DOUBLE_EQ(GetDistanceMeters(north, west),
//                    M_PI_2 * RADIUS_METERS);

//   EXPECT_FLOAT_EQ(
//       util.units.Feet(GetDistance(S2LatLng.FromDegrees(0, -90),
//                                              S2LatLng.FromDegrees(-90, -38)))
//           .value(),
//       util.units.Feet(GetDistance(west, south)).value());

//   EXPECT_DOUBLE_EQ(GetDistanceKm(S2LatLng.FromRadians(0, 0.6),
//                                           S2LatLng.FromRadians(0, -0.4)),
//                    RADIUS_KM());

//   EXPECT_DOUBLE_EQ(GetDistanceMeters(S2LatLng.FromDegrees(80, 27),
//                                               S2LatLng.FromDegrees(55, -153)),
//                    1000 * RADIUS_KM() * M_PI / 4);
// }
