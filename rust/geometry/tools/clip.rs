use crate::{
    data_structures::{HasLayer, Tile},
    geometry::S2CellId,
};
use alloc::{string::ToString, vec, vec::Vec};
use s2json::{
    Axis, BBox3D, MValue, Properties, VectorFeature, VectorGeometry, VectorGeometryType,
    VectorLineString, VectorLineStringGeometry, VectorMultiLineOffset, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorMultiPolygonOffset, VectorPoint, VectorPointGeometry,
    VectorPolygonGeometry,
};

// TODO: Cases of `to_vec` clones large swathes of data. Can we optimize this?

/// The children of a tile
#[derive(Debug)]
pub struct TileChildren<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// The bottom left child tile
    pub bottom_left: Tile<M, P, D>,
    /// The bottom right child tile
    pub bottom_right: Tile<M, P, D>,
    /// The top left child tile
    pub top_left: Tile<M, P, D>,
    /// The top right child tile
    pub top_right: Tile<M, P, D>,
}

impl<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default> Tile<M, P, D> {
    /// split tile into 4 children
    pub fn split(&mut self, buffer: Option<f64>) -> TileChildren<M, P, D> {
        split_tile(self, buffer)
    }
}

/// @param tile - the tile to split
/// @param buffer - the buffer around the tile for lines and polygons
/// @returns - the tile's children split into 4 sub-tiles
pub fn split_tile<M: HasLayer + Clone, P: Clone + Default, D: Clone + Default>(
    tile: &mut Tile<M, P, D>,
    buffer: Option<f64>,
) -> TileChildren<M, P, D> {
    let buffer = buffer.unwrap_or(0.0625);
    let face = tile.id.face();
    let (zoom, i, j) = tile.id.to_zoom_ij(None);
    let [bl_id, br_id, tl_id, tr_id] = S2CellId::children_ij(face, zoom, i, j);
    let mut children = TileChildren {
        bottom_left: Tile::new(bl_id),
        bottom_right: Tile::new(br_id),
        top_left: Tile::new(tl_id),
        top_right: Tile::new(tr_id),
    };
    let scale = (1 << zoom) as f64;
    let k1 = 0.;
    let k2 = 0.5;
    let k3 = 0.5;
    let k4 = 1.;
    let i = i as f64;
    let j = j as f64;

    let mut tl: Option<Vec<VectorFeature<M, P, D>>>;
    let mut bl: Option<Vec<VectorFeature<M, P, D>>>;
    let mut tr: Option<Vec<VectorFeature<M, P, D>>>;
    let mut br: Option<Vec<VectorFeature<M, P, D>>>;

    for (name, layer) in tile.layers.iter_mut() {
        let features = &layer.features;
        let left = _clip(features, scale, i - k1, i + k3, Axis::X, buffer);
        let right = _clip(features, scale, i + k2, i + k4, Axis::X, buffer);

        if let Some(left) = left {
            bl = _clip(&left, scale, j - k1, j + k3, Axis::Y, buffer);
            tl = _clip(&left, scale, j + k2, j + k4, Axis::Y, buffer);
            if let Some(bl) = bl {
                for d in bl {
                    children.bottom_left.add_feature(d, Some(name.to_string()));
                }
            }
            if let Some(tl) = tl {
                for d in tl {
                    children.top_left.add_feature(d, Some(name.to_string()));
                }
            }
        }

        if let Some(right) = right {
            br = _clip(&right, scale, j - k1, j + k3, Axis::Y, buffer);
            tr = _clip(&right, scale, j + k2, j + k4, Axis::Y, buffer);
            if let Some(br) = br {
                for d in br {
                    children.bottom_right.add_feature(d, Some(name.to_string()));
                }
            }
            if let Some(tr) = tr {
                for d in tr {
                    children.top_right.add_feature(d, Some(name.to_string()));
                }
            }
        }
    }

    children
}

