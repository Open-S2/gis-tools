use super::ToGisJSON;
use crate::{
    geometry::ConvertFeature,
    readers::{FeatureReader, Reader},
};
use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{cell::RefCell, marker::PhantomData};
use s2json::{Features, MValue, VectorFeature};
use serde::de::DeserializeOwned;

const LEFT_BRACE: u8 = 0x7b;
const RIGHT_BRACE: u8 = 0x7d;
const BACKSLASH: u8 = 0x5c;
const STRING: u8 = 0x22;

#[derive(Debug)]
struct JSONParser {
    buffer: Vec<u8>,
    chunk_size: u64,
    offset: u64,
    pos: usize,
    brace_depth: isize,
    feature: Vec<Vec<u8>>,
    start: Option<usize>,
    end: Option<usize>,
    is_object: bool,
}

/// # JSON Reader
///
/// ## Description
/// Parse (Geo|S2)JSON. Can handle millions of features.
#[derive(Debug)]
pub struct JSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: Clone + Default + DeserializeOwned = MValue,
    D: Clone + Default + DeserializeOwned = MValue,
> {
    reader: T,
    length: u64,
    parser: RefCell<JSONParser>,
    _phantom: PhantomData<VectorFeature<M, P, D>>,
}
impl<
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> JSONReader<T, M, P, D>
{
    /// Create a new JSONReader
    pub fn new(reader: T, chunk_size: Option<u64>) -> JSONReader<T, M, P, D> {
        let length = reader.len();
        let json_reader = JSONReader {
            reader,
            length,
            parser: RefCell::new(JSONParser {
                buffer: vec![],
                chunk_size: chunk_size.unwrap_or(65_536),
                offset: 0,
                pos: 0,
                brace_depth: 0,
                feature: vec![],
                start: None,
                end: None,
                is_object: true,
            }),
            _phantom: PhantomData,
        };

        // buffer the first chunk
        {
            let mut parser = json_reader.parser.borrow_mut();
            parser.chunk_size = u64::min(65_536, json_reader.length - parser.offset);
            parser.buffer = json_reader.reader.slice(Some(0), Some(parser.chunk_size)).to_vec();
        }
        // find out starting position
        let set = json_reader.set_start_position();
        if !set {
            panic!("File is not geojson or s2json");
        }

        json_reader
    }

    /// since we know that a '{' is the start of a feature after we read a '"features"',
    /// than we start there to avoid reading in values that are not features.
    /// This is a modified Knuth–Morris–Pratt algorithm
    fn set_start_position(&self) -> bool {
        let features = "\"features\":".as_bytes();
        let features_size = features.len();
        let mut parser = self.parser.borrow_mut();

        let mut k = 0;
        while parser.pos < parser.chunk_size as usize {
            if features[k] == parser.buffer[parser.pos] {
                k += 1;
                parser.pos += 1;
                if k == features_size {
                    return true;
                }
            } else {
                k = 0;
                parser.pos += 1;
            }
        }
        // if we made it here, we need to read in the next buffer chunk.
        // If we hit the end of the file, return false
        parser.offset += parser.chunk_size;
        if parser.offset < self.length {
            parser.pos = 0;
            let chunk_size = u64::min(65_536, self.length - parser.offset);
            parser.buffer =
                self.reader.slice(Some(parser.offset), Some(parser.offset + chunk_size));
            drop(parser);
            self.set_start_position()
        } else {
            false
        }
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

    /// everytime we see a "{" we start 'recording' the feature. If we see more "{" on our journey, we increment.
    /// Once we find the end of the feature, store the "start" and "end" indexes, slice the buffer and send out
    /// as a return. If we run out of buffer to read AKA we finish the file, we return a null. If we run
    /// out of the buffer, but we still have file left to read, just read into the buffer and continue on
    pub fn next_feature(&self) -> Option<VectorFeature<M, P, D>> {
        let mut parser = self.parser.borrow_mut();
        // get started
        while parser.pos < parser.chunk_size as usize {
            if parser.buffer[parser.pos] == BACKSLASH {
                parser.pos += 1;
            } else if parser.buffer[parser.pos] == STRING {
                parser.is_object = !parser.is_object;
            } else if parser.buffer[parser.pos] == LEFT_BRACE && parser.is_object {
                if parser.brace_depth == 0 {
                    parser.start = Some(parser.pos);
                }
                parser.brace_depth += 1; // first brace is the start of the feature
            } else if parser.buffer[parser.pos] == RIGHT_BRACE && parser.is_object {
                parser.brace_depth -= 1; // if this hits zero, we are at the end of the feature
                if parser.brace_depth == 0 {
                    parser.end = Some(parser.pos);
                    break;
                }
            }
            parser.pos += 1;
        }

        // what if the last char in current buffer was a BACKSLASH?
        // we need to make sure in the next buffer we account for increment
        let chunk_size = parser.chunk_size as usize;
        let increment_space = parser.pos.saturating_sub(chunk_size);

        if let (Some(start), Some(end)) = (parser.start, parser.end) {
            parser.pos += 1;
            let buf: Vec<u8> = parser.buffer[start..end + 1].to_vec();
            parser.feature.push(buf);
            let feature = parser.feature.concat();
            // reset variables
            parser.feature = vec![];
            parser.start = None;
            parser.end = None;
            parser.brace_depth = 0;
            parser.is_object = true;
            // convert feature to a &str and parse it
            let feature_str: String = String::from_utf8_lossy(&feature).to_string();
            self.parse_line(&feature_str)
        } else {
            // if offset isn't at filesize, increment buffer and start again
            if let Some(start) = parser.start {
                let buf = parser.buffer[start..].to_vec();
                parser.feature.push(buf);
                parser.start = Some(0);
            }
            parser.offset += parser.chunk_size;
            if parser.offset < self.length {
                parser.pos = if increment_space > 0 { increment_space } else { 0 };
                if parser.offset + parser.chunk_size > self.length {
                    parser.chunk_size = self.length - parser.offset;
                }
                parser.chunk_size = u64::min(65_536, self.length - parser.offset);
                parser.buffer =
                    self.reader.slice(Some(parser.offset), Some(parser.offset + parser.chunk_size));
                self.next_feature()
            } else {
                None
            } // end of file
        }
    }
}
impl<
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> Iterator for JSONReader<T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_feature()
    }
}
/// The JSON Iterator tool
#[derive(Debug)]
pub struct JSONIterator<
    'a,
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> {
    reader: &'a JSONReader<T, M, P, D>,
}
impl<
    T: Reader,
    M: Clone + DeserializeOwned,
    P: Clone + Default + DeserializeOwned,
    D: Clone + Default + DeserializeOwned,
> Iterator for JSONIterator<'_, T, M, P, D>
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
> FeatureReader<M, P, D> for JSONReader<T, M, P, D>
{
    type FeatureIterator<'a>
        = JSONIterator<'a, T, M, P, D>
    where
        T: 'a,
        M: 'a,
        P: 'a,
        D: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        JSONIterator { reader: self }
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
    fn test_json_line() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/json/fixtures/points.geojson");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let line_del_reader = JSONReader::new(FileReader::from(path.clone()), None);
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

        let line_del_reader = JSONReader::new(FileReader::from(path), None);
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
}
