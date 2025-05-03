#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use gistools::geometry::ConvertVectorFeatureS2;
    use s2json::{
        BBox3D, VectorFeature, VectorGeometry, VectorLineStringGeometry,
        VectorMultiLineStringGeometry, VectorMultiPointGeometry, VectorMultiPolygonGeometry,
        VectorPoint, VectorPointGeometry, VectorPolygonGeometry,
    };

    #[test]
    fn test_convert_geometry_point() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multipoint() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_linestring() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                    VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                    VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multi_linestring() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_polygon() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::Polygon(VectorPolygonGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn test_convert_geometry_multi_polygon() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "S2Feature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPoint".into(),
                is_3d: false,
                coordinates: vec![vec![
                    vec![
                        VectorPoint { x: 0.5, y: 0.5, z: None, m: None, t: None },
                        VectorPoint { x: 1., y: 1., z: None, m: None, t: None },
                    ],
                    vec![
                        VectorPoint { x: -0.5, y: -0.5, z: None, m: None, t: None },
                        VectorPoint { x: 2., y: 2., z: None, m: None, t: None },
                    ],
                ]],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            }),
            metadata: None,
        };
        let wm_feature = s2_feature.to_wm();
        assert_eq!(
            wm_feature.geometry,
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![vec![
                    vec![
                        VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                        VectorPoint { x: 45.0, y: 35.264389682754654, z: None, m: None, t: None }
                    ],
                    vec![
                        VectorPoint {
                            x: -69.44395478041653,
                            y: -43.11666555262819,
                            z: None,
                            m: None,
                            t: None
                        },
                        VectorPoint {
                            x: 78.69006752597979,
                            y: 44.43824067114979,
                            z: None,
                            m: None,
                            t: None
                        }
                    ]
                ]],
                bbox: Some(BBox3D::new(0., 0., 0.5, 1., 0., 0.)),
                ..Default::default()
            })
        );
    }

    #[test]
    fn convert_vector_feature() {
        let s2_feature: VectorFeature = VectorFeature {
            _type: "VectorFeature".into(),
            id: 2.into(),
            face: 0.into(),
            properties: Default::default(),
            geometry: VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                is_3d: false,
                coordinates: VectorPoint { x: 0., y: 0., z: None, m: None, t: None },
                ..Default::default()
            }),
            metadata: None,
        };
        let converted = s2_feature.to_wm();
        assert_eq!(converted, s2_feature);
    }
}
