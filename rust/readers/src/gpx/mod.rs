/// 1.1 Specification for the GPX format
pub mod spec;

use alloc::{string::String, vec::Vec};
use parsers::FeatureReader;
use s2json::{MValueCompatible, VectorFeature};
pub use spec::*;

/// Represents a route, which is an ordered list of waypoints leading to a destination.
#[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
pub struct GPXProperties {
    /// Route name
    pub name: Option<String>,
    /// Route comment
    pub cmt: Option<String>,
    /// Route description
    pub desc: Option<String>,
    /// Source of data
    pub src: Option<String>,
    /// Links to external information
    pub link: Option<Vec<GPXLink>>,
    /// Route number
    pub number: Option<usize>,
    /// Classification type of the route
    pub route_type: Option<String>,
    /// Classification type of the track
    pub track_type: Option<String>,
}

/// A GPX Shaped Vector Feature
pub type GPXVectorFeature = VectorFeature<(), GPXProperties, GPXWaypoint>;

/// # GPX Reader
///
/// ## Description
/// The GPX Reader is an XML-based GPS Exchange Format (GPX) reader.
///
/// GPX (the GPS Exchange Format) is a light-weight XML data format for the interchange of GPS data
/// (waypoints, routes, and tracks) between applications and Web services on the Internet.
///
/// ## Links
/// https://www.topografix.com/gpx.asp
#[derive(Debug)]
pub struct GPXReader {
    /// GPX object
    pub gpx: GPX,
}
impl GPXReader {
    /// Create a new GPX Reader
    pub fn new(input: &str) -> Self {
        Self { gpx: GPX::new(input) }
    }
    /// Grab the metadata
    pub fn metadata(&self) -> GPXMetadata {
        self.gpx.metadata.clone().unwrap_or_default()
    }
}
/// The GPX Iterator tool
#[derive(Debug)]
pub struct GPXIterator<'a> {
    reader: &'a GPXReader,
    wpt_offset: usize,
    wpt_count: usize,
    rte_offset: usize,
    rte_count: usize,
    trk_offset: usize,
    trk_count: usize,
}
impl Iterator for GPXIterator<'_> {
    type Item = GPXVectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        let gpx = &self.reader.gpx;
        if self.wpt_offset < self.wpt_count {
            self.wpt_offset += 1;
            return gpx.wpt.as_ref().map(|w| w[self.wpt_offset - 1].feature());
        }
        if self.rte_offset < self.rte_count {
            self.rte_offset += 1;
            return gpx.rte.as_ref().map(|w| w[self.rte_offset - 1].feature());
        }
        if self.trk_offset < self.trk_count {
            self.trk_offset += 1;
            return gpx.trk.as_ref().map(|w| w[self.trk_offset - 1].feature());
        }
        None
    }
}
/// A feature reader trait with a callback-based approach
impl FeatureReader<(), GPXProperties, GPXWaypoint> for GPXReader {
    type FeatureIterator<'a> = GPXIterator<'a>;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        GPXIterator {
            reader: self,
            wpt_offset: 0,
            wpt_count: self.gpx.wpt.as_ref().map(|w| w.len()).unwrap_or_default(),
            rte_offset: 0,
            rte_count: self.gpx.rte.as_ref().map(|r| r.len()).unwrap_or_default(),
            trk_offset: 0,
            trk_count: self.gpx.trk.as_ref().map(|t| t.len()).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;
    use alloc::vec;
    use s2json::{
        VectorBaseGeometry, VectorFeatureType, VectorGeometry, VectorGeometryType, VectorPoint,
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
