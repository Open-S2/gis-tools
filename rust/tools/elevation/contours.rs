use crate::{
    geometry::{Area, build_sq_dist, polyline_in_polyline, simplify_line},
    parsers::Buffer,
    tools::{ElevationConverter, IsoGrid, get_elevation_grid},
};
use alloc::{collections::BTreeMap, rc::Rc};
use core::cell::RefCell;
use libm::{ceil, trunc};
use s2json::{
    FeatureCollection, Features, GetXY, MValue, SetXY, VectorFeature, VectorGeometry,
    VectorLineString, VectorMultiLineString, VectorMultiPolygon, VectorPoint,
};
use serde::{Deserialize, Serialize};

/// Contour Properties
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize, MValue)]
pub struct ContourProperties {
    /// Elevation in meters
    pub elev: f64,
    /// Elevation in feet
    #[serde(rename = "elevFt")]
    pub elev_ft: f64,
}

#[derive(Debug, Clone)]
struct Fragment {
    pub start: usize,
    pub end: usize,
    pub ring: VectorLineString,
}
impl Fragment {
    pub fn new(start: usize, end: usize, ring: VectorLineString) -> Rc<RefCell<Fragment>> {
        Rc::new(RefCell::new(Fragment { start, end, ring }))
    }
}

// Each index 0-15 returns pairs of edges to connect.
// Example: Case 3 (binary 0011) connects edge 3 to edge 1.
static MS_LUT: &[&[[(f64, f64); 2]]] = &[
    &[],                                                   // 0
    &[[(1.0, 1.5), (0.5, 1.0)]],                           // 1
    &[[(1.5, 1.0), (1.0, 1.5)]],                           // 2
    &[[(1.5, 1.0), (0.5, 1.0)]],                           // 3
    &[[(1.0, 0.5), (1.5, 1.0)]],                           // 4
    &[[(1.0, 1.5), (0.5, 1.0)], [(1.0, 0.5), (1.5, 1.0)]], // 5
    &[[(1.0, 0.5), (1.0, 1.5)]],                           // 6
    &[[(1.0, 0.5), (0.5, 1.0)]],                           // 7
    &[[(0.5, 1.0), (1.0, 0.5)]],                           // 8
    &[[(1.0, 1.5), (1.0, 0.5)]],                           // 9
    &[[(0.5, 1.0), (1.0, 0.5)], [(1.5, 1.0), (1.0, 1.5)]], // 10
    &[[(1.5, 1.0), (1.0, 0.5)]],                           // 11
    &[[(0.5, 1.0), (1.5, 1.0)]],                           // 12
    &[[(1.0, 1.5), (1.5, 1.0)]],                           // 13
    &[[(0.5, 1.0), (1.0, 1.5)]],                           // 14
    &[],                                                   // 15
];

/// Get the isoline thresholds relative to a minimum, maximum, and step size
///
/// ## Parameters
/// - `min`: The minimum elevation
/// - `max`: The maximum elevation
/// - `step`: The step size
///
/// ## Returns
/// An array of thresholds within the range provided
pub fn isoline_thresholds(min: f64, max: f64, step: f64) -> Vec<f64> {
    let mut thresholds: Vec<f64> = vec![];
    let mut current = ceil(min / step) * step;

    while current <= max {
        thresholds.push(current);
        current += step;
    }

    thresholds
}

