use crate::geometry::{
    S1Angle, S1ChordAngle, S2CellId, S2CellVertices, S2Point, K_MAX_EDGE, K_MAX_LENGTH_2,
};

use alloc::{vec, vec::Vec};

use core::f64::consts::PI;

/// S2Cap represents a disc-shaped region defined by a center and radius.
/// Technically this shape is called a "spherical cap" (rather than disc)
/// because it is not planar; the cap represents a portion of the sphere that
/// has been cut off by a plane.  The boundary of the cap is the circle defined
/// by the intersection of the sphere and the plane.  For containment purposes,
/// the cap is a closed set, i.e. it contains its boundary.
///
/// For the most part, you can use a spherical cap wherever you would use a
/// disc in planar geometry.  The radius of the cap is measured along the
/// surface of the sphere (rather than the straight-line distance through the
/// interior).  Thus a cap of radius Pi/2 is a hemisphere, and a cap of radius
/// Pi covers the entire sphere.
///
/// A cap can also be defined by its center point and height.  The height is
/// simply the distance from the center point to the cutoff plane.  There is
/// also support for empty and full caps, which contain no points and all
/// points respectively.
///
/// This class is intended to be copied by value as desired.  It uses the
/// default copy constructor and assignment operator, however it is not a
/// "plain old datatype" (POD) because it has virtual functions.
///
/// Here are some useful relationships between the cap height (h), the cap
/// radius (r), the maximum chord length from the cap's center (d), and the
/// radius of cap's base (a).
/// ```latext
///     h = 1 - cos(r)
///       = 2 * sin^2(r/2)
///   d^2 = 2 * h
///       = a^2 + h^2
/// ```
pub struct S2Cap<T = ()> {
    /// the center of the cap
    pub center: S2Point,
    /// the radius of the cap
    pub radius: S1ChordAngle,
    /// the data associated with the cap
    pub data: Option<T>,
}
impl<T> S2Cap<T>
where
    T: Clone,
{
    /// Constructs a cap with the given center and radius.
    pub fn new(center: S2Point, radius: S1ChordAngle, data: Option<T>) -> Self {
        S2Cap { center, radius, data }
    }

    /// Return an empty cap, i.e. a cap that contains no points.
    pub fn empty_cap(data: Option<T>) -> Self {
        S2Cap::new(S2Point::new(1.0, 0.0, 0.0), S1ChordAngle::negative_angle(), data)
    }

    /// Return a full cap, i.e. a cap that contains all points.
    pub fn full_cap(data: Option<T>) -> Self {
        S2Cap::new(S2Point::new(1.0, 0.0, 0.0), S1ChordAngle::straight_angle(), data)
    }

    /// Return the area of the cap.
    pub fn area(&self) -> f64 {
        2.0 * PI * f64::max(0., self.height())
    }

    /// Return true if the cap is empty, i.e. it contains no points.
    pub fn is_empty(&self) -> bool {
        self.radius < 0.
    }

    /// Return true if the cap is full, i.e. it contains all points.
    pub fn is_full(&self) -> bool {
        self.radius >= 4.
    }

    /// Returns the height of the cap, i.e. the distance from the center point to
    /// the cutoff plane.
    pub fn height(&self) -> f64 {
        0.5 * self.radius.length2
    }

    /// Constructs a cap with the given center and radius.  A negative radius
    /// yields an empty cap; a radius of 180 degrees or more yields a full cap
    /// (containing the entire sphere).  "center" should be unit length.
    pub fn from_s1_angle(center: S2Point, radius: &S1Angle, data: Option<T>) -> Self {
        S2Cap::new(center, S1ChordAngle::from_angle(radius), data)
    }

    /// Convenience function that creates a cap containing a single point. This
    /// method is more efficient that the S2Cap(center, radius) constructor.
    pub fn from_s2_point(center: S2Point, data: Option<T>) -> Self {
        S2Cap::new(center, S1ChordAngle::zero(), data)
    }

    /// Return the cap radius as an S1Angle.  (Note that the cap angle is stored
    /// internally as an S1ChordAngle, so this method requires a trigonometric
    /// operation and may yield a slightly different result than the value passed
    /// to the (S2Point, S1Angle) constructor.)
    pub fn radius(&self) -> S1Angle {
        self.radius.to_angle()
    }

    /// Returns true if the cap contains the given point.
    /// NOTE: The point "p" should be a unit-length vector.
    pub fn contains_s2_point(&self, p: &S2Point) -> bool {
        S1ChordAngle::from_s2_points(&self.center, p) <= self.radius
    }

    /// Return the complement of the interior of the cap.  A cap and its
    /// complement have the same boundary but do not share any interior points.
    /// The complement operator is not a bijection because the complement of a
    /// singleton cap (containing a single point) is the same as the complement
    /// of an empty cap.
    pub fn complement(&self) -> S2Cap<T> {
        // The complement of a full cap is an empty cap, not a singleton.
        // Also make sure that the complement of an empty cap is full.
        if self.is_full() {
            return S2Cap::empty_cap(self.data.clone());
        } else if self.is_empty() {
            return S2Cap::full_cap(self.data.clone());
        }
        S2Cap::new(
            self.center,
            S1ChordAngle::from_length2(K_MAX_LENGTH_2 - self.radius.length2),
            self.data.clone(),
        )
    }

    /// Return count of vertices the cap contains for the given cell.
    pub fn contains_s2_cell_vertex_count(&self, cell: &S2CellId) -> usize {
        // If the cap does not contain all cell vertices, return false.
        let mut count = 0;
        for vertex in cell.get_vertices() {
            if self.contains_s2_point(&vertex) {
                count += 1;
            }
        }
        count
    }

    /// Return true if the cap contains the given cell.
    pub fn contains_s2_cell(&self, cell: &S2CellId) -> bool {
        // If the cap does not contain all cell vertices, return false.
        // We check the vertices before taking the complement() because we can't
        // accurately represent the complement of a very small cap (a height
        // of 2-epsilon is rounded off to 2).
        let vertices = cell.get_vertices();
        for vertex in vertices {
            if !self.contains_s2_point(&vertex) {
                return false;
            }
        }
        // Otherwise, return true if the complement of the cap does not intersect
        // the cell.  (This test is slightly conservative, because technically we
        !self.complement().intersects_s2_cell(cell, vertices)
    }

    /// Return true if the cap intersects "cell", given that the cap does intersect
    /// any of the cell vertices or edges.
    pub fn intersects_s2_cell_fast(&self, cell: &S2CellId) -> bool {
        // If the cap contains any cell vertex, return true.
        let vertices = cell.get_vertices();
        for vertex in vertices {
            if self.contains_s2_point(&vertex) {
                return true;
            }
        }

        self.intersects_s2_cell(cell, vertices)
    }

    /// Return true if the cap intersects "cell", given that the cap does contain
    /// any of the cell vertices (supplied in "vertices", an array of length 4).
    /// Return true if this cap intersects any point of 'cell' excluding its
    /// vertices (which are assumed to already have been checked).
    pub fn intersects_s2_cell(&self, cell: &S2CellId, vertices: S2CellVertices) -> bool {
        // If the cap is a hemisphere or larger, the cell and the complement of the
        // cap are both convex.  Therefore since no vertex of the cell is contained,
        // no other interior point of the cell is contained either.
        if self.radius >= S1ChordAngle::right_angle() {
            return false;
        }
        // We need to check for empty caps due to the center check just below.
        if self.is_empty() {
            return false;
        }
        // Optimization: return true if the cell contains the cap center.  (This
        // allows half of the edge checks below to be skipped.)
        if cell.contains_s2point(&self.center) {
            return true;
        }

        // At this point we know that the cell does not contain the cap center,
        // and the cap does not contain any cell vertex.  The only way that they
        // can intersect is if the cap intersects the interior of some edge.
        //   let sin2Angle = chordAngleSin2(self.radius);
        let sin2_angle = self.radius.chord_angle_sin2();
        let edges: [S2Point; 4] = cell.get_edges_raw();
        //   for (let k = 0; k < 4; k += 1) {
        for k in 0..4 {
            let edge = edges[k];
            let dot = self.center.dot(&edge);
            if dot > 0. {
                // The center is in the interior half-space defined by the edge.  We don't
                // need to consider these edges, since if the cap intersects this edge
                // then it also intersects the edge on the opposite side of the cell
                // (because we know the center is not contained with the cell).
                continue;
            }
            // The Norm2() factor is necessary because "edge" is not normalized.
            if dot * dot > sin2_angle * edge.norm2() {
                return false; // Entire cap is on the exterior side of this edge.
            }
            // Otherwise, the great circle containing this edge intersects
            // the interior of the cap.  We just need to check whether the point
            // of closest approach occurs between the two edge endpoints.
            let dir = edge.cross(&self.center);
            if dir.dot(&vertices[k]) < 0. && dir.dot(&vertices[(k + 1) & 3]) > 0. {
                return true;
            }
        }

        false
    }

    /// Return the cells that intersect the cap.
    pub fn get_intersecting_cells(&self) -> Vec<S2CellId> {
        let mut res: Vec<S2CellId> = vec![];
        // Find appropriate max depth for radius
        // while loop:
        // - if cell corners are all in cap, store in res.
        // - if cell corners are all outside cap, move on.
        // - if even one cell corner is outside cap:
        // - - if reached max depth, store in res
        // - - if not reached max depth, store children in cache for another pass

        if self.is_empty() {
            return res;
        }
        let mut queue: Vec<S2CellId> = vec![
            S2CellId::from_face(0),
            S2CellId::from_face(1),
            S2CellId::from_face(2),
            S2CellId::from_face(3),
            S2CellId::from_face(4),
            S2CellId::from_face(5),
        ];
        if self.is_full() {
            return queue;
        }
        let max_depth = K_MAX_EDGE.get_closest_level(self.radius.to_angle().radians) as u8;
        loop {
            let Some(cell) = queue.pop() else {
                break;
            }; // cell = queue.pop();
            let vertex_count = self.contains_s2_cell_vertex_count(&cell);
            let max_level = cell.level() >= max_depth;
            if vertex_count == 4 || (vertex_count > 0 && max_level) {
                res.push(cell);
            } else if vertex_count == 0 && !max_level {
                // if cap center is in the cell, then we check all children because the cell is larger than the cap
                if cell.contains_s2point(&self.center) {
                    for child in cell.children(None) {
                        queue.push(child);
                    }
                } else {
                    continue;
                };
            } else {
                if max_level {
                    continue;
                };
                for child in cell.children(None) {
                    queue.push(child);
                }
            }
        }

        res
    }
}
