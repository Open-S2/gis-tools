use crate::space::EARTH_RADIUS;
use libm::{asin, atan2, cos, sin};
use s2json::{GetXY, NewXY};

/// Get the destination given a start point, bearing, and distance
///
/// Assumes the starting point is in degrees
///
/// Assumes the bearing is in degrees
///
/// Assumes the distance is in meters
///
/// If no radius is provided, defaults to the Earth's radius
pub fn destination<P: GetXY, Q: NewXY>(
    start: &P,
    bearing: f64,
    distance: f64,
    radius: Option<f64>,
) -> Q {
    let s_lon = start.x().to_radians();
    let s_lat = start.y().to_radians();
    let bearing = bearing.to_radians();
    let radius = radius.unwrap_or(EARTH_RADIUS);
    let radians = distance / radius;

    let e_lat = asin(sin(s_lat) * cos(radians) + cos(s_lat) * sin(radians) * cos(bearing));
    let e_lon = s_lon
        + atan2(sin(bearing) * sin(radians) * cos(s_lat), cos(radians) - sin(s_lat) * sin(e_lat));

    Q::new_xy(e_lon.to_degrees(), e_lat.to_degrees())
}
