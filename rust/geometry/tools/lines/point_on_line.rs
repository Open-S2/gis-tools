use crate::geometry::orient2d;
use libm::{fabs, fmax, fmin};
use s2json::GetXY;

/// Check to see if a point is on a line. Uses predicates to ensure the point is truly on the line
///
/// ## Parameters
/// - `line`: the line to check against
/// - `point`: the point to check if it is on the line
/// - `epsilon`: the buffer to use to check if the point is on the line within epsilon. Defaults to 0
///
/// ## Returns
/// True if the point is on the line
pub fn point_on_line<P: GetXY, Q: GetXY>(line: &[P], point: &Q, eps: Option<f64>) -> bool {
    let eps = eps.unwrap_or(0.0);

    if line.len() < 2 {
        return false;
    }

    let mut i = 0;
    while i < line.len() - 1 {
        // check if in bounding box of each segment
        let a = &line[i];
        let b = &line[i + 1];
        if point.x() >= fmin(a.x(), b.x())
            && point.x() <= fmax(a.x(), b.x())
            && point.y() >= fmin(a.y(), b.y())
            && point.y() <= fmax(a.y(), b.y())
        {
            // lastly check if the point is on the segment
            let cross = orient2d(a.x(), a.y(), b.x(), b.y(), point.x(), point.y());
            if fabs(cross) <= eps {
                return true;
            }
        }

        i += 1;
    }

    false
}
