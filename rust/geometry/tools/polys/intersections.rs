use crate::{
    data_structures::{BoxIndex, BoxIndexAccessor},
    geometry::{IntersectionOfSegmentsRobust, intersection_of_segments_robust},
};
use alloc::{collections::BTreeMap, rc::Rc, vec, vec::Vec};
use core::{cell::RefCell, cmp::Ordering};
use libm::{fmax, fmin};
use s2json::{BBox, FullXY, GetXY, NewXY, Point};

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    /// The first segment
    pub segment1: Segment,
    /// The second segment
    pub segment2: Segment,
    /// The intersection
    pub point: Point,
    /// The distance along segment1 that the intersection occurs from [0,1]
    pub u: f64,
    /// The distance along segment2 that the intersection occurs from [0,1]
    pub t: f64,
}

/// Local Intersection to a [poly_index][ring_index]
#[derive(Debug, Clone, PartialEq)]
pub struct RingIntersection {
    /// The index of the ring's start
    pub from: usize,
    /// The index of the ring's end
    pub to: usize,
    /// The intersection point
    pub point: Rc<RefCell<Point>>,
    /// The t value (where the intersection occurs on the line segment from 0->1)
    pub t: f64,
    /// The vector from the start to the intersection
    pub t_vec: Point,
    /// The angle of the vector from the start to the intersection
    pub t_angle: f64,
}
impl RingIntersection {
    /// Create a new Intersection
    pub fn new<P: FullXY>(
        from: usize,
        to: usize,
        point: Rc<RefCell<Point>>,
        t: f64,
        t_vec: &P,
        t_angle: f64,
    ) -> Self {
        RingIntersection { from, to, point, t, t_vec: t_vec.into(), t_angle }
    }
}
/// [poly_index][ring_index] -> Intersections
pub type RingIntersectionLookup = BTreeMap<usize, BTreeMap<usize, Vec<RingIntersection>>>;

