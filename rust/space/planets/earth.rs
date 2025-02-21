/// Earth's radius in meters
pub const EARTH_RADIUS: f64 = 6_371_008.8;
/// Earth's equatorial radius in meters
pub const EARTH_RADIUS_EQUATORIAL: f64 = 6_378_137.0;
/// Earth's polar radius in meters
pub const EARTH_RADIUS_POLAR: f64 = 6_356_752.3;
/// The average circumference of the world in meters
pub const EARTH_CIRCUMFERENCE: f64 = 2.0 * core::f64::consts::PI * EARTH_RADIUS;
/**
 * Returns the altitude of the lowest known point on Earth in meters. The lowest known
 * point on Earth is the Challenger Deep with an altitude of 10898 meters
 * below the surface of the spherical earth.
 */
pub const LOWEST_ALTITUDE: f64 = -10_898.0;
/**
 * Returns the altitude of the highest known point on Earth in meters. The highest
 * known point on Earth is Mount Everest with an altitude of 8846 meters
 * above the surface of the spherical earth.
 */
pub const HIGHEST_ALTITUDE: f64 = 8_846.0;
