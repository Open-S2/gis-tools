use crate::tools::{GridPoint, IsolineSegment, MarchingSquaresResult, OrderedF64};
use alloc::{collections::BTreeMap, rc::Rc};
use core::cell::RefCell;
use s2json::{MValue, VectorFeature, VectorGeometry, VectorLineString, VectorMultiLineString};
use serde::{Deserialize, Serialize};

/// GridPoint: To Segment
pub type SegmentLookup = BTreeMap<GridPoint, Vec<Rc<RefCell<IsolineSegment>>>>;

/// Contour Properties
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, MValue)]
pub struct ContourProperties {
    /// Elevation in meters
    pub elev: f64,
    /// Elevation in feet
    #[serde(rename = "elevFt")]
    pub elev_ft: f64,
}

/// Stitch all segments together
pub fn stitch_marching_square_segments(
    marching_squares: MarchingSquaresResult,
) -> Vec<VectorFeature> {
    let mut result: Vec<VectorFeature> = vec![];

    for (OrderedF64(threshold), mut segments) in marching_squares {
        // setup graph for threshold
        let mut point_to_segments: SegmentLookup = BTreeMap::new();
        for seg in &segments {
            let IsolineSegment { from: p1, to: p2, .. } = *seg.borrow();
            let p1_set = point_to_segments.entry(p1).or_insert(vec![]);
            p1_set.push(seg.clone());
            let p2_set = point_to_segments.entry(p2).or_insert(vec![]);
            p2_set.push(seg.clone());
        }

        let mut coordinates: VectorMultiLineString = vec![];
        // Stitching
        for start_seg in &mut segments {
            if start_seg.borrow().visited {
                continue;
            }

            let mut forward_points: VectorLineString =
                vec![start_seg.borrow().from.into(), start_seg.borrow().to.into()];
            start_seg.borrow_mut().visited = true;

            // Grow forward from 'to'
            grow_line(&mut forward_points, start_seg.borrow().to, &mut point_to_segments);
            // Grow backward from 'from' (and reverse the result)
            let mut backward_points: VectorLineString = vec![];
            grow_line(&mut backward_points, start_seg.borrow().from, &mut point_to_segments);
            // merge forward and backward
            let mut line: VectorLineString = vec![];
            line.extend(backward_points.iter().rev().cloned());
            line.extend(forward_points.drain(..));

            if line.len() <= 2 {
                continue;
            }
            coordinates.push(line);
        }

        if coordinates.is_empty() {
            continue;
        }
        result.push(VectorFeature::new_wm(
            None,
            (ContourProperties { elev: threshold, elev_ft: threshold * 3.28084 }).into(),
            VectorGeometry::new_multilinestring(coordinates, None),
            None,
        ));
    }

    result
}

fn grow_line(line: &mut VectorLineString, mut curr_pt: GridPoint, graph: &mut SegmentLookup) {
    loop {
        let Some(neighbors) = graph.get_mut(&curr_pt) else {
            break;
        };
        let Some(next_seg) = neighbors.iter_mut().find(|s| !s.borrow().visited) else {
            break;
        };

        let next_seg = &mut next_seg.borrow_mut();

        next_seg.visited = true;
        // Figure out which end of the segment is the new point
        let next_pt = if next_seg.from == curr_pt { next_seg.to } else { next_seg.from };
        line.push(next_pt.into());
        curr_pt = next_pt;
    }
}
