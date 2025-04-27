use crate::geometry::{
    BBox3D, ClipLineResultWithBBox, Face, LonLat, MValue, S2Point, STPoint, VectorGeometry,
    VectorGeometryType, VectorLineString, VectorLineStringGeometry, VectorMultiLineStringGeometry,
    VectorMultiPointGeometry, VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
    VectorPolygon, VectorPolygonGeometry, clip_line,
};
use alloc::{collections::BTreeSet, vec, vec::Vec};

/// The resultant geometry after conversion
#[derive(Debug)]
pub struct ConvertedGeometry<M: Clone + Default = MValue> {
    /// The converted geometry
    pub geometry: VectorGeometry<M>,
    /// The face of the geometry
    pub face: Face,
}
/// A list of converted geometries
pub type ConvertedGeometryList<M> = Vec<ConvertedGeometry<M>>;

// TODO: We may be able to optimize clones. Do we just take ownership with mutables?

/// Underlying conversion mechanic to move GeoJSON Geometry to S2Geometry
pub fn convert_geometry_wm_to_s2<M: Clone + Default>(
    geometry: &VectorGeometry<M>,
) -> ConvertedGeometryList<M> {
    let mut res: ConvertedGeometryList<M> = vec![];

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

    res
}

/// Convert a GeoJSON PointGeometry to a S2 PointGeometry
fn convert_geometry_point<M: Clone + Default>(
    geometry: &VectorPointGeometry<M>,
) -> ConvertedGeometryList<M> {
    let VectorPointGeometry::<M> { _type, is_3d, coordinates, bbox, .. } = geometry;
    let mut new_point = coordinates.clone();
    let ll: S2Point = (&LonLat::<M>::new(new_point.x, new_point.y, None)).into();
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
            ..Default::default()
        }),
    }]
}

/// Convert a GeoJSON MultiPointGeometry to S2 MultiPointGeometry
fn convert_geometry_multipoint<M: Clone + Default>(
    geometry: &VectorMultiPointGeometry<M>,
) -> ConvertedGeometryList<M> {
    let VectorMultiPointGeometry { is_3d, coordinates, bbox, .. } = geometry;
    coordinates
        .iter()
        .flat_map(|coordinates| {
            convert_geometry_point(&VectorPointGeometry {
                _type: VectorGeometryType::Point,
                is_3d: *is_3d,
                coordinates: coordinates.clone(),
                bbox: *bbox,
                ..Default::default()
            })
        })
        .collect()
}

