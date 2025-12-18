use alloc::{vec, vec::Vec};
use s2json::{
    Axis, BBox3D, VectorFeature, VectorGeometry, VectorGeometryType, VectorLineString,
    VectorLineStringGeometry, VectorMultiLineOffset, VectorMultiLineString,
    VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygon,
    VectorMultiPolygonGeometry, VectorMultiPolygonOffset, VectorPoint, VectorPointGeometry,
    VectorPolygonGeometry,
};

// TODO: Cases of `to_vec` clones large swathes of data. Can we optimize this?

/// Internal clip function for a collection of VectorFeatures
pub fn clip_features<M, P: Clone + Default, D: Clone + Default>(
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
pub fn clip_point<M: Clone + Default>(
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
pub fn clip_multi_point<M: Clone + Default>(
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
pub fn clip_line_string<M: Clone + Default>(
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
pub fn clip_multi_line_string<M: Clone + Default>(
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
pub fn clip_polygon<M: Clone + Default>(
    geometry: &VectorPolygonGeometry<M>,
    axis: Axis,
    k1: f64,
    k2: f64,
) -> Option<VectorGeometry<M>> {
    clip_multi_line_string(geometry, axis, k1, k2, true)
}

/// Clip a MultiPolygon geometry to an axis and range
pub fn clip_multi_polygon<M: Clone + Default>(
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
    let last_point = &geom[last];
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
