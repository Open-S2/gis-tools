use crate::geometry::{
    Area, InterPointLookup, NextRingChunk, PolyPath, PolyPathRef, RingChunkRef,
    RingIntersectionLookup, Segment, build_paths_and_chunks, merge_intersection_pairs,
    polygons_intersections_lookup, polyline_in_polyline,
};
use alloc::{collections::BTreeMap, vec, vec::Vec};
use core::{cmp::Ordering, mem::take};
use s2json::{BBox, FullXY};
use std::collections::BTreeSet;

/// Given a collection of polygons, if any of the polygons are kinked, dekink them
///
/// ## Parameters
/// `polygons`: the polygons are from either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
///
/// ## Returns
/// The dekinked polygons
pub fn dekink_polygons<P: FullXY>(polygons: &Vec<Vec<Vec<P>>>) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    polygons.into_iter().filter_map(|p| dekink_polygon(p)).fold(
        Some((vec![], BBox::default())),
        |acc, mut p| {
            let (mut res, mut bbox) = acc.unwrap();
            res.extend(p.0.drain(..));
            bbox.merge_in_place(&p.1);
            Some((res, bbox))
        },
    )
}

/// Given a polygon, if the polygon is kinked, dekink it
///
/// ## Parameters
/// - `polygon`: the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
///
/// ## Returns
/// The dekinked polygon
pub fn dekink_polygon<P: FullXY>(polygon: &Vec<Vec<P>>) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    // not enough data, just clone
    if polygon.len() == 0 {
        return None;
    }
    let vector_polygons = vec![polygon.to_vec()];

    // 1) build intersections `[poly_index][ring_index] -> Intersections`. Store where on the ring other rings intersect
    let mut ring_int_lookup: RingIntersectionLookup = polygons_intersections_lookup(
        &vector_polygons,
        Some(|seg1: &Segment, seg2: &Segment| -> bool {
            // if same id ignore
            seg2.id != seg1.id &&
        // only pass forward not backward
        seg2.id > seg1.id &&
        // TODO: At some point intersections of inner rings against the outer ring should be considered.
        // For now the ring_index must be the same, polygonsIntersectionsLookup should return
        // two problem sets down the road, one for cleaning individual rings, and one for fixing holes that go out of bounds
        seg2.ring_index == seg1.ring_index
        }),
    );

    // 2) Build Poly Pieces
    // Setup result paths with chunks that are the final structure of joined polygons.
    // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
    // If no intersections for the poly_index+Ring_index -> it's immediately consumed into paths. Otherwise it's a chunk
    let (mut paths, _, chnks, mut int_lookup, _b) =
        build_paths_and_chunks(&vector_polygons, &mut ring_int_lookup);

    // 3) Consume chunks into PolyPaths
    // If no intersections for the poly_index+Ring_index -> push as completed ring
    build_paths_from_chunks(&mut paths, &mut int_lookup, &chnks);

    // 4) Convert PolyPaths into the resultant MultiPolygon
    let coordinates: Vec<Vec<Vec<P>>> =
        paths.iter_mut().filter_map(|p| p.borrow_mut().get_path()).collect();
    let bbox = paths.iter().map(|p| p.borrow().bbox).fold(BBox::default(), |mut acc, b| {
        acc.merge_in_place(&b);
        acc
    });
    if coordinates.len() == 0 {
        return None;
    } else {
        return Some((coordinates, bbox));
    }
}

/// Simplified ring with guide of how it was rebuild
#[derive(Debug)]
struct Ring<P: FullXY> {
    linestring: Vec<P>,
    is_ccw: bool,
    is_hole: bool,
    bbox: BBox,
    area: f64, // bbox area
}
impl<P: FullXY> Ring<P> {
    fn new(linestring: Vec<P>, is_ccw: bool, is_hole: bool, bbox: BBox, area: f64) -> Self {
        Self { linestring, is_ccw, is_hole, bbox, area }
    }
}
/// Collection of rings sorted by poly_index
/// [poly_index: number]: Collection of Rings for that polygon
type RingStore<P> = BTreeMap<usize, Vec<Ring<P>>>;

