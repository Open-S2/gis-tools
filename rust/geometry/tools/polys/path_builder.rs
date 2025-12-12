use crate::geometry::{RingIntersection, RingIntersectionLookup};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    vec,
    vec::Vec,
};
use core::{
    cell::RefCell,
    cmp::Ordering,
    f64::consts::{PI, TAU},
    mem::take,
};
use libm::atan2;
use s2json::{BBox, FullXY, Point};

/// Reconstructing a poly line that interacts with intersections
#[derive(Debug)]
pub struct PolyPath<P: FullXY> {
    /// helps down the road to spot duplicate pulls of this Path
    pub id: usize,
    /// True outer ring
    pub outer: Option<Vec<P>>,
    /// Outers that have already been consumed
    pub old_outers: Vec<BBox>,
    /// Holes
    pub holes: Vec<Vec<P>>,
    /// indexes of the polygons in the multipolygon. So we can quickly consume holes.
    pub polys_consumed: BTreeSet<usize>,
    /// Bounding box
    pub bbox: BBox,
}
/// A reference to a PolyPath wrapped in an RC & RefCell
pub type PolyPathRef<P> = Rc<RefCell<PolyPath<P>>>;
impl<P: FullXY> PolyPath<P> {
    /// Create a new PolyPath as a PolyPathRef
    pub fn new_ref(
        ring: Vec<P>,
        polys_consumed: BTreeSet<usize>,
        is_outer: bool,
        bbox: Option<BBox>,
    ) -> PolyPathRef<P> {
        Rc::new(RefCell::new(PolyPath::new(ring, polys_consumed, is_outer, bbox)))
    }

    /// Create a new PolyPath
    pub fn new(
        ring: Vec<P>,
        polys_consumed: BTreeSet<usize>,
        is_outer: bool,
        bbox: Option<BBox>,
    ) -> Self {
        let bbox = bbox.unwrap_or(BBox::from_linestring(&ring));
        let mut outer = None;
        let mut holes = vec![];
        if is_outer {
            outer = Some(ring);
        } else {
            holes.push(ring);
        }
        PolyPath { id: 0, outer, old_outers: vec![], holes, polys_consumed, bbox }
    }

    /// Get the path as a vector polygon
    ///
    /// ## Returns
    /// The resultant poly if it exists
    pub fn get_path(&mut self) -> Option<Vec<Vec<P>>> {
        self.outer.as_ref()?;
        let outer = self.outer.as_mut().unwrap();
        if outer.len() < 4 {
            return None;
        }
        let mut res = vec![take(outer)];
        for hole in self.holes.iter_mut() {
            if hole.len() < 4 {
                continue;
            }
            res.push(take(hole));
        }
        Some(res)
    }
}

/// The next poly chunk
#[derive(Debug, Clone, PartialEq)]
pub struct NextRingChunk<P: FullXY> {
    /// The intersection point
    pub int_point: Point,
    /// The next chunk
    pub chunk: RingChunkRef<P>,
}

/// A path/piece/chunk from a polygon
#[derive(Debug, Clone, PartialEq)]
pub struct RingChunk<P: FullXY> {
    /// If the chunk has been visited. Used by connection algorithms
    pub visted: bool,
    /// The index of the polygon it belongs to
    pub poly_index: usize,
    /// The index of the ring
    pub ring_index: usize,
    /// The bounding box
    pub bbox: BBox,
    /// Always stars with either the beginning of the poly ring OR an intersection point.
    pub mid: Vec<P>,
    /// The intersection point this chunk starts at
    pub from: Point,
    /// The intersection point this chunk ends at
    pub to: Point,
    /// can point to just one or multiple chunks. Many polys can touch the same point.
    /// If none provided could be a start-end point
    pub next: Option<NextRingChunk<P>>,
    /// Precomputed from angle. Useful for when intersections are computed
    pub from_angle: f64,
    /// Precomputed to angle. Useful for when intersections are computed
    pub to_angle: f64,
}
/// A reference to a RingChunk wrapped in an RC & RefCell
pub type RingChunkRef<P> = Rc<RefCell<RingChunk<P>>>;
impl<P: FullXY> RingChunk<P> {
    /// Create a new RingChunk
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        poly_index: usize,
        ring_index: usize,
        bbox: BBox,
        mid: Vec<P>,
        from: Point,
        to: Point,
        from_angle: f64,
        to_angle: f64,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(RingChunk {
            visted: false,
            poly_index,
            ring_index,
            bbox,
            mid,
            from,
            to,
            next: None,
            from_angle,
            to_angle,
        }))
    }

    /// Check if two chunks are equal
    pub fn equal_chunk(&self, other: &RingChunkRef<P>) -> bool {
        let other = &other.borrow();
        (self.ring_index > 0) == (other.ring_index > 0)
            && self.from == other.from
            && self.to == other.to
            && self.mid == other.mid
    }
}