/// # Create Isolines or Isobands
///
/// ## Description
/// Creates isolines or isobands from an RGB(A) elevation image.
///
/// NOTE: Defaults to the Mapbox elevation data converter [`crate::readers::convert_mapbox_elevation_data`]. However,
/// to use the Terrarium elevation data converter, use [`crate::readers::convert_terrarium_elevation_data`].
///
/// NOTE: Using a buffer with a small padding works, but it only extends the line ends of the isolines.
/// A better method would be to pull in grid data with large padding.
///
/// ## Example
/// ```rust
/// use std::{fs, path::PathBuf};
/// use gistools::{
///     parsers::Buffer,
///     tools::build_contours,
/// };
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push(format!("tests/tools/elevation/fixtures/13_1556_3084.webp"));
/// let elevation_image = fs::read(path).unwrap();
/// let image_buffer = Buffer::from(elevation_image);
///
/// let isolines =
///        build_contours(&image_buffer, None, None, None, None, None);
/// ```
///
/// ## Parameters
/// - `image_data`: image data
/// - `elevation_converter`: the conversion function to convert the pixels to elevation
/// - `step`: the step size for the heightmap. Defaults to 100 meters for the Mapbox elevation data.
/// - `tms_style`: if true, the y position will be inverted
/// - `padding`: The number of pixels that extend around the main data
/// - `tolerance`: The Douglas-Peucker tolerance. Defaults to `1 / 2_096`
///
/// ## Returns
/// The isolines stored in a FeatureCollection
pub fn build_contours(
    image_data: &Buffer,
    elevation_converter: Option<ElevationConverter>,
    step: Option<f64>,
    tms_style: Option<bool>,
    padding: Option<usize>,
    tolerance: Option<f64>,
) -> FeatureCollection {
    let step = step.unwrap_or(100.0);
    let padding = padding.unwrap_or(1);
    let tolerance = tolerance.unwrap_or(1.0 / 2_096.0);
    // create the elevation grid
    let elevation_grid = get_elevation_grid(image_data, elevation_converter, tms_style);
    let IsoGrid { width, height, min, max, elevations } = elevation_grid;
    // setup the thresholds
    let thresholds = isoline_thresholds(min, max, step);

    let mut features: Vec<VectorFeature> = vec![];
    for threshold in thresholds {
        let coordinates =
            build_isobands(&elevations, threshold, width, height, padding, Some(tolerance));
        features.push(VectorFeature::new_wm(
            None,
            (ContourProperties { elev: threshold, elev_ft: threshold * 3.28084 }).into(),
            VectorGeometry::new_multipolygon(coordinates, None),
            None,
        ));
    }

    let mut feature_collection = FeatureCollection::new(None);
    feature_collection.features = features.into_iter().map(Features::VectorFeature).collect();
    feature_collection
}

/// Accumulate, smooth contour rings, assign holes to exterior rings.
/// Based on https://github.com/mbostock/shapefile/blob/v0.6.2/shp/polygon.js
/// @param tolerance - The Douglas-Peucker tolerance. A good default is `1 / 2_096`
pub fn build_isobands(
    values: &[f64],
    threshold: f64,
    width: usize,
    height: usize,
    padding: usize,
    tolerance: Option<f64>,
) -> VectorMultiPolygon {
    let mut polygons: VectorMultiPolygon = vec![];
    let mut holes: VectorMultiLineString = vec![];

    let isorings = build_isorings(values, threshold, width, height, padding, tolerance);
    // Store rings in the correct groups. Skip rings with zero area.
    for ring in isorings {
        let area = ring.area(Some(1.));
        if area == 0. {
            continue;
        } else if area > 0. {
            polygons.push(vec![ring]);
        } else {
            holes.push(ring);
        }
    }
    // sort the holes into their correct polygons
    for hole in holes {
        for poly in &mut polygons {
            if polyline_in_polyline(&hole, &poly[0]) {
                poly.push(hole);
                break;
            }
        }
    }

    polygons
}

