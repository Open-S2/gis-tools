use crate::geometry::{bearing, destination};
use libm::{pow, sqrt};
use s2json::{GetXY, VectorPoint};

/// Given a linestring in degrees and a distance, create a [`VectorPoint`] along the line
///
/// If no radius is provided, defaults to the Earth's radius
pub fn along_line<P: GetXY>(coords: &[P], distance: f64, radius: Option<f64>) -> VectorPoint {
    let mut travelled = 0.;
    for i in 0..coords.len() {
        if distance >= travelled && i == coords.len() - 1 {
            break;
        } else if travelled >= distance {
            let overshot = distance - travelled;
            if overshot == 0. {
                return VectorPoint::from_xy(coords[i].x(), coords[i].y());
            } else {
                let direction = bearing(&coords[i], &coords[i - 1]) - 180.;
                let interpolated = destination(&coords[i], overshot, direction, radius);
                return interpolated;
            }
        } else {
            travelled += sqrt(
                pow(coords[i].x() - coords[i + 1].x(), 2.)
                    + pow(coords[i].y() - coords[i + 1].y(), 2.),
            );
        }
    }
    let last = coords.len() - 1;
    VectorPoint::from_xy(coords[last].x(), coords[last].y())
}
