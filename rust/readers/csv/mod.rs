use super::{FeatureReader, Reader};
use alloc::{
    collections::VecDeque,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{cell::RefCell, marker::PhantomData};
use s2json::{MValue, MValueCompatible, Properties, VectorFeature, VectorGeometry, VectorPoint};
use serde::de::DeserializeOwned;

/// User defined options on how to parse the CSV file
#[derive(Debug, Default)]
pub struct CSVReaderOptions {
    /// The delimiter to use to separate lines [Default: `","`]
    delimiter: Option<char>,
    /// The line_delimiter to use to separate lines [Default: `"\n"`]
    line_delimiter: Option<char>,
    /// If provided the lookup of the longitude [Default: `"lon"`]
    lon_key: Option<String>,
    /// If provided the lookup of the latitude [Default: `"lat"`]
    lat_key: Option<String>,
    /// If provided the lookup for the height value [Default: `None`]
    height_key: Option<String>,
}

#[derive(Debug)]
struct CSVParser {
    first_line: bool,
    fields: Vec<String>,
    offset: u64,
    partial_line: String,
    parsed_lines: VecDeque<String>,
}
impl CSVParser {
    /// given the fields in the first line split by the delimiter and store them
    pub fn parse_first_line(&mut self, line: &str, delimiter: char) {
        self.fields = line.split(delimiter).map(|v| v.trim().to_string()).collect();
    }
}

/// # CSV Reader
///
/// ## Description
/// Parse (Geo|S2)JSON from a file that is in the CSV format
///
/// ## Links
/// - https://en.wikipedia.org/wiki/Comma-separated_values
/// - https://cesium.com/blog/2015/04/07/quadtree-cheatseet/
#[derive(Debug)]
pub struct CSVReader<T: Reader, P: MValueCompatible + DeserializeOwned = MValue> {
    reader: T,
    delimiter: char,
    line_delimiter: char,
    lon_key: String,
    lat_key: String,
    height_key: Option<String>,
    parser: RefCell<CSVParser>,
    _phantom: PhantomData<VectorFeature<(), P, ()>>,
}
impl<T: Reader, P: MValueCompatible + DeserializeOwned> CSVReader<T, P> {
    /// @param input - the input data to parse from
    /// @param options - user defined options on how to parse the CSV file
    pub fn new(reader: T, options: Option<CSVReaderOptions>) -> CSVReader<T, P> {
        let options = options.unwrap_or_default();
        CSVReader {
            reader,
            delimiter: options.delimiter.unwrap_or(','),
            line_delimiter: options.line_delimiter.unwrap_or('\n'),
            lon_key: options.lon_key.unwrap_or("lon".into()),
            lat_key: options.lat_key.unwrap_or("lat".into()),
            height_key: options.height_key,
            parser: RefCell::new(CSVParser {
                first_line: true,
                fields: vec![],
                offset: 0,
                partial_line: String::new(),
                parsed_lines: VecDeque::new(),
            }),
            _phantom: PhantomData,
        }
    }

    /// Grab the next feature if it exists
    pub fn next_feature(&self) -> Option<VectorFeature<(), P, ()>> {
        let mut parser = self.parser.borrow_mut();
        // Keep returning from the queue if there's data
        while let Some(line) = parser.parsed_lines.pop_front() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if parser.first_line {
                parser.parse_first_line(trimmed, self.delimiter);
                parser.first_line = false;
            } else {
                return Some(self.parse_line(trimmed, &parser.fields));
            }
        }

        // Read more if we're not done
        if parser.offset < self.reader.len() {
            let length = u64::min(65_536, self.reader.len() - parser.offset);
            let chunk = parser.partial_line.clone()
                + &self.reader.parse_string(Some(parser.offset), Some(length));
            parser.offset += length;

            parser.partial_line.clear();
            let mut lines = chunk
                .split(self.line_delimiter)
                .map(str::to_string)
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();

            if let Some(last) = lines.pop() {
                parser.partial_line = last;
            }

            parser.parsed_lines.extend(lines);
            // drop the parser before calling next_feature
            drop(parser);
            return self.next_feature(); // recurse now that buffer is filled
        }

        // Final line after file ends
        if !parser.partial_line.is_empty() {
            let line = std::mem::take(&mut parser.partial_line);
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }
            if parser.first_line {
                parser.parse_first_line(trimmed, self.delimiter);
                parser.first_line = false;
                drop(parser);
                return self.next_feature(); // recurse again to skip header
            } else {
                return Some(self.parse_line(trimmed, &parser.fields));
            }
        }

        None
    }

    /// given a line, parse the values mapped to the first lines fields
    /// returns a GeoJSON Vector Feature
    fn parse_line(&self, line: &str, fields: &[String]) -> VectorFeature<(), P, ()> {
        let values: Vec<String> =
            line.split(self.delimiter).map(|v| v.trim().to_string()).collect();
        let mut properties = Properties::new();
        let mut coordinates: VectorPoint<()> = VectorPoint::default();

        for (value, field) in values.iter().zip(fields.iter()) {
            if field.is_empty() || value.is_empty() {
                continue;
            }

            let value_num: f64 = value.parse().unwrap_or(0.0);
            if *field == self.lon_key {
                coordinates.x = value_num;
            } else if *field == self.lat_key {
                coordinates.y = value_num;
            } else if Some(field) == self.height_key.as_ref() {
                coordinates.z = Some(value_num);
            } else {
                properties.insert(field.clone(), value.into());
            }
        }
        if coordinates.x.is_nan() || coordinates.y.is_nan() {
            panic!("coordinates must be finite numbers");
        }

        VectorFeature {
            _type: "VectorFeature".into(),
            geometry: VectorGeometry::new_point(coordinates, None),
            properties: (&properties).into(),
            ..Default::default()
        }
    }
}
impl<T: Reader, P: MValueCompatible + DeserializeOwned> Iterator for CSVReader<T, P> {
    type Item = VectorFeature<(), P, ()>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_feature()
    }
}
/// The CSV Iterator tool
#[derive(Debug)]
pub struct CSVIterator<'a, T: Reader, P: MValueCompatible + DeserializeOwned> {
    reader: &'a CSVReader<T, P>,
}
impl<T: Reader, P: MValueCompatible + DeserializeOwned> Iterator for CSVIterator<'_, T, P> {
    type Item = VectorFeature<(), P, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next_feature()
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader, P: MValueCompatible + DeserializeOwned> FeatureReader<(), P, ()>
    for CSVReader<T, P>
{
    type FeatureIterator<'a>
        = CSVIterator<'a, T, P>
    where
        T: 'a,
        P: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        CSVIterator { reader: self }
    }
}

