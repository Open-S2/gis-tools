/** Mercury's radius in meters */
pub const MERCURY_RADIUS: f64 = 2_439_700.0;
/** Mercury's equitorial radius in meters */
pub const MERCURY_RADIUS_EQUATORIAL: f64 = 2_440_500.0;
/** Mercury's polar radius in meters */
pub const MERCURY_RADIUS_POLAR: f64 = 2_438_300.0;
/** The average circumference of Jupiter in meters. */
pub const MERCURY_CIRCUMFERENCE: f64 = 2.0 * core::f64::consts::PI * MERCURY_RADIUS;
/**
 * The altitude of the highest known point on Mercury in meters.
 * https://www.usgs.gov/news/national-news-release/first-global-topographic-map-mercury-released
 */
pub const MERCURY_HIGHEST_ALTITUDE: f64 = 4_480.0;
/**
 * The altitude of the lowest known point on Mercury in meters.
 * https://www.usgs.gov/news/national-news-release/first-global-topographic-map-mercury-released
 */
pub const MERCURY_LOWEST_ALTITUDE: f64 = -5_380.0;
