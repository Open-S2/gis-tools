use crate::{
    build_sq_dists, clip_line, BBox3D, ClipLineResultWithBBox, Face, Feature, Geometry, LonLat,
    S2Feature, S2Point, STPoint, VectorFeature, VectorGeometry, VectorGeometryType,
    VectorLineString, VectorLineStringGeometry, VectorMultiLineStringGeometry,
    VectorMultiPointGeometry, VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
    VectorPolygon, VectorPolygonGeometry,
};

use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::vec::Vec;

// TODO: We are cloning geometry twice at times. Let's optimize (check "to_vec" and "clone" cases)

impl<M: Clone> Feature<M> {
    /// Convert a GeoJSON Feature to a GeoJSON Vector Feature
    pub fn to_vector(data: &Feature<M>, build_bbox: Option<bool>) -> VectorFeature<M> {
        let build_bbox = build_bbox.unwrap_or(false);
        let Feature { id, properties, metadata, geometry, .. } = data;
        let vector_geo = convert_geometry(geometry, build_bbox);
        VectorFeature::new_wm(*id, properties.clone(), vector_geo, metadata.clone())
    }
}

impl<M: Clone> VectorFeature<M> {
    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    pub fn to_unit_scale(&mut self, tolerance: Option<f64>, maxzoom: Option<u8>) {
        let mut bbox: Option<BBox3D> = None;
        match &mut self.geometry {
            VectorGeometry::Point(geo) => {
                geo.coordinates.project(&mut bbox);
                geo.vec_bbox = bbox;
            }
            VectorGeometry::LineString(geo) | VectorGeometry::MultiPoint(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.project(&mut bbox));
                geo.vec_bbox = bbox;
            }
            VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
                geo.coordinates
                    .iter_mut()
                    .for_each(|p| p.iter_mut().for_each(|p| p.project(&mut bbox)));
                geo.vec_bbox = bbox;
            }
            VectorGeometry::MultiPolygon(geo) => {
                geo.coordinates.iter_mut().for_each(|p| {
                    p.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.project(&mut bbox)))
                });
                geo.vec_bbox = bbox;
            }
        }

        if let Some(tolerance) = tolerance {
            build_sq_dists(&mut self.geometry, tolerance, maxzoom);
        }
    }

    /// Reproject GeoJSON geometry coordinates from lon-lat to a 0->1 coordinate system in place
    pub fn to_ll(&mut self) {
        match &mut self.geometry {
            VectorGeometry::Point(geo) => {
                geo.coordinates.unproject();
            }
            VectorGeometry::LineString(geo) | VectorGeometry::MultiPoint(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.unproject());
            }
            VectorGeometry::Polygon(geo) | VectorGeometry::MultiLineString(geo) => {
                geo.coordinates.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.unproject()));
            }
            VectorGeometry::MultiPolygon(geo) => {
                geo.coordinates.iter_mut().for_each(|p| {
                    p.iter_mut().for_each(|p| p.iter_mut().for_each(|p| p.unproject()))
                });
            }
        }
    }

    /// Convet a GeoJSON Feature to an S2Feature
    pub fn to_s2(&self, tolerance: Option<f64>, maxzoom: Option<u8>) -> Vec<S2Feature<M>> {
        let VectorFeature { _type, id, properties, metadata, geometry, .. } = self;
        let mut res: Vec<S2Feature<M>> = vec![];

        if _type == "S2Feature" {
            res.push(self.clone());
        } else {
            let vector_geo = convert_geometry_wm_to_s2(geometry, tolerance, maxzoom);
            for ConvertedGeometry { geometry, face } in vector_geo {
                res.push(S2Feature::<M>::new_s2(
                    *id,
                    face,
                    properties.clone(),
                    geometry,
                    metadata.clone(),
                ));
            }
        }

        res
    }
}

