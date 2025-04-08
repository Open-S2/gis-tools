use super::ToGisJSON;
use crate::{
    geometry::ConvertFeature,
    readers::{FeatureReader, Reader},
};
use alloc::{
    collections::VecDeque,
    string::{String, ToString},
};
use core::{cell::RefCell, marker::PhantomData};
use s2json::{Features, MValue, VectorFeature};
use serde::de::DeserializeOwned;

#[derive(Debug)]
struct NewLineDelimitedJSONParser {
    offset: u64,
    tmp_chunks: VecDeque<String>,
    partial_line: String,
}

/// # NewLine Delimited JSON Reader
#[derive(Debug)]
pub struct NewLineDelimitedJSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: Clone + Default + DeserializeOwned = MValue,
    D: Clone + Default + DeserializeOwned = MValue,
> {
    reader: T,
    seperator: char, // default is '\n'
    parser: RefCell<NewLineDelimitedJSONParser>,
    _phantom: PhantomData<VectorFeature<M, P, D>>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > NewLineDelimitedJSONReader<T, M, P, D>
{
    /// Create a Newline-Delimited JSON Reader
    pub fn new(reader: T, seperator: Option<char>) -> NewLineDelimitedJSONReader<T, M, P, D> {
        NewLineDelimitedJSONReader {
            reader,
            _phantom: PhantomData,
            seperator: seperator.unwrap_or('\n'),
            parser: RefCell::new(NewLineDelimitedJSONParser {
                offset: 0,
                tmp_chunks: VecDeque::new(),
                partial_line: String::new(),
            }),
        }
    }

    /// Get the next feature
    pub fn next_feature(&self) -> Option<VectorFeature<M, P, D>> {
        let mut parser = self.parser.borrow_mut();
        // 1) Serve from buffer if available
        if let Some(line) = parser.tmp_chunks.pop_front() {
            return self.parse_line(&line);
        }

        // 2) Refill buffer from reader
        if parser.offset < self.reader.len() {
            let length = u64::min(65_536, self.reader.len() - parser.offset);
            let chunk = self.reader.parse_string(Some(parser.offset), Some(length));
            // Prepend any leftover partial line
            let combined = core::mem::take(&mut parser.partial_line) + &chunk;
            // Split on separator (e.g. '\n') into complete lines
            let mut parts: VecDeque<String> = combined
                .split(self.seperator)
                .map(str::to_string)
                .filter(|s| !s.is_empty())
                .collect();
            // Handle trailing separator
            parser.partial_line = if combined.ends_with(self.seperator) {
                String::new()
            } else {
                parts.pop_back().unwrap_or_default()
            };

            parser.tmp_chunks = parts;
            parser.offset += length;

            return parser.tmp_chunks.pop_front().and_then(|line| self.parse_line(&line));
        }

        // 3) Final cleanup: parse trailing partial line if any
        if !parser.partial_line.is_empty() {
            let line = std::mem::take(&mut parser.partial_line);
            let feature = self.parse_line(&line);
            parser.partial_line.clear();
            return feature;
        }

        None
    }

    fn parse_line(&self, line: &str) -> Option<VectorFeature<M, P, D>> {
        if line.len() > 1 {
            if let Ok(feature) = line.to_features() {
                match feature {
                    Features::Feature(feature) => {
                        return Some(feature.to_vector(Some(true)));
                    }
                    Features::VectorFeature(vf) => {
                        return Some(vf);
                    }
                }
            }
        }
        None
    }
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > Iterator for NewLineDelimitedJSONReader<T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_feature()
    }
}
/// The Newline Delimited JSON Iterator tool
pub struct NewLineDelimitedJSONIterator<
    'a,
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> {
    reader: &'a NewLineDelimitedJSONReader<T, M, P, D>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > Iterator for NewLineDelimitedJSONIterator<'_, T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next_feature()
    }
}
/// A feature reader trait with a callback-based approach
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > FeatureReader<M, P, D> for NewLineDelimitedJSONReader<T, M, P, D>
{
    type FeatureIterator<'a>
        = NewLineDelimitedJSONIterator<'a, T, M, P, D>
    where
        T: 'a,
        M: 'a,
        P: 'a,
        D: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        NewLineDelimitedJSONIterator { reader: self }
    }
}

/// # Text Sequence JSON Reader
pub struct SequenceJSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: Clone + Default + DeserializeOwned = MValue,
    D: Clone + Default + DeserializeOwned = MValue,
