use crate::data_structures::PriorityQueue;
use alloc::vec::Vec;
use core::f64::consts::SQRT_2;
use libm::sqrt;
use s2json::{MValueCompatible, VectorMultiPolygon, VectorPoint, VectorPolygon};

/// The metadata inserted into the Vector Feature
#[derive(Debug, Default, MValueCompatible, Clone)]
pub struct PolyLabelMetadata {
    /// The distance to the label
    pub distance: f64,
}
impl PolyLabelMetadata {
    /// Create a new PolyLabelMetadata
    pub fn new(distance: f64) -> PolyLabelMetadata {
        PolyLabelMetadata { distance }
    }
}

/// A cell in the polygon label algorithm
#[derive(Debug, Default, Clone)]
pub struct PolyLabelCell {
    /// cell center x
    pub x: f64,
    /// cell center y
    pub y: f64,
    /// half the cell size
    pub h: f64,
    /// distance from cell center to polygon
    pub d: f64,
    /// max distance to polygon within a cell
    pub max: f64,
}

/// # Polylabels
///
/// ## Description
/// Find the labels for a collection of vector polygons
///
/// ## Links
/// - https://sites.google.com/site/polesofinaccessibility/
pub fn polylabels<M: Clone>(
    polygons: &VectorMultiPolygon<M>,
    precision: Option<f64>,
) -> Vec<VectorPoint<PolyLabelMetadata>> {
    polygons.iter().map(|polygon| polylabel(polygon, precision)).collect()
}