/// Marching squares with isolines stitched into rings.
/// Based on https://github.com/topojson/topojson-client/blob/v3.0.0/src/stitch.js
/// @param tolerance - The Douglas-Peucker tolerance. A good default is `1 / 2_096`
pub fn build_isorings(
    values: &[f64],
    threshold: f64,
    width: usize,
    height: usize,
    padding: usize,
    tolerance: Option<f64>,
) -> VectorMultiLineString {
    let tolerance = tolerance.unwrap_or(0.);
    let mut res: VectorMultiLineString = vec![];
    let iwidth = width as isize;
    let mut frag_by_start = BTreeMap::<usize, Rc<RefCell<Fragment>>>::new();
    let mut frag_by_end = BTreeMap::<usize, Rc<RefCell<Fragment>>>::new();
    let mut t0: usize;
    let mut t1: usize;
    let mut t2: usize;
    let mut t3: usize;

    // Special case for the first row (y = -1, t2 = t3 = 0).
    let mut x: isize = -1;
    let mut y: isize = -1;
    t1 = above(values.first().cloned(), threshold);
    MS_LUT[t1 << 1].iter().for_each(|p| {
        stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
    });
    while {
        x += 1;
        x < iwidth - 1
    } {
        t0 = t1;
        t1 = above(values.get((x + 1) as usize).cloned(), threshold);
        MS_LUT[t0 | (t1 << 1)].iter().for_each(|p| {
            stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
        });
    }
    MS_LUT[t1].iter().for_each(|p| {
        stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
    });

    // General case for the intermediate rows.
    while {
        y += 1;
        y < height as isize - 1
    } {
        x = -1;
        t1 = above(values.get((y * iwidth + iwidth) as usize).cloned(), threshold);
        t2 = above(values.get((y * iwidth) as usize).cloned(), threshold);
        MS_LUT[(t1 << 1) | (t2 << 2)].iter().for_each(|p| {
            stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
        });
        while {
            x += 1;
            x < iwidth - 1
        } {
            t0 = t1;
            t1 = above(values.get((y * iwidth + iwidth + x + 1) as usize).cloned(), threshold);
            t3 = t2;
            t2 = above(values.get((y * iwidth + x + 1) as usize).cloned(), threshold);
            MS_LUT[t0 | (t1 << 1) | (t2 << 2) | (t3 << 3)].iter().for_each(|p| {
                stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
            });
        }
        MS_LUT[t1 | (t2 << 3)].iter().for_each(|p| {
            stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
        })
    }

    // Special case for the last row (y = height - 1, t0 = t1 = 0).
    x = -1;
    t2 = (values[(y * iwidth) as usize] >= threshold).into();
    MS_LUT[t2 << 2].iter().for_each(|p| {
        stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
    });
    while {
        x += 1;
        x < iwidth - 1
    } {
        t3 = t2;
        t2 = above(values.get((y * iwidth + x + 1) as usize).cloned(), threshold);
        MS_LUT[(t2 << 2) | (t3 << 3)].iter().for_each(|p| {
            stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
        })
    }
    MS_LUT[t2 << 3].iter().for_each(|p| {
        stitch(p, width, x, y, &mut frag_by_start, &mut frag_by_end, &mut res);
    });

    for ring in &mut res {
        // reverse the ring
        ring.reverse();
        // smooth the edges
        smooth(ring, values, threshold, width as f64, height as f64);
        // convert points to 0->1
        for point in &mut *ring {
            remap(point, width as f64, height as f64, padding as f64);
        }
        // lastly simplify if tolerance is not 0
        if tolerance != 0. {
            // Prep Douglas-Peucker simplification by setting t-values.
            build_sq_dist(ring, 0, ring.len() - 1, tolerance * tolerance);
            // Apply Douglas-Peucker
            *ring = simplify_line(ring, tolerance, false, false);
        }
    }

    res
}

