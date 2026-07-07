#[cfg(test)]
// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
mod tests {
    use gistools::{
        parsers::{BufferWriter, FeatureReader, Writer},
        readers::{JSONCollectionReader, shapefile_from_gzip},
        util::{WriteZipItem, zip_folder},
        writers::{to_dbf, to_dbf_meta, to_shp},
    };
    use s2json::{
        FeatureCollection, FeatureCollectionType, Features, MValue, Properties, VectorFeature,
        VectorGeometry, VectorLineStringGeometry, VectorMultiLineStringGeometry,
        VectorMultiPointGeometry, VectorMultiPolygonGeometry, VectorPoint, VectorPointGeometry,
        VectorPolygonGeometry,
    };
    use std::collections::BTreeMap;

    // --- Mock Iterator Helpers ---

    fn create_mock_iterator_point_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Point(VectorPointGeometry {
                    coordinates: VectorPoint { x: ind, y: ind, ..Default::default() },
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };

        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_point_z_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Point(VectorPointGeometry {
                    coordinates: VectorPoint { x: ind, y: ind, z: Some(ind), ..Default::default() },
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };

        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_point_zm_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Point(VectorPointGeometry {
                    coordinates: VectorPoint {
                        x: ind,
                        y: ind,
                        z: Some(ind),
                        m: Some(MValue::from([("value".into(), ind.into())])),
                        ..Default::default()
                    },
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };

        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_point_m_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Point(VectorPointGeometry {
                    coordinates: VectorPoint {
                        x: ind,
                        y: ind,
                        m: Some(MValue::from([("value".into(), ind.into())])),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };

        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multipoint_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                    coordinates: vec![
                        VectorPoint { x: ind, y: ind, ..Default::default() },
                        VectorPoint { x: ind + 1.0, y: ind + 1.0, ..Default::default() },
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multipoint_z_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                    coordinates: vec![
                        VectorPoint { x: ind, y: ind, z: Some(ind), ..Default::default() },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            z: Some(ind + 1.0),
                            ..Default::default()
                        },
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multipoint_zm_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                    coordinates: vec![
                        VectorPoint {
                            x: ind,
                            y: ind,
                            z: Some(ind),
                            m: Some(MValue::from([("value".into(), ind.into())])),
                            ..Default::default()
                        },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            z: Some(ind + 1.0),
                            m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                            ..Default::default()
                        },
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multipoint_m_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                    coordinates: vec![
                        VectorPoint {
                            x: ind,
                            y: ind,
                            m: Some(MValue::from([("value".into(), ind.into())])),
                            ..Default::default()
                        },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_linestring_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::LineString(VectorLineStringGeometry {
                    coordinates: vec![
                        VectorPoint { x: ind, y: ind, ..Default::default() },
                        VectorPoint { x: ind + 1.0, y: ind + 1.0, ..Default::default() },
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_linestring_z_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::LineString(VectorLineStringGeometry {
                    coordinates: vec![
                        VectorPoint { x: ind, y: ind, z: Some(ind), ..Default::default() },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            z: Some(ind + 1.0),
                            ..Default::default()
                        },
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_linestring_zm_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::LineString(VectorLineStringGeometry {
                    coordinates: vec![
                        VectorPoint {
                            x: ind,
                            y: ind,
                            z: Some(ind),
                            m: Some(MValue::from([("value".into(), ind.into())])),
                            ..Default::default()
                        },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            z: Some(ind + 1.0),
                            m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                            ..Default::default()
                        },
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_linestring_m_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::LineString(VectorLineStringGeometry {
                    coordinates: vec![
                        VectorPoint {
                            x: ind,
                            y: ind,
                            m: Some(MValue::from([("value".into(), ind.into())])),
                            ..Default::default()
                        },
                        VectorPoint {
                            x: ind + 1.0,
                            y: ind + 1.0,
                            m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                            ..Default::default()
                        },
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multilinestring_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint { x: ind, y: ind, ..Default::default() },
                            VectorPoint { x: ind + 1.0, y: ind + 1.0, ..Default::default() },
                        ],
                        vec![
                            VectorPoint { x: ind + 2.0, y: ind + 2.0, ..Default::default() },
                            VectorPoint { x: ind + 3.0, y: ind + 3.0, ..Default::default() },
                        ],
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multilinestring_z_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint { x: ind, y: ind, z: Some(ind), ..Default::default() },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                z: Some(ind + 1.0),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                z: Some(ind + 2.0),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                z: Some(ind + 3.0),
                                ..Default::default()
                            },
                        ],
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multilinestring_zm_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint {
                                x: ind,
                                y: ind,
                                z: Some(ind),
                                m: Some(MValue::from([("value".into(), ind.into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                z: Some(ind + 1.0),
                                m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                z: Some(ind + 2.0),
                                m: Some(MValue::from([("value".into(), (ind + 2.0).into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                z: Some(ind + 3.0),
                                m: Some(MValue::from([("value".into(), (ind + 3.0).into())])),
                                ..Default::default()
                            },
                        ],
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multilinestring_m_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint {
                                x: ind,
                                y: ind,
                                m: Some(MValue::from([("value".into(), ind.into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                m: Some(MValue::from([("value".into(), (ind + 2.0).into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                m: Some(MValue::from([("value".into(), (ind + 3.0).into())])),
                                ..Default::default()
                            },
                        ],
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_polygon_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Polygon(VectorPolygonGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint { x: ind, y: ind, ..Default::default() },
                            VectorPoint { x: ind + 1.0, y: ind + 1.0, ..Default::default() },
                        ],
                        vec![
                            VectorPoint { x: ind + 2.0, y: ind + 2.0, ..Default::default() },
                            VectorPoint { x: ind + 3.0, y: ind + 3.0, ..Default::default() },
                        ],
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_polygon_z_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Polygon(VectorPolygonGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint { x: ind, y: ind, z: Some(ind), ..Default::default() },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                z: Some(ind + 1.0),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                z: Some(ind + 2.0),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                z: Some(ind + 3.0),
                                ..Default::default()
                            },
                        ],
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_polygon_zm_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Polygon(VectorPolygonGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint {
                                x: ind,
                                y: ind,
                                z: Some(ind),
                                m: Some(MValue::from([("value".into(), ind.into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                z: Some(ind + 1.0),
                                m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                z: Some(ind + 2.0),
                                m: Some(MValue::from([("value".into(), (ind + 2.0).into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                z: Some(ind + 3.0),
                                m: Some(MValue::from([("value".into(), (ind + 3.0).into())])),
                                ..Default::default()
                            },
                        ],
                    ],
                    is_3d: true,
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_polygon_m_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::Polygon(VectorPolygonGeometry {
                    coordinates: vec![
                        vec![
                            VectorPoint {
                                x: ind,
                                y: ind,
                                m: Some(MValue::from([("value".into(), ind.into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 1.0,
                                y: ind + 1.0,
                                m: Some(MValue::from([("value".into(), (ind + 1.0).into())])),
                                ..Default::default()
                            },
                        ],
                        vec![
                            VectorPoint {
                                x: ind + 2.0,
                                y: ind + 2.0,
                                m: Some(MValue::from([("value".into(), (ind + 2.0).into())])),
                                ..Default::default()
                            },
                            VectorPoint {
                                x: ind + 3.0,
                                y: ind + 3.0,
                                m: Some(MValue::from([("value".into(), (ind + 3.0).into())])),
                                ..Default::default()
                            },
                        ],
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    fn create_mock_iterator_multipolygon_shp(
        properties_array: Vec<Properties>,
    ) -> JSONCollectionReader<(), Properties, MValue> {
        let mut features: Vec<VectorFeature<(), Properties, MValue>> = vec![];
        let mut ind = 0.0;
        for props in properties_array {
            features.push(VectorFeature::new_wm(
                None,
                props,
                VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                    coordinates: vec![
                        vec![
                            vec![
                                VectorPoint { x: ind, y: ind, ..Default::default() },
                                VectorPoint { x: ind + 1.0, y: ind + 1.0, ..Default::default() },
                            ],
                            vec![
                                VectorPoint { x: ind + 2.0, y: ind + 2.0, ..Default::default() },
                                VectorPoint { x: ind + 3.0, y: ind + 3.0, ..Default::default() },
                            ],
                        ],
                        vec![
                            vec![
                                VectorPoint { x: ind + 4.0, y: ind + 4.0, ..Default::default() },
                                VectorPoint { x: ind + 5.0, y: ind + 5.0, ..Default::default() },
                            ],
                            vec![
                                VectorPoint { x: ind + 6.0, y: ind + 6.0, ..Default::default() },
                                VectorPoint { x: ind + 7.0, y: ind + 7.0, ..Default::default() },
                            ],
                        ],
                    ],
                    ..Default::default()
                }),
                None,
            ));
            ind += 1.0;
        }
        let mut feature_collection: FeatureCollection<(), Properties, MValue> = FeatureCollection {
            _type: FeatureCollectionType::FeatureCollection,
            features: features.into_iter().map(Features::VectorFeature).collect(),
            bbox: None,
            attributions: None,
        };
        JSONCollectionReader::from(&mut feature_collection)
    }

    // --- DBF Writer Tests ---

    #[test]
    fn test_to_dbf_meta_tracks_exact_feature_counts_and_schema_fields() {
        let mock1 = create_mock_iterator_point_shp(vec![
            Properties::from([
                ("name".into(), "Raleigh".into()),
                ("elevation".into(), (110.5).into()),
            ]),
            Properties::from([
                ("name".into(), "Durham".into()),
                ("elevation".into(), (121).into()),
            ]),
        ]);
        let mock2 = create_mock_iterator_point_shp(vec![Properties::from([
            ("name".into(), "Charlotte".into()),
            ("population".into(), (870000).into()),
        ])]);

        let (meta, feature_count) = to_dbf_meta(&[&mock1, &mock2]);

        assert_eq!(feature_count, 3);
        assert_eq!(meta.len(), 3);

        let name_field = meta.iter().find(|f| f.name == "name");
        assert_eq!(name_field.unwrap().data_type, 'C');
    }

    #[test]
    fn test_to_dbf_meta_computes_numeric_decimal_precision_up_to_15_decimals_maximum() {
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("coords".into(), (35.7796).into())]),
            Properties::from([("coords".into(), (-78.638212345678912).into())]),
            Properties::from([("coords".into(), (12.1).into())]),
        ]);

        let (meta, _feature_count) = to_dbf_meta(&[&mock]);
        let coords_field = meta.iter().find(|f| f.name == "coords");
        assert!(coords_field.is_some());
        assert_eq!(coords_field.unwrap().data_type, 'N');
        assert_eq!(coords_field.unwrap().decimal, 15);
        assert!(coords_field.unwrap().len <= 18);
    }

    #[test]
    fn tests_to_dbf_meta_forces_type_widening_up_to_string_characters_if_data_clashes() {
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("mixed".into(), (true).into())]),
            Properties::from([("mixed".into(), (42.12).into())]),
            Properties::from([("mixed".into(), ("Fallback String Content").into())]),
        ]);

        let (meta, _feature_count) = to_dbf_meta(&[&mock]);
        let mixed_field = meta.iter().find(|f| f.name == "mixed");
        assert!(mixed_field.is_some());
        assert_eq!(mixed_field.unwrap().data_type, 'C');
    }

    #[test]
    fn test_to_dbf_streams_complete_structural_byte_data_without_leaking_previous_row_buffers() {
        let mut writer = BufferWriter::new(vec![]);
        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        to_dbf(&mut writer, vec![&mock]);
        let final_bytes = writer.take();

        assert_eq!(final_bytes.last(), Some(&0x1a));

        let raw_string_output = String::from_utf8_lossy(&final_bytes);
        assert!(raw_string_output.contains("Asheville "));
        assert!(raw_string_output.contains("Durham    "));
        assert!(!raw_string_output.contains("Durhamville"));
    }

    #[test]
    fn test_to_dbf_serializes_logical_boolean_switches_and_strict_iso_date_strings() {
        let mut writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([
                ("active".into(), true.into()),
                ("updated".into(), "2026-06-05".into()),
            ]),
            Properties::from([
                ("active".into(), false.into()),
                ("updated".into(), "2026-12-25".into()),
            ]),
        ]);

        to_dbf(&mut writer, vec![&mock]);
        let final_bytes = writer.take();

        let raw_string_output = String::from_utf8_lossy(&final_bytes);
        assert!(raw_string_output.contains("T") || raw_string_output.contains("C"));
        assert!(raw_string_output.contains("2026-06-05"));
        assert!(raw_string_output.contains("2026-12-25"));
    }

    // --- to_shp Geometry Tests ---

    #[test]
    fn test_to_shp_base_case_point() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);
        let mut shx_writer = BufferWriter::new(vec![]);
        let mut prj_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            Some(&mut shx_writer),
            Some(&mut prj_writer),
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();
        let shx_bytes = shx_writer.take();
        let prj_bytes = prj_writer.take();

        assert_eq!(shp_bytes.len(), 156);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert_eq!(
            features[0].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Asheville"
        );
        assert_eq!(
            features[1].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Durham"
        );

        let first = features[0].geometry.point().unwrap();
        assert_eq!(first.x, 0.);
        assert_eq!(first.y, 0.);

        // next test the shx writer
        assert_eq!(shx_bytes.len(), 116);

        // next test the prj writer
        assert_eq!(prj_bytes.len(), 331);
        let prj_string = String::from_utf8_lossy(&prj_bytes);
        assert_eq!(
            prj_string,
            "GEOGCS[\"WGS 84\",
  DATUM[\"WGS_1984\",
    SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],
    AUTHORITY[\"EPSG\",\"6326\"]],
  PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],
  UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],
  AXIS[\"Latitude\",NORTH],
  AXIS[\"Longitude\",EAST],
  AUTHORITY[\"EPSG\",\"4326\"]]
"
        );
    }

    #[test]
    fn test_to_shp_base_case_point_z() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_z_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
            // Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 172);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert_eq!(
            features[0].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Asheville"
        );
        assert_eq!(
            features[1].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Durham"
        );

        let first = features[0].geometry.point().unwrap();
        assert_eq!(first.x, 0.);
        assert_eq!(first.y, 0.);
        assert_eq!(first.z, Some(0.));
        let second = features[1].geometry.point().unwrap();
        assert_eq!(second.x, 1.);
        assert_eq!(second.y, 1.);
        assert_eq!(second.z, Some(1.));
    }

    #[test]
    fn test_to_shp_base_case_point_zm() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_zm_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 188);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert_eq!(
            features[0].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Asheville"
        );
        assert_eq!(
            features[1].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Durham"
        );

        let first = features[0].geometry.point().unwrap();
        assert_eq!(first.x, 0.);
        assert_eq!(first.y, 0.);
        assert_eq!(first.z, Some(0.));
        assert_eq!(first.m, Some(MValue::from([("value".into(), 0.0_f64.into())])));
        let second = features[1].geometry.point().unwrap();
        assert_eq!(second.x, 1.);
        assert_eq!(second.y, 1.);
        assert_eq!(second.z, Some(1.));
        assert_eq!(second.m, Some(MValue::from([("value".into(), 1.0_f64.into())])));
    }

    #[test]
    fn test_to_shp_base_case_point_m() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_m_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 172);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert_eq!(
            features[0].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Asheville"
        );
        assert_eq!(
            features[1].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Durham"
        );

        let first = features[0].geometry.point().unwrap();
        assert_eq!(first.x, 0.);
        assert_eq!(first.y, 0.);
        assert_eq!(first.z, None);
        assert_eq!(first.m, Some(MValue::from([("value".into(), 0.0_f64.into())])));
        let second = features[1].geometry.point().unwrap();
        assert_eq!(second.x, 1.);
        assert_eq!(second.y, 1.);
        assert_eq!(second.z, None);
        assert_eq!(second.m, Some(MValue::from([("value".into(), 1.0_f64.into())])));
    }

    #[test]
    fn test_to_shp_base_case_larger() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_point_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
            Properties::from([("name".into(), "Durham".into())]),
            Properties::from([("name".into(), "Durham".into())]),
            Properties::from([("name".into(), "Test5".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 240);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 5);
        assert_eq!(
            features[4].properties.get("name").unwrap().to_prim().unwrap().to_string().unwrap(),
            "Test5"
        );
    }

    #[test]
    fn test_to_shp_base_case_multipoint() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multipoint_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 260);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.multipoint().is_some());
    }

    #[test]
    fn test_to_shp_base_case_multipoint_z() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multipoint_z_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 324);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.multipoint().is_some());

        let first = features[0].geometry.multipoint().unwrap();
        assert_eq!(
            *first,
            vec![
                VectorPoint::new(0.0, 0.0, Some(0.0), None),
                VectorPoint::new(1.0, 1.0, Some(1.0), None)
            ]
        );
        let second = features[1].geometry.multipoint().unwrap();
        assert_eq!(
            *second,
            vec![
                VectorPoint::new(1.0, 1.0, Some(1.0), None),
                VectorPoint::new(2.0, 2.0, Some(2.0), None)
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multipoint_zm() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multipoint_zm_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 388);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.multipoint().is_some());

        let first = features[0].geometry.multipoint().unwrap();
        assert_eq!(
            *first,
            vec![
                VectorPoint::new(0.0, 0.0, Some(0.0), None),
                VectorPoint::new(1.0, 1.0, Some(1.0), None)
            ]
        );
        let second = features[1].geometry.multipoint().unwrap();
        assert_eq!(
            *second,
            vec![
                VectorPoint::new(1.0, 1.0, Some(1.0), None),
                VectorPoint::new(2.0, 2.0, Some(2.0), None)
            ]
        );

        // first m-value map
        let first_ms = first.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                Some(MValue::from([("value".into(), 0.0_f64.into())])),
                Some(MValue::from([("value".into(), 1.0_f64.into())]))
            ]
        );
        // second m-value map
        let second_ms = second.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            second_ms,
            vec![
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
                Some(MValue::from([("value".into(), 2.0_f64.into())]))
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multipoint_m() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multipoint_m_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 324);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.multipoint().is_some());

        let first = features[0].geometry.multipoint().unwrap();
        assert_eq!(
            *first,
            vec![VectorPoint::new(0.0, 0.0, None, None), VectorPoint::new(1.0, 1.0, None, None)]
        );
        let second = features[1].geometry.multipoint().unwrap();
        assert_eq!(
            *second,
            vec![VectorPoint::new(1.0, 1.0, None, None), VectorPoint::new(2.0, 2.0, None, None)]
        );

        // first m-value map
        let first_ms = first.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                Some(MValue::from([("value".into(), 0.0_f64.into())])),
                Some(MValue::from([("value".into(), 1.0_f64.into())]))
            ]
        );
        // second m-value map
        let second_ms = second.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            second_ms,
            vec![
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
                Some(MValue::from([("value".into(), 2.0_f64.into())]))
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_linestring() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_linestring_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 276);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.linestring().is_some());
    }

    #[test]
    fn test_to_shp_base_case_linestring_z() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_linestring_z_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 404);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.linestring().unwrap();
        assert_eq!(
            *first,
            vec![
                VectorPoint::new(0.0, 0.0, Some(0.0), None),
                VectorPoint::new(1.0, 1.0, Some(1.0), None)
            ]
        );
        let second = features[1].geometry.linestring().unwrap();
        assert_eq!(
            *second,
            vec![
                VectorPoint::new(1.0, 1.0, Some(1.0), None),
                VectorPoint::new(2.0, 2.0, Some(2.0), None)
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_linestring_zm() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_linestring_zm_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 468);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.linestring().unwrap();
        assert_eq!(
            *first,
            vec![
                VectorPoint::new(0.0, 0.0, Some(0.0), None),
                VectorPoint::new(1.0, 1.0, Some(1.0), None)
            ]
        );
        let second = features[1].geometry.linestring().unwrap();
        assert_eq!(
            *second,
            vec![
                VectorPoint::new(1.0, 1.0, Some(1.0), None),
                VectorPoint::new(2.0, 2.0, Some(2.0), None)
            ]
        );

        let first_ms = first.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                Some(MValue::from([("value".into(), 0.0_f64.into())])),
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
            ]
        );
        let second_ms = second.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            second_ms,
            vec![
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
                Some(MValue::from([("value".into(), 2.0_f64.into())])),
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_linestring_m() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_linestring_m_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 340);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.linestring().unwrap();
        assert_eq!(
            *first,
            vec![VectorPoint::new(0.0, 0.0, None, None), VectorPoint::new(1.0, 1.0, None, None)]
        );
        let second = features[1].geometry.linestring().unwrap();
        assert_eq!(
            *second,
            vec![VectorPoint::new(1.0, 1.0, None, None), VectorPoint::new(2.0, 2.0, None, None)]
        );

        let first_ms = first.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                Some(MValue::from([("value".into(), 0.0_f64.into())])),
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
            ]
        );
        let second_ms = second.iter().map(|p| p.m.clone()).collect::<Vec<_>>();
        assert_eq!(
            second_ms,
            vec![
                Some(MValue::from([("value".into(), 1.0_f64.into())])),
                Some(MValue::from([("value".into(), 2.0_f64.into())])),
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multilinestring() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multilinestring_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 348);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
    }

    #[test]
    fn test_to_shp_base_case_multilinestring_z() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multilinestring_z_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 540);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.multilinestring().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, Some(0.0), None),
                    VectorPoint::new(1.0, 1.0, Some(1.0), None)
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                    VectorPoint::new(3.0, 3.0, Some(3.0), None)
                ]
            ]
        );
        let second = features[1].geometry.multilinestring().unwrap();
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                    VectorPoint::new(2.0, 2.0, Some(2.0), None)
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                    VectorPoint::new(4.0, 4.0, Some(4.0), None)
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multilinestring_zm() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multilinestring_zm_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 636);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.multilinestring().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, Some(0.0), None),
                    VectorPoint::new(1.0, 1.0, Some(1.0), None)
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                    VectorPoint::new(3.0, 3.0, Some(3.0), None)
                ]
            ]
        );
        let second = features[1].geometry.multilinestring().unwrap();
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                    VectorPoint::new(2.0, 2.0, Some(2.0), None)
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                    VectorPoint::new(4.0, 4.0, Some(4.0), None)
                ]
            ]
        );

        let first_ms = first
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let second_ms = second
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 0.0_f64.into())])),
                    Some(MValue::from([("value".into(), 1.0_f64.into())]))
                ],
                vec![
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                    Some(MValue::from([("value".into(), 3.0_f64.into())]))
                ]
            ]
        );
        assert_eq!(
            second_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                    Some(MValue::from([("value".into(), 2.0_f64.into())]))
                ],
                vec![
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                    Some(MValue::from([("value".into(), 4.0_f64.into())]))
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multilinestring_m() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multilinestring_m_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 444);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.multilinestring().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, None, None),
                    VectorPoint::new(1.0, 1.0, None, None)
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, None, None),
                    VectorPoint::new(3.0, 3.0, None, None)
                ]
            ]
        );
        let second = features[1].geometry.multilinestring().unwrap();
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, None, None),
                    VectorPoint::new(2.0, 2.0, None, None)
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, None, None),
                    VectorPoint::new(4.0, 4.0, None, None)
                ]
            ]
        );

        let first_ms = first
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let second_ms = second
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 0.0_f64.into())])),
                    Some(MValue::from([("value".into(), 1.0_f64.into())]))
                ],
                vec![
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                    Some(MValue::from([("value".into(), 3.0_f64.into())]))
                ]
            ]
        );
        assert_eq!(
            second_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                    Some(MValue::from([("value".into(), 2.0_f64.into())]))
                ],
                vec![
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                    Some(MValue::from([("value".into(), 4.0_f64.into())]))
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_polygon() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_polygon_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 348);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);
        assert!(features[0].geometry.polygon().is_some());
    }

    #[test]
    fn test_to_shp_base_case_polygon_z() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_polygon_z_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 540);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.polygon().unwrap();
        let second = features[1].geometry.polygon().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, Some(0.0), None),
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                ]
            ]
        );
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                    VectorPoint::new(4.0, 4.0, Some(4.0), None),
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_polygon_zm() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_polygon_zm_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 636);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.polygon().unwrap();
        let second = features[1].geometry.polygon().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, Some(0.0), None),
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                ]
            ]
        );
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, Some(1.0), None),
                    VectorPoint::new(2.0, 2.0, Some(2.0), None),
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, Some(3.0), None),
                    VectorPoint::new(4.0, 4.0, Some(4.0), None),
                ]
            ]
        );

        let first_ms = first
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let second_ms = second
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 0.0_f64.into())])),
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                ],
                vec![
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                ]
            ]
        );
        assert_eq!(
            second_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                ],
                vec![
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                    Some(MValue::from([("value".into(), 4.0_f64.into())])),
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_polygon_m() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_polygon_m_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            Some(|m| m?.get("value")?.to_prim()?.to_f64()),
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 444);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        assert_eq!(features.len(), 2);

        let first = features[0].geometry.polygon().unwrap();
        let second = features[1].geometry.polygon().unwrap();
        assert_eq!(
            *first,
            vec![
                vec![
                    VectorPoint::new(0.0, 0.0, None, None),
                    VectorPoint::new(1.0, 1.0, None, None),
                ],
                vec![
                    VectorPoint::new(2.0, 2.0, None, None),
                    VectorPoint::new(3.0, 3.0, None, None),
                ]
            ]
        );
        assert_eq!(
            *second,
            vec![
                vec![
                    VectorPoint::new(1.0, 1.0, None, None),
                    VectorPoint::new(2.0, 2.0, None, None),
                ],
                vec![
                    VectorPoint::new(3.0, 3.0, None, None),
                    VectorPoint::new(4.0, 4.0, None, None),
                ]
            ]
        );

        let first_ms = first
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let second_ms = second
            .iter()
            .map(|v| v.iter().map(|v| v.m.clone()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert_eq!(
            first_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 0.0_f64.into())])),
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                ],
                vec![
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                ]
            ]
        );
        assert_eq!(
            second_ms,
            vec![
                vec![
                    Some(MValue::from([("value".into(), 1.0_f64.into())])),
                    Some(MValue::from([("value".into(), 2.0_f64.into())])),
                ],
                vec![
                    Some(MValue::from([("value".into(), 3.0_f64.into())])),
                    Some(MValue::from([("value".into(), 4.0_f64.into())])),
                ]
            ]
        );
    }

    #[test]
    fn test_to_shp_base_case_multipolygon() {
        let mut shp_writer = BufferWriter::new(vec![]);
        let mut dbf_writer = BufferWriter::new(vec![]);

        let mock = create_mock_iterator_multipolygon_shp(vec![
            Properties::from([("name".into(), "Asheville".into())]),
            Properties::from([("name".into(), "Durham".into())]),
        ]);

        let shx_writer: Option<&mut BufferWriter> = None;
        let prj_writer: Option<&mut BufferWriter> = None;

        to_shp(
            &mut shp_writer,
            vec![&mock],
            Some(&mut dbf_writer),
            shx_writer,
            prj_writer,
            None,
            None,
        );

        let shp_bytes = shp_writer.take();
        let dbf_bytes = dbf_writer.take();

        assert_eq!(shp_bytes.len(), 596);

        let zipped_data = zip_folder(vec![
            WriteZipItem {
                filename: "points.shp".into(),
                comment: Some("shapefile data".into()),
                bytes: shp_bytes,
            },
            WriteZipItem {
                filename: "points.dbf".into(),
                comment: Some("properties data".into()),
                bytes: dbf_bytes,
            },
        ])
        .unwrap();

        let reader = shapefile_from_gzip::<Properties>(&zipped_data, BTreeMap::default());
        let features: Vec<_> = reader.iter().collect();
        // Matching TS expectation: multipolygons can explode out to separate polygon segments
        assert_eq!(features.len(), 4);
    }

    // just writing this to setup docs
    // #[test]
    // fn test_shp_write_test() {
    //     // read in data
    //     #[derive(Debug, Default, Clone, MValueCompatible, PartialEq, Serialize, Deserialize)]
    //     #[serde(default)]
    //     struct Props {
    //         name: String,
    //     }
    //     let cargo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     let path = cargo_path.join("tests/writers/fixtures/points.geojson");
    //     let reader: JSONReader<FileReader, (), Props, MValue> =
    //         JSONReader::new(FileReader::from(path));

    //     // setup writers
    //     let shp_path = cargo_path.join("tests/writers/fixtures/points.shp");
    //     let mut shp_writer = FileWriter::new(shp_path).unwrap();
    //     let dbf_path = cargo_path.join("tests/writers/fixtures/points.dbf");
    //     let mut dbf_writer = FileWriter::new(dbf_path).unwrap();
    //     let shx_path = cargo_path.join("tests/writers/fixtures/points.shx");
    //     let mut shx_writer = FileWriter::new(shx_path).unwrap();
    //     let prj_path = cargo_path.join("tests/writers/fixtures/points.prj");
    //     let mut prj_writer = FileWriter::new(prj_path).unwrap();

    //     // write to files
    //     to_shp(
    //         &mut shp_writer,
    //         vec![&reader],
    //         Some(&mut dbf_writer),
    //         Some(&mut shx_writer),
    //         Some(&mut prj_writer),
    //         None,
    //         None,
    //     );

    //     // const shpFile = await Bun.file(`${__dirname}/fixtures/points.shp`).arrayBuffer();
    //     // const dbfFile = await Bun.file(`${__dirname}/fixtures/points.dbf`).arrayBuffer();
    //     // const shxFile = await Bun.file(`${__dirname}/fixtures/points.shx`).arrayBuffer();
    //     // const prjFile = await Bun.file(`${__dirname}/fixtures/points.prj`).arrayBuffer();
    //     //
    //     // const zippedData = await zipFolder([
    //     //   { name: 'points.shp', comment: 'shapefile data', data: shpFile },
    //     //   { name: 'points.dbf', comment: 'properties data', data: dbfFile },
    //     //   { name: 'points.shx', comment: 'index data', data: shxFile },
    //     //   { name: 'points.prj', comment: 'projection', data: prjFile },
    //     // ]);

    //     let cargo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     let shp_path = cargo_path.join("tests/writers/fixtures/points.shp");
    //     let dbf_path = cargo_path.join("tests/writers/fixtures/points.dbf");
    //     let shx_path = cargo_path.join("tests/writers/fixtures/points.shx");
    //     let prj_path = cargo_path.join("tests/writers/fixtures/points.prj");
    //     let shp_file = std::fs::read(shp_path).unwrap();
    //     let dbf_file = std::fs::read(dbf_path).unwrap();
    //     let shx_file = std::fs::read(shx_path).unwrap();
    //     let prj_file = std::fs::read(prj_path).unwrap();

    //     let zipped_data = zip_folder(vec![
    //         WriteZipItem {
    //             filename: "points.shp".into(),
    //             comment: Some("shapefile data".into()),
    //             bytes: shp_file,
    //         },
    //         WriteZipItem {
    //             filename: "points.dbf".into(),
    //             comment: Some("properties data".into()),
    //             bytes: dbf_file,
    //         },
    //         WriteZipItem {
    //             filename: "points.shx".into(),
    //             comment: Some("index data".into()),
    //             bytes: shx_file,
    //         },
    //         WriteZipItem {
    //             filename: "points.prj".into(),
    //             comment: Some("projection".into()),
    //             bytes: prj_file,
    //         },
    //     ])
    //     .unwrap();
    // }
}