/// Internal clip function for a collection of VectorFeatures
fn _clip<M, P: Clone + Default, D: Clone + Default>(
    features: &[VectorFeature<M, P, D>],
    scale: f64,
    k1: f64,
    k2: f64,
    axis: Axis,
    base_buffer: f64,
) -> Option<Vec<VectorFeature<M, P, D>>>
where
    M: Clone,
{
    // scale
    let k1 = k1 / scale;
    let k2 = k2 / scale;
    // prep buffer and result container
    let buffer = base_buffer / scale;
    let k1b = k1 - buffer;
    let k2b = k2 + buffer;
    let mut clipped: Vec<VectorFeature<M, P, D>> = vec![];
    let axis_x = axis == Axis::X;

    for feature in features {
        let geometry = &feature.geometry;
        // trivial accept and reject cases
        if let Some(vec_bbox) = geometry.vec_bbox() {
            let min = if axis_x { vec_bbox.left } else { vec_bbox.bottom };
            let max = if axis_x { vec_bbox.right } else { vec_bbox.top };
            if min >= k1 && max < k2 {
                clipped.push(feature.clone());
                continue;
            } else if max < k1 || min >= k2 {
                continue;
            }
        }
        // build the new clipped geometry
        let new_geometry: Option<VectorGeometry<D>> = match geometry {
            VectorGeometry::Point(geo) => clip_point(geo, axis, k1, k2),
            VectorGeometry::MultiPoint(geo) => clip_multi_point(geo, axis, k1, k2),
            VectorGeometry::LineString(geo) => clip_line_string(geo, axis, k1b, k2b),
            VectorGeometry::MultiLineString(geo) => {
                clip_multi_line_string(geo, axis, k1b, k2b, false)
            }
            VectorGeometry::Polygon(geo) => clip_polygon(geo, axis, k1b, k2b),
            VectorGeometry::MultiPolygon(geo) => clip_multi_polygon(geo, axis, k1b, k2b),
        };
        // store if the geometry was inside the range
        if let Some(new_geometry) = new_geometry {
            clipped.push(VectorFeature::from_vector_feature(feature, Some(new_geometry)));
        }
    }

    if clipped.is_empty() { None } else { Some(clipped) }
}

/// Clip a point to an axis and range
fn clip_point<M: Clone + Default>(
    geometry: &VectorPointGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    let coords = &geometry.coordinates;
    let value = if axis == Axis::X { coords.x } else { coords.y };
    if value >= k1 && value < k2 { Some(VectorGeometry::Point(geometry.clone())) } else { None }
}

/// Clip a MultiPoint to an axis and range
fn clip_multi_point<M: Clone + Default>(
    geometry: &VectorMultiPointGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    let mut new_geo = geometry.clone();
    new_geo.coordinates = geometry
        .coordinates
        .iter()
        .filter(|point| {
            let value = if axis == Axis::X { point.x } else { point.y };
            value >= k1 && value < k2
        })
        .cloned()
        .collect();

    if new_geo.coordinates.is_empty() { None } else { Some(VectorGeometry::MultiPoint(new_geo)) }
}

/// Clip a LineString to an axis and range
fn clip_line_string<M: Clone + Default>(
    geometry: &VectorLineStringGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    let VectorLineStringGeometry { is_3d, coordinates: line, bbox, vec_bbox, .. } = geometry;
    let init_o = geometry.offset.unwrap_or(0.);
    let mut new_offsets: VectorMultiLineOffset = vec![];
    let mut new_lines: VectorMultiLineString<M> = vec![];
    for clip in
        _clip_line(ClipLineResult { line: line.to_vec(), offset: init_o }, k1, k2, axis, false)
    {
        new_offsets.push(clip.offset);
        new_lines.push(clip.line);
    }
    if new_lines.is_empty() {
        None
    } else {
        Some(VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: VectorGeometryType::MultiLineString,
            is_3d: *is_3d,
            coordinates: new_lines,
            bbox: *bbox,
            offset: Some(new_offsets),
            vec_bbox: Some(vec_bbox.unwrap_or_default().clip(axis, k1, k2)),
            ..Default::default()
        }))
    }
}

