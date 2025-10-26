use crate::geometry::{Intersection, polygons_intersections_ref};
use alloc::{vec, vec::Vec};
use s2json::{GetXY, NewXY};

// TODO: At some point intersections of inner rings against the outer ring should be considered
// be sure to address the `segment1.ring_index === segment2.ring_index` filter when implementing

/// Given a collection of polygons, if any of the polygons are kinked, dekink them
///
/// ## Parameters
/// `polygons`: the polygons are from either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
///
/// ## Returns
/// The dekinked polygons
pub fn dekink_polygons<P: NewXY + GetXY + PartialEq + Clone>(
    polygons: &Vec<Vec<Vec<P>>>,
) -> Vec<Vec<Vec<P>>> {
    polygons.into_iter().flat_map(|polygon| dekink_polygon(polygon)).collect()
}

/// Given a polygon, if the polygon is kinked, dekink it
///
/// ## Parameters
/// - `polygon`: the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
///
/// ## Returns
/// The dekinked polygon
pub fn dekink_polygon<P: NewXY + GetXY + PartialEq + Clone>(
    polygon: &Vec<Vec<P>>,
) -> Vec<Vec<Vec<P>>> {
    // build all segments, filter out "segments" that are endpoints and intersections that are not on the same ring
    let mut intersections: Vec<Intersection> = polygons_intersections_ref(&vec![polygon], true)
        .into_iter()
        .filter(|Intersection { segment1, segment2, u, t, .. }| {
            *u != 0.
                && *u != 1.
                && *t != 0.
                && *t != 1.
                && segment1.ring_index == segment2.ring_index
        })
        .collect();
    // Sort intersections by `ring_index` then `from`
    intersections.sort_by(|a, b| {
        a.segment1
            .ring_index
            .cmp(&b.segment1.ring_index)
            .then_with(|| a.segment1.from.cmp(&b.segment1.from))
    });

    let mut res: Vec<Vec<Vec<P>>> = vec![];

    // if there are no intersections, return a clone of the original polygon
    if intersections.len() == 0 {
        res.push(polygon.clone());
        return res;
    }

    // The points outside the kinks are summed up from the beginning of the polygon ring till it
    // reaches intersections, then each intersection you move to the intersection point itself and
    // keep going onwards the "self-intersecting" ring data are the intersection segment
    // from -> intersection to IF the ring length is greater than 4 total points
    let mut dekinked_polygon: Vec<Vec<P>> = vec![];
    for r in 0..polygon.len() {
        let ring_intersections: Vec<_> =
            intersections.iter().filter(|i| i.segment1.ring_index == r).collect();
        let ring = &polygon[r];
        let mut dekinked_ring: Vec<P> = vec![];
        // build the outer ring slicing around intersections
        let mut index = 0;
        for Intersection { point, segment1: start_segment, segment2: end_segment, .. } in
            &ring_intersections
        {
            dekinked_ring.extend((&ring[index..start_segment.from + 1]).to_vec());
            dekinked_ring.push(NewXY::new_xy(point.x(), point.y()));
            index = end_segment.to;
        }
        dekinked_ring.extend((&ring[index..]).to_vec());
        dekinked_polygon.push(dekinked_ring);

        // build the portions inside the kinks of the ring using inside each segment intersection
        for Intersection { segment1, segment2, point, .. } in &ring_intersections {
            let mut self_intersect_ring: Vec<P> = vec![];
            // begin at intersection
            self_intersect_ring.push(NewXY::new_xy(point.x(), point.y()));
            // add all internal points
            self_intersect_ring
                .extend((&ring[segment1.to..segment2.from + 1]).iter().map(|p| p.clone()));
            // end at intersection
            self_intersect_ring.push(NewXY::new_xy(point.x(), point.y()));
            // If the ring is an inner polygon ring (hole), keep adding the holes to the dekinked_polygon
            // otherwise its a new poylgon outer ring
            if r != 0 {
                dekinked_polygon.push(self_intersect_ring);
            } else {
                // add the ring that's now it's own polygon outer-ring
                res.push(vec![self_intersect_ring]);
            }
        }
    }
    res.insert(0, dekinked_polygon);

    res
}