/// # Polylabel
///
/// ## Description
/// Find the label for a vector polygon
///
/// ## Usage
/// ```ts
/// import { polylabel } from 'gis-tools-ts'
/// import type { VectorPolygon } from 'gis-tools-ts'
///
/// const vectorGeometry: VectorPolygon = [];
/// const polylabel_high_precision = polylabel(vectorGeometry, 1);
/// ```
///
/// ## Links
/// - https://sites.google.com/site/polesofinaccessibility/
///
/// returns the label position and the distance to the label
pub fn polylabel<M: Clone>(
    polygon: &VectorPolygon<M>,
    precision: Option<f64>,
) -> VectorPoint<PolyLabelMetadata> {
    let precision = precision.unwrap_or(1.0);
    // find the bounding box of the outer ring
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    if polygon.is_empty() || polygon[0].is_empty() {
        return VectorPoint::new_xy(0.0, 0.0, Some(PolyLabelMetadata::default()));
    }

    for VectorPoint { x, y, .. } in &polygon[0] {
        if *x < min_x {
            min_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    let width = max_x - min_x;
    let height = max_y - min_y;
    let cell_size = f64::max(precision, f64::min(width, height));

    if cell_size == precision {
        return VectorPoint::new_xy(min_x, min_y, Some(PolyLabelMetadata::default()));
    }

    // a priority queue of cells in order of their "potential" (max distance to polygon)
    let mut cell_queue =
        PriorityQueue::<PolyLabelCell>::new(|a: &PolyLabelCell, b: &PolyLabelCell| {
            b.max.partial_cmp(&a.max).unwrap_or(core::cmp::Ordering::Equal)
        });

    // take centroid as the first best guess
    let mut best_cell = get_centroid_cell(polygon);

    // second guess: bounding box centroid
    let bbox_cell = build_cell(min_x + width / 2., min_y + height / 2., 0., polygon);
    if bbox_cell.d > best_cell.d {
        best_cell = bbox_cell;
    }

    // add a cell to the queue
    let potentially_queue =
        |x: f64,
         y: f64,
         h: f64,
         best_cell: &mut PolyLabelCell,
         cell_queue: &mut PriorityQueue<PolyLabelCell>| {
            let cell = build_cell(x, y, h, polygon);
            if cell.max > best_cell.d + precision {
                cell_queue.push(cell.clone());
            }
            // update the best cell if we found a better one
            if cell.d > best_cell.d {
                *best_cell = cell;
            }
        };

    // cover polygon with initial cells
    let mut h = cell_size / 2.;
    let mut x = min_x;
    while x < max_x {
        let mut y = min_y;
        while y < max_y {
            potentially_queue(x + h, y + h, h, &mut best_cell, &mut cell_queue);
            y += cell_size;
        }
        x += cell_size;
    }

    loop {
        // pick the most promising cell from the queue
        let cell = cell_queue.pop();
        if cell.is_none() {
            break;
        }
        let PolyLabelCell { max, x, y, h: ch, .. } = &cell.unwrap();

        // do not drill down further if there's no chance of a better solution
        if max - best_cell.d <= precision {
            break;
        }

        // split the cell into four cells
        h = ch / 2.;
        potentially_queue(x - h, y - h, h, &mut best_cell, &mut cell_queue);
        potentially_queue(x + h, y - h, h, &mut best_cell, &mut cell_queue);
        potentially_queue(x - h, y + h, h, &mut best_cell, &mut cell_queue);
        potentially_queue(x + h, y + h, h, &mut best_cell, &mut cell_queue);
    }

    VectorPoint::new_xy(best_cell.x, best_cell.y, Some(PolyLabelMetadata::new(best_cell.d)))
}

/// build a cell
/// @param x - the cell x coordinate
/// @param y - the cell y coordinate
/// @param h - half the cell size
/// @param polygon - the vector polygon
/// @returns - the cell
fn build_cell<M: Clone>(x: f64, y: f64, h: f64, polygon: &VectorPolygon<M>) -> PolyLabelCell {
    let d = point_to_polygon_dist(x, y, polygon);
    PolyLabelCell { x, y, h, d, max: d + h * SQRT_2 }
}

/// signed distance from point to polygon outline (negative if point is outside)
/// @param x - the point x coordinate
/// @param y - the point y coordinate
/// @param polygon - the vector polygon to check
/// @returns - the signed distance
fn point_to_polygon_dist<M: Clone>(x: f64, y: f64, polygon: &VectorPolygon<M>) -> f64 {
    let mut inside = false;
    let mut min_dist_sq = f64::MAX;

    for ring in polygon {
        let len = ring.len();
        let mut j = len - 1;
        for i in 0..len {
            let a = &ring[i];
            let b = &ring[j];

            if (a.y > y) != (b.y > y) && x < ((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x {
                inside = !inside;
            }

            min_dist_sq = f64::min(min_dist_sq, get_seg_dist_sq(x, y, a, b));
            j = i; // Update j to the previous i (j = i++)
        }
    }

    if min_dist_sq == 0. { 0. } else { (if inside { 1. } else { -1. }) * sqrt(min_dist_sq) }
}

/// get polygon centroid
/// return the centroid as a cell
fn get_centroid_cell<M: Clone>(polygon: &VectorPolygon<M>) -> PolyLabelCell {
    let mut area = 0.;
    let mut x = 0.;
    let mut y = 0.;
    let points = &polygon[0];

    let len = points.len();
    let mut j = len - 1;
    for i in 0..len {
        let a = &points[i];
        let b = &points[j];
        let f = a.x * b.y - b.x * a.y;
        x += (a.x + b.x) * f;
        y += (a.y + b.y) * f;
        area += f * 3.;
        j = i; // Update j to the previous i (j = i++)
    }
    let centroid = build_cell(x / area, y / area, 0., polygon);
    if area == 0. || centroid.d < 0. {
        build_cell(points[0].x, points[0].y, 0., polygon)
    } else {
        centroid
    }
}

/// get squared distance from a point to a segment AB
/// return the squared distance
fn get_seg_dist_sq<M: Clone>(px: f64, py: f64, a: &VectorPoint<M>, b: &VectorPoint<M>) -> f64 {
    let mut x = a.x;
    let mut y = a.y;
    let mut dx = b.x - x;
    let mut dy = b.y - y;

    if dx != 0. || dy != 0. {
        let t = ((px - x) * dx + (py - y) * dy) / (dx * dx + dy * dy);

        if t > 1. {
            x = b.x;
            y = b.y;
        } else if t > 0. {
            x += dx * t;
            y += dy * t;
        }
    }

    dx = px - x;
    dy = py - y;

    dx * dx + dy * dy
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use s2json::{Polygon, VectorLineString};
    use std::{fs, path::PathBuf};

    #[test]
    fn empty() {
        let data: Vec<Vec<VectorPoint<()>>> = vec![vec![]];
        let emp = polylabel(&data, None);

        assert_eq!(emp, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));
    }

    #[test]
    fn works_on_degenerate_polygons() {
        let p1 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(2., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p1, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));

        let p2 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(1., 1., None),
                VectorPoint::new_xy(1., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p2, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));

        let p3 = polylabel(
            &vec![vec![
                VectorPoint::<()>::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
                VectorPoint::new_xy(0., 0., None),
            ]],
            None,
        );
        assert_eq!(p3, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));
    }

    #[test]
    fn water1_pole_of_inaccess_precision_1() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water1.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_1: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_1: VectorPolygon = convert_poly(&water_1);
        let polylabel_high_precision = polylabel(&vector_water_1, Some(1.));
        assert_eq!(
            polylabel_high_precision,
            VectorPoint::new_xy(
                3865.85009765625,
                2124.87841796875,
                Some(PolyLabelMetadata::new(288.8493574779127)),
            )
        );

        let polylabel_low_precision = polylabel(&vector_water_1, Some(50.));
        assert_eq!(
            polylabel_low_precision,
            VectorPoint::new_xy(
                3854.296875,
                2123.828125,
                Some(PolyLabelMetadata::new(278.5795872381558)),
            )
        );
    }

    #[test]
    fn water1_pole_of_inaccess_precision_1_multi() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water1.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_1: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_1: VectorPolygon = convert_poly(&water_1);
        let polylabel_high_precision = polylabels(&vec![vector_water_1.clone()], Some(1.));
        assert_eq!(
            polylabel_high_precision,
            vec![VectorPoint::new_xy(
                3865.85009765625,
                2124.87841796875,
                Some(PolyLabelMetadata::new(288.8493574779127)),
            )]
        );

        let polylabel_low_precision = polylabels(&vec![vector_water_1], Some(50.));
        assert_eq!(
            polylabel_low_precision,
            vec![VectorPoint::new_xy(
                3854.296875,
                2123.828125,
                Some(PolyLabelMetadata::new(278.5795872381558)),
            )]
        );
    }

    #[test]
    fn water2_pole_of_inaccess_precision() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/tools/fixtures/water2.json");
        let file_as_str = fs::read_to_string(path).unwrap();
        let water_2: Polygon = serde_json::from_str(&file_as_str).unwrap();

        let vector_water_2: VectorPolygon = convert_poly(&water_2);
        let polylabel_high_precision = polylabel(&vector_water_2, Some(1.));
        assert_eq!(
            polylabel_high_precision,
            VectorPoint::new_xy(3263.5, 3263.5, Some(PolyLabelMetadata::new(960.5)),)
        );
    }

    fn convert_poly<M: Clone>(input: &Polygon) -> VectorPolygon<M> {
        let mut res: VectorPolygon<M> = vec![];
        for ring in input {
            let mut new_ring: VectorLineString<M> = vec![];
            for point in ring {
                new_ring.push(VectorPoint::new_xy(point.0, point.1, None));
            }
            res.push(new_ring);
        }

        res
    }
}