/// Convert a GeoJSON Geometry to an Vector Geometry
fn convert_geometry(geometry: &Geometry, _build_bbox: bool) -> VectorGeometry {
    // TODO: build a bbox if user wants it
    match geometry {
        Geometry::Point(geo) => {
            let mut coordinates: VectorPoint = (&geo.coordinates).into();
            coordinates.m = geo.m_values.clone();
            VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d: false,
                coordinates,
                bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            })
        }
        Geometry::Point3D(geo) => {
            let mut coordinates: VectorPoint = (&geo.coordinates).into();
            coordinates.m = geo.m_values.clone();
            VectorGeometry::Point(VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d: true,
                coordinates,
                bbox: geo.bbox,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            })
        }
        Geometry::MultiPoint(geo) => VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorGeometryType::MultiPoint,
            is_3d: false,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mut vp = VectorPoint::from(p);
                    if let Some(m) = &geo.m_values {
                        vp.m = Some(m[i].clone());
                    }
                    vp
                })
                .collect(),
            bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
            ..Default::default()
        }),
        Geometry::MultiPoint3D(geo) => VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorGeometryType::MultiPoint,
            is_3d: true,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mut vp = VectorPoint::from(p);
                    if let Some(m) = &geo.m_values {
                        vp.m = Some(m[i].clone());
                    }
                    vp
                })
                .collect(),
            bbox: geo.bbox,
            ..Default::default()
        }),
        Geometry::LineString(geo) => VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
            is_3d: false,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mut vp = VectorPoint::from(p);
                    if let Some(m) = &geo.m_values {
                        vp.m = Some(m[i].clone());
                    }
                    vp
                })
                .collect(),
            bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
            ..Default::default()
        }),
        Geometry::LineString3D(geo) => VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
            is_3d: true,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let mut vp = VectorPoint::from(p);
                    if let Some(m) = &geo.m_values {
                        vp.m = Some(m[i].clone());
                    }
                    vp
                })
                .collect(),
            bbox: geo.bbox,
            ..Default::default()
        }),
        Geometry::MultiLineString(geo) => {
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: VectorGeometryType::MultiLineString,
                is_3d: false,
                coordinates: geo
                    .coordinates
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        line.iter()
                            .enumerate()
                            .map(|(j, p)| {
                                let mut vp = VectorPoint::from(p);
                                if let Some(m) = &geo.m_values {
                                    vp.m = Some(m[i][j].clone());
                                }
                                vp
                            })
                            .collect()
                    })
                    .collect(),
                bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
                ..Default::default()
            })
        }
        Geometry::MultiLineString3D(geo) => {
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: VectorGeometryType::MultiLineString,
                is_3d: true,
                coordinates: geo
                    .coordinates
                    .iter()
                    .enumerate()
                    .map(|(i, line)| {
                        line.iter()
                            .enumerate()
                            .map(|(j, p)| {
                                let mut vp = VectorPoint::from(p);
                                if let Some(m) = &geo.m_values {
                                    vp.m = Some(m[i][j].clone());
                                }
                                vp
                            })
                            .collect()
                    })
                    .collect(),
                bbox: geo.bbox,
                ..Default::default()
            })
        }
        Geometry::Polygon(geo) => VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            is_3d: false,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(j, p)| {
                            let mut vp = VectorPoint::from(p);
                            if let Some(m) = &geo.m_values {
                                vp.m = Some(m[i][j].clone());
                            }
                            vp
                        })
                        .collect()
                })
                .collect(),
            bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
            ..Default::default()
        }),
        Geometry::Polygon3D(geo) => VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            is_3d: true,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(j, p)| {
                            let mut vp = VectorPoint::from(p);
                            if let Some(m) = &geo.m_values {
                                vp.m = Some(m[i][j].clone());
                            }
                            vp
                        })
                        .collect()
                })
                .collect(),
            bbox: geo.bbox,
            ..Default::default()
        }),
        Geometry::MultiPolygon(geo) => VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            is_3d: false,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, polygon)| {
                    polygon
                        .iter()
                        .enumerate()
                        .map(|(j, line)| {
                            line.iter()
                                .enumerate()
                                .map(|(k, p)| {
                                    let mut vp = VectorPoint::from(p);
                                    if let Some(m) = &geo.m_values {
                                        vp.m = Some(m[i][j][k].clone());
                                    }
                                    vp
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect(),
            bbox: geo.bbox.as_ref().map(|bbox| bbox.into()),
            ..Default::default()
        }),
        Geometry::MultiPolygon3D(geo) => VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            is_3d: true,
            coordinates: geo
                .coordinates
                .iter()
                .enumerate()
                .map(|(i, polygon)| {
                    polygon
                        .iter()
                        .enumerate()
                        .map(|(j, line)| {
                            line.iter()
                                .enumerate()
                                .map(|(k, p)| {
                                    let mut vp = VectorPoint::from(p);
                                    if let Some(m) = &geo.m_values {
                                        vp.m = Some(m[i][j][k].clone());
                                    }
                                    vp
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect(),
            bbox: geo.bbox,
            ..Default::default()
        }),
    }
}

/// The resultant geometry after conversion
pub struct ConvertedGeometry {
    pub geometry: VectorGeometry,
    pub face: Face,
}
pub type ConvertedGeometryList = Vec<ConvertedGeometry>;

/// Underlying conversion mechanic to move GeoJSON Geometry to S2Geometry
fn convert_geometry_wm_to_s2(
    geometry: &VectorGeometry,
    tolerance: Option<f64>,
    maxzoom: Option<u8>,
) -> ConvertedGeometryList {
    let mut res: ConvertedGeometryList = vec![];

    match geometry {
        VectorGeometry::Point(geo) => {
            res.extend(convert_geometry_point(geo));
        }
        VectorGeometry::MultiPoint(geo) => {
            res.extend(convert_geometry_multipoint(geo));
        }
        VectorGeometry::LineString(geo) => {
            res.extend(convert_geometry_linestring(geo));
        }
        VectorGeometry::MultiLineString(geo) => {
            res.extend(convert_geometry_multilinestring(geo));
        }
        VectorGeometry::Polygon(geo) => {
            res.extend(convert_geometry_polygon(geo));
        }
        VectorGeometry::MultiPolygon(geo) => {
            res.extend(convert_geometry_multipolygon(geo));
        }
    }

    if let Some(tolerance) = tolerance {
        for c_geo in &mut res {
            build_sq_dists(&mut c_geo.geometry, tolerance, maxzoom);
        }
    }

    res
}

// /**
//  * @param geometry - GeoJSON PointGeometry
//  * @returns - S2 PointGeometry
//  */
/// Convert a GeoJSON PointGeometry to a S2 PointGeometry
fn convert_geometry_point(geometry: &VectorPointGeometry) -> ConvertedGeometryList {
    let VectorPointGeometry { _type, is_3d, coordinates, bbox, .. } = geometry;
    let mut new_point = coordinates.clone();
    let ll: S2Point = (&LonLat::new(new_point.x, new_point.y)).into();
    let (face, s, t) = ll.to_face_st();
    new_point.x = s;
    new_point.y = t;
    let vec_bbox = Some(BBox3D::from_point(&new_point));
    vec![ConvertedGeometry {
        face: face.into(),
        geometry: VectorGeometry::Point(VectorPointGeometry {
            _type: VectorGeometryType::Point,
            coordinates: new_point,
            is_3d: *is_3d,
            bbox: *bbox,
            vec_bbox,
            offset: None,
            indices: None,
            tesselation: None,
        }),
    }]
}

// /**
//  * @param geometry - GeoJSON PointGeometry
//  * @returns - S2 PointGeometry
//  */
fn convert_geometry_multipoint(geometry: &VectorMultiPointGeometry) -> ConvertedGeometryList {
    let VectorMultiPointGeometry { is_3d, coordinates, bbox, .. } = geometry;
    coordinates
        .iter()
        .flat_map(|coordinates| {
            convert_geometry_point(&VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d: *is_3d,
                coordinates: coordinates.clone(),
                bbox: *bbox,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            })
        })
        .collect()
}

/// Convert a GeoJSON LineStringGeometry to S2 LineStringGeometry
fn convert_geometry_linestring(geometry: &VectorLineStringGeometry) -> ConvertedGeometryList {
    let VectorLineStringGeometry { _type, is_3d, coordinates, bbox, .. } = geometry;

    convert_line_string(coordinates, false)
        .into_iter()
        .map(|cline| {
            let ConvertedLineString { face, line, offset, vec_bbox } = cline;
            ConvertedGeometry {
                face,
                geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                    _type: VectorGeometryType::LineString,
                    is_3d: *is_3d,
                    coordinates: line.to_vec(),
                    bbox: *bbox,
                    offset: Some(offset),
                    vec_bbox: Some(vec_bbox),
                    ..Default::default()
                }),
            }
        })
        .collect()
}

/// Convert a GeoJSON MultiLineStringGeometry to S2 MultiLineStringGeometry
fn convert_geometry_multilinestring(
    geometry: &VectorMultiLineStringGeometry,
) -> ConvertedGeometryList {
    let VectorMultiLineStringGeometry { is_3d, coordinates, bbox, .. } = geometry;

    coordinates
        .iter()
        .flat_map(|line| convert_line_string(line, false))
        .map(|ConvertedLineString { face, line, offset, vec_bbox }| ConvertedGeometry {
            face,
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                _type: VectorGeometryType::LineString,
                is_3d: *is_3d,
                coordinates: line,
                bbox: *bbox,
                offset: Some(offset),
                vec_bbox: Some(vec_bbox),
                ..Default::default()
            }),
        })
        .collect()
}

