/** Mars' radius in meters */
pub const MARS_RADIUS: f64 = 3_389_500.0;
/** Mars' equitorial radius in meters */
pub const MARS_RADIUS_EQUATORIAL: f64 = 3_396_200.0;
/** Mars' polar radius in meters */
pub const MARS_RADIUS_POLAR: f64 = 3_376_200.0;
/** The average circumference of Mars in meters. */
pub const MARS_CIRCUMFERENCE: f64 = 2.0 * core::f64::consts::PI * MARS_RADIUS;
/**
 * The altitude of the highest known point on Mars in meters.
 * https://geology.com/articles/highest-point-on-mars.shtml
 */
pub const MARS_HIGHEST_ALTITUDE: f64 = 21_229.0;
/**
 * The altitude of the lowest known point on Mars in meters.
 * https://en.wikipedia.org/wiki/Hellas_Planitia
 */
pub const MARS_LOWEST_ALTITUDE: f64 = -7_152.0;