/// Clip a MultiLineString geometry to an axis and range
fn clip_multi_line_string<M: Clone + Default>(
    geometry: &VectorMultiLineStringGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
    is_polygon: bool,
) -> Option<VectorGeometry<M>> {
    let VectorMultiLineStringGeometry::<M> { is_3d, coordinates, bbox, vec_bbox, .. } = geometry;
    let init_o =
        geometry.offset.clone().unwrap_or_else(|| coordinates.iter().map(|_| 0.).collect());
    let mut new_offsets: VectorMultiLineOffset = vec![];
    let mut new_lines: VectorMultiLineString<M> = vec![];
    let vec_bbox = vec_bbox.unwrap_or_default().clip(axis, k1, k2);
    coordinates.iter().enumerate().for_each(|(i, line)| {
        for clip in _clip_line(
            ClipLineResult { line: line.to_vec(), offset: init_o[i] },
            k1,
            k2,
            axis,
            is_polygon,
        ) {
            new_offsets.push(clip.offset);
            new_lines.push(clip.line);
        }
    });
    if new_lines.is_empty() || (is_polygon && new_lines[0].len() < 4) {
        None
    } else if !is_polygon {
        Some(VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: VectorGeometryType::MultiLineString,
            is_3d: *is_3d,
            coordinates: new_lines,
            bbox: *bbox,
            offset: Some(new_offsets),
            vec_bbox: Some(vec_bbox),
            ..Default::default()
        }))
    } else {
        Some(VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            is_3d: *is_3d,
            coordinates: new_lines,
            bbox: *bbox,
            offset: Some(new_offsets),
            vec_bbox: Some(vec_bbox),
            ..Default::default()
        }))
    }
}

/// Clip a Polygon geometry to an axis and range
fn clip_polygon<M: Clone + Default>(
    geometry: &VectorPolygonGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    clip_multi_line_string(geometry, axis, k1, k2, true)
}

/// Clip a MultiPolygon geometry to an axis and range
fn clip_multi_polygon<M: Clone + Default>(
    geometry: &VectorMultiPolygonGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    let VectorMultiPolygonGeometry { is_3d, coordinates, bbox, vec_bbox, .. } = geometry;
    let init_o = geometry
        .offset
        .clone()
        .unwrap_or_else(|| coordinates.iter().map(|l| l.iter().map(|_| 0.).collect()).collect());
    let mut new_coordinates: VectorMultiPolygon<M> = vec![];
    let mut new_offsets: VectorMultiPolygonOffset = vec![];
    coordinates.iter().enumerate().for_each(|(p, polygon)| {
        let new_polygon = clip_polygon(
            &VectorPolygonGeometry {
                _type: VectorGeometryType::Polygon,
                is_3d: *is_3d,
                coordinates: polygon.to_vec(),
                offset: Some(init_o[p].clone()),
                ..Default::default()
            },
            axis,
            k1,
            k2,
        );
        if let Some(VectorGeometry::Polygon(new_polygon)) = new_polygon {
            new_coordinates.push(new_polygon.coordinates);
            if let Some(new_offset) = new_polygon.offset {
                new_offsets.push(new_offset);
            }
        }
    });

    if new_coordinates.is_empty() {
        None
    } else {
        Some(VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            is_3d: *is_3d,
            coordinates: new_coordinates,
            bbox: *bbox,
            offset: Some(new_offsets),
            vec_bbox: Some(vec_bbox.unwrap_or_default().clip(axis, k1, k2)),
            ..Default::default()
        }))
    }
}

/// After clipping a line, return the altered line,
/// the offset the new line starts at,
/// and if the line is ccw
#[derive(Debug)]
pub struct ClipLineResult<M: Clone + Default> {
    /// The clipped line
    pub line: VectorLineString<M>,
    /// The offset the new line starts at
    pub offset: f64,
}
/// Ensuring `vec_bbox` exists
#[derive(Debug)]
pub struct ClipLineResultWithBBox<M: Clone + Default> {
    /// The clipped line
    pub line: VectorLineString<M>,
    /// The offset the new line starts at
    pub offset: f64,
    /// The new vector bounding box
    pub vec_bbox: BBox3D,
}