/// Parse CSV data into a record
/// the source is the source of the CSV data
/// the delimiter is the character used to separate fields
/// the line_delimiter is the character used to separate lines
/// returns an object with key-value pairs whose keys and values are both strings
pub fn parse_csv_as_record<T: MValueCompatible>(
    source: &str,
    delimiter: Option<char>,
    line_delimiter: Option<char>,
) -> Vec<T> {
    let delimiter = delimiter.unwrap_or(',');
    let line_delimiter = line_delimiter.unwrap_or('\n');
    let mut res = vec![];
    let lines: Vec<&str> = source.split(line_delimiter).collect();
    let header = parse_csv_line(lines[0], delimiter);

    for raw_line in lines.iter().skip(1) {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut record = MValue::new();
        let values = parse_csv_line(line, delimiter);

        for (value, header) in values.iter().take(header.len()).zip(header.iter()) {
            record.insert(header.into(), value.into());
        }

        res.push(record.into());
    }

    res
}

/// Parses a line of a CSV file into a vector of values split by the delimiter.
/// Handles quoted values that contain the delimiter.
fn parse_csv_line(line: &str, delimiter: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = None;

    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if (ch == '"' || ch == '\'') && !in_quotes {
            in_quotes = true;
            quote_char = Some(ch);
        } else if Some(ch) == quote_char && in_quotes {
            // Check for escaped quote
            if i + 1 < chars.len() && chars[i + 1] == ch {
                current.push(ch);
                i += 1; // Skip the next quote
            } else {
                in_quotes = false;
            }
        } else if ch == delimiter && !in_quotes {
            result.push(current.trim().into());
            current.clear();
        } else {
            current.push(ch);
        }

        i += 1;
    }

    // Push the final field
    if !current.is_empty() {
        result.push(current.trim().into());
    }

    result
}

#[cfg(test)]
mod tests {
    use s2json::{VectorBaseGeometry, VectorFeatureType, VectorGeometryType};
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::readers::FileReader;
    use std::path::PathBuf;

    #[test]
    fn test_parse_csv_line() {
        assert_eq!(parse_csv_line("a,b,c", ','), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_csv_as_record() {
        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible)]
        struct Test {
            a: String,
            b: String,
            c: String,
        }
        let source = "a,b,c\n1,2,3\n4,5,6";
        let res = parse_csv_as_record::<MValue>(source, None, None);
        assert_eq!(
            res,
            vec![
                MValue::from([
                    ("a".into(), "1".into()),
                    ("b".into(), "2".into()),
                    ("c".into(), "3".into()),
                ]),
                MValue::from([
                    ("a".into(), "4".into()),
                    ("b".into(), "5".into()),
                    ("c".into(), "6".into()),
                ]),
            ]
        );

        let res = parse_csv_as_record::<Test>(source, None, None);
        assert_eq!(
            res,
            vec![
                Test { a: "1".into(), b: "2".into(), c: "3".into() },
                Test { a: "4".into(), b: "5".into(), c: "6".into() },
            ]
        );
    }

    #[test]
    fn test_csv_reader() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/csv/fixtures/basic.csv");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let reader = CSVReader::new(FileReader::from(path), None);

        let features: Vec<VectorFeature<(), Test, ()>> = reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "3".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 2.0, y: 1.0, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
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
                    properties: Test { name: "a".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: false,
                        coordinates: VectorPoint { x: 1.1, y: 3.2, z: None, m: None, t: None },
                        offset: None,
                        bbox: None,
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
    fn test_csv_reader_3d() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/readers/csv/fixtures/basic3D.csv");

        #[derive(Debug, Default, Clone, PartialEq, MValueCompatible, Serialize, Deserialize)]
        struct Test {
            name: String,
        }

        let reader = CSVReader::new(
            FileReader::from(path),
            Some(CSVReaderOptions {
                delimiter: Some(','),
                line_delimiter: Some('\n'),
                lon_key: Some("Longitude".into()),
                lat_key: Some("Latitude".into()),
                height_key: Some("height".into()),
            }),
        );

        let features: Vec<VectorFeature<(), Test, ()>> = reader.collect();

        assert_eq!(
            features,
            vec![
                VectorFeature {
                    _type: VectorFeatureType::VectorFeature,
                    id: None,
                    face: 0.into(),
                    properties: Test { name: "3".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 2.0,
                            y: 1.0,
                            z: Some(55.0),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: None,
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
                    properties: Test { name: "a".into() },
                    geometry: VectorGeometry::Point(VectorBaseGeometry {
                        _type: VectorGeometryType::Point,
                        is_3d: true,
                        coordinates: VectorPoint {
                            x: 1.1,
                            y: 3.2,
                            z: Some(-2.2),
                            m: None,
                            t: None
                        },
                        offset: None,
                        bbox: None,
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
