use alloc::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    vec,
    vec::Vec,
};
use core::{cell::RefCell, cmp::Ordering, f64::consts::PI};
use libm::atan2;
use s2json::{BBox, GetXY, NewXY};

use crate::{
    data_structures::{BoxIndex, BoxIndexAccessor},
    geometry::{Area, Segment, build_polygon_segments, find_polygon_intersection},
};

/// A simpler way to define what is required from a point to be used in the union
pub trait UnionPoint: GetXY + NewXY + Clone + PartialEq + Ord {}
impl<T> UnionPoint for T where T: GetXY + NewXY + Clone + PartialEq + Ord {}

/// Reconstructing a poly line that interacts with intersections
#[derive(Debug, Clone, PartialEq)]
pub struct PolyPath<P: UnionPoint> {
    outer: Option<Vec<P>>,
    holes: Vec<Vec<P>>,
    // indexes of the polygons in the multipolygon. So we can quickly consume holes.
    polys_consumed: BTreeSet<usize>,
    bbox: BBox,
}
/// A reference to a PolyPath wrapped in an RC & RefCell
pub type PolyPathRef<P> = Rc<RefCell<PolyPath<P>>>;
impl<P: UnionPoint> PolyPath<P> {
    /// Create a new PolyPath as a PolyPathRef
    pub fn new_ref(ring: Vec<P>, poly_index: usize, is_outer: bool) -> PolyPathRef<P> {
        Rc::new(RefCell::new(PolyPath::new(ring, poly_index, is_outer)))
    }
    /// Create a new PolyPath
    pub fn new(ring: Vec<P>, poly_index: usize, is_outer: bool) -> Self {
        let bbox = BBox::from_linestring(&ring);
        let mut outer = None;
        let mut holes = vec![];
        if is_outer {
            outer = Some(ring);
        } else {
            holes.push(ring);
        }
        let mut polys_consumed = BTreeSet::new();
        polys_consumed.insert(poly_index);
        PolyPath { outer, holes, polys_consumed, bbox }
    }

    /// Add a collection of chunks built into a line+bbox to the path
    pub fn add_chunks(
        &mut self,
        ring: Vec<P>,
        poly_indexes: &mut BTreeSet<usize>,
        bbox: &BBox,
        is_ccw: bool,
        was_hole: bool,
    ) {
        self.polys_consumed.append(poly_indexes);

        // If one poly outer ring is entirely in another poly AND its CCW, it gets "consumed" (deleted. this is
        // because of the ordering, the first chunk to be an outer will be the one creating the path,
        // so we know all future CCW chunks that share a path will be "outers" that are inside the existing
        // path outer)
        // If one poly outer ring is entirely in another poly AND its CW, it converts to a hole
        // If one poly inner ring is CW, it gets consumed by an associated outer
        // If one poly inner ring is CCW, remove it
        if is_ccw {
            if was_hole {
                return;
            }
            if self.outer.is_none() {
                self.outer = Some(ring);
            } else {
                // If this bbox is smaller than the existing outer, delete. Otherwise replace
                if bbox.inside(&self.bbox) {
                    return;
                } else {
                    self.outer = Some(ring);
                }
            }
        } else {
            self.holes.push(ring);
        }

        self.bbox.merge(bbox);
    }
}

/// A path/piece/chunk from a polygon
#[derive(Debug, Clone, PartialEq)]
pub struct PolyChunk<P: UnionPoint> {
    visted: bool,
    poly_index: usize,
    ring_index: usize,
    bbox: BBox,
    line: Vec<P>, // Always stars with either the beginning of the poly ring OR an intersection point.
    next: IntersectionPointRef<P>, // can point to just one or multiple chunks. Many polys can touch the same point. If none provided could be a start-end point
}
/// A reference to a PolyChunk wrapped in an RC & RefCell
pub type PolyChunkRef<P> = Rc<RefCell<PolyChunk<P>>>;
impl<P: UnionPoint> PolyChunk<P> {
    /// Create a new PolyChunk
    pub fn new(
        line: Vec<P>,
        poly_index: usize,
        ring_index: usize,
        bbox: BBox,
        next: IntersectionPointRef<P>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(PolyChunk { visted: false, poly_index, ring_index, bbox, line, next }))
    }
}

