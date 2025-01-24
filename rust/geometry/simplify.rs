use crate::{VectorGeometry, VectorLineString, VectorPoint};

use libm::pow;

use alloc::vec;

impl VectorGeometry {
    /// Build sqdistances for a vector geometry
    pub fn build_sq_dists(&mut self, tolerance: f64, maxzoom: Option<u8>) {
        build_sq_dists(self, tolerance, maxzoom);
    }

    /// Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
    pub fn simplify(&mut self, tolerance: f64, zoom: u8, maxzoom: Option<u8>) {
        simplify(self, tolerance, zoom, maxzoom);
    }
}

/// build sqdistances for line vector data
pub fn build_sq_dists(geometry: &mut VectorGeometry, tolerance: f64, maxzoom: Option<u8>) {
    let maxzoom = maxzoom.unwrap_or(16);
    let tol = pow(tolerance / ((1 << maxzoom) as f64 * 4_096.), 2.);

    match geometry {
        VectorGeometry::LineString(geo) => {
            let coords = &mut geo.coordinates;
            build_sq_dist(coords, 0, coords.len() - 1, tol);
        }
        VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
            let coords = &mut geo.coordinates;
            coords.iter_mut().for_each(|line| build_sq_dist(line, 0, line.len() - 1, tol));
        }
        VectorGeometry::MultiPolygon(geo) => {
            let coords = &mut geo.coordinates;
            coords.iter_mut().for_each(|polygon| {
                polygon.iter_mut().for_each(|line| build_sq_dist(line, 0, line.len() - 1, tol))
            });
        }
        _ => {}
    }
}

/// calculate simplification of line vector data using
/// optimized Douglas-Peucker algorithm
fn build_sq_dist(coords: &mut VectorLineString, first: usize, last: usize, sq_tolerance: f64) {
    coords[first].t = Some(1.);
    _build_sq_dist(coords, first, last, sq_tolerance);
    coords[last].t = Some(1.);
}

/// calculate simplification of line vector data using
/// optimized Douglas-Peucker algorithm
fn _build_sq_dist(coords: &mut VectorLineString, first: usize, last: usize, sq_tolerance: f64) {
    let mid = (last - first) >> 1;
    let mut max_sq_dist = sq_tolerance;
    let mut min_pos_to_mid = last - first;
    let mut index: Option<usize> = None;

    let VectorPoint { x: ax, y: ay, .. } = coords[first];
    let VectorPoint { x: bx, y: by, .. } = coords[last];

    let mut i = first;
    while i < last {
        let VectorPoint { x, y, .. } = coords[i];
        let d = get_sq_seg_dist(x, y, ax, ay, bx, by);

        if d > max_sq_dist {
            index = Some(i);
            max_sq_dist = d;
        } else if d == max_sq_dist {
            // a workaround to ensure we choose a pivot close to the middle of the list,
            // reducing recursion depth, for certain degenerate inputs
            let pos_to_mid = isize::abs(i as isize - mid as isize) as usize;
            if pos_to_mid < min_pos_to_mid {
                index = Some(i);
                min_pos_to_mid = pos_to_mid;
            }
        }

        i += 1;
    }

    if max_sq_dist > sq_tolerance {
        if let Some(index) = index {
            if index - first > 1 {
                _build_sq_dist(coords, first, index, sq_tolerance);
            }
            coords[index].t = Some(max_sq_dist);
            if last - index > 1 {
                _build_sq_dist(coords, index, last, sq_tolerance);
            }
        }
    }
}

/// square distance from a point to a segment
fn get_sq_seg_dist(ps: f64, pt: f64, s: f64, t: f64, bs: f64, bt: f64) -> f64 {
    let mut s = s;
    let mut t = t;
    let mut ds = bs - s;
    let mut dt = bt - t;

    if ds != 0. || dt != 0. {
        let m = ((ps - s) * ds + (pt - t) * dt) / (ds * ds + dt * dt);

        if m > 1. {
            s = bs;
            t = bt;
        } else if m > 0. {
            s += ds * m;
            t += dt * m;
        }
    }

    ds = ps - s;
    dt = pt - t;

    ds * ds + dt * dt
}

/// Simplify a vector geometry
pub fn simplify(geometry: &mut VectorGeometry, tolerance: f64, zoom: u8, maxzoom: Option<u8>) {
    let maxzoom = maxzoom.unwrap_or(16);
    let zoom_tol = if zoom >= maxzoom { 0. } else { tolerance / ((1 << zoom) as f64 * 4_096.) };
    match geometry {
        VectorGeometry::LineString(geo) => {
            geo.coordinates = simplify_line(&geo.coordinates, zoom_tol, false, false);
        }
        VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
            geo.coordinates = geo
                .coordinates
                .iter()
                .map(|line| simplify_line(line, zoom_tol, false, false))
                .collect();
        }
        VectorGeometry::MultiPolygon(geo) => {
            geo.coordinates = geo
                .coordinates
                .iter()
                .map(|polygon| {
                    polygon.iter().map(|line| simplify_line(line, zoom_tol, true, true)).collect()
                })
                .collect();
        }
        _ => (),
    }
}

/// simplified a vector line
fn simplify_line(
    line: &VectorLineString,
    tolerance: f64,
    is_polygon: bool,
    is_outer: bool,
) -> VectorLineString {
    let sq_tolerance = tolerance * tolerance;
    let size = line.len() as f64;
    if tolerance > 0. && size < (if is_polygon { sq_tolerance } else { tolerance }) {
        return line.clone();
    }

    let mut ring: VectorLineString = vec![];
    for point in line {
        if tolerance == 0. || (point.t.unwrap_or(0.0)) > sq_tolerance {
            ring.push(point.clone());
        }
    }
    if is_polygon {
        rewind(&mut ring, is_outer);
    }

    ring
}

/// In place adjust the ring if necessary
pub fn rewind(ring: &mut VectorLineString, clockwise: bool) {
    let len = ring.len();
    let mut area: f64 = 0.;
    let mut i = 0;
    let mut j = len - 2;
    while i < len {
        area += (ring[i].x - ring[j].x) * (ring[i].y + ring[j].y);
        j = i;
        i += 2;
    }
    i = 0;
    if (area > 0.) == clockwise {
        let len_half = len / 2;
        while i < len_half {
            ring.swap(i, len - i - 1);
            i += 2;
        }
    }
}