/// Find all intersections within a collection of polygons
///
/// NOTE: Use [`polygons_intersections_ref`] instead if each polygon is their own ref
///
/// ## Parameters
/// - `polygons`: the collection of polygons
/// - `include_self_intersections`: whether to include self intersections or not
///
/// ## Returns
/// Found intersections
pub fn polygons_intersections<P: GetXY + PartialEq>(
    vector_polygons: &Vec<Vec<Vec<P>>>,
    include_self_intersections: bool,
) -> Vec<Intersection> {
    let mut res: Vec<Intersection> = vec![];
    // build all segments
    let segments = build_polygon_segments(vector_polygons);

    // setup a 2D box index
    let box_index = BoxIndex::new(segments.clone(), None);

    // iterate each segment and check for intersections with other segments
    for segment1 in segments {
        let potential_intersections = box_index.search(
            &segment1.bbox(),
            Some(|seg: &Segment| {
                seg.id != segment1.id
                    && seg.id > segment1.id
                    // if self-intersections are not included skip all segments from the same polyIndex
                    // otherwise skip all segments from the same ringIndex whose end points interact
                    && (if !include_self_intersections {
                            seg.poly_index != segment1.poly_index
                        } else {
                            seg.ring_index != segment1.ring_index
                                || (seg.from != segment1.from
                                    && seg.to != segment1.to
                                    && seg.to != segment1.from
                                    && seg.from != segment1.to)
                        })
            }),
        );
        for segment2 in potential_intersections {
            if let Some(int_point) =
                find_polygon_intersection(vector_polygons, &segment1, &segment2)
            {
                let IntersectionOfSegmentsRobust { point, u, t, .. } = int_point;
                res.push(Intersection { segment1, segment2, point, u, t });
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
pub fn build_polygon_segments<P: GetXY>(vector_polygons: &[Vec<Vec<P>>]) -> Vec<Segment> {
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

/// Run through the vector_polygons and Builds the ring intersection lookup
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
///
/// ## Returns
/// The ring intersection lookup for all rings in the multipolygon collection
pub fn polygons_intersections_lookup<P: FullXY>(
    vector_polygons: &[Vec<Vec<P>>],
    segment_filter: Option<impl Fn(&Segment, &Segment) -> bool>,
) -> RingIntersectionLookup {
    let segments = build_polygon_segments(vector_polygons);
    let mut ring_intersect_lookup: RingIntersectionLookup = BTreeMap::new();

    // setup a 2D box index
    let box_index = BoxIndex::new(segments.clone(), None);
    // iterate each segment and check for intersections with other segments
    for seg1 in segments {
        let potential_intersections = box_index.search(
            &seg1.bbox(),
            Some(|seg2: &Segment| {
                segment_filter.as_ref().map(|f| f(&seg1, &seg2)).unwrap_or_else(|| {
                    // if same id ignore
                    seg2.id != seg1.id &&
                    // only pass forward not backward
                    seg2.id > seg1.id &&
                    // if same poly_index ignore
                    seg2.poly_index != seg1.poly_index
                })
            }),
        );
        for seg2 in potential_intersections {
            let p_int = find_polygon_intersection::<P, P>(&vector_polygons, &seg1, &seg2);
            // ignore points that interact tangentially or precisely at an existing edge or vertex.
            if let Some(int) = p_int {
                let IntersectionOfSegmentsRobust { u, t, point, u_angle, t_angle, u_vec, t_vec } =
                    int;
                // skip if u and t are equal
                if u == t && (u == 0. || u == 1.) {
                    continue;
                }
                let p = Rc::new(RefCell::new((&point).into()));
                // first segment intersection
                let s1 = ring_intersect_lookup
                    .entry(seg1.poly_index)
                    .or_default()
                    .entry(seg1.ring_index)
                    .or_default();
                s1.push(RingIntersection::new(seg1.from, seg1.to, p.clone(), u, &u_vec, u_angle));
                // second segment intersection
                let s2 = ring_intersect_lookup
                    .entry(seg2.poly_index)
                    .or_default()
                    .entry(seg2.ring_index)
                    .or_default();
                s2.push(RingIntersection::new(seg2.from, seg2.to, p, t, &t_vec, t_angle));
            }
        }
    }

    // finally clean the intersections before return
    for (_, polys) in ring_intersect_lookup.iter_mut() {
        for (_, intersections) in polys.iter_mut() {
            *intersections = clean_intersections(intersections);
        }
    }

    ring_intersect_lookup
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
pub fn find_polygon_intersection<P: GetXY + PartialEq, Q: NewXY + Clone>(
    vector_polygons: &[Vec<Vec<P>>],
    segment1: &Segment,
    segment2: &Segment,
) -> Option<IntersectionOfSegmentsRobust<Q>> {
    let p1 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.from];
    let p2 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.to];
    let q1 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.from];
    let q2 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.to];
    intersection_of_segments_robust(
        (p1, p2),
        (q1, q2),
        segment1.poly_index == segment2.poly_index && segment1.ring_index == segment2.ring_index,
    )
}

/// Find all intersections within a collection of polygons where each polygon is their own ref.
///
/// NOTE: Use [`polygons_intersections`] instead if you are referencing a full VectorMultiPolygon instead
///
/// ## Parameters
/// - `polygons`: the collection of polygons
/// - `include_self_intersections`: whether to include self intersections or not
///
/// ## Returns
/// Found intersections
pub fn polygons_intersections_ref<P: GetXY + PartialEq>(
    vector_polygons: &[&Vec<Vec<P>>],
    include_self_intersections: bool,
) -> Vec<Intersection> {
    let mut res: Vec<Intersection> = vec![];
    // build all segments
    let segments = build_polygon_segments_ref(vector_polygons);

    // setup a 2D box index
    let box_index = BoxIndex::new(segments.clone(), None);

    // iterate each segment and check for intersections with other segments
    for segment1 in segments {
        let potential_intersections = box_index.search(
            &segment1.bbox(),
            Some(|seg: &Segment| {
                seg.id != segment1.id
                    && seg.id > segment1.id
                    // if self-intersections are not included skip all segments from the same polyIndex
                    // otherwise skip all segments from the same ringIndex whose end points interact
                    && (if !include_self_intersections {
                            seg.poly_index != segment1.poly_index
                        } else {
                            seg.ring_index != segment1.ring_index
                                || (seg.from != segment1.from
                                    && seg.to != segment1.to
                                    && seg.to != segment1.from
                                    && seg.from != segment1.to)
                        })
            }),
        );
        for segment2 in potential_intersections {
            if let Some(int_point) = find_intersection_ref(vector_polygons, &segment1, &segment2) {
                res.push(Intersection {
                    segment1,
                    segment2,
                    point: int_point.point,
                    u: int_point.u,
                    t: int_point.t,
                });
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
pub fn build_polygon_segments_ref<P: GetXY>(vector_polygons: &[&Vec<Vec<P>>]) -> Vec<Segment> {
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
fn find_intersection_ref<P: GetXY + PartialEq, Q: NewXY + Clone>(
    vector_polygons: &[&Vec<Vec<P>>],
    segment1: &Segment,
    segment2: &Segment,
) -> Option<IntersectionOfSegmentsRobust<Q>> {
    let p1 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.from];
    let p2 = &vector_polygons[segment1.poly_index][segment1.ring_index][segment1.to];
    let q1 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.from];
    let q2 = &vector_polygons[segment2.poly_index][segment2.ring_index][segment2.to];
    intersection_of_segments_robust(
        (p1, p2),
        (q1, q2),
        segment1.poly_index == segment2.poly_index && segment1.ring_index == segment2.ring_index,
    )
}

/// Given a ring's of intersections, clean them up
///
/// ## Parameters
/// - `intersections`: a collection of intersections to clean up
///
/// ## Returns
/// The cleaned up intersections
fn clean_intersections(intersections: &mut Vec<RingIntersection>) -> Vec<RingIntersection> {
    if intersections.len() == 0 {
        return vec![];
    }
    intersections
        .sort_by(|a, b| a.from.cmp(&b.from).then(a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal)));

    // 1) Remove duplicates
    let mut dedup_ints: Vec<RingIntersection> = vec![];
    for int in intersections.iter() {
        if dedup_ints.iter().any(|c| c.from == int.from && c.t == int.t && c.point == int.point) {
            continue;
        }

        dedup_ints.push(int.clone());
    }
    // 2) Cancel out any intersections with other rings we only touch once with a single point
    if dedup_ints.len() == 2 {
        let first = &dedup_ints[0];
        let second = &dedup_ints[1];
        if (first.t == 0. || first.t == 1.)
            && (second.t == 0. || second.t == 1.)
            && first.point == second.point
        {
            return vec![];
        }
    }
    // 3) Intersections whose t values are not 0 or 1 but are equal to the start, end, or other
    // intersections with different t values need to be shifted by the smallest float possible to ensure
    // it doesn't conflict but on the line segment.
    update_intersection_points(&mut dedup_ints);

    dedup_ints
}

/// Update all intersection points to ensure they are not equal to the start or end points if their t
/// values are not 0 or 1.
///
/// When there is an intersection that the resultant point is equal to one of the segment edges,
/// then we shift the point by the smallest float possible.
///
/// NOTE: If we have two or more points that are equal to one of the segment edges BUT the t values
/// are barely different, we need to keep shifting forward as needed.
///
/// NOTE: What if we have TWO points that are equal to one of the segment edges BUT the t values
/// are different? We need to shift again as needed. There are also cases where two different lines
/// intersect another line and the resultant intersection is the same point but the t value along
/// the line is different.
///
/// ## Parameters
/// - `intersections`: the collection of intersections
fn update_intersection_points(intersections: &mut Vec<RingIntersection>) {
    let mut starts = vec![];
    let mut ends = vec![];
    for i in 1..intersections.len() {
        let int = &intersections[i];
        let prev = &intersections[i - 1];
        if int.from != prev.from || int.t == 0. || int.t == 1. || prev.t == 0. || prev.t == 1. {
            continue;
        }
        if i != 0 && int.point == prev.point && int.t != prev.t {
            // because they are sorted by t, starts we want to inc "forward" the NEXT one; ends we want to dec "back" the PREVIOUS one
            if int.t <= 0.5 {
                starts.push(i);
            } else {
                ends.push(i - 1);
            }
        }
    }
    // Choose direction as further away from the end point it's closer to.
    for (i, start) in starts.into_iter().enumerate() {
        let RingIntersection { point, t_vec, .. } = &mut intersections[start];
        let point = &mut point.borrow_mut();
        if t_vec.0 != 0.0 {
            (0..=(i + 1)).for_each(|_| {
                point.0 = if t_vec.0 > 0.0 { point.0.next_up() } else { point.0.next_down() }
            });
        }
        if t_vec.1 != 0.0 {
            (0..=(i + 1)).for_each(|_| {
                point.1 = if t_vec.1 > 0.0 { point.1.next_up() } else { point.1.next_down() }
            });
        }
    }
    for (i, end) in ends.into_iter().enumerate() {
        let RingIntersection { point, t_vec, .. } = &mut intersections[end];
        let point = &mut point.borrow_mut();
        if t_vec.0 != 0.0 {
            (0..=(i + 1)).for_each(|_| {
                point.0 = if t_vec.0 < 0.0 { point.0.next_up() } else { point.0.next_down() }
            });
        }
        if t_vec.1 != 0.0 {
            (0..=(i + 1)).for_each(|_| {
                point.1 = if t_vec.1 < 0.0 { point.1.next_up() } else { point.1.next_down() }
            });
        }
    }
}
