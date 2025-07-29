use libm::{atan2, cos, sin};
use s2json::GetXY;

/// Get the bearing in degrees between two points
pub fn bearing<P1: GetXY, P2: GetXY>(start: &P1, end: &P2) -> f64 {
    let lat1 = start.y().to_radians();
    let lat2 = end.y().to_radians();
    let lon1 = start.x().to_radians();
    let lon2 = end.x().to_radians();
    let y = sin(lon2 - lon1) * cos(lat2);
    let x = cos(lat1) * sin(lat2) - sin(lat1) * cos(lat2) * cos(lon2 - lon1);

    (atan2(y, x).to_degrees() + 360.) % 360.
}
