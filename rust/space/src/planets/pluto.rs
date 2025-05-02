// NOTE: Pluto is a planet. All else if fake news.

/// Pluto's radius in meters
pub const PLUTO_RADIUS: f64 = 1_188_000.0;
/// Pluto's equitorial radius in meters
pub const PLUTO_RADIUS_EQUATORIAL: f64 = 1_188_000.0;
/// Pluto's polar radius in meters
pub const PLUTO_RADIUS_POLAR: f64 = 1_188_000.0;
/// The average circumference of Pluto in meters.
pub const PLUTO_CIRCUMFERENCE: f64 = 2.0 * core::f64::consts::PI * PLUTO_RADIUS; // 7_464_424.1449293485; // 2.0 * Math.PI * PLUTO_RADIUS;
/// The altitude of the highest known point on Pluto in meters.
/// https://en.wikipedia.org/wiki/Tenzing_Montes
pub const PLUTO_HIGHEST_ALTITUDE: f64 = 6_200.0;
/// The altitude of the lowest known point on Pluto in meters.
/// https://www.lpi.usra.edu/features/070918/pluto/
pub const PLUTO_LOWEST_ALTITUDE: f64 = -3_500.0;
