use crate::geometry::{VectorGeometry, VectorLineString, VectorPoint};

use libm::pow;

use alloc::vec;

/// Functions to simplify a vector geometry
pub trait SimplifyVectorGeometry {
    /// Build sequential distances for a vector geometry
    fn build_sq_dists(&mut self, tolerance: f64, maxzoom: Option<u8>);
    /// Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
    fn simplify(&mut self, tolerance: f64, zoom: u8, maxzoom: Option<u8>);
}

impl SimplifyVectorGeometry for VectorGeometry {
    /// Build sqdistances for a vector geometry
    fn build_sq_dists(&mut self, tolerance: f64, maxzoom: Option<u8>) {
        build_sq_dists(self, tolerance, maxzoom);
    }

    /// Simplify the geometry to have a tolerance which will be relative to the tile's zoom level.
    fn simplify(&mut self, tolerance: f64, zoom: u8, maxzoom: Option<u8>) {
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
    let zoom_tol =
        if zoom >= maxzoom { 0. } else { tolerance / ((1 << (zoom as u64)) as f64 * 4_096.) };
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

#[cfg(test)]
mod tests {
    use s2json::{
        VectorLineStringGeometry, VectorMultiLineStringGeometry, VectorMultiPolygonGeometry,
        VectorPolygonGeometry,
    };

    use super::*;

    const SIMPLIFY_MAXZOOM: u8 = 16;

    #[test]
    fn test_rewind() {
        let mut ring = vec![
            VectorPoint::new(0., 0., None, None),
            VectorPoint::new(0., 1., None, None),
            VectorPoint::new(1., 1., None, None),
            VectorPoint::new(1., 0., None, None),
        ];

        rewind(&mut ring, false);

        assert_eq!(
            ring,
            vec![
                VectorPoint::new(1., 0., None, None),
                VectorPoint::new(0., 1., None, None),
                VectorPoint::new(1., 1., None, None),
                VectorPoint::new(0., 0., None, None),
            ]
        );
    }

    #[test]
    fn test_line_string() {
        let mut line_string_geo = VectorGeometry::LineString(VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![
                VectorPoint::new(0.25, 0.25, None, None),
                VectorPoint::new(0.75, 0.25, None, None),
                VectorPoint::new(0.75, 0.75, None, None),
                VectorPoint::new(0.25, 0.75, None, None),
            ],
            offset: None,
            bbox: None,
            vec_bbox: None,
            indices: None,
            tesselation: None,
        });
        line_string_geo.build_sq_dists(3., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::LineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                    VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::LineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                    VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                    VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_multi_line_string() {
        let mut line_string_geo = VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            coordinates: vec![
                vec![
                    VectorPoint::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiLineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiLineString(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_polygon() {
        let mut line_string_geo = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![
                vec![
                    VectorPoint::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::Polygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::Polygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }

    #[test]
    fn test_multi_polygon() {
        let mut line_string_geo = VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![
                vec![
                    VectorPoint::new(0.25, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.75, None, None),
                    VectorPoint::new(0.25, 0.75, None, None),
                ],
                vec![
                    VectorPoint::new(0.5, 0.5, None, None),
                    VectorPoint::new(0.5, 0.25, None, None),
                    VectorPoint::new(0.75, 0.25, None, None),
                    VectorPoint::new(0.75, 0.5, None, None),
                    VectorPoint::new(0.5, 0.5, None, None),
                ],
            ]],
            ..Default::default()
        });
        line_string_geo.build_sq_dists(3., Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiPolygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.75, t: Some(0.25), z: None, m: None },
                        VectorPoint { x: 0.25, y: 0.75, t: Some(1.), z: None, m: None },
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.25, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.25, t: Some(0.125), z: None, m: None },
                        VectorPoint { x: 0.75, y: 0.5, t: Some(0.03125), z: None, m: None },
                        VectorPoint { x: 0.5, y: 0.5, t: Some(1.), z: None, m: None },
                    ]
                ]]
            );
        } else {
            panic!("Expected LineString geometry");
        }

        // simplify
        line_string_geo.simplify(3., 0, Some(SIMPLIFY_MAXZOOM));

        if let VectorGeometry::MultiPolygon(ref mut line) = line_string_geo {
            assert_eq!(
                line.coordinates,
                vec![vec![
                    vec![
                        VectorPoint { x: 0.25, y: 0.25, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.75, z: None, m: None, t: Some(0.25) },
                        VectorPoint { x: 0.25, y: 0.75, z: None, m: None, t: Some(1.0) }
                    ],
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.25, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.75, y: 0.25, z: None, m: None, t: Some(0.125) },
                        VectorPoint { x: 0.75, y: 0.5, z: None, m: None, t: Some(0.03125) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: Some(1.0) }
                    ]
                ]]
            );
        } else {
            panic!("Expected LineString geometry");
        }
    }
}