/// Intersection Point
#[derive(Debug, Clone, PartialEq)]
pub struct IntersectionPoint<P: FullXY> {
    /// The intersection point
    pub point: Point,
    /// The chunks whose end point intersect this
    pub from: Vec<RingChunkRef<P>>,
    /// The chunks whose start point intersect this
    pub to: Vec<RingChunkRef<P>>,
}
/// A reference to a IntersectionPoint wrapped in an RC & RefCell
pub type IntersectionPointRef<P> = Rc<RefCell<IntersectionPoint<P>>>;
impl<P: FullXY> IntersectionPoint<P> {
    /// Create a new IntersectionPoint wrapped in an RC & RefCell
    pub fn new(point: Point) -> IntersectionPointRef<P> {
        Rc::new(RefCell::new(IntersectionPoint { point, from: vec![], to: vec![] }))
    }
}

/// Intersection Lookup for chunks
#[derive(Debug, Clone, PartialEq)]
pub struct InterPointLookup<P: FullXY> {
    /// The lookup
    pub lookup: BTreeMap<Point, IntersectionPointRef<P>>,
}
impl<P: FullXY> Default for InterPointLookup<P> {
    fn default() -> Self {
        Self { lookup: BTreeMap::new() }
    }
}
impl<P: FullXY> InterPointLookup<P> {
    /// Get the intersection point
    pub fn get(&mut self, point: Point) -> IntersectionPointRef<P> {
        self.lookup.entry(point).or_insert_with(|| IntersectionPoint::new(point)).clone()
    }

    /// Link two points to eachother
    #[allow(clippy::too_many_arguments)]
    pub fn link_ints(
        &mut self,
        poly_index: usize,
        ring_index: usize,
        from: Point,
        to: Point,
        mid: Vec<P>,
        from_angle: Option<f64>,
        to_angle: Option<f64>,
    ) -> RingChunkRef<P> {
        // first build a chunk
        let bbox = BBox::from_linestring(&mid).merge(&BBox::from_linestring(&[from, to]));
        let from_angle =
            from_angle.unwrap_or(angle(&mid.last().map(Point::from).unwrap_or(from), &to));
        let to_angle =
            to_angle.unwrap_or(angle(&mid.first().map(Point::from).unwrap_or(to), &from));
        let chunk =
            RingChunk::new(poly_index, ring_index, bbox, mid, from, to, from_angle, to_angle);
        self.get(from).borrow_mut().to.push(chunk.clone());
        self.get(to).borrow_mut().from.push(chunk.clone());
        chunk
    }
}