> {
    newline: NewLineDelimitedJSONReader<T, M, P, D>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > SequenceJSONReader<T, M, P, D>
{
    /// Create a new SequenceJSONReader
    pub fn new(reader: T) -> SequenceJSONReader<T, M, P, D> {
        SequenceJSONReader { newline: NewLineDelimitedJSONReader::new(reader, Some('␞')) }
    }
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > Iterator for SequenceJSONReader<T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.newline.next()
    }
}
/// The  Delimited JSON Iterator tool
pub struct SequenceJSONIterator<
    'a,
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> {
    reader: &'a SequenceJSONReader<T, M, P, D>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > Iterator for SequenceJSONIterator<'_, T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.newline.next_feature()
    }
}
/// A feature reader trait with a callback-based approach
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: Clone + Default + DeserializeOwned,
        D: Clone + Default + DeserializeOwned,
    > FeatureReader<M, P, D> for SequenceJSONReader<T, M, P, D>
{
    type FeatureIterator<'a>
        = SequenceJSONIterator<'a, T, M, P, D>
    where
        T: 'a,
        M: 'a,
        P: 'a,
        D: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        SequenceJSONIterator { reader: self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::readers::FileReader;
    use s2json::{
        BBox3D, MValueCompatible, VectorBaseGeometry, VectorFeatureType, VectorGeometry,
        VectorGeometryType, VectorPoint,
    };
    use serde::{Deserialize, Serialize};
    use std::{path::PathBuf, vec, vec::Vec};

    #[test]
    fn test_json_line_delimited() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/points.geojsonld");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let line_del_reader = NewLineDelimitedJSONReader::new(FileReader::from(path.clone()), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Melbourne".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 144.9584,
                            y: -37.8173,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 144.9584,
                            bottom: -37.8173,
                            right: 144.9584,
                            top: -37.8173,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Canberra".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 149.1009,
                            y: -35.3039,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 149.1009,
                            bottom: -35.3039,
                            right: 149.1009,
                            top: -35.3039,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Sydney".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 151.2144,
                            y: -33.8766,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 151.2144,
                            bottom: -33.8766,
                            right: 151.2144,
                            top: -33.8766,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let line_del_reader = NewLineDelimitedJSONReader::new(FileReader::from(path), None);
        let features: Vec<VectorFeature<(), Test, MValue>> = line_del_reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Melbourne".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 144.9584,
                            y: -37.8173,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 144.9584,
                            bottom: -37.8173,
                            right: 144.9584,
                            top: -37.8173,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Canberra".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 149.1009,
                            y: -35.3039,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 149.1009,
                            bottom: -35.3039,
                            right: 149.1009,
                            top: -35.3039,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "Sydney".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint {
                            x: 151.2144,
                            y: -33.8766,
                            z: None,
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 151.2144,
                            bottom: -33.8766,
                            right: 151.2144,
                            top: -33.8766,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }

    #[test]
    fn test_json_line_seq() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/features.geojsonseq");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            prop0: String,
        }

        let seq_del_reader = SequenceJSONReader::new(FileReader::from(path.clone()));
        let features: Vec<VectorFeature<(), Test, MValue>> = seq_del_reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 102.0, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.5,
                            right: 102.0,
                            top: 0.5,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 102.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 103.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 104.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 105.0, y: 1.0, z: None, m: None, t: None }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.0,
                            right: 105.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Polygon(VectorBaseGeometry {
                        _type: VectorGeometryType::Polygon,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None }
                        ]],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 100.0,
                            bottom: 0.0,
                            right: 101.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );

        let seq_del_reader = SequenceJSONReader::new(FileReader::from(path));
        let features: Vec<VectorFeature<(), Test, MValue>> = seq_del_reader.iter().collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 102.0, y: 0.5, z: None, m: None, t: None },
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.5,
                            right: 102.0,
                            top: 0.5,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::LineString(VectorBaseGeometry {
                        _type: VectorGeometryType::LineString,
                        is_3d: false,
                        coordinates: vec![
                            VectorPoint { x: 102.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 103.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 104.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 105.0, y: 1.0, z: None, m: None, t: None }
                        ],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 102.0,
                            bottom: 0.0,
                            right: 105.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                },
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { prop0: "value0".into() },
                    geometry: VectorGeometry::Polygon(VectorBaseGeometry {
                        _type: VectorGeometryType::Polygon,
                        is_3d: false,
                        coordinates: vec![vec![
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 0.0, z: None, m: None, t: None },
                            VectorPoint { x: 101.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 1.0, z: None, m: None, t: None },
                            VectorPoint { x: 100.0, y: 0.0, z: None, m: None, t: None }
                        ]],
                        offset: None,
                        bbox: Some(BBox3D {
                            left: 100.0,
                            bottom: 0.0,
                            right: 101.0,
                            top: 1.0,
                            near: 1.7976931348623157e308,
                            far: -1.7976931348623157e308
                        }),
                        vec_bbox: None,
                        indices: None,
                        tessellation: None
                    }),
                    metadata: None
                }
            ]
        );
    }
}
