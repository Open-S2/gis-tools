#[cfg(test)]
// #[coverage(off)]
mod tests {
    use parsers::FileReader;
    use readers::{
        OSMLocalReader,
        header_block::OSMHeader,
        info::InfoBlock,
        primitive::OSMMetadata,
        relation::{IntermediateNodeMember, MemberType},
    };
    use s2json::{
        BBox, BBox3D, Map, VectorBaseGeometry, VectorFeature, VectorFeatureType, VectorGeometry,
        VectorGeometryType, VectorPoint,
    };
    use std::{path::PathBuf, vec};

    #[test]
    fn base_case() {
        let mut path = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
        path.push("tests/readers/osm/fixtures/test.pbf");
        let path_str = path.to_str().unwrap();
        let reader = FileReader::from(path_str);

        let mut osm = OSMLocalReader::new(reader, None);
        osm.parse_blocks();

        let header: OSMHeader = osm.get_header();

        assert_eq!(
            header,
            OSMHeader {
                bbox: BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 },
                required_features: vec!["-x�S��/�\rN�H�M�\r3�3S�rI�+N".into()],
                optional_features: vec![],
                writingprogram: None,
                source: None,
                osmosis_replication_timestamp: 0,
                osmosis_replication_sequence_number: 0,
                osmosis_replication_base_url: None
            }
        );

        let features: Vec<_> = osm.iter().collect();

        assert_eq!(features.len(), 8);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(275452090),
                    face: 0.into(),
                    properties: Map::from([
                        ("amenity".into(), "cafe".into()),
                        ("name".into(), "Jam's Sandwich Bar".into()),
                    ]),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10761860000000001,
                            y: 51.5075933,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10761860000000001,
                            bottom: 51.5075933,
                            right: -0.10761860000000001,
                            top: 51.5075933,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -2,
                            time_stamp: Some(1256818475000),
                            changeset: Some(2540257),
                            uid: Some(1697),
                            user_sid: Some("service".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994979),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10833480000000001,
                            y: 51.507406,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10833480000000001,
                            top: 51.507406,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: 2,
                            time_stamp: Some(1250040812000),
                            changeset: Some(1739860),
                            uid: Some(38244),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994980),
                    face: 0.into(),
                    properties: Map::from([("barrier".into(), "gate".into())]),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.1075735,
                            y: 51.507464500000005,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.1075735,
                            bottom: 51.507464500000005,
                            right: -0.1075735,
                            top: 51.507464500000005,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: 1,
                            time_stamp: Some(1234485707000),
                            changeset: Some(-2591627),
                            uid: Some(3516),
                            user_sid: Some("private".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(304994981),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10750140000000001,
                            y: 51.5074723,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10750140000000001,
                            bottom: 51.5074723,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1224174957000),
                            changeset: Some(-14817),
                            uid: Some(70),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(319408586),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.1080108,
                            y: 51.5074089,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.1080108,
                            bottom: 51.5074089,
                            right: -0.1080108,
                            top: 51.5074089,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1229476722000),
                            changeset: Some(440330),
                            uid: Some(6871),
                            user_sid: Some("name".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(319408587),
                    face: 0.into(),
                    properties: Map::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -0.10812640000000001,
                            y: 51.5074343,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10812640000000001,
                            bottom: 51.5074343,
                            right: -0.10812640000000001,
                            top: 51.5074343,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Node,
                        info: Some(InfoBlock {
                            version: -1,
                            time_stamp: Some(1229476722000),
                            changeset: Some(0),
                            uid: Some(6871),
                            user_sid: Some("".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(27776903),
                    face: 0.into(),
                    properties: Map::from([
                        ("access".into(), "private".into()),
                        ("highway".into(), "service".into()),
                        ("name".into(), "üßé€".into())
                    ]),
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.10833480000000001,
                                y: 51.507406,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10812640000000001,
                                y: 51.5074343,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint { x: -0.1080108, y: 51.5074089, z: None, m: None, t: None },
                            VectorPoint {
                                x: -0.1075735,
                                y: 51.507464500000005,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10750140000000001,
                                y: 51.5074723,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Way,
                        info: Some(InfoBlock {
                            version: -2,
                            time_stamp: Some(-621888578000),
                            changeset: Some(684276),
                            uid: Some(35),
                            user_sid: Some("Matt".into()),
                            visible: true
                        }),
                        nodes: None,
                        relation: None
                    })
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: Some(56688),
                    face: 0.into(),
                    properties: Map::from([
                        ("network".into(), "VVW".into()),
                        ("ref".into(), "123".into()),
                        ("route".into(), "bus".into()),
                        ("type".into(), "route".into())
                    ]),
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -0.10833480000000001,
                                y: 51.507406,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10812640000000001,
                                y: 51.5074343,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint { x: -0.1080108, y: 51.5074089, z: None, m: None, t: None },
                            VectorPoint {
                                x: -0.1075735,
                                y: 51.507464500000005,
                                z: None,
                                m: None,
                                t: None
                            },
                            VectorPoint {
                                x: -0.10750140000000001,
                                y: 51.5074723,
                                z: None,
                                m: None,
                                t: None
                            }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: -0.10833480000000001,
                            bottom: 51.507406,
                            right: -0.10750140000000001,
                            top: 51.5074723,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: Some(OSMMetadata {
                        osm_type: MemberType::Relation,
                        info: Some(InfoBlock {
                            version: 14,
                            time_stamp: Some(-647421115000),
                            changeset: Some(-3473819),
                            uid: Some(28095),
                            user_sid: Some("kmvar".into()),
                            visible: true
                        }),
                        nodes: Some(vec![IntermediateNodeMember {
                            role: "".into(),
                            node_id: 319408586
                        }]),
                        relation: None
                    })
                }
            ]
        );

        osm.cleanup();
    }
}
