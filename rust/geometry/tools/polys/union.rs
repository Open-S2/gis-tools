use crate::geometry::{
    Area, InterPointLookup, NextRingChunk, PolyPath, PolyPathRef, RingChunkRef,
    RingIntersectionLookup, Segment, build_paths_and_chunks, merge_intersection_pairs,
    polygons_intersections_lookup,
};
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec,
    vec::Vec,
};
use core::mem::take;
use s2json::{BBox, FullXY};

/// Given a collection of polygons, if any of the polygons interact/overlap eachother, merge them.
///
/// ## Parameters
/// - `polygons`: the collection of polygons to apply a union to. The points in the polygons are
///   expected to implement the trait [`FullXY`] which extends [`GetXY`] and [`NewXY`]
///
/// ## Returns
/// A union of polygons should a union exist.
pub fn polygons_union<P: FullXY>(
    vector_polygons: &[Vec<Vec<P>>],
) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    // not enough data, just clone
    if vector_polygons.is_empty() {
        return None;
    }

    // 1) build intersections `[poly_index][ring_index] -> Intersections`. Store where on the ring other rings intersect
    let mut ring_intersect_lookup: RingIntersectionLookup =
        polygons_intersections_lookup(vector_polygons, None::<fn(&Segment, &Segment) -> bool>);

    // 2) Build Poly Pieces
    // Setup result paths with chunks that are the final structure of joined polygons.
    // Lookup is a helper for quickly finding the right path in the future as paths can consume multiple polygons
    // If no intersections for the poly_index+Ring_index -> it's immediately consumed into paths. Otherwise it's a chunk
    let (mut paths, mut path_lookup, chnks, ints, bboxes) =
        build_paths_and_chunks(vector_polygons, &mut ring_intersect_lookup);

    // 3) Consume chunks into PolyPaths
    // If no intersections for the poly_index+Ring_index -> push as completed ring
    // build_paths_from_chunks(&mut chunks, &mut path_lookup, &mut paths);
    build_paths_from_chunks(&mut paths, &mut path_lookup, &ints, &chnks, &bboxes);

    // 4) Convert PolyPaths into the resultant MultiPolygon
    let coordinates: Vec<Vec<Vec<P>>> =
        paths.iter_mut().filter_map(|p| p.borrow_mut().get_path()).collect();
    let bbox = paths.iter().map(|p| p.borrow().bbox).fold(BBox::default(), |mut acc, b| {
        acc.merge_in_place(&b);
        acc
    });
    if coordinates.is_empty() { None } else { Some((coordinates, bbox)) }
}

