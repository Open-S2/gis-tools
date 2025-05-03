mod spec;

#[cfg(test)]
// #[coverage(off)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use parsers::FeatureReader;
    use readers::{GPXFixType, GPXMetadata, GPXProperties, GPXReader, GPXWaypoint};
    use s2json::{
        VectorBaseGeometry, VectorFeature, VectorFeatureType, VectorGeometry, VectorGeometryType,
        VectorPoint,
    };

    #[test]
    fn test_gpx_reader_iter() {
        let gpx_input = r#"
            <?xml version="1.0" encoding="UTF-8"?>
            <gpx version="1.1" creator="Test Creator">
                <wpt lat="37.7749" lon="-122.4194" fix="2d">
                    <name>Test Waypoint 1</name>
                </wpt>
                <rte>
                    <name>Test Route 1</name>
                    <rtept lat="37.7750" lon="-122.4195">
                        <name>Route Point 1</name>
                    </rtept>
                    <rtept lat="37.7751" lon="-122.4196">
                        <name>Route Point 2</name>
                    </rtept>
                </rte>
                <trk>
                    <name>Test Track 1</name>
                    <trkseg>
                        <trkpt lat="37.7752" lon="-122.4197">
                            <name>Track Point 1</name>
                        </trkpt>
                        <trkpt lat="37.7753" lon="-122.4198">
                            <name>Track Point 2</name>
                        </trkpt>
                    </trkseg>
                </trk>
            </gpx>
        "#;

        let reader = GPXReader::new(gpx_input);
        let features: Vec<_> = reader.iter().collect();

        let metadata = reader.metadata();
        assert_eq!(metadata, GPXMetadata::default());

        assert_eq!(features.len(), 3);

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    properties: GPXProperties::default(),
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: -122.4194,
                            y: 37.7749,
                            z: None,
                            m: Some(GPXWaypoint {
                                lat: 37.7749,
                                lon: -122.4194,
                                name: Some("Test Waypoint 1".into()),
                                fix: Some(GPXFixType::D2),
                                ..Default::default()
                            }),
                            t: None
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    properties: GPXProperties {
                        name: Some("Test Route 1".into()),
                        ..Default::default()
                    },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint {
                                x: -122.4195,
                                y: 37.775,
                                z: None,
                                m: Some(GPXWaypoint {
                                    lat: 37.775,
                                    lon: -122.4195,
                                    name: Some("Route Point 1".into()),
                                    ..Default::default()
                                }),
                                t: None
                            },
                            VectorPoint {
                                x: -122.4196,
                                y: 37.7751,
                                z: None,
                                m: Some(GPXWaypoint {
                                    lat: 37.7751,
                                    lon: -122.4196,
                                    name: Some("Route Point 2".into()),
                                    ..Default::default()
                                }),
                                t: None
                            }
                        ],
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    properties: GPXProperties {
                        name: Some("Test Track 1".into()),
                        ..Default::default()
                    },
                    geometry: VectorGeometry::MultiLineString(VectorBaseGeometry {
                        _type: VectorGeometryType::MultiLineString,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint {
                                x: -122.4197,
                                y: 37.7752,
                                z: None,
                                m: Some(GPXWaypoint {
                                    lat: 37.7752,
                                    lon: -122.4197,
                                    name: Some("Track Point 1".into()),
                                    ..Default::default()
                                }),
                                t: None
                            },
                            VectorPoint {
                                x: -122.4198,
                                y: 37.7753,
                                z: None,
                                m: Some(GPXWaypoint {
                                    lat: 37.7753,
                                    lon: -122.4198,
                                    name: Some("Track Point 2".into()),
                                    ..Default::default()
                                }),
                                t: None
                            }
                        ]],
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ]
        );
    }
}