/// Convert a GeoJSON PolygonGeometry to S2 PolygonGeometry
fn convert_geometry_polygon(geometry: &VectorPolygonGeometry) -> ConvertedGeometryList {
    let VectorPolygonGeometry { _type, is_3d, coordinates, bbox, .. } = geometry;
    let mut res: ConvertedGeometryList = vec![];

    // conver all lines
    let mut outer_ring = convert_line_string(&coordinates[0], true);
    let mut inner_rings = coordinates[1..].iter().flat_map(|line| convert_line_string(line, true));

    // for each face, build a new polygon
    for ConvertedLineString { face, line, offset, vec_bbox: poly_bbox } in &mut outer_ring {
        let mut polygon: VectorPolygon = vec![line.to_vec()];
        let mut polygon_offsets = vec![*offset];
        let mut poly_bbox = *poly_bbox;
        for ConvertedLineString {
            face: inner_face,
            line: inner_line,
            offset: inner_offset,
            vec_bbox,
        } in &mut inner_rings
        {
            if inner_face == *face {
                polygon.push(inner_line);
                polygon_offsets.push(inner_offset);
                poly_bbox = poly_bbox.merge(&vec_bbox);
            }
        }

        res.push(ConvertedGeometry {
            face: *face,
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: VectorGeometryType::Polygon,
                is_3d: *is_3d,
                coordinates: polygon,
                bbox: *bbox,
                offset: Some(polygon_offsets),
                vec_bbox: Some(poly_bbox),
                ..Default::default()
            }),
        });
    }

    res
}