/// Given a set of chunks, build a set of paths
///
/// ## Parameters
/// - `chunks`: a set of chunks
/// - `path_lookup`: a lookup of existing paths
/// - `paths`: a set of paths to add to
fn build_paths_from_chunks<P: FullXY>(
    paths: &mut Vec<PolyPathRef<P>>,
    path_lookup: &mut BTreeMap<usize, PolyPathRef<P>>,
    intersections: &InterPointLookup<P>,
    chunks: &[RingChunkRef<P>],
    bboxes: &[BBox],
) {
    // merge in all potential "dead" outer-rings (do we need this?)
    for path in paths.iter() {
        store_inner_old_outers(path, bboxes);
    }
    // for each intersections, connect all the from and to, smallest angle between from->to first slowly work your way through
    for int in intersections.lookup.values() {
        merge_intersection_pairs(int);
    }
    for chunk in chunks {
        if chunk.borrow().visted {
            continue;
        }
        // follow along a chunk until we find our start point again
        let start = chunk.borrow().from;
        let mut curr_chunk: RingChunkRef<P> = chunk.clone();
        let mut found_polygons: BTreeSet<usize> = BTreeSet::new();
        let mut linestring: Vec<P> = vec![P::new_xy(start.0, start.1)];
        let mut bbox = curr_chunk.borrow().bbox;
        loop {
            let next = {
                let curr_chunk = &mut curr_chunk.borrow_mut();
                if curr_chunk.visted {
                    break;
                }
                // add the chunk and mark it as visited
                curr_chunk.visted = true;
                linestring.append(&mut curr_chunk.mid);
                found_polygons.insert(curr_chunk.poly_index);
                bbox.merge_in_place(&curr_chunk.bbox);
                if let Some(NextRingChunk { chunk, int_point }) = curr_chunk.next.as_ref() {
                    linestring.push(P::new_xy(int_point.0, int_point.1));
                    chunk.clone()
                } else {
                    break;
                }
            };
            curr_chunk = next;
            if linestring.last() == linestring.first() {
                break;
            }
        }

        let area = (&linestring).area(Some(1.));
        if area == 0. || linestring.len() < 4 || linestring.first() != linestring.last() {
            continue;
        }
        // now build the path or add to an existing path
        let is_ccw = area > 0.;
        // Find the correct PolyPath to insert into, otherwise create a new one, update the lookup to
        // include all new polygon indexes used in the path
        let mut found_paths = vec![];
        // Pull in all the old paths to merge with this one (may expand upon multiple paths, consume the holes)
        let mut curr_id = 0;
        for poly_index in &found_polygons {
            if let Some(path) = path_lookup.get(poly_index) {
                path.borrow_mut().id = curr_id;
                curr_id += 1;
                found_paths.push(path.clone());
            }
        }
        // filter foundPaths if they have the same id as the previous
        found_paths.sort_by(|a, b| a.borrow().id.cmp(&b.borrow().id));
        found_paths = found_paths
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if i == 0 || p.borrow().id != found_paths[i - 1].borrow().id {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .collect();
        // merge
        let path: PolyPathRef<P>;
        if found_paths.is_empty() {
            path = PolyPath::new_ref(linestring, found_polygons.clone(), is_ccw, Some(bbox));
            paths.push(path.clone());
        } else {
            // TODO: `chunk.ringIndex !== 0` may not be enough as may contain a hole chunk but started as an outer chunk
            // if only one found, update that one, otherwise create a new merged path and store the new result
            path = if found_paths.len() == 1 {
                found_paths[0].clone()
            } else {
                merge_paths(&found_paths)
            };
            add_chunk_to_path(
                &mut path.borrow_mut(),
                linestring,
                &found_polygons,
                &bbox,
                is_ccw,
                chunk.borrow().ring_index != 0,
            );
        }
        // Store all inner outer rings that have not yet been consumed by the new outer but are inside the new outer
        store_inner_old_outers(&path, bboxes);
        // All found poly_index references now point to the new path
        for poly_index in found_polygons {
            path_lookup.insert(poly_index, path.clone());
        }
        // TODO: Poly's may still be able to consume eachother
    }
}

/**
 * Add a chunks built into a line+bbox to a path
 * @param path - the path to add to
 * @param ring - the linestring to add
 * @param poly_indexes - all polygon indexes touched
 * @param bbox - the bounding box of the collection of chunks (ring)
 * @param is_ccw - whether the ring is CCW
 * @param was_hole - whether the ring is a hole
 */
fn add_chunk_to_path<P: FullXY>(
    path: &mut PolyPath<P>,
    ring: Vec<P>,
    poly_indexes: &BTreeSet<usize>,
    bbox: &BBox,
    is_ccw: bool,
    was_hole: bool,
) {
    path.polys_consumed.extend(poly_indexes.iter());

    // If one poly outer ring is entirely in another poly AND its CCW, it gets "consumed" (deleted. path is
    // because of the ordering, the first chunk to be an outer will be the one creating the path,
    // so we know all future CCW chunks that share a path will be "outers" that are inside the existing
    // path outer)
    // If one poly outer ring is entirely in another poly AND its CW, it converts to a hole
    // If one poly inner ring is CW, it gets consumed by an associated outer
    // If one poly inner ring is CCW, remove it
    // If a hole is found, it didn't come from a hole, and it's inside one of the old outer's bboxes, delete the pair.
    if is_ccw {
        if was_hole {
            return;
        }
        if path.outer.is_none() {
            path.outer = Some(ring);
            path.bbox.merge_in_place(bbox);
        } else {
            // If the ring's bbox is smaller than the existing outer, store. Otherwise replace
            if bbox.inside(&path.bbox) {
                path.old_outers.push(*bbox);
            } else {
                path.old_outers.push(path.bbox);
                path.outer = Some(ring);
                path.bbox.merge_in_place(bbox);
            }
        }
    } else {
        if !was_hole {
            // Store discarded smaller outer rings, if hole is inside inner outer-ring, it cancels out the hole
            for old_outer in &path.old_outers {
                if bbox.inside(old_outer) {
                    return;
                }
            }
        }
        path.holes.push(ring);
    }
}

/// Store all inner old outers that have not yet been consumed by the new outer
/// @param path - the path
/// @param bboxes - the bboxes of all outer rings we are merging
fn store_inner_old_outers<P: FullXY>(path: &PolyPathRef<P>, bboxes: &[BBox]) {
    let path = &mut path.borrow_mut();
    if path.outer.is_none() {
        return;
    }
    for (i, bbox) in bboxes.iter().enumerate().skip(1) {
        if path.polys_consumed.contains(&i) {
            continue;
        }
        if bbox.inside(&path.bbox) {
            path.old_outers.push(*bbox);
        }
    }
}

/// Merge in a collection of paths
///
/// ## Parameters
/// - `paths_to_merge`: the collection of paths
/// - `path_store`: the collection of paths
///
/// ## Returns
/// The result of all paths merged into one
fn merge_paths<P: FullXY>(paths_to_merge: &[PolyPathRef<P>]) -> PolyPathRef<P> {
    let result = paths_to_merge[0].clone();
    {
        let res = &mut result.borrow_mut();
        for other in paths_to_merge.iter().skip(1) {
            let other = &mut other.borrow_mut();
            // If this bbox is smaller than the existing outer, replace
            let other_bbox = other.bbox;
            if res.bbox.inside(&other_bbox)
                && let Some(other_outer) = &mut other.outer
            {
                if res.outer.is_some() {
                    res.old_outers.push(other_bbox);
                }
                res.outer = Some(take(other_outer));
            }
            res.holes.append(&mut other.holes);
            res.polys_consumed.extend(&other.polys_consumed);
            res.bbox.merge_in_place(&other.bbox);
            // clear the path now that we comsumed it
            other.outer = None;
            other.old_outers = vec![];
        }
    }

    result
}