/// Build the PolyPaths and PolyChunks
///
/// ## Parameters
/// - `vector_polygons`: the collection of polygons
/// - `ring_intersect_lookup`: the ring intersection lookup for all rings in the multipolygon collection
///
/// ## Returns
/// The PolyPaths, their lookups, and PolyChunks
#[allow(clippy::type_complexity)]
pub fn build_paths_and_chunks<P: FullXY>(
    vector_polygons: &[Vec<Vec<P>>],
    ring_intersect_lookup: &mut RingIntersectionLookup,
) -> (
    Vec<PolyPathRef<P>>,
    BTreeMap<usize, PolyPathRef<P>>,
    Vec<RingChunkRef<P>>,
    InterPointLookup<P>,
    Vec<BBox>,
) {
    // Setup result. Paths are the final structure of joined polygons.
    let mut paths: Vec<PolyPathRef<P>> = vec![];
    // Lookup is a helper for quickly finding paths in the future
    let mut path_lookup: BTreeMap<usize, PolyPathRef<P>> = BTreeMap::new();
    // let mut outer_ring_bboxes: VecBBox> = new Array(vectorPolygons.length);
    let mut outer_ring_bboxes: Vec<BBox> = vec![Default::default(); vector_polygons.len()];

    // 2) Build Poly Pieces
    // If no intersections for the poly_index+Ring_index -> push as completed ring (into paths)
    let mut chunks: Vec<RingChunkRef<P>> = vec![];
    let mut int_lookup = InterPointLookup::<P>::default();
    for (p_i, poly) in vector_polygons.iter().enumerate() {
        for (r_i, ring) in poly.iter().enumerate() {
            let intersections = ring_intersect_lookup.get_mut(&p_i).and_then(|r| r.get_mut(&r_i));
            // Case 1: Insert into paths because it's already completed or expand existing path
            if intersections.is_none() || intersections.as_ref().unwrap().is_empty() {
                if let Some(existing_path) = path_lookup.get(&p_i) {
                    let existing_path = &mut existing_path.borrow_mut();
                    if r_i == 0 {
                        existing_path.bbox.merge_in_place(&BBox::from_linestring(ring));
                        existing_path.outer = Some(ring.clone());
                        outer_ring_bboxes[p_i] = existing_path.bbox;
                    } else {
                        existing_path.holes.push(ring.clone());
                    }
                } else {
                    let path =
                        PolyPath::new_ref(ring.clone(), BTreeSet::from([p_i]), r_i == 0, None);
                    if r_i == 0 {
                        outer_ring_bboxes[p_i] = path.borrow().bbox;
                    }
                    path_lookup.insert(p_i, path.clone());
                    paths.push(path);
                }
                continue;
            }
            // Case 2: Handle the intersections and build RingChunks
            if r_i == 0 {
                outer_ring_bboxes[p_i] = BBox::from_linestring(ring);
            }
            let intersections: Vec<&RingIntersection> =
                intersections.as_ref().unwrap().iter().filter(|i| i.t != 0.).collect();
            let mut curr_index = 0;
            let mut int_index = 0;
            while curr_index < ring.len() - 1 {
                // if we are still working with intersections, build points with them
                if let Some(cur_inter) = intersections.get(int_index) {
                    // until we get to the next intersection, we link the points
                    if curr_index != cur_inter.from {
                        let start = curr_index;
                        while curr_index != cur_inter.from {
                            curr_index += 1;
                        }
                        let mid = ring[start + 1..curr_index].to_vec();
                        let from = &ring[start];
                        let to = &ring[curr_index];
                        let chunk =
                            int_lookup.link_ints(p_i, r_i, from.into(), to.into(), mid, None, None);
                        chunks.push(chunk);
                    }
                    // now build links with the intersections until we get to the next intersection that isn't the same index
                    let mut from: Point = (&ring[curr_index]).into();
                    let mut cur_int = Some(cur_inter);
                    while let Some(c_i) = cur_int
                        && c_i.from == curr_index
                    {
                        // NOTE: For robustness, we have to store the angles we found when studying the intersections.
                        // We make decisions about the polygons during the analysis of the intersections using
                        // robust predicates. otherwise we would actually compute slightly different angles
                        // that could percieve the intersection lines as swapped (non-existent).
                        if from != *c_i.point.borrow() {
                            chunks.push(int_lookup.link_ints(
                                p_i,
                                r_i,
                                from,
                                *c_i.point.borrow(),
                                vec![],
                                Some(invert_angle(c_i.t_angle)),
                                Some(c_i.t_angle),
                            ));
                        }
                        int_index += 1;
                        from = *c_i.point.borrow();
                        cur_int = intersections.get(int_index);
                    }
                    // if the intersection t is not 1, then we need to link the point to the end of the curr_index
                    let next: Point = (&ring[curr_index + 1]).into();
                    if from != next {
                        let ang = intersections[int_index - 1].t_angle;
                        chunks.push(int_lookup.link_ints(
                            p_i,
                            r_i,
                            from,
                            next,
                            vec![],
                            Some(invert_angle(ang)),
                            Some(ang),
                        ));
                    }
                } else {
                    // no intersection, just build the point
                    chunks.push(int_lookup.link_ints(
                        p_i,
                        r_i,
                        (&ring[curr_index]).into(),
                        (&ring[curr_index + 1]).into(),
                        vec![],
                        None,
                        None,
                    ));
                }
                curr_index += 1;
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

    (paths, path_lookup, chunks, int_lookup, outer_ring_bboxes)
}

#[derive(Debug)]
struct IntPair<P: FullXY> {
    from: RingChunkRef<P>,
    to: RingChunkRef<P>,
    angle: f64,
}

/// Given an of intersection, find the best way to connect the from->to chunks
///
/// ## Parameters
/// - `intersection`: the intersection to analyze
pub fn merge_intersection_pairs<P: FullXY>(intersection: &IntersectionPointRef<P>) {
    let IntersectionPoint { from, to, point, .. } = &mut *intersection.borrow_mut();
    let int_point = *point;
    if from.is_empty() || to.is_empty() {
        return;
    }
    if from.len() == 1 && to.len() == 1 {
        // connect the two chunks and move on
        from[0].borrow_mut().next = Some(NextRingChunk { chunk: to[0].clone(), int_point });
        return;
    }

    // remove "duplicate"/"same" chunks
    let mut froms: Vec<RingChunkRef<P>> = vec![];
    for c in from {
        if c.borrow().visted {
            continue;
        }
        if !froms.iter().any(|r| r.borrow().equal_chunk(c)) {
            froms.push(c.clone());
        } else {
            c.borrow_mut().visted = true;
        }
    }
    let mut tos: Vec<RingChunkRef<P>> = vec![];
    for c in to {
        if c.borrow().visted {
            continue;
        }
        if !tos.iter().any(|r| r.borrow().equal_chunk(c)) {
            tos.push(c.clone());
        } else {
            c.borrow_mut().visted = true;
        }
    }

    // build pairs
    let mut pairs: Vec<IntPair<P>> = vec![];
    for f in froms.iter() {
        for t in tos.iter() {
            let mut angle = t.borrow().to_angle - f.borrow().from_angle;
            angle = if angle < 0. { angle + TAU } else { angle };
            pairs.push(IntPair { from: f.clone(), to: t.clone(), angle });
        }
    }
    pairs.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap_or(Ordering::Equal));

    for IntPair { from, to, .. } in &pairs {
        let from = &mut from.borrow_mut();
        if from.visted || to.borrow().visted {
            continue;
        }
        from.next = Some(NextRingChunk { chunk: to.clone(), int_point });
        from.visted = true;
        to.borrow_mut().visted = true;
    }

    // cleanup visited
    for f in froms {
        f.borrow_mut().visted = false;
    }
    for t in tos {
        t.borrow_mut().visted = false;
    }
}

/// Returns the absolute angle between points A->B->C
///
/// ## Parameters
/// - `a`: First point
/// - `b`: Second Point
///
/// ## Returns
/// Angle in degrees [-PI, PI]
fn angle<P: FullXY, Q: FullXY>(a: &P, b: &Q) -> f64 {
    atan2(a.y() - b.y(), a.x() - b.x())
}

/// Returns the absolute angle between points A->B->C
///
/// ## Parameters
/// - `angle`: Angle in degrees [-PI, PI]
///
/// ## Returns
/// Angle in degrees [-PI, PI]
fn invert_angle(angle: f64) -> f64 {
    if angle >= 0. { angle - PI } else { angle + PI }
}