/// clip an input line to a bounding box
/// Data should always be in a 0->1 coordinate system to use this clip function
pub fn clip_line<M: Clone + Default>(
    geom: &VectorLineString<M>,
    bbox: BBox3D,
    is_polygon: bool,
    offset: Option<f64>,
    buffer: Option<f64>, /* default for a full size tile. Assuming 1024 extent and a 64 point buffer */
) -> Vec<ClipLineResultWithBBox<M>> {
    let offset = offset.unwrap_or(0.);
    let buffer = buffer.unwrap_or(0.0625);
    let mut res: Vec<ClipLineResult<M>> = vec![];
    let BBox3D { left, bottom, right, top, .. } = bbox;
    // clip horizontally
    let horizontal_clips = _clip_line(
        ClipLineResult { line: geom.clone(), offset },
        left - buffer,
        right + buffer,
        Axis::X,
        is_polygon,
    );
    for clip in horizontal_clips {
        // clip vertically
        let mut vertical_clips =
            _clip_line(clip, bottom - buffer, top + buffer, Axis::Y, is_polygon);
        res.append(&mut vertical_clips);
    }
    res.iter_mut()
        .map(|clip| {
            let mut vec_bbox: Option<BBox3D> = None;
            for p in clip.line.iter() {
                match &mut vec_bbox {
                    Some(bbox) => bbox.extend_from_point(p),
                    None => vec_bbox = Some(BBox3D::from_point(p)),
                }
            }
            ClipLineResultWithBBox {
                line: core::mem::take(&mut clip.line),
                offset: clip.offset,
                vec_bbox: vec_bbox.unwrap(),
            }
        })
        .collect()
}

/// Interal clip tool
fn _clip_line<M: Clone + Default>(
    input: ClipLineResult<M>,
    k1: f64,
    k2: f64,
    axis: Axis,
    is_polygon: bool,
) -> Vec<ClipLineResult<M>> {
    //   let { line: geom, offset: startOffset } = input;
    let geom = &input.line;
    let start_offset = input.offset;
    let mut new_geom: Vec<ClipLineResult<M>> = vec![];
    let mut slice: VectorLineString<M> = vec![];
    if geom.is_empty() {
        return new_geom;
    }
    let mut last = geom.len() - 1;
    let intersect = if axis == Axis::X { intersect_x } else { intersect_y };

    let mut cur_offset = start_offset;
    let mut acc_offset = start_offset;
    let mut prev_p = &geom[0];
    let mut first_enter = false;

    let mut i = 0;
    while i < last {
        let VectorPoint::<M> { x: ax, y: ay, t: az, m: am, .. } = &geom[i];
        let VectorPoint::<M> { x: bx, y: by, m: bm, .. } = &geom[i + 1];
        let a: f64 = if axis == Axis::X { *ax } else { *ay };
        let b: f64 = if axis == Axis::X { *bx } else { *by };
        let mut entered = false;
        let mut exited = false;
        let mut int_p: Option<VectorPoint<M>> = None;

        // ENTER OR CONTINUE CASES
        if a < k1 {
            // ---|-->  | (line enters the clip region from the left)
            if b > k1 {
                int_p = Some(intersect(*ax, *ay, *bx, *by, k1, bm));
                slice.push(int_p.clone().unwrap());
                entered = true;
            }
        } else if a > k2 {
            // |  <--|--- (line enters the clip region from the right)
            if b < k2 {
                int_p = Some(intersect(*ax, *ay, *bx, *by, k2, bm));
                slice.push(int_p.clone().unwrap());
                entered = true;
            }
        } else {
            int_p = Some(VectorPoint { x: *ax, y: *ay, z: None, m: am.clone(), t: *az });
            slice.push(int_p.clone().unwrap());
        }

        // Update the intersection point and offset if the int_p exists
        if let Some(int_p) = int_p.as_ref() {
            // our first enter will change the offset for the line
            if entered && !first_enter {
                cur_offset = acc_offset + prev_p.distance(int_p);
                first_enter = true;
            }
        }

        // EXIT CASES
        if b < k1 && a >= k1 {
            // <--|---  | or <--|-----|--- (line exits the clip region on the left)
            int_p = Some(intersect(*ax, *ay, *bx, *by, k1, if bm.is_some() { bm } else { am }));
            slice.push(int_p.unwrap());
            exited = true;
        }
        if b > k2 && a <= k2 {
            // |  ---|--> or ---|-----|--> (line exits the clip region on the right)
            int_p = Some(intersect(*ax, *ay, *bx, *by, k2, if bm.is_some() { bm } else { am }));
            slice.push(int_p.unwrap());
            exited = true;
        }

        // update the offset
        acc_offset += prev_p.distance(&geom[i + 1]);
        prev_p = &geom[i + 1];

        // If not a polygon, we can cut it into parts, otherwise we just keep tracking the edges
        if !is_polygon && exited {
            new_geom.push(ClipLineResult { line: slice, offset: cur_offset });
            slice = vec![];
            first_enter = false;
        }

        i += 1;
    }

    // add the last point if inside the clip
    let last_point = geom[last].clone();
    let a = if axis == Axis::X { last_point.x } else { last_point.y };
    if a >= k1 && a <= k2 {
        slice.push(last_point.clone());
    }

    // close the polygon if its endpoints are not the same after clipping
    if !slice.is_empty() && is_polygon {
        last = slice.len() - 1;
        let first_p = &slice[0];
        if last >= 1 && (slice[last].x != first_p.x || slice[last].y != first_p.y) {
            slice.push(first_p.clone());
        }
    }

    // add the final slice
    if !slice.is_empty() {
        new_geom.push(ClipLineResult { line: slice, offset: cur_offset });
    }

    new_geom
}