/// Convert a GeoJSON MultiPolygonGeometry to S2 MultiPolygonGeometry
fn convert_geometry_multipolygon(geometry: &VectorMultiPolygonGeometry) -> ConvertedGeometryList {
    let VectorMultiPolygonGeometry { is_3d, coordinates, bbox, offset, .. } = geometry;
    coordinates
        .iter()
        .enumerate()
        .flat_map(|(i, polygon)| {
            let offset: Option<Vec<f64>> = offset.as_ref().map(|offset| offset[i].clone());
            convert_geometry_polygon(&VectorPolygonGeometry {
                _type: VectorGeometryType::Polygon,
                is_3d: *is_3d,
                coordinates: polygon.to_vec(),
                bbox: *bbox,
                offset,
                ..Default::default()
            })
        })
        .collect()
}

/// LineString converted from WM to S2
pub struct ConvertedLineString {
    face: Face,
    line: VectorLineString,
    offset: f64,
    vec_bbox: BBox3D,
}

/// Convert WM LineString to S2
fn convert_line_string(line: &VectorLineString, is_polygon: bool) -> Vec<ConvertedLineString> {
    let mut res: Vec<ConvertedLineString> = vec![];
    // first re-project all the coordinates to S2
    let mut new_geometry: Vec<STPoint> = vec![];
    for VectorPoint { x: lon, y: lat, z, m, .. } in line {
        let ll: S2Point = (&LonLat::new(*lon, *lat)).into();
        let (face, s, t) = ll.to_face_st();
        new_geometry.push(STPoint { face: face.into(), s, t, z: *z, m: m.clone() });
    }
    // find all the faces that exist in the line
    let mut faces = BTreeSet::<Face>::new();
    new_geometry.iter().for_each(|stpoint| {
        faces.insert(stpoint.face);
    });
    // for each face, build a line
    for face in faces {
        let mut line: VectorLineString = vec![];
        for st_point in &new_geometry {
            line.push(st_point_to_face(face, st_point));
        }
        let clipped_lines = clip_line(&line, BBox3D::default(), is_polygon, None, None);
        for ClipLineResultWithBBox { line, offset, vec_bbox } in clipped_lines {
            res.push(ConvertedLineString { face, line, offset, vec_bbox });
        }
    }

    res
}

