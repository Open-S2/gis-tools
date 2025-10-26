use crate::data_structures::PriorityQueue;
use alloc::vec::Vec;
use core::f64::consts::SQRT_2;
use libm::{fmax, fmin, sqrt};
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
/// ## Usage
///
/// ```rust
/// use gistools::tools::{PolyLabelMetadata, polylabels};
/// use s2json::VectorPoint;
///
/// let data: Vec<Vec<Vec<VectorPoint<()>>>> = vec![vec![vec![]]];
/// let emp = polylabels(&data, None);
///
/// assert_eq!(emp, vec![VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.)))]);
/// ```
///
/// ## Links
/// - <https://sites.google.com/site/polesofinaccessibility/>
///
/// ## Parameters
/// - `polygons`: the vector multi-polygon to find the label for
/// - `precision`: the precision of the label
///
/// ## Returns
/// The collection of label positions and the distances to the labels
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
///
/// ```rust
/// use gistools::tools::{PolyLabelMetadata, polylabel};
/// use s2json::VectorPoint;
///
/// let data: Vec<Vec<VectorPoint<()>>> = vec![vec![]];
/// let emp = polylabel(&data, None);
///
/// assert_eq!(emp, VectorPoint::new_xy(0., 0., Some(PolyLabelMetadata::new(0.))));
/// ```
///
/// ## Links
/// - <https://sites.google.com/site/polesofinaccessibility/>
///
/// ## Parameters
/// - `polygon`: the vector polygon to find the label for
/// - `precision`: the precision of the label
///
/// ## Returns
/// The label position and the distance to the label
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
    let cell_size = fmax(precision, fmin(width, height));

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
///
/// ## Parameters
/// - `x`: the cell x coordinate
/// - `y`: the cell y coordinate
/// - `h`: half the cell size
/// - `polygon`: the vector polygon
///
/// ## Returns
/// The cell
fn build_cell<M: Clone>(x: f64, y: f64, h: f64, polygon: &VectorPolygon<M>) -> PolyLabelCell {
    let d = point_to_polygon_dist(x, y, polygon);
    PolyLabelCell { x, y, h, d, max: d + h * SQRT_2 }
}

/// signed distance from point to polygon outline (negative if point is outside)
///
/// ## Parameters
/// - `x`: the point x coordinate
/// - `y`: the point y coordinate
/// - `polygon`: the vector polygon to check
///
/// ## Returns
/// The signed distance
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

            min_dist_sq = fmin(min_dist_sq, get_seg_dist_sq(x, y, a, b));
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