/// Given a set of chunks, build a set of paths
///
/// ## Parameters
/// - `paths`: a set of paths to add to
/// - `intersections`: all intersections
/// - `chunks`: a set of chunks
fn build_paths_from_chunks<P: FullXY>(
    paths: &mut Vec<PolyPathRef<P>>,
    intersections: &mut InterPointLookup<P>,
    chunks: &Vec<RingChunkRef<P>>,
) {
    let mut ring_store: RingStore<P> = BTreeMap::new();
    // store existing paths and reset.
    for path in paths.iter() {
        let PolyPath { outer, polys_consumed, bbox, holes, .. } = &mut *path.borrow_mut();
        let poly_index = polys_consumed.first().unwrap();
        let poly_store = ring_store.entry(*poly_index).or_insert(vec![]);
        if let Some(outer) = outer {
            poly_store.push(Ring::new(take(outer), true, false, *bbox, bbox.area()));
        }
        for hole in holes {
            poly_store.push(Ring::new(take(hole), false, true, *bbox, bbox.area()));
        }
    }
    paths.clear();
    // for each intersections, connect all the from and to, smallest angle between from->to first slowly work your way through
    for int in intersections.lookup.values() {
        merge_intersection_pairs(int);
    }
    // run through all chunks, if unvisited, add to paths
    for chunk in chunks {
        if chunk.borrow().visted {
            continue;
        }
        // follow along a chunk until we find our start point again
        let start = chunk.borrow().from;
        let mut curr_chunk: RingChunkRef<P> = chunk.clone();
        let mut linestring: Vec<P> = vec![P::new_xy(start.0, start.1)];
        let mut bbox = curr_chunk.borrow().bbox;
        loop {
            if curr_chunk.borrow().visted {
                break;
            }
            curr_chunk.borrow_mut().visted = true;
            linestring.extend(curr_chunk.borrow_mut().mid.drain(..));
            bbox.merge_in_place(&curr_chunk.borrow().bbox);
            if curr_chunk.borrow().next.is_none() {
                break;
            }
            let (next_chunk, int_point) = {
                let chnk = &mut curr_chunk.borrow_mut();
                let NextRingChunk { chunk, int_point } = chnk.next.as_ref().unwrap();
                (chunk.clone(), *int_point)
            };
            linestring.push(P::new_xy(int_point.0, int_point.1));
            curr_chunk = next_chunk;
            if int_point == start {
                break;
            }
        }
        let area = (&linestring).area(Some(1.));
        if area == 0. || linestring.len() < 4 || linestring.first() != linestring.last() {
            continue;
        }
        // now build the path or add to an existing path
        let is_ccw = area > 0.;
        let is_hole = chunk.borrow().ring_index != 0;
        // store in correct location
        let poly_store = ring_store.entry(chunk.borrow().poly_index).or_insert(vec![]);
        poly_store.push(Ring::new(linestring, is_ccw, is_hole, bbox, area));
    }
    // For each ring_store, build out polys and store them in paths
    for rings in ring_store.values_mut() {
        ring_set_to_paths(paths, rings);
    }
}

/// Convert a set of rings into a set of paths
///
/// ## Parameters
/// - `paths`: the collection of paths to store the results in
/// - `ring_set`: the current set of rings re-built from a polygon
fn ring_set_to_paths<P: FullXY>(paths: &mut Vec<PolyPathRef<P>>, ring_set: &mut Vec<Ring<P>>) {
    if ring_set.len() == 0 {
        return;
    }
    // sort by bbox area desc and prep real outers store
    ring_set.sort_by(|a, b| b.area.partial_cmp(&a.area).unwrap_or(Ordering::Equal));
    let mut actual_outers: Vec<PolyPathRef<P>> = vec![];
    // filter by case type

    // store all true outers
    {
        let mut outers: Vec<&mut Ring<P>> =
            ring_set.iter_mut().filter(|r| !r.is_hole && r.is_ccw).collect();
        // store all true outers. We know the first outer is the largest original outer ring.
        // The future outers are holes either kink inside or outside the original. Only store kinks that are outside the original
        if outers.len() > 0 {
            let first = outers.remove(0);
            // store the first one without the ring we still need it for now
            actual_outers.push(PolyPath::new_ref(vec![], BTreeSet::new(), true, Some(first.bbox)));
            // check all the others are not in the first
            for outer in outers {
                if outer.bbox.inside(&first.bbox)
                    && polyline_in_polyline(&outer.linestring, &first.linestring)
                {
                    continue;
                }
                actual_outers.push(PolyPath::new_ref(
                    take(&mut outer.linestring),
                    BTreeSet::new(),
                    true,
                    Some(outer.bbox),
                ));
            }
            // now store the first one's ring
            actual_outers[0].borrow_mut().outer = Some(take(&mut first.linestring));
        }
    }

    // If outer in `outers_maybe_hole` is inside an actual outer, it's a hole; Otherwise it's another outer
    {
        let outers_maybe_hole: Vec<&mut Ring<P>> =
            ring_set.iter_mut().filter(|r| !r.is_hole && !r.is_ccw).collect();
        for Ring { linestring, bbox, .. } in outers_maybe_hole {
            let mut found = false;
            for actual_outer in actual_outers.iter() {
                let actual_outer = &mut actual_outer.borrow_mut();
                if bbox.inside(&actual_outer.bbox)
                    && polyline_in_polyline(linestring, actual_outer.outer.as_ref().unwrap())
                {
                    // store the hole in this outer
                    actual_outer.holes.push(take(linestring));
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            // otherwise, it's a new outer
            linestring.reverse();
            actual_outers.push(PolyPath::new_ref(
                take(linestring),
                BTreeSet::new(),
                true,
                Some(*bbox),
            ))
        }
    }

    // now organize holes
    {
        let holes: Vec<&mut Ring<P>> =
            ring_set.iter_mut().filter(|r| r.is_hole && !r.is_ccw).collect();
        if actual_outers.len() != 0 {
            for Ring { linestring, bbox, .. } in holes {
                if actual_outers.len() == 1 {
                    actual_outers[0].borrow_mut().holes.push(take(linestring));
                } else {
                    // find the outer this hole belongs to
                    for actual_outer in actual_outers.iter() {
                        if bbox.inside(&actual_outer.borrow().bbox)
                            && polyline_in_polyline(
                                linestring,
                                actual_outer.borrow().outer.as_ref().unwrap(),
                            )
                        {
                            // store the hole in this outer
                            actual_outer.borrow_mut().holes.push(take(linestring));
                            break;
                        }
                    }
                }
            }
        }
    }

    // Now store all actual_outers we built
    for actual_outer in actual_outers.iter() {
        paths.push(actual_outer.clone());
    }
}