/// Local Intersection to a [poly_index][ring_index]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RingIntersection<P: UnionPoint> {
    from: usize, // index in the polys[polygon][ring][from]
    to: usize,   // index in the polys[polygon][ring][to]
    point: P,
}
impl<P: UnionPoint> RingIntersection<P> {
    /// Create a new Intersection
    pub fn new(from: usize, to: usize, point: P) -> Self {
        RingIntersection { from, to, point }
    }
}
/// [poly_index][ring_index] -> Intersections
pub type RingIntersectionLookup<P> = BTreeMap<usize, BTreeMap<usize, Vec<RingIntersection<P>>>>;

/// Intersection Point
#[derive(Debug, Clone, PartialEq)]
pub struct IntersectionPoint<P: UnionPoint> {
    point: P,
    chunks: Vec<PolyChunkRef<P>>, // reference to all chunks
}
/// A reference to a IntersectionPoint wrapped in an RC & RefCell
pub type IntersectionPointRef<P> = Rc<RefCell<IntersectionPoint<P>>>;
impl<P: UnionPoint> IntersectionPoint<P> {
    /// Create a new IntersectionPoint wrapped in an RC & RefCell
    pub fn new(point: P) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(IntersectionPoint { point, chunks: vec![] }))
    }
}

/// Given a collection of polygons, if any of the polygons interact/overlap eachother, merge them.
///
/// ## Parameters
/// - `polygons`: the collection of polygons to apply a union to. The points in the polygons are
/// expected to implement the trait [`UnionPoint`] which extends [`GetXY`] and [`NewXY`]
///
/// ## Returns
/// A union of polygons should a union exist.
pub fn polygons_union<P: UnionPoint>(
    vector_polygons: &[Vec<Vec<P>>],
) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    // bbox
    let bbox = BBox::from_multi_polygon(vector_polygons);

    // not enough data, just clone
    if vector_polygons.len() < 2 {
        return Some((vector_polygons.to_vec(), bbox));
    }

    // 1) build intersections `[poly_index][ring_index] -> Intersections`. Store where on the ring other rings intersect
    let mut ring_intersect_lookup: RingIntersectionLookup<P> =
        build_ring_intersect_lookup(vector_polygons);

    // 2) Build Poly Pieces
    // Setup result paths with chunks that are the final structure of joined polygons.
    // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
    // If no intersections for the poly_index+Ring_index -> it's immediately consumed into paths. Otherwise it's a chunk
    let (mut paths, mut path_lookup, mut chunks) =
        build_paths_and_chunks(vector_polygons, &mut ring_intersect_lookup);

    // 3) Consume chunks into PolyPaths
    // If no intersections for the poly_index+Ring_index -> push as completed ring
    build_paths_from_chunks(&mut chunks, &mut path_lookup, &mut paths);

    // 4) Convert PolyPaths into the resultant MultiPolygon
    let coordinates: Vec<Vec<Vec<P>>> = paths
        .iter_mut()
        .filter_map(|p| {
            let mut p = p.borrow_mut();
            let mut res: Vec<Vec<P>> = vec![];
            if let Some(outer) = p.outer.take() {
                res.push(outer);
                for hole in p.holes.drain(..) {
                    res.push(hole);
                }
            }
            if res.len() > 0 { Some(res) } else { None }
        })
        .collect();
    if coordinates.len() == 0 {
        return None;
    } else {
        return Some((coordinates, bbox));
    }
}