/// Given a face, rotate the point into it's 0->1 coordinate system
fn st_point_to_face(target_face: Face, stp: &STPoint) -> VectorPoint {
    let cur_face = stp.face;
    if target_face == cur_face {
        return VectorPoint { x: stp.s, y: stp.t, z: stp.z, m: stp.m.clone(), t: None };
    }

    let (rot, x, y) = &FACE_RULE_SET[target_face as usize][cur_face as usize];
    let (new_s, new_t) = rotate(*rot, stp.s, stp.t);

    VectorPoint { x: new_s + *x as f64, y: new_t + *y as f64, z: stp.z, m: stp.m.clone(), t: None }
}

/**
 * @param rot - rotation
 * @param s - input s
 * @param t - input t
 * @returns - new [s, t] after rotating
 */
fn rotate(rot: Rotation, s: f64, t: f64) -> (f64, f64) {
    match rot {
        Rotation::_0 => (s, t),
        Rotation::_90 => (t, 1. - s),
        Rotation::_Neg90 => (1. - t, s),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rotation {
    _0,
    _90,
    _Neg90,
}

/// Ruleset for converting an S2Point from a face to another.
/// While this this set includes opposite side faces, without axis mirroring,
/// it is not technically accurate and shouldn't be used. Instead, data should let two points travel
/// further than a full face width.
/// FACE_RULE_SET[target_face][currentFace] = [rot, x, y]
pub const FACE_RULE_SET: [[(Rotation, i8, i8); 6]; 6] = [
    // Target Face 0
    [
        (Rotation::_0, 0, 0),      // Current Face 0
        (Rotation::_0, 1, 0),      // Current Face 1
        (Rotation::_90, 0, 1),     // Current Face 2
        (Rotation::_Neg90, 2, 0),  // Current Face 3
        (Rotation::_Neg90, -1, 0), //  Current Face 4
        (Rotation::_0, 0, -1),     //  Current Face 5
    ],
    // Target Face 1
    [
        (Rotation::_0, -1, 0),    // Current Face 0
        (Rotation::_0, 0, 0),     // Current Face 1
        (Rotation::_0, 0, 1),     // Current Face 2
        (Rotation::_Neg90, 1, 0), // Current Face 3
        (Rotation::_Neg90, 2, 0), // Current Face 4
        (Rotation::_90, 0, -1),   // Current Face 5
    ],
    // Target Face 2
    [
        (Rotation::_Neg90, -1, 0), // Current Face 0
        (Rotation::_0, 0, -1),     // Current Face 1
        (Rotation::_0, 0, 0),      // Current Face 2
        (Rotation::_0, 1, 0),      // Current Face 3
        (Rotation::_90, 0, 1),     // Current Face 4
        (Rotation::_Neg90, 2, 0),  // Current Face 5
    ],
    // Target Face 3
    [
        (Rotation::_Neg90, 2, 0), // Current Face 0
        (Rotation::_90, 0, -1),   // Current Face 1
        (Rotation::_0, -1, 0),    // Current Face 2
        (Rotation::_0, 0, 0),     // Current Face 3
        (Rotation::_0, 0, 1),     // Current Face 4
        (Rotation::_Neg90, 1, 0), // Current Face 5
    ],
    // Target Face 4
    [
        (Rotation::_90, 0, 1),     // Current Face 0
        (Rotation::_Neg90, 2, 0),  // Current Face 1
        (Rotation::_Neg90, -1, 0), // Current Face 2
        (Rotation::_0, 0, -1),     // Current Face 3
        (Rotation::_0, 0, 0),      // Current Face 4
        (Rotation::_0, 1, 0),      // Current Face 5
    ],
    // Target Face 5
    [
        (Rotation::_0, 0, 1),     // Current Face 0
        (Rotation::_Neg90, 1, 0), // Current Face 1
        (Rotation::_Neg90, 2, 0), // Current Face 2
        (Rotation::_90, 0, -1),   // Current Face 3
        (Rotation::_0, -1, 0),    // Current Face 4
        (Rotation::_0, 0, 0),     // Current Face 5
    ],
];
