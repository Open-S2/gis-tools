use crate::{
    geometry::{euclidean_distance, haversine_distance},
    proj::Coords,
};
use libm::{fmax, fmin, hypot};
use s2json::{GetXY, GetZ};

/// The method to use to calculate the distance
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum DistanceMethod {
    /// Euclidean
    #[default]
    Euclidean,
    /// Haversine
    Haversine,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
struct ClosestIndex {
    index: usize,
    dist: f64,
}

/// Check to see how far away the point is from the line. Supports both Euclidean and Haversine methods
///
/// ## Parameters
/// - `line`: the line to check against
/// - `point`: the point to check if it is on the line
/// - `method`: the method to use, either 'euclidean' or 'haversine'. Defaults to [`DistanceMethod::Euclidean`]
///
/// ## Returns
/// The shortest distance between the point and a line. Returns -1 if line is empty
pub fn point_to_line_distance<P: GetXY + GetZ, Q: GetXY + GetZ>(
    line: &[P],
    point: &Q,
    method: Option<DistanceMethod>,
) -> f64 {
    let method = method.unwrap_or(DistanceMethod::Euclidean);
    let haversine = method == DistanceMethod::Haversine;

    let mut closest_index: Option<ClosestIndex> = None;
    for i in 0..line.len() {
        // get the distance between the point and the line's point at index
        let dist = if haversine {
            haversine_distance(point, &line[i])
        } else {
            euclidean_distance(point, &line[i])
        };
        if dist == 0. {
            return 0.;
        }
        if closest_index.is_none() || dist < closest_index.unwrap().dist {
            closest_index = Some(ClosestIndex { index: i, dist });
        }
    }

    // If there is no closest point, return -1
    if closest_index == None {
        return -1.;
    }
    let closest_index = closest_index.unwrap();

    // If line is a single point, return distance to that point
    if line.len() == 1 {
        return closest_index.dist;
    }

    let curr = &line[closest_index.index];
    // If the point is the start or end of the line, return distance to that point and next/prev
    if closest_index.index == 0 {
        return distance_point_to_segment(curr, &line[closest_index.index + 1], point, method);
    }
    if closest_index.index == line.len() - 1 {
        return distance_point_to_segment(curr, &line[closest_index.index - 1], point, method);
    }
    let prev = &line[closest_index.index - 1];
    let next = &line[closest_index.index + 1];

    // Check against both sides of the line's closest point
    let dist1 = distance_point_to_segment(curr, prev, point, method);
    let dist2 = distance_point_to_segment(curr, next, point, method);

    if dist1 < dist2 { dist1 } else { dist2 }
}

/// Get the distance between a point and a segment
///
/// @param a - the segment start point
/// @param b - the segment end point
/// @param p - the point to check
/// @param method - the method to use, either 'euclidean' or 'haversine'. Defaults to 'euclidean'
/// @returns - the distance
fn distance_point_to_segment<A: GetXY, B: GetXY, P: GetXY>(
    a: &A,
    b: &B,
    p: &P,
    method: DistanceMethod,
) -> f64 {
    if method == DistanceMethod::Haversine {
        // approximate by sampling along the great-circle segment
        // project p onto AB using Euclidean math in lat/lon degrees
        // but compute distances with haversine_distance
        let abx = b.x() - a.x();
        let aby = b.y() - a.y();
        let apx = p.x() - a.x();
        let apy = p.y() - a.y();
        let ab_len_sq = abx * abx + aby * aby;

        if ab_len_sq == 0. {
            return haversine_distance(p, a);
        }

        let mut t = (apx * abx + apy * aby) / ab_len_sq;
        t = fmax(0., fmin(1., t));

        let proj = Coords::new_xy(a.x() + t * abx, a.y() + t * aby);
        return haversine_distance(p, &proj);
    }

    // Euclidean fallback
    let ab = Coords::new_xy(b.x() - a.x(), b.y() - a.y());
    let ap = Coords::new_xy(p.x() - a.x(), p.y() - a.y());
    let ab_len_sq = ab.x() * ab.x() + ab.y() * ab.y();

    if ab_len_sq == 0. {
        return hypot(p.x() - a.x(), p.y() - a.y());
    }

    let mut t = (ap.x() * ab.x() + ap.y() * ab.y()) / ab_len_sq;
    t = fmax(0., fmin(1., t));

    let closest = Coords::new_xy(a.x() + t * ab.x(), a.y() + t * ab.y());
    hypot(p.x() - closest.x(), p.y() - closest.y())
}
