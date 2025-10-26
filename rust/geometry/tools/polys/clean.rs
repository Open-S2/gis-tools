use crate::geometry::{Area, clean_linestring, dekink_polygon};
use alloc::{vec, vec::Vec};
use s2json::{GetXY, NewXY};

/// Ensures the collection of polygon ring order is correct, removes duplicate points,
/// and runs a dekink to be thorough.
///
/// NOTE: This will not remove/reduce points that follow a path angle like [[0, 0], [0, 1], [0, 2], ...].
/// The decision to leave this to the user is due to the fact that not all projections are guaranteed
/// to support a linear relationship. Also sometimes the user want's to have these extra points for
/// future/cleaner projection changes. For example, having higher precision works well when
/// translating to spherical projections for instance. If you do want to remove these points, pass
/// in true to `remove_collinear_points`
///
/// ## Parameters
/// - `polygons`: the collection of polygon as either a VectorFeature, VectorMultiPolygonGeometry, or raw VectorMultiPolygon
/// - `remove_collinear_points`: - if true, remove superfluous points
///
/// ## Returns
/// The cleaned polygons as a new collection of polygons
pub fn clean_polygons<P: NewXY + GetXY + PartialEq + Clone>(
    polygons: &Vec<Vec<Vec<P>>>,
    remove_collinear_points: bool,
) -> Vec<Vec<Vec<P>>> {
    polygons.into_iter().flat_map(|p| clean_polygon(p, remove_collinear_points)).collect()
}

/// Ensures the polygon ring order is correct, removes duplicate points, and runs a dekink to be
/// thorough.
///
/// NOTE: This will not remove/reduce points that follow a path angle like [[0, 0], [0, 1], [0, 2], ...].
/// The decision to leave this to the user is due to the fact that not all projections are guaranteed
/// to support a linear relationship. Also sometimes the user want's to have these extra points for
/// future/cleaner projection changes. For example, having higher precision works well when
/// translating to spherical projections for instance. If you do want to remove these points, pass
/// in true to `remove_collinear_points`
///
/// ## Parameters
/// - `polygon`: the polygon as either a VectorFeature, VectorPolygonGeometry, or raw VectorPolygon
/// - `remove_collinear_points`: if true, remove superfluous points
///
/// ## Returns
/// The cleaned polygon, split into a multi-polygon as necessary
pub fn clean_polygon<P: NewXY + GetXY + PartialEq + Clone>(
    polygon: &Vec<Vec<P>>,
    remove_collinear_points: bool,
) -> Vec<Vec<Vec<P>>> {
    // remove duplicates from the rings
    let mut res: Vec<Vec<P>> = vec![];
    for ring in polygon {
        let mut last_point: Option<&P> = None;

        if remove_collinear_points {
            res.push(clean_linestring(ring, true, None));
        } else {
            let mut new_ring: Vec<P> = vec![];
            for point in ring {
                if last_point.is_none() || *point != *last_point.unwrap() {
                    new_ring.push(point.clone());
                    last_point = Some(point);
                }
            }
            res.push(new_ring);
        }
    }
    // run polygon_ring_area for each ring and invert if it's direction is wrong for the ring type
    for i in 0..res.len() {
        let ring = &res[i];
        let area = ring.area(Some(1.));
        // flip the ring if outer-ring and area is negative OR inner-ring and area is positive
        if if i == 0 { area < 0. } else { area > 0. } {
            res[i].reverse();
        }
    }

    dekink_polygon(&res)
}