fn stitch(
    line: &[(f64, f64); 2],
    width: usize,
    cx: isize,
    cy: isize,
    start_map: &mut BTreeMap<usize, Rc<RefCell<Fragment>>>,
    end_map: &mut BTreeMap<usize, Rc<RefCell<Fragment>>>,
    results: &mut VectorMultiLineString,
) {
    let start = VectorPoint::from_xy(line[0].0 + cx as f64, line[0].1 + cy as f64);
    let end = VectorPoint::from_xy(line[1].0 + cx as f64, line[1].1 + cy as f64);
    let start_index = index(&start, width as f64);
    let end_index = index(&end, width as f64);

    let f_opt = end_map.get(&start_index).cloned();
    let g_opt = start_map.get(&end_index).cloned();

    if let Some(f) = f_opt {
        if let Some(g) = g_opt {
            end_map.remove(&f.borrow().end);
            start_map.remove(&g.borrow().start);

            if Rc::ptr_eq(&f, &g) {
                // Fragment closes a ring completely
                f.borrow_mut().ring.push(end);
                let finalized_ring = Rc::try_unwrap(f)
                    .unwrap_or_else(|rc| rc.borrow().clone().into())
                    .into_inner()
                    .ring;
                results.push(finalized_ring);
            } else {
                // Merge fragment f and fragment g
                let mut f_borrow = f.borrow_mut();
                let mut g_borrow = g.borrow_mut();

                f_borrow.ring.append(&mut g_borrow.ring);
                f_borrow.end = g_borrow.end;

                let merged_frag =
                    Fragment::new(f_borrow.start, f_borrow.end, f_borrow.ring.clone());
                start_map.insert(f_borrow.start, merged_frag.clone());
                end_map.insert(f_borrow.end, merged_frag);
            }
        } else {
            // Append line segment to existing end map fragment
            end_map.remove(&f.borrow().end);
            f.borrow_mut().ring.push(end);
            f.borrow_mut().end = end_index;
            end_map.insert(end_index, f);
        }
    } else if let Some(f) = start_map.get(&end_index).cloned() {
        if let Some(g) = end_map.get(&start_index).cloned() {
            start_map.remove(&f.borrow().start);
            end_map.remove(&g.borrow().end);

            if Rc::ptr_eq(&f, &g) {
                f.borrow_mut().ring.push(end);
                let finalized_ring = Rc::try_unwrap(f)
                    .unwrap_or_else(|rc| rc.borrow().clone().into())
                    .into_inner()
                    .ring;
                results.push(finalized_ring);
            } else {
                let mut g_borrow = g.borrow_mut();
                let mut f_borrow = f.borrow_mut();

                g_borrow.ring.append(&mut f_borrow.ring);
                g_borrow.end = f_borrow.end;

                let merged_frag =
                    Fragment::new(g_borrow.start, g_borrow.end, g_borrow.ring.clone());
                start_map.insert(g_borrow.start, merged_frag.clone());
                end_map.insert(g_borrow.end, merged_frag);
            }
        } else {
            // Prepend segment to existing start map fragment
            start_map.remove(&f.borrow().start);
            f.borrow_mut().ring.insert(0, start);
            f.borrow_mut().start = start_index;
            start_map.insert(start_index, f);
        }
    } else {
        // Create an entirely new fragment tracking this segment
        let frag = Fragment::new(start_index, end_index, vec![start, end]);
        start_map.insert(start_index, frag.clone());
        end_map.insert(end_index, frag);
    }
}

// Convert a point to an index.
fn index<P: GetXY>(point: &P, width: f64) -> usize {
    (point.x() * 2. + point.y() * (width + 1.) * 4.) as usize
}

// Convert to a number (0 or 1) whether the value is above the threshold.
fn above(x: Option<f64>, value: f64) -> usize {
    x.map(|x| (x >= value) as usize).unwrap_or(0)
}

// Smooth a contour ring.
fn smooth<P: GetXY + SetXY>(
    ring: &mut [P],
    values: &[f64],
    threshold: f64,
    width: f64,
    height: f64,
) {
    ring.iter_mut().for_each(|point| {
        let (x, y) = point.xy();
        let xt = trunc(x);
        let yt = trunc(y);
        let v1 = valid(values.get((yt * width + xt) as usize).cloned());
        if x > 0. && x < width && xt == x {
            point.set_x(smooth1(
                x,
                valid(values.get((yt * width + xt - 1.) as usize).cloned()),
                v1,
                threshold,
            ));
        }
        if y > 0. && y < height && yt == y {
            point.set_y(smooth1(
                y,
                valid(values.get(((yt - 1.) * width + xt) as usize).cloned()),
                v1,
                threshold,
            ));
        }
    });
}

fn valid(v: Option<f64>) -> f64 {
    v.filter(|n| !n.is_nan()).unwrap_or(f64::NEG_INFINITY)
}

fn smooth1(x: f64, v0: f64, v1: f64, value: f64) -> f64 {
    let a = value - v0;
    let b = v1 - v0;
    let d = if a.is_finite() || b.is_finite() { a / b } else { a.signum() / b.signum() };
    if d.is_nan() { x } else { x + d - 0.5 }
}

// Convert the x-y values from width-height scale to 0->1.
// Input data was already offest to represent cell centers ([0.5, 0.5] start)
fn remap<P: GetXY + SetXY>(point: &mut P, width: f64, height: f64, padding: f64) {
    let active_width = width - 2. * padding;
    let active_height = height - 2. * padding;
    point.set_x((point.x() - padding) / active_width);
    point.set_y((point.y() - padding) / active_height);
}