/// Find the intersection of two points on the X axis
fn intersect_x<M: Clone + Default>(
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    x: f64,
    m: &Option<M>,
) -> VectorPoint<M> {
    let t = (x - ax) / (bx - ax);
    VectorPoint::<M> { x, y: ay + (by - ay) * t, z: None, m: m.clone(), t: Some(1.) }
}

/// Find the intersection of two points on the Y axis
fn intersect_y<M: Clone + Default>(
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    y: f64,
    m: &Option<M>,
) -> VectorPoint<M> {
    let t = (y - ay) / (by - ay);
    VectorPoint::<M> { x: ax + (bx - ax) * t, y, z: None, m: m.clone(), t: Some(1.) }
}

#[cfg(test)]
#[allow(clippy::approx_constant)]
mod tests {
    use super::*;
    use s2json::VectorBaseGeometry;

    #[test]
    fn test_clip_point() {
        let geo = VectorPointGeometry {
            _type: "Point".into(),
            is_3d: false,
            coordinates: VectorPoint::from_xy(2.0, 2.0),
            ..Default::default()
        };
        let res = clip_point(&geo, Axis::X, 0., 1.);
        assert_eq!(res, None);

        let geo = VectorPointGeometry {
            _type: "Point".into(),
            is_3d: false,
            coordinates: VectorPoint::from_xy(0.5, 0.5),
            ..Default::default()
        };
        let res = clip_point(&geo, Axis::X, 0., 1.);
        assert_eq!(res, Some(VectorGeometry::new_point(VectorPoint::from_xy(0.5, 0.5), None)));
    }

