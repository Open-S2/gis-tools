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
/// ## Parameters
/// - `a`: the first segment
/// - `b`: the second segment
/// - `a_ring_id`: the ring id of the first segment if provided
/// - `b_ring_id`: the ring id of the second segment if provided
///
/// ## Returns
/// A point if the segments intersect where the intersection occurs, otherwise undefined
pub fn intersection_of_segments_robust<P: GetXY + PartialEq, Q: NewXY>(
    a: (&P, &P),
    b: (&P, &P),
    a_ring_id: Option<usize>,
    b_ring_id: Option<usize>,
) -> Option<IntersectionOfSegments<Q>> {
    let x1 = a.0.x();
    let y1 = a.0.y();
    let x2 = a.1.x();
    let y2 = a.1.y();
    let x3 = b.0.x();
    let y3 = b.0.y();
    let x4 = b.1.x();
    let y4 = b.1.y();

    if a_ring_id == b_ring_id {
        if a.1 == b.0 || a.1 == b.1 || a.0 == b.0 || a.0 == b.1 {
            return None;
        }
    } else {
        if a.1 == b.0 {
            return Some(IntersectionOfSegments { point: Q::new_xy(x2, y2), u: 1., t: 0. });
        }
        if a.1 == b.1 {
            return Some(IntersectionOfSegments { point: Q::new_xy(x2, y2), u: 1., t: 1. });
        }
        if a.0 == b.0 {
            return Some(IntersectionOfSegments { point: Q::new_xy(x1, y1), u: 0., t: 0. });
        }
        if a.0 == b.1 {
            return Some(IntersectionOfSegments { point: Q::new_xy(x1, y1), u: 0., t: 1. });
        }
    }

    let orient1 = orient2d(x1, y1, x2, y2, x3, y3);
    let orient2 = orient2d(x1, y1, x2, y2, x4, y4);

    if (orient1 > 0. && orient2 > 0.) || (orient1 < 0. && orient2 < 0.) {
        return None;
    }

    let denom = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);
    let nume_a = (x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3);
    let nume_b = (x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3);

    if denom == 0. {
        return None;
    }

    let u_a = nume_a / denom;
    let u_b = nume_b / denom;

    if (0. ..=1.).contains(&u_a) && (0. ..=1.).contains(&u_b) {
        Some(IntersectionOfSegments {
            point: Q::new_xy(x1 + u_a * (x2 - x1), y1 + u_a * (y2 - y1)),
            u: u_a,
            t: u_b,
        })
    } else {
        None
    }
}
