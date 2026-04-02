use alloc::{collections::BTreeMap, rc::Rc};
use core::cell::RefCell;
use libm::{ceil, fabs, round};
use s2json::{Point, VectorPoint};

// https://github.com/urbanspr1nter/marching-squares/blob/main/Teaching-Marching-Squares.pdf

// Each index 0-15 returns pairs of edges to connect.
// Example: Case 3 (binary 0011) connects edge 3 to edge 1.
static MS_LUT: &[&[u8]] = &[
    &[],           // 0: All points outside
    &[3, 2],       // 1
    &[2, 1],       // 2
    &[3, 1],       // 3
    &[1, 0],       // 4
    &[3, 0, 1, 2], // 5: Saddle case
    &[2, 0],       // 6
    &[3, 0],       // 7
    &[3, 0],       // 8
    &[2, 0],       // 9
    &[3, 2, 1, 0], // 10: Saddle case
    &[1, 0],       // 11
    &[3, 1],       // 12
    &[2, 1],       // 13
    &[3, 2],       // 14
    &[],           // 15: All points inside
];

/// An ordered float for BTreeMap sorting
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct OrderedF64(pub f64);
impl Eq for OrderedF64 {}
impl Ord for OrderedF64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// A grid point
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct GridPoint {
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
}
impl From<GridPoint> for VectorPoint {
    fn from(value: GridPoint) -> Self {
        VectorPoint::from_xy(value.x as f64 / 32_768., value.y as f64 / 32_768.)
    }
}

/// A segment of an isoline
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct IsolineSegment {
    /// The start point
    pub from: GridPoint,
    /// The end point
    pub to: GridPoint,
    /// If the segment has already been visited (used in the stitcher algorithm)
    pub visited: bool,
}
impl IsolineSegment {
    /// Create a new IsolineSegment but wrap it in an Rc-RefCell
    pub fn new(from: GridPoint, to: GridPoint) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(IsolineSegment { from, to, visited: false }))
    }
}

// /// Get the isoline thresholds relative to a minimum, maximum, and step size
// pub fn isoline_thresholds(min: f64, max: f64, step: f64) -> Vec<f64> {
//     let mut thresholds: Vec<f64> = vec![];
//     let mut current = ceil(min / step) * step;

//     while current <= max {
//         thresholds.push(current);
//         current += step;
//     }

//     thresholds
// }

/// The result of a marching squares operation
pub type MarchingSquaresResult = BTreeMap<OrderedF64, Vec<Rc<RefCell<IsolineSegment>>>>;

/// Create isolines from a flat 2D heightmap
///
/// ## Parameters
/// - `heightmap`: the heightmap data
/// - `width`: the width of the heightmap
/// - `height`: the height of the heightmap
/// - `padding`: how many pixels in the heightmap are padding
/// - `thresholds`: the thresholds to use
///
/// ## Returns
/// The isolines
pub fn marching_squares(
    heightmap: &[f64],
    width: usize,
    height: usize,
    padding: f64,
    thresholds: &[f64],
) -> MarchingSquaresResult {
    let mut all_segments_by_level: MarchingSquaresResult = BTreeMap::new();

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            // Get the 4 corners
            let h0 = heightmap[y * width + x];
            let h1 = heightmap[y * width + (x + 1)];
            let h2 = heightmap[(y + 1) * width + (x + 1)];
            let h3 = heightmap[(y + 1) * width + x];
            let corners = [h0, h1, h2, h3];
            let min = corners.into_iter().reduce(f64::min).unwrap();
            let max = corners.into_iter().reduce(f64::max).unwrap();

            // Only check thresholds that actually pass through this specific cell
            for t in thresholds {
                if *t >= min && *t <= max {
                    let segments = march_cell(
                        &corners,
                        width as f64,
                        height as f64,
                        padding,
                        x as f64,
                        y as f64,
                        *t,
                    );
                    if !segments.is_empty() {
                        // Push to a flat array for now (The "Soup")
                        let level_segments =
                            all_segments_by_level.entry(OrderedF64(*t)).or_default();
                        level_segments.extend(segments);
                    }
                }
            }
        }
    }

    all_segments_by_level
}

/// Processes a cell and returns interpolated segments
///
/// - `corners`: Array of 4 corner values [top-left, top-right, bottom-right, bottom-left]
/// - `width`: Grid width
/// - `height`: Grid height
/// - `padding`: The number of pixels that extend around the main data
/// - `x`: Grid X coordinate
/// - `y`: Grid Y coordinate
/// - `threshold`: The elevation value we are looking for
///
/// ## Returns
/// Array of segments. If there are no segments, an empty array is returned
fn march_cell(
    corners: &[f64],
    width: f64,
    height: f64,
    padding: f64,
    x: f64,
    y: f64,
    t: f64,
) -> Vec<Rc<RefCell<IsolineSegment>>> {
    // 1. Determine the 4-bit case index
    let mut case_index = 0;
    if corners[0] >= t {
        case_index |= 8;
    } // Top-Left
    if corners[1] >= t {
        case_index |= 4;
    } // Top-Right
    if corners[2] >= t {
        case_index |= 2;
    } // Bottom-Right
    if corners[3] >= t {
        case_index |= 1;
    } // Bottom-Left

    let edges = MS_LUT[case_index];
    let mut segments: Vec<Rc<RefCell<IsolineSegment>>> = vec![];
    if edges.len() == 0 {
        return segments;
    }

    // 2. Process edges in pairs (usually 1 segment, 2 for saddle cases)
    for i in (0..edges.len()).step_by(2) {
        let from = interpolate(edges[i], corners, width, height, padding, x, y, t);
        let to = interpolate(edges[i + 1], corners, width, height, padding, x, y, t);
        segments.push(IsolineSegment::new(from, to));
    }

    segments
}

// Interpolate an edge
fn interpolate(
    edge: u8,
    corners: &[f64],
    width: f64,
    height: f64,
    padding: f64,
    x: f64,
    y: f64,
    t: f64,
) -> GridPoint {
    let point: Point;
    // interpolate edge
    if edge == 0 {
        point = Point(x + safe_interp(corners[0], corners[1], t), y); // Top
    } else if edge == 1 {
        point = Point(x + 1., y + safe_interp(corners[1], corners[2], t)); // Right
    } else if edge == 2 {
        point = Point(x + safe_interp(corners[3], corners[2], t), y + 1.); // Bottom
    } else if edge == 3 {
        point = Point(x, y + safe_interp(corners[0], corners[3], t)); // Left
    } else {
        point = Point(x, y); // Center
    }

    // remap to 0->32_768
    remap(point, width, height, padding)
}

// epsilon check to avoid division by zero
fn safe_interp(v1: f64, v2: f64, t: f64) -> f64 {
    if fabs(v1 - v2) < 1e-10 {
        return 0.5;
    }
    return (t - v1) / (v2 - v1);
}

/// Convert the x-y values from width-height scale to 0->32_768
///
/// ## Parameters
/// - `point`: VectorPoint to mutate
/// - `width`: width of the grid
/// - `height`: height of the grid
/// - `padding`: The number of pixels that extend around the main data
fn remap(point: Point, width: f64, height: f64, padding: f64) -> GridPoint {
    let active_width = width - 1. - 2. * padding;
    let active_height = height - 1. - 2. * padding;
    GridPoint {
        x: round(((point.0 + 0.5 - padding) * 32_768.) / active_width) as i32,
        y: round(((point.1 + 0.5 - padding) * 32_768.) / active_height) as i32,
    }
}