    #[test]
    fn test_multi_point() {
        let geo = VectorMultiPointGeometry {
            _type: "MultiPoint".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(0.5, 0.5)],
            ..Default::default()
        };
        let res = clip_multi_point(&geo, Axis::X, 0., 1.);
        assert_eq!(
            res,
            Some(VectorGeometry::new_multipoint(vec![VectorPoint::from_xy(0.5, 0.5),], None))
        );
    }

    #[test]
    fn test_clip_line_string() {
        let geo = VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(0.5, 0.5)],
            ..Default::default()
        };
        let res = clip_line_string(&geo, Axis::X, 0., 1.);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                ]],
                offset: Some(vec![1.4142135623730951]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );

        let geo = VectorLineStringGeometry {
            _type: "LineString".into(),
            is_3d: false,
            coordinates: vec![VectorPoint::from_xy(-2.0, -2.0), VectorPoint::from_xy(-0.5, -0.5)],
            ..Default::default()
        };
        let res = clip_line_string(&geo, Axis::X, 0., 1.);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_line_string_all_outside() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(3.0, 3.0)], /* Line 1 (outside > 1) */
                vec![VectorPoint::from_xy(-1.0, -1.0), VectorPoint::from_xy(-2.0, -2.0)], /* Line 2 (outside < 0) */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(res, None); // Everything clipped away
    }

    #[test]
    fn test_clip_multi_line_string_all_inside() {
        let coords = vec![
            vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], // Line 1
            vec![VectorPoint::from_xy(0.3, 0.7), VectorPoint::from_xy(0.7, 0.3)], // Line 2
        ];
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: coords.clone(), // Clone for comparison
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                        VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint { x: 0.3, y: 0.7, z: None, m: None, t: None },
                        VectorPoint { x: 0.7, y: 0.3, z: None, m: None, t: None }
                    ]
                ],
                offset: Some(vec![0.0, 0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_mixed_inside_outside() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(2.0, 2.0), VectorPoint::from_xy(3.0, 3.0)], /* Line 1 (outside) */
                vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], /* Line 2 (inside) */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);

        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                    VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_crossing_boundaries() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![VectorPoint::from_xy(-1.0, -1.0), VectorPoint::from_xy(0.5, 0.5)], /* Line 1 crosses x=0 */
                vec![VectorPoint::from_xy(0.8, 0.8), VectorPoint::from_xy(2.0, 2.0)], /* Line 2 crosses x=1 */
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);

        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.0, y: 0.0, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None },
                        VectorPoint { x: 1.0, y: 1.0, z: None, m: None, t: Some(1.0) }
                    ]
                ],
                offset: Some(vec![1.4142135623730951, 0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_line_string_empty_input() {
        let geo = VectorMultiLineStringGeometry::<MValue> {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![], // Empty coordinate array
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_line_string_with_empty_line() {
        let geo = VectorMultiLineStringGeometry {
            _type: "MultiLineString".into(),
            is_3d: false,
            coordinates: vec![
                vec![], // Empty line string inside
                vec![VectorPoint::from_xy(0.2, 0.2), VectorPoint::from_xy(0.8, 0.8)], // Valid line
            ],
            ..Default::default()
        };
        let res = clip_multi_line_string(&geo, Axis::X, 0.0, 1.0, false);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiLineString(VectorBaseGeometry {
                _type: "MultiLineString".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.2, z: None, m: None, t: None },
                    VectorPoint { x: 0.8, y: 0.8, z: None, m: None, t: None }
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_all_inside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(0.1, 0.9),
                VectorPoint::from_xy(0.1, 0.1), // Close the polygon
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_partially_outside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(-0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(-0.1, 0.9),
                VectorPoint::from_xy(-0.1, 0.1), // Close the polygon
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.0, y: 0.9, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]],
                offset: Some(vec![0.1]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_all_outside() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(1.1, 0.1),
                VectorPoint::from_xy(1.9, 0.1),
                VectorPoint::from_xy(1.9, 0.9),
                VectorPoint::from_xy(1.1, 0.9),
                VectorPoint::from_xy(1.1, 0.1),
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_polygon_empty() {
        let geo = VectorPolygonGeometry::<MValue> {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_polygon_open_ring_gets_closed() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.9, 0.1),
                VectorPoint::from_xy(0.9, 0.9),
                VectorPoint::from_xy(0.1, 0.9),
            ]], // Not explicitly closed
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.9, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]],
                offset: Some(vec![0.0]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_polygon_resulting_in_degenerate_polygon() {
        let geo = VectorPolygonGeometry {
            _type: "Polygon".into(),
            is_3d: false,
            coordinates: vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.1, 0.1),
            ]],
            ..Default::default()
        };
        let res = clip_polygon(&geo, Axis::X, 0.2, 0.4);
        assert_eq!(
            res,
            Some(VectorGeometry::Polygon(VectorBaseGeometry {
                _type: "Polygon".into(),
                is_3d: false,
                coordinates: vec![vec![
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]],
                offset: Some(vec![0.1]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_all_inside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1
                    vec![
                        VectorPoint::from_xy(0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(0.1, 0.4),
                        VectorPoint::from_xy(0.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2
                    vec![
                        VectorPoint::from_xy(0.6, 0.6),
                        VectorPoint::from_xy(0.9, 0.6),
                        VectorPoint::from_xy(0.9, 0.9),
                        VectorPoint::from_xy(0.6, 0.9),
                        VectorPoint::from_xy(0.6, 0.6),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![
                    vec![vec![
                        VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.1, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None }
                    ]],
                    vec![vec![
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None }
                    ]]
                ],
                offset: Some(vec![vec![0.0], vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_mixed_inside_outside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1 (partially outside)
                    vec![
                        VectorPoint::from_xy(-0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(-0.1, 0.4),
                        VectorPoint::from_xy(-0.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2 (completely inside)
                    vec![
                        VectorPoint::from_xy(0.6, 0.6),
                        VectorPoint::from_xy(0.9, 0.6),
                        VectorPoint::from_xy(0.9, 0.9),
                        VectorPoint::from_xy(0.6, 0.9),
                        VectorPoint::from_xy(0.6, 0.6),
                    ],
                ],
                vec![
                    // Polygon 3 (completely outside)
                    vec![
                        VectorPoint::from_xy(1.1, 0.1),
                        VectorPoint::from_xy(1.4, 0.1),
                        VectorPoint::from_xy(1.4, 0.4),
                        VectorPoint::from_xy(1.1, 0.4),
                        VectorPoint::from_xy(1.1, 0.1),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![
                    vec![vec![
                        VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                        VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                        VectorPoint { x: 0.0, y: 0.4, z: None, m: None, t: Some(1.0) },
                        VectorPoint { x: 0.0, y: 0.1, z: None, m: None, t: Some(1.0) }
                    ]],
                    vec![vec![
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.6, z: None, m: None, t: None },
                        VectorPoint { x: 0.9, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.9, z: None, m: None, t: None },
                        VectorPoint { x: 0.6, y: 0.6, z: None, m: None, t: None }
                    ]]
                ],
                offset: Some(vec![vec![0.1], vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_all_outside() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![
                    // Polygon 1 (outside)
                    vec![
                        VectorPoint::from_xy(1.1, 0.1),
                        VectorPoint::from_xy(1.4, 0.1),
                        VectorPoint::from_xy(1.4, 0.4),
                        VectorPoint::from_xy(1.1, 0.4),
                        VectorPoint::from_xy(1.1, 0.1),
                    ],
                ],
                vec![
                    // Polygon 2 (also outside)
                    vec![
                        VectorPoint::from_xy(-0.9, -0.6),
                        VectorPoint::from_xy(-0.6, -0.6),
                        VectorPoint::from_xy(-0.6, -0.9),
                        VectorPoint::from_xy(-0.9, -0.9),
                        VectorPoint::from_xy(-0.9, -0.6),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_polygon_empty() {
        let geo = VectorMultiPolygonGeometry::<MValue> {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(res, None);
    }

    #[test]
    fn test_clip_multi_polygon_with_empty_polygon() {
        let geo = VectorMultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![
                vec![], // Empty polygon
                vec![
                    // Valid polygon
                    vec![
                        VectorPoint::from_xy(0.1, 0.1),
                        VectorPoint::from_xy(0.4, 0.1),
                        VectorPoint::from_xy(0.4, 0.4),
                        VectorPoint::from_xy(0.1, 0.4),
                        VectorPoint::from_xy(0.1, 0.1),
                    ],
                ],
            ],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.0, 1.0);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![vec![vec![
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: None },
                    VectorPoint { x: 0.4, y: 0.4, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.4, z: None, m: None, t: None },
                    VectorPoint { x: 0.1, y: 0.1, z: None, m: None, t: None },
                ]]],
                offset: Some(vec![vec![0.0]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        );
    }

    #[test]
    fn test_clip_multi_polygon_resulting_in_degenerate_polygon() {
        let geo = VectorMultiPolygonGeometry::<MValue> {
            _type: "MultiPolygon".into(),
            is_3d: false,
            coordinates: vec![vec![vec![
                VectorPoint::from_xy(0.1, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.5, 0.1),
                VectorPoint::from_xy(0.1, 0.1),
            ]]],
            ..Default::default()
        };
        let res = clip_multi_polygon(&geo, Axis::X, 0.2, 0.4);
        assert_eq!(
            res,
            Some(VectorGeometry::MultiPolygon(VectorBaseGeometry {
                _type: "MultiPolygon".into(),
                is_3d: false,
                coordinates: vec![vec![vec![
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.4, y: 0.1, z: None, m: None, t: Some(1.0) },
                    VectorPoint { x: 0.2, y: 0.1, z: None, m: None, t: Some(1.0) }
                ]]],
                offset: Some(vec![vec![0.1]]),
                bbox: None,
                vec_bbox: Some(BBox3D {
                    left: 1.7976931348623157e308,
                    bottom: 1.7976931348623157e308,
                    right: -1.7976931348623157e308,
                    top: -1.7976931348623157e308,
                    near: 1.7976931348623157e308,
                    far: -1.7976931348623157e308
                }),
                indices: None,
                tessellation: None
            }))
        ); // The single polygon becomes degenerate and should be filtered out
    }
}