/// Run through the vector_polygons and Builds the ring intersection lookup
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
///
/// ## Returns
/// The ring intersection lookup for all rings in the multipolygon collection
fn build_ring_intersect_lookup<P: UnionPoint>(
    vector_polygons: &[Vec<Vec<P>>],
) -> RingIntersectionLookup<P> {
    let segments = build_polygon_segments(vector_polygons);
    let mut ring_intersect_lookup: RingIntersectionLookup<P> = BTreeMap::new();

    // setup a 2D box index
    let box_index = BoxIndex::new(segments.clone(), None);
    // iterate each segment and check for intersections with other segments
    for segment1 in segments {
        let seg_is_outer = segment1.ring_index == 0;
        let potential_intersections = box_index.search(
            &segment1.bbox(),
            Some(|seg: &Segment| {
                // if same id ignore
                seg.id != segment1.id &&
            // only pass forward not backward
            seg.id > segment1.id &&
            // if same poly_index ignore
            seg.poly_index != segment1.poly_index &&
            // only pass if both inner or if both outer.
            ((seg.ring_index == 0 && seg_is_outer) || (seg.ring_index != 0 && !seg_is_outer))
            }),
        );
        for segment2 in potential_intersections {
            let p_int = find_polygon_intersection::<P, P>(&vector_polygons, &segment1, &segment2);
            // ignore points that interact tangentially or precisely at an existing edge or vertex.
            if let Some(p_int) = p_int {
                // first segment intersection
                let s1 = ring_intersect_lookup
                    .entry(segment1.poly_index)
                    .or_default()
                    .entry(segment1.ring_index)
                    .or_default();
                s1.push(RingIntersection::new(segment1.from, segment1.to, p_int.point.clone()));
                // second segment intersection
                let s2 = ring_intersect_lookup
                    .entry(segment2.poly_index)
                    .or_default()
                    .entry(segment2.ring_index)
                    .or_default();
                s2.push(RingIntersection::new(segment2.from, segment2.to, p_int.point));
            }
        }
    }

    ring_intersect_lookup
}

/// Build the PolyPaths and PolyChunks
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
/// - `ring_intersect_lookup`: the ring intersection lookup for all rings in the multipolygon collection
///
/// ## Returns
/// The PolyPaths, their lookups, and PolyChunks
fn build_paths_and_chunks<P: UnionPoint>(
    vector_polygons: &[Vec<Vec<P>>],
    ring_intersect_lookup: &mut RingIntersectionLookup<P>,
) -> (Vec<PolyPathRef<P>>, BTreeMap<usize, PolyPathRef<P>>, Vec<PolyChunkRef<P>>) {
    // Setup result. Paths are the final structure of joined polygons.
    let mut paths: Vec<PolyPathRef<P>> = vec![];
    // Lookup is a helper for quickly finding paths in the future
    let mut path_lookup: BTreeMap<usize, PolyPathRef<P>> = BTreeMap::new();

    // 2) Build Poly Pieces
    // If no intersections for the poly_index+Ring_index -> push as completed ring (into paths)
    let mut chunks: Vec<PolyChunkRef<P>> = vec![];
    //   let inter_point_lookup: { [x: number]: { [y: number]: IntersectionPoint<D> } } = {};
    let mut inter_point_lookup: BTreeMap<P, IntersectionPointRef<P>> = BTreeMap::new();
    for (poly_index, poly) in vector_polygons.iter().enumerate() {
        for (ring_index, ring) in poly.iter().enumerate() {
            //   let intersections = ring_intersect_lookup[poly_index]?.[ring_index];
            let mut intersections =
                ring_intersect_lookup.get_mut(&poly_index).and_then(|r| r.get_mut(&ring_index));
            // Case 1: Insert into paths because it's already completed or expand existing path
            if intersections.is_none() || intersections.as_mut().unwrap().is_empty() {
                if let Some(existing_path) = path_lookup.get(&poly_index) {
                    let mut existing_path = existing_path.borrow_mut();
                    existing_path.polys_consumed.insert(poly_index);
                    if ring_index == 0 {
                        existing_path.outer = Some(ring.clone());
                    } else {
                        existing_path.holes.push(ring.clone());
                    }
                } else {
                    let path: PolyPathRef<P> =
                        PolyPath::new_ref(ring.clone(), poly_index, ring_index == 0);
                    path_lookup.insert(poly_index, path.clone());
                    paths.push(path);
                }
                continue;
            }
            // Case 2: Insert into chunks for further processing
            // ensure we split the full ring in order
            let intersections = intersections.unwrap();
            intersections.sort_by(|a, b| a.from.cmp(&b.from));
            let mut curr_index = 0;
            let mut curr_int_p: Option<P> = None;
            for RingIntersection { from, to, point: next_int_p, .. } in intersections {
                // TODO: Sometimes we want tangential intersections. Build a test case to ensure this works
                // skip points that interact tangentially or precisely at ends of the ring
                if (*from == 0 && *next_int_p == ring[0])
                    || (*to == ring.len() - 1 && *next_int_p == ring[*to])
                {
                    continue;
                }
                // build the chunk's line
                let mut line = vec![];
                curr_int_p.as_ref().map(|p| line.push(p.clone()));
                line.extend_from_slice(&ring[curr_index..(*from + 1)]); // include to from.
                // add to the lookup if needed otherwise grab the existing one
                let intr_p = inter_point_lookup
                    .entry(next_int_p.clone())
                    .or_insert(IntersectionPoint::new(next_int_p.clone()));
                // build the chunk and point it to the next intersection "point"
                let bbox = BBox::from_linestring(&line);
                let chunk = PolyChunk::new(line, poly_index, ring_index, bbox, intr_p.clone());
                // Place this chunk in the lookup where it began if it started at an intersection, otherwise
                // the "intersection" is the start of the ring
                let start_point = curr_int_p.as_ref().map(|p| p.clone()).unwrap_or(ring[0].clone());
                let intr_ps = inter_point_lookup
                    .entry(start_point.clone())
                    .or_insert(IntersectionPoint::new(start_point));
                intr_ps.borrow_mut().chunks.push(chunk.clone());
                chunks.push(chunk);
                // update current
                curr_int_p = Some(next_int_p.clone());
                curr_index = *to;
            }
            // lastly if we have an open ring add it
            if curr_index != ring.len() {
                let mut line = vec![];
                curr_int_p.as_ref().map(|p| line.push(p.clone()));
                line.extend_from_slice(&ring[curr_index..]);
                let bbox = BBox::from_linestring(&line);
                // more than likely previous intersection is the start of the ring
                let start_point = curr_int_p.as_ref().map(|p| p.clone()).unwrap_or(ring[0].clone());
                let intr_ps = inter_point_lookup
                    .entry(start_point.clone())
                    .or_insert(IntersectionPoint::new(start_point))
                    .clone();
                // gaurenteed the beginning of the ring
                let intr_pf = inter_point_lookup
                    .entry(ring[0].clone())
                    .or_insert(IntersectionPoint::new(ring[0].clone()));
                let new_chunk = PolyChunk::new(line, poly_index, ring_index, bbox, intr_pf.clone());
                intr_ps.borrow_mut().chunks.push(new_chunk);
            }
        }
    }
    // sort the chunks by leftmost bboxes then bottom most if there is a tie
    chunks.sort_by(|a, b| {
        let BBox { left: a_left, bottom: a_bottom, .. } = a.borrow().bbox;
        let BBox { left: b_left, bottom: b_bottom, .. } = b.borrow().bbox;
        a_left
            .partial_cmp(&b_left)
            .unwrap_or(Ordering::Equal)
            .then(a_bottom.partial_cmp(&b_bottom).unwrap_or(Ordering::Equal))
    });

    (paths, path_lookup, chunks)
}

