use crate::{
    data_structures::{BoxIndex, BoxIndexAccessor},
    geometry::intersection_of_segments_robust,
};
use alloc::{vec, vec::Vec};
use libm::{fmax, fmin};
use s2json::{BBox, GetXY, VectorMultiPolygon, VectorPoint};

/// A segment in a polygon
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Segment {
    /// segment id
    pub id: usize,
    /// index in the polys
    pub poly_index: usize,
    /// index in the polys[polygon_index]
    pub ring_index: usize,
    /// index in the polys[polygon_index][ring_index][from]
    pub from: usize,
    /// index in the polys[polygon_index][ring_index][to]
    pub to: usize,
    /// Bounding box
    pub bbox: BBox,
}
impl BoxIndexAccessor for Segment {
    fn bbox(&self) -> BBox {
        self.bbox
    }
}

/// An intersection of two segments
#[derive(Debug, Clone, PartialEq)]
pub struct Intersection<M: Clone> {
    /// The first segment
    pub segment1: Segment,
    /// The second segment
    pub segment2: Segment,
    /// The intersection
    pub point: VectorPoint<M>,
}

/// Find all intersections within a collection of polygons
///
/// ## Parameters
/// - `polygons`: the collection of polygons
///
/// ## Returns
/// Found intersections
pub fn polygons_intersections<M: Clone>(
    vector_polygons: &VectorMultiPolygon<M>,
) -> Vec<Intersection<M>> {
    let mut res: Vec<Intersection<M>> = vec![];
    // build all segments
    let segments = build_segments(vector_polygons);

    // setup a 2D box index
    let box_index = BoxIndex::new(segments.clone(), None);

    // iterate each segment and check for intersections with other segments
    for segment1 in segments {
        let potential_intersections = box_index.search(
            &segment1.bbox(),
            Some(|seg: &Segment| {
                seg.id != segment1.id
                    && seg.poly_index != segment1.poly_index
                    && seg.id > segment1.id
            }),
        );
        for segment2 in potential_intersections {
            if let Some(point) = find_intersection(vector_polygons, &segment1, &segment2) {
                res.push(Intersection { segment1, segment2, point });
            }
        }
    }

    res
}

/// Build all segments
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
///
/// ## Returns
/// The collection of segments
fn build_segments<M: Clone>(vector_polygons: &VectorMultiPolygon<M>) -> Vec<Segment> {
    let mut segments = vec![];

    for (p, polygon) in vector_polygons.iter().enumerate() {
        for (r, ring) in polygon.iter().enumerate() {
            for s in 0..ring.len() - 1 {
                let from = &ring[s];
                let to = &ring[s + 1];
                segments.push(Segment {
                    id: segments.len(),
                    poly_index: p,
                    ring_index: r,
                    from: s,
                    to: s + 1,
                    bbox: BBox::new(
                        fmin(from.x(), to.x()),
                        fmin(from.y(), to.y()),
                        fmax(from.x(), to.x()),
                        fmax(from.y(), to.y()),
                    ),
                });
            }
        }
    }

    segments
}

/// Find the intersection of two segments if it exists
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
/// - `segment1`: the first segment
/// - `segment2`: the second segment
///
/// ## Returns
/// The intersection if it exists. Undefined otherwise.
fn find_intersection<M: Clone>(
    vector_polygons: &VectorMultiPolygon<M>,
    segment1: &Segment,
    segment2: &Segment,
) -> Option<VectorPoint<M>> {
    let p1 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.from];
    let p2 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.to];
    let q1 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.from];
    let q2 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.to];
    intersection_of_segments_robust((p1, p2), (q1, q2), None, None)
}