/// Convert a GeoJSON LineStringGeometry to S2 LineStringGeometry
fn convert_geometry_linestring<M: Clone + Default>(
    geometry: &VectorLineStringGeometry<M>,
) -> ConvertedGeometryList<M> {
    let VectorLineStringGeometry { _type, is_3d, coordinates, bbox, .. } = geometry;

    convert_line_string(coordinates, false)
        .into_iter()
        .map(|cline| {
            let ConvertedLineString { face, mut line, offset, vec_bbox } = cline;
            ConvertedGeometry {
                face,
                geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                    _type: VectorGeometryType::LineString,
                    is_3d: *is_3d,
                    coordinates: core::mem::take(&mut line),
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
fn convert_geometry_multilinestring<M: Clone + Default>(
    geometry: &VectorMultiLineStringGeometry<M>,
) -> ConvertedGeometryList<M> {
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
fn convert_geometry_polygon<M: Clone + Default>(
    geometry: &VectorPolygonGeometry<M>,
) -> ConvertedGeometryList<M> {
    let VectorPolygonGeometry { _type, is_3d, coordinates, bbox, .. } = geometry;
    let mut res: ConvertedGeometryList<M> = vec![];

    // conver all lines
    let mut outer_ring = convert_line_string(&coordinates[0], true);
    let mut inner_rings = coordinates[1..].iter().flat_map(|line| convert_line_string(line, true));

    // for each face, build a new polygon
    for ConvertedLineString { face, line, offset, vec_bbox: poly_bbox } in &mut outer_ring {
        let mut polygon: VectorPolygon<M> = vec![core::mem::take(line)];
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
                poly_bbox.merge_in_place(&vec_bbox);
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
fn convert_geometry_multipolygon<M: Clone + Default>(
    geometry: &VectorMultiPolygonGeometry<M>,
) -> ConvertedGeometryList<M> {
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
pub struct ConvertedLineString<M: Clone + Default = MValue> {
    face: Face,
    line: VectorLineString<M>,
    offset: f64,
    vec_bbox: BBox3D,
}

/// Convert WM LineString to S2
fn convert_line_string<M: Clone + Default>(
    line: &VectorLineString<M>,
    is_polygon: bool,
) -> Vec<ConvertedLineString<M>> {
    let mut res: Vec<ConvertedLineString<M>> = vec![];
    // find all the faces that exist in the line while we re-project
    let mut faces = BTreeSet::<Face>::new();
    // first re-project all the coordinates to S2
    let mut new_geometry: Vec<STPoint<M>> = vec![];
    for VectorPoint { x: lon, y: lat, z, m, .. } in line {
        let ll: S2Point = (&LonLat::<M>::new(*lon, *lat, None)).into();
        let (face, s, t) = ll.to_face_st();
        let stpoint = STPoint { face: face.into(), s, t, z: *z, m: m.clone() };
        faces.insert(stpoint.face);
        new_geometry.push(stpoint);
    }
    // for each face, build a line
    for face in faces {
        let mut line: VectorLineString<M> = vec![];
        for st_point in &mut new_geometry {
            line.push(st_point_to_face(face, st_point));
        }
        let clipped_lines =
            clip_line(&line, BBox3D::new(0., 0., 1., 1., 0., 1.), is_polygon, None, None);
        for ClipLineResultWithBBox { line, offset, vec_bbox } in clipped_lines {
            res.push(ConvertedLineString { face, line, offset, vec_bbox });
        }
    }

    res
}

/// Given a face, rotate the point into it's 0->1 coordinate system
fn st_point_to_face<M: Clone + Default>(target_face: Face, stp: &mut STPoint<M>) -> VectorPoint<M> {
    let cur_face = stp.face;
    if target_face == cur_face {
        return VectorPoint {
            x: stp.s,
            y: stp.t,
            z: stp.z,
            m: core::mem::take(&mut stp.m),
            t: None,
        };
    }

    let (rot, x, y) = &FACE_RULE_SET[target_face as usize][cur_face as usize];
    let (new_s, new_t) = rotate(*rot, stp.s, stp.t);

    VectorPoint {
        x: new_s + *x as f64,
        y: new_t + *y as f64,
        z: stp.z,
        m: core::mem::take(&mut stp.m),
        t: None,
    }
}

/// @param rot - rotation
/// @param s - input s
/// @param t - input t
/// @returns - new [s, t] after rotating
fn rotate(rot: Rotation, s: f64, t: f64) -> (f64, f64) {
    match rot {
        Rotation::_0 => (s, t),
        Rotation::_90 => (t, 1. - s),
        Rotation::_Neg90 => (1. - t, s),
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// Track the rotation of a face
pub enum Rotation {
    /// No rotation
    _0,
    /// Rotate 90 degrees
    _90,
    /// Rotate -90 degrees
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

#[cfg(test)]
mod tests {
    use s2json::{
        FeatureCollection, Features, JSONCollection, Projection, S2FeatureCollection, VectorFeature,
    };

    use crate::geometry::{ConvertVectorFeatureWM, convert};

    use super::*;

    #[test]
    fn test_face_rule_set() {
        assert_eq!(FACE_RULE_SET[0][0], (Rotation::_0, 0, 0));
    }

    #[test]
    fn to_s2_point() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_wm_vectorpoint_convert() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = convert(
            Projection::WG,
            &JSONCollection::VectorFeature(feature),
            Some(true),
            Some(true),
        );

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "VectorFeature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_point_fc_convert() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., None, m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let fc: FeatureCollection = FeatureCollection {
            _type: "FeatureCollection".into(),
            features: vec![Features::VectorFeature(feature.clone())],
            ..Default::default()
        };
        let s2_feature =
            convert(Projection::S2, &JSONCollection::FeatureCollection(fc), Some(true), None);

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: false,
                    coordinates: VectorPoint::new(0.5, 0.5, None, m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_already_s2() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let s2f: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: Some(1337),
            face: 0.into(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint::new(0.5, 0.5, None, m_value.clone()),
                bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let s2fc: S2FeatureCollection = S2FeatureCollection {
            _type: "S2FeatureCollection".into(),
            features: vec![s2f.clone()],
            ..Default::default()
        };
        let s2_feature =
            convert(Projection::S2, &JSONCollection::S2FeatureCollection(s2fc), Some(true), None);

        assert_eq!(s2_feature, vec![s2f]);
    }

    #[test]
    fn to_s2_point_3d() {
        let m_value = Some(MValue::from([("a".into(), (1_u64).into())]));
        let coords = VectorPoint::new(0., 0., Some(1.), m_value.clone());
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_point(coords, bbox),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![VectorFeature {
                _type: "S2Feature".into(),
                id: Some(1337),
                face: 0.into(),
                geometry: VectorGeometry::Point(VectorPointGeometry {
                    _type: "Point".into(),
                    is_3d: true,
                    coordinates: VectorPoint::new(0.5, 0.5, Some(1.), m_value),
                    bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                    vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, 1., 1.)),
                    ..Default::default()
                }),
                ..Default::default()
            }]
        );
    }

    #[test]
    fn to_s2_multipoint() {
        let coords = vec![
            VectorPoint::new(0., 0., None, Some(MValue::from([("a".into(), (1_u64).into())]))),
            VectorPoint::new(-180., -90., None, Some(MValue::from([("b".into(), (2_u64).into())]))),
            VectorPoint::new(180., 90., None, Some(MValue::from([("c".into(), (3_u64).into())]))),
        ];
        let properties = MValue::from([("d".into(), (4_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multipoint(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("a".into(), (1_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 5.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("b".into(), (2_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Point(VectorPointGeometry {
                        _type: "Point".into(),
                        is_3d: false,
                        coordinates: VectorPoint::new(
                            0.5,
                            0.5,
                            None,
                            Some(MValue::from([("c".into(), (3_u64).into())]))
                        ),
                        bbox: Some(BBox3D::new(1., 2., 3., 4., 5., 6.)),
                        vec_bbox: Some(BBox3D::new(0.5, 0.5, 0.5, 0.5, f64::MAX, f64::MIN)),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn to_s2_linestring() {
        let coords = vec![
            VectorPoint::new(0., 0., None, Some(MValue::from([("a".into(), (1_u64).into())]))),
            VectorPoint::new(20., 20., None, Some(MValue::from([("b".into(), (2_u64).into())]))),
            VectorPoint::new(30., 30., None, Some(MValue::from([("c".into(), (3_u64).into())]))),
            VectorPoint::new(40., 40., None, Some(MValue::from([("d".into(), (4_u64).into())]))),
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_linestring(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.5,
                                y: 0.5,
                                z: None,
                                m: Some(MValue::from([("a".into(), (1_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.7231719544476624,
                                y: 0.7351848576118168,
                                z: None,
                                m: Some(MValue::from([("b".into(), (2_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.8264458251405347,
                                y: 0.8660254037844386,
                                z: None,
                                m: Some(MValue::from([("c".into(), (3_u64).into())])),
                                t: None
                            },
                            VectorPoint {
                                x: 0.6953495465482081,
                                y: 1.0625,
                                z: None,
                                m: Some(MValue::from([("d".into(), (4_u64).into())])),
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.8264458251405347,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.17012925937810885,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(1.5284052199258356),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.17012925937810885,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_multilinestring() {
        let coords = vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(40., 40., None, None),
            ],
            vec![
                VectorPoint::new(-120., -30., None, None),
                VectorPoint::new(-130., -40., None, None),
                VectorPoint::new(-140., -50., None, None),
                VectorPoint::new(-150., -60., None, None),
            ],
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multilinestring(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 4);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                            VectorPoint {
                                x: 0.7231719544476624,
                                y: 0.7351848576118168,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.8264458251405347,
                                y: 0.8660254037844386,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.6953495465482081,
                                y: 1.0625,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.8264458251405347,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.17012925937810885,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(1.5284052199258356),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.17012925937810885,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 4.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: 0.8660254037844386,
                                y: 0.17355417485946534,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 1.0332000398839454,
                                y: 0.0919618222017129,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 1.0625,
                                y: 0.1016957300340185,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ],
                        offset: Some(0.0),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.8660254037844386,
                            bottom: 0.0919618222017129,
                            right: 1.0625,
                            top: 0.17355417485946534,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 5.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                        _type: "LineString".into(),
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.13866981323286479,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.0919618222017129,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.1909745772474294,
                                y: 0.14437700634864636,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.3169872981077806,
                                y: 0.209430584957905,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: Some(0.07953324204553078),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.0919618222017129,
                            right: 0.3169872981077806,
                            top: 0.209430584957905,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_polygon() {
        let coords = vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 0., None, None),
                VectorPoint::new(40., 0., None, None),
                VectorPoint::new(40., 20., None, None),
                VectorPoint::new(40., 40., None, None),
                VectorPoint::new(20., 40., None, None),
                VectorPoint::new(0., 40., None, None),
                VectorPoint::new(0., 20., None, None),
                VectorPoint::new(0., 0., None, None),
            ],
            vec![
                VectorPoint::new(10., 10., None, None),
                VectorPoint::new(20., 10., None, None),
                VectorPoint::new(30., 10., None, None),
                VectorPoint::new(30., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(20., 30., None, None),
                VectorPoint::new(10., 30., None, None),
                VectorPoint::new(10., 20., None, None),
                VectorPoint::new(10., 10., None, None),
            ],
        ];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_polygon(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![
                            vec![
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.7786828928924201,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7356879031193608,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.6583568237637192,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.7231719544476622,
                                    y: 0.9590168832161913,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.9377231592442196,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.7231719544476624,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.6250859462252395,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.6345893512076446,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.7518028126416558,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.8660254037844386,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.8430910345588061,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.8304773451370653,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.7260776792851733,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                }
                            ]
                        ],
                        offset: Some(vec![3.241841444519629, 0.0]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.9377231592442196,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.15284249599867805,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ]],
                        offset: Some(vec![1.7505894300567113]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.19165525141383033,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn to_s2_multipolygon() {
        let coords = vec![vec![
            vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(20., 0., None, None),
                VectorPoint::new(40., 0., None, None),
                VectorPoint::new(40., 20., None, None),
                VectorPoint::new(40., 40., None, None),
                VectorPoint::new(20., 40., None, None),
                VectorPoint::new(0., 40., None, None),
                VectorPoint::new(0., 20., None, None),
                VectorPoint::new(0., 0., None, None),
            ],
            vec![
                VectorPoint::new(10., 10., None, None),
                VectorPoint::new(20., 10., None, None),
                VectorPoint::new(30., 10., None, None),
                VectorPoint::new(30., 20., None, None),
                VectorPoint::new(30., 30., None, None),
                VectorPoint::new(20., 30., None, None),
                VectorPoint::new(10., 30., None, None),
                VectorPoint::new(10., 20., None, None),
                VectorPoint::new(10., 10., None, None),
            ],
        ]];
        let properties = MValue::from([("names".into(), (20000_u64).into())]);
        let bbox = Some(BBox3D::new(1., 2., 3., 4., 5., 6.));
        let feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: Some(1337),
            geometry: VectorGeometry::new_multipolygon(coords, bbox),
            properties: properties.clone(),
            ..Default::default()
        };
        let s2_feature = feature.to_s2();
        assert_eq!(s2_feature.len(), 2);

        assert_eq!(
            s2_feature,
            vec![
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 0.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![
                            vec![
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.5,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.9377231592442196,
                                    y: 0.7786828928924201,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7356879031193608,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.6583568237637192,
                                    y: 1.0625,
                                    z: None,
                                    m: None,
                                    t: Some(1.0)
                                },
                                VectorPoint {
                                    x: 0.7231719544476622,
                                    y: 0.9590168832161913,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.9377231592442196,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.5,
                                    y: 0.7231719544476624,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None }
                            ],
                            vec![
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.6250859462252395,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.6345893512076446,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.7518028126416558,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.8264458251405347,
                                    y: 0.8660254037844386,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.7231719544476624,
                                    y: 0.8430910345588061,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.8304773451370653,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.7260776792851733,
                                    z: None,
                                    m: None,
                                    t: None
                                },
                                VectorPoint {
                                    x: 0.6182598446699807,
                                    y: 0.6199075184683839,
                                    z: None,
                                    m: None,
                                    t: None
                                }
                            ]
                        ],
                        offset: Some(vec![3.241841444519629, 0.0]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: 0.5,
                            bottom: 0.5,
                            right: 0.9377231592442196,
                            top: 1.0625,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: "S2Feature".into(),
                    id: Some(1337),
                    face: 2.into(),
                    properties: properties.clone(),
                    geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                        _type: "Polygon".into(),
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: 0.033200039883945376,
                                y: 0.091961822201713,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.15284249599867805,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.0625,
                                y: 0.19165525141383033,
                                z: None,
                                m: None,
                                t: Some(1.0)
                            }
                        ]],
                        offset: Some(vec![1.7505894300567113]),
                        bbox: Some(BBox3D {
                            left: 1.0,
                            bottom: 2.0,
                            right: 3.0,
                            top: 4.0,
                            near: 5.0,
                            far: 6.0
                        }),
                        vec_bbox: Some(BBox3D {
                            left: -0.0625,
                            bottom: 0.091961822201713,
                            right: 0.033200039883945376,
                            top: 0.19165525141383033,
                            near: f64::MAX,
                            far: f64::MIN
                        }),
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }
}