/// Given a set of chunks, build a set of paths
///
/// ## Parameters
/// - `chunks`: a set of chunks
/// - `path_lookup`: a lookup of existing paths
/// - `paths`: a set of paths to add to
fn build_paths_from_chunks<P: UnionPoint>(
    chunks: &mut [PolyChunkRef<P>],
    path_lookup: &mut BTreeMap<usize, PolyPathRef<P>>,
    paths: &mut Vec<PolyPathRef<P>>,
) {
    for chunk in chunks {
        if chunk.borrow().visted {
            continue;
        }
        // follow along a chunk until we find our start point again
        let start = chunk.borrow().line[0].clone();

        let mut merged_chunks: Vec<PolyChunkRef<P>> = vec![];
        let mut curr_chunk: PolyChunkRef<P> = chunk.clone();
        let mut found_polygons: BTreeSet<usize> = BTreeSet::new();
        loop {
            // add the chunk and mark it as visited
            curr_chunk.borrow_mut().visted = true;
            merged_chunks.push(curr_chunk.clone());
            found_polygons.insert(curr_chunk.borrow().poly_index);
            // if the current chunk ends at the start, we are done.
            if merged_chunks.len() > 1 && start == *curr_chunk.borrow().line.last().unwrap() {
                break;
            }
            // two directions now:
            // A) if the next intersection is the start, we are done
            let int_p = curr_chunk.borrow_mut().next.clone();
            let IntersectionPoint { point: int_p_p, chunks: int_p_c } = &*int_p.borrow();
            if *int_p_p == start {
                curr_chunk.borrow_mut().line.push(int_p_p.clone());
                break;
            }
            // B) Grab the needed chunks from ring_intersect_lookup, filter by visited, grab the chunk that is the most counter-clockwise with where we are.
            // For all unused_chunks, find the one that continues the chunks as most counter-clockwise as possible
            // Using chunk_end->point->unused_chunk.line[1]
            let chunk_end = curr_chunk.borrow().line.last().unwrap().clone();
            if let Some(next_chunk) = maximum_angle(&chunk_end, &int_p_p, &int_p_c) {
                curr_chunk = next_chunk;
            } else {
                break;
            }
        }

        if merged_chunks.len() == 0 {
            continue;
        }

        // Ensure merged_chunks starts and ends at the same point, otherwise drop
        let first = merged_chunks.first().unwrap().borrow().line.first().unwrap().clone();
        let last = merged_chunks.last().unwrap().borrow().line.last().unwrap().clone();
        if first != last {
            continue;
        }
        // Convert merged_chunks to a ring and find the orientation
        let line_string: Vec<P> =
            merged_chunks.iter().flat_map(|c| c.borrow().line.clone()).collect();
        if line_string.len() < 4 {
            continue;
        }
        let mut bbox = merged_chunks[0].borrow().bbox;
        merged_chunks.iter().for_each(|c| bbox = bbox.merge(&c.borrow().bbox));
        let is_ccw = (&line_string).area(Some(1.)) > 0.;
        // Find the correct PolyPath to insert into, otherwise create a new one, update the lookup to include all new polygon indexes used in the path
        let mut found_path = None;
        for poly_index in &found_polygons {
            if let Some(path) = path_lookup.get(poly_index) {
                found_path = Some(path.clone());
                break;
            }
        }
        if let Some(path) = &found_path {
            path.borrow_mut().add_chunks(
                line_string,
                &mut found_polygons,
                &bbox,
                is_ccw,
                merged_chunks[0].borrow().ring_index != 0,
            );
        } else {
            let new_path = PolyPath::new(line_string, merged_chunks[0].borrow().poly_index, is_ccw);
            let new_path_ref = Rc::new(RefCell::new(new_path));
            found_path = Some(new_path_ref.clone());
            paths.push(new_path_ref);
        }
        // Update the lookup and consumed polygons found
        if let Some(path) = &found_path {
            for poly_index in found_polygons {
                path_lookup.insert(poly_index, path.clone());
            }
        }
    }
}

