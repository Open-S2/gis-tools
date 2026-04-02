use crate::geometry::{Area, clean_linestring, dekink_polygon};
use alloc::{vec, vec::Vec};
use s2json::{BBox, FullXY};

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
/// - `clean_wgs84`: if true, clean WGS84 points to be valid WGS84 points
/// - `remove_zero_area`: if true, remove zero-area polygons
///
/// ## Returns
/// The cleaned polygons as a new collection of polygons
pub fn clean_polygons<P: FullXY>(
    polygons: &[Vec<Vec<P>>],
    remove_collinear_points: bool,
    clean_wgs84: bool,
    remove_zero_area: bool,
) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    let mut res: Vec<Vec<Vec<P>>> = vec![];
    let mut final_bbox: BBox = BBox::default();

    for p in polygons {
        if let Some((mut cleaned, bbox)) =
            clean_polygon(p, remove_collinear_points, clean_wgs84, remove_zero_area)
        {
            res.append(&mut cleaned);
            final_bbox.merge_in_place(&bbox);
        }
    }
    if res.is_empty() { None } else { Some((res, final_bbox)) }
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
/// - `clean_wgs84`: if true, clean WGS84 points to be valid WGS84 points
/// - `remove_zero_area`: if true, remove zero-area polygons
///
/// ## Returns
/// The cleaned polygon, split into a multi-polygon as necessary
pub fn clean_polygon<P: FullXY>(
    polygon: &[Vec<P>],
    remove_collinear_points: bool,
    clean_wgs84: bool,
    remove_zero_area: bool,
) -> Option<(Vec<Vec<Vec<P>>>, BBox)> {
    // remove duplicates from the rings
    let mut res: Vec<Vec<P>> = vec![];
    for (index, ring) in polygon.iter().enumerate() {
        let mut last_point: Option<&P> = None;

        if remove_collinear_points {
            match clean_linestring(ring, true, None, clean_wgs84) {
                Some(cleaned) => res.push(cleaned),
                None => {
                    if index == 0 {
                        return None;
                    }
                }
            }
        } else {
            let mut new_ring: Vec<P> = vec![];
            for point in ring {
                if last_point.is_none() || *point != *last_point.unwrap() {
                    new_ring.push(point.clone());
                    last_point = Some(point);
                }
            }
            if new_ring.len() >= 4 {
                res.push(new_ring);
            } else if index == 0 {
                return None;
            }
        }
    }
    // run polygon_ring_area for each ring and invert if it's direction is wrong for the ring type
    for (i, ring) in res.iter_mut().enumerate() {
        let area = ring.area(Some(1.));
        // 0 area rings are removed
        if remove_zero_area && area == 0. {
            if i == 0 {
                return None;
            } else {
                continue;
            }
        }
        // flip the ring if outer-ring and area is negative OR inner-ring and area is positive
        if if i == 0 { area < 0. } else { area > 0. } {
            ring.reverse();
        }
    }

    dekink_polygon(&res)
}
