use crate::geometry::orient2d;
use alloc::fmt::Debug;
use s2json::{GetXY, NewXY, Point};

/// An intersection of two segments
/// u and t are where the intersection occurs
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntersectionOfSegments<Q: NewXY> {
    /// the point of intersection
    pub point: Q,
    /// where along the first segment the intersection occurs
    pub u: f64,
    /// where along the second segment the intersection occurs
    pub t: f64,
}

/// An intersection of two segments including displacement vectors
/// u and t are where the intersection occurs
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntersectionOfSegmentsRobust<Q: NewXY> {
    /// the point of intersection
    pub point: Q,
    /// where along the first segment the intersection occurs
    pub u: f64,
    /// where along the second segment the intersection occurs
    pub t: f64,
    /// displacement vector from the first segment
    pub u_vec: Q,
    /// displacement vector from the second segment
    pub t_vec: Q,
}
impl<Q: NewXY> IntersectionOfSegmentsRobust<Q> {
    /// Create a new IntersectionOfSegmentsRobust
    pub fn new(x: f64, y: f64, u: f64, t: f64, u_vec: Q, t_vec: Q) -> Self {
        Self { point: Q::new_xy(x, y), u, t, u_vec, t_vec }
    }
}

/// Find the intersection of two segments
///
/// NOTE: Segments that are only touching eachothers endpoints are considered intersections
///
/// ## Parameters
/// - `a`: the first segment
/// - `b`: the second segment
///
/// ## Returns
/// A point if the segments intersect where the intersection occurs, otherwise undefined
pub fn intersection_of_segments<P: GetXY, Q: NewXY>(
    a: (&P, &P),
    b: (&P, &P),
) -> Option<IntersectionOfSegments<Q>> {
    let (p, p2) = a;
    let (q, q2) = b;

    let r = Point::new_xy(p2.x() - p.x(), p2.y() - p.y());
    let s = Point::new_xy(q2.x() - q.x(), q2.y() - q.y());

    let cross = r.x() * s.y() - r.y() * s.x();
    if cross == 0. {
        return None;
    }

    let u = ((q.x() - p.x()) * s.y() - (q.y() - p.y()) * s.x()) / cross;
    let t = ((q.x() - p.x()) * r.y() - (q.y() - p.y()) * r.x()) / cross;

    if (0. ..=1.).contains(&t) && (0. ..=1.).contains(&u) {
        Some(IntersectionOfSegments {
            point: Q::new_xy(p.x() + u * r.x(), p.y() + u * r.y()),
            u,
            t,
        })
    } else {
        None
    }
}

/// Find the intersection of two segments. A more robust approach that uses predicates to ensure no
/// false positives/negatives
///
/// NOTE:
/// If the segments are touching at end points, they PASS in this function. However, the caviat is
/// that if the segments are coming from the same ring, then the result will be undefined (not
/// considered an intersection).
///
/// NOTE:
/// The resultant vectors are displacement vectors not normalized.
///
/// ## Parameters
/// - `a`: the first segment
/// - `b`: the second segment
/// - `same_ring`: if both segments are from the same ring. By default it assumes they are
/// - `a_ring_id`: the ring id of the first segment if provided
/// - `b_ring_id`: the ring id of the second segment if provided
///
/// ## Returns
/// A point if the segments intersect where the intersection occurs, otherwise undefined
pub fn intersection_of_segments_robust<P: GetXY + PartialEq, Q: NewXY>(
    a: (&P, &P),
    b: (&P, &P),
    same_ring: bool,
) -> Option<IntersectionOfSegmentsRobust<Q>> {
    let x1 = a.0.x();
    let y1 = a.0.y();
    let x2 = a.1.x();
    let y2 = a.1.y();
    let x3 = b.0.x();
    let y3 = b.0.y();
    let x4 = b.1.x();
    let y4 = b.1.y();
    let dx_a = x2 - x1;
    let dy_a = y2 - y1;
    let dx_b = x4 - x3;
    let dy_b = y4 - y3;

    // build numerators and denominator. Extrapolate vectors from them
    let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let nume_a = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let nume_b = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);
    let u_a = nume_a / denom;
    let u_b = nume_b / denom;
    let u_vec = Q::new_xy(u_a * dx_a, u_a * dy_a);
    let t_vec = Q::new_xy(u_b * dx_b, u_b * dy_b);

    if same_ring {
        if a.1 == b.0 || a.1 == b.1 || a.0 == b.0 || a.0 == b.1 {
            return None;
        }
    } else {
        if a.1 == b.0 {
            return Some(IntersectionOfSegmentsRobust::new(x2, y2, 1., 0., u_vec, t_vec));
        }
        if a.1 == b.1 {
            return Some(IntersectionOfSegmentsRobust::new(x2, y2, 1., 1., u_vec, t_vec));
        }
        if a.0 == b.0 {
            return Some(IntersectionOfSegmentsRobust::new(x1, y1, 0., 0., u_vec, t_vec));
        }
        if a.0 == b.1 {
            return Some(IntersectionOfSegmentsRobust::new(x1, y1, 0., 1., u_vec, t_vec));
        }
    }

    if denom == 0. {
        return None;
    }
    let orient1 = orient2d(x1, y1, x2, y2, x3, y3);
    let orient2 = orient2d(x1, y1, x2, y2, x4, y4);
    if (orient1 > 0. && orient2 > 0.) || (orient1 < 0. && orient2 < 0.) {
        return None;
    }

    if (0. ..=1.).contains(&u_a) && (0. ..=1.).contains(&u_b) {
        let x = x1 + u_a * (x2 - x1);
        let y = y1 + u_a * (y2 - y1);
        Some(IntersectionOfSegmentsRobust::new(x, y, u_a, u_b, u_vec, t_vec))
    } else {
        None
    }
}