/// Returns the PolyChunk with the largest angle relative to A->B->Chunk
///
/// ## Parameters
/// - `a`: starting point
/// - `b`: pivot point
/// - `chunks`: list of chunks to choose from
///
/// ## Returns
/// The chunk with the largest angle
fn maximum_angle<P: UnionPoint>(
    a: &P,
    b: &P,
    chunks: &[PolyChunkRef<P>],
) -> Option<PolyChunkRef<P>> {
    let mut max_chunk = None;
    let mut max_angle = 0.0;

    for chunk in chunks {
        let chunk_ref = chunk.borrow_mut();
        if chunk_ref.visted {
            continue;
        }
        let angle = angle_rad(a, b, chunk_ref.line.get(1));
        if max_chunk.is_none() || angle > max_angle {
            max_angle = angle;
            max_chunk = Some(chunk.clone());
        }
    }

    max_chunk
}

/// Returns the absolute angle between points A->B->C
///
/// ## Parameters
/// - `a`: First point
/// - `b`: Vertex point (angle at this point)
/// - `c`: Third point
///
/// ## Returns
/// Angle in degrees [0, 2 * PI]
fn angle_rad<P: UnionPoint>(a: &P, b: &P, c: Option<&P>) -> f64 {
    if c.is_none() {
        return 0.0;
    }
    let c = c.unwrap();
    let angle_ba = atan2(a.y() - b.y(), a.x() - b.x());
    let angle_bc = atan2(c.y() - b.y(), c.x() - b.x());

    // Difference in radians
    let mut angle = angle_ba - angle_bc;
    if angle < 0. {
        angle += 2. * PI;
    }
    angle
}
