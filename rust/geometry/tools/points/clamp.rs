use s2json::{GetXY, SetXY};

/// Trait to ensure a WGS84 point is valid
pub trait ClampWGS84Point {
    /// Updates the WGS84 point's x and y values as needed to be valid WGS84
    fn clamp_wgs84(&mut self);
}
impl<T: GetXY + SetXY> ClampWGS84Point for T {
    fn clamp_wgs84(&mut self) {
        clamp_wgs84_point(self);
    }
}

/// Updates the WGS84 point's x and y values as needed to be valid WGS84
///
/// ## Parameters
/// `point`: the WGS 84 point to clamp/wrap
///
/// ## Returns
/// The point itself post update
pub fn clamp_wgs84_point<P: GetXY + SetXY>(point: &mut P) {
    let x = point.x();
    let y = point.y();
    // Don't touch the point if it's already in bounds
    if x < -180. || x >= 180. {
        point.set_x(((((x + 180.) % 360.) + 360.) % 360.) - 180.);
    }
    point.set_y(y.clamp(-90., 90.));
}
