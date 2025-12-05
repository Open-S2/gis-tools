use crate::parsers::{FeatureReader, Reader};
use alloc::{
    collections::{BTreeMap, VecDeque},
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::{cell::RefCell, marker::PhantomData};
use s2json::{
    MValue, MValueCompatible, PrimitiveValue, Properties, VectorFeature, VectorGeometry,
    VectorPoint,
};
use serde::de::DeserializeOwned;

/// User defined options on how to parse the CSV file
#[derive(Debug, Default)]
pub struct CSVReaderOptions {
    /// The delimiter to use to separate lines [Default: `","`]
    pub delimiter: Option<char>,
    /// The line_delimiter to use to separate lines [Default: `"\n"`]
    pub line_delimiter: Option<char>,
    /// If provided the lookup of the longitude [Default: `"lon"`]
    pub lon_key: Option<String>,
    /// If provided the lookup of the latitude [Default: `"lat"`]
    pub lat_key: Option<String>,
    /// If provided the lookup for the height value [Default: `None`]
    pub height_key: Option<String>,
}

#[derive(Debug, Clone)]
struct CSVParser {
    first_line: bool,
    fields: Vec<String>,
    offset: u64,
    end: u64,
    partial_line: String,
    parsed_lines: VecDeque<String>,
}
impl CSVParser {
    /// Create a new CSVParser
    pub fn new(end: u64) -> Self {
        Self {
            first_line: true,
            fields: vec![],
            offset: 0,
            end,
            partial_line: String::new(),
            parsed_lines: VecDeque::new(),
        }
    }
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
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
///
/// ### File Reader
/// ```rust
/// use gistools::{
///     parsers::{FeatureReader, FileReader},
///     readers::{CSVReader, CSVReaderOptions},
/// };
/// use s2json::{MValue, VectorFeature};
/// use serde::{Deserialize, Serialize};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/csv/fixtures/basic.csv");
///
/// #[derive(Debug, Default, Clone, PartialEq, MValue, Serialize, Deserialize)]
/// struct Test {
///     name: String,
/// }
///
/// let reader = CSVReader::new(FileReader::from(path), None);
/// let features: Vec<VectorFeature<(), Test, MValue>> = reader.iter().collect();
/// ```
///
/// ### Buffer Reader
/// ```rust
/// use gistools::{
///     parsers::{FeatureReader, BufferReader},
///     readers::{CSVReader, CSVReaderOptions},
/// };
/// use s2json::{MValue, VectorFeature};
/// use serde::{Deserialize, Serialize};
///
/// // Ignore this setup, just ensures a passing test.
/// use std::path::PathBuf;
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path.push("tests/readers/csv/fixtures/basic.csv");
/// let data = std::fs::read(path).unwrap();
///
/// #[derive(Debug, Default, Clone, PartialEq, MValue, Serialize, Deserialize)]
/// struct Test {
///     name: String,
/// }
///
/// let reader = CSVReader::new(BufferReader::from(data), None);
/// let features: Vec<VectorFeature<(), Test, MValue>> = reader.iter().collect();
/// ```
///
/// ## Links
/// - <https://en.wikipedia.org/wiki/Comma-separated_values>
/// - <https://cesium.com/blog/2015/04/07/quadtree-cheatseet/>
#[derive(Debug, Clone)]
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
    /// Create a new CSVReader
    ///
    /// ## Parameters
    /// - `input`: the input data to parse from
    /// - `options`: user defined options on how to parse the CSV file
    ///
    /// ## Returns
    /// A new [`CSVReader`]
    pub fn new(reader: T, options: Option<CSVReaderOptions>) -> CSVReader<T, P> {
        let options = options.unwrap_or_default();
        let len = reader.len();
        CSVReader {
            reader,
            delimiter: options.delimiter.unwrap_or(','),
            line_delimiter: options.line_delimiter.unwrap_or('\n'),
            lon_key: options.lon_key.unwrap_or("lon".into()),
            lat_key: options.lat_key.unwrap_or("lat".into()),
            height_key: options.height_key,
            parser: RefCell::new(CSVParser::new(len)),
            _phantom: PhantomData,
        }
    }

    /// Set a new position in the file for parallel processing
    pub fn par_seek(&self, pool_size: u64, thread_id: u64) {
        // FIRST we run next_feature just to set the contents of the parser (specifically the fields)
        {
            *self.parser.borrow_mut() = CSVParser::new(self.reader.len());
            self.next_feature();
        }
        // setup chunk size, start and end
        let len = self.reader.len();
        let chunk_size = len.div_ceil(pool_size);
        let mut start = thread_id.saturating_mul(chunk_size);
        let mut end = u64::min(start + chunk_size, len);
        // align to separators
        if thread_id > 0 {
            start = align_to_line_delimiter(&self.reader, start, end, self.line_delimiter);
        }
        if thread_id < pool_size - 1 {
            end = align_to_line_delimiter(&self.reader, end, len, self.line_delimiter);
        }
        // update parser
        let mut parser = self.parser.borrow_mut();
        if thread_id == 0 {
            *parser = CSVParser::new(end);
        } else {
            parser.partial_line.clear();
            parser.parsed_lines.clear();
        }
        parser.offset = start;
        parser.end = end;
    }

    /// Grab the next feature if it exists
    pub fn next_feature(&self) -> Option<VectorFeature<(), P, MValue>> {
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
        if parser.offset < parser.end {
            let length = u64::min(65_536, parser.end - parser.offset);
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
            let line = core::mem::take(&mut parser.partial_line);
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
    fn parse_line(&self, line: &str, fields: &[String]) -> VectorFeature<(), P, MValue> {
        let values: Vec<String> =
            line.split(self.delimiter).map(|v| v.trim().to_string()).collect();
        let mut properties = Properties::new();
        let mut coordinates: VectorPoint<MValue> = VectorPoint::default();

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
    type Item = VectorFeature<(), P, MValue>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_feature()
    }
}
/// The CSV Iterator tool
#[derive(Debug, Clone)]
pub struct CSVIterator<'a, T: Reader, P: MValueCompatible + DeserializeOwned> {
    reader: &'a CSVReader<T, P>,
}
impl<T: Reader, P: MValueCompatible + DeserializeOwned> Iterator for CSVIterator<'_, T, P> {
    type Item = VectorFeature<(), P, MValue>;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.next_feature()
    }
}
/// A feature reader trait with a callback-based approach
impl<T: Reader, P: MValueCompatible + DeserializeOwned> FeatureReader<(), P, MValue>
    for CSVReader<T, P>
{
    type FeatureIterator<'a>
        = CSVIterator<'a, T, P>
    where
        T: 'a,
        P: 'a;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        *self.parser.borrow_mut() = CSVParser::new(self.reader.len());
        CSVIterator { reader: self }
    }

    fn par_iter(&self, pool_size: usize, thread_id: usize) -> Self::FeatureIterator<'_> {
        *self.parser.borrow_mut() = CSVParser::new(self.reader.len());
        self.par_seek(pool_size as u64, thread_id as u64);
        CSVIterator { reader: self }
    }
}

/// Parse CSV data into a record
/// the source is the source of the CSV data
/// the delimiter is the character used to separate fields
/// the line_delimiter is the character used to separate lines
/// returns an object with key-value pairs whose keys and values are both strings
///
/// Example:
/// ```rust
/// use gistools::readers::parse_csv_as_record;
/// use s2json::MValue;
///
/// #[derive(Debug, Default, Clone, PartialEq, MValue)]
/// struct Test {
///     a: String,
///     b: String,
///     c: String,
/// }
/// let source = "a,b,c\n1,2,3\n4,5,6";
/// let res = parse_csv_as_record::<Test>(source, None, None);
///
/// assert_eq!(
///     res,
///     vec![
///         Test { a: "1".into(), b: "2".into(), c: "3".into() },
///         Test { a: "4".into(), b: "5".into(), c: "6".into() },
///     ]
/// );
/// ```
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
            let val =
                if value.trim().is_empty() { (&PrimitiveValue::Null).into() } else { value.into() };
            record.insert(header.into(), val);
        }

        res.push(record.into());
    }

    res
}

/// Parse CSV data into a BTreeMap record where both the keys and values are strings
pub fn parse_csv_as_btree(
    source: &str,
    delimiter: Option<char>,
    line_delimiter: Option<char>,
) -> Vec<BTreeMap<String, String>> {
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

        let mut record = BTreeMap::new();
        let values = parse_csv_line(line, delimiter);

        for (value, header) in values.iter().take(header.len()).zip(header.iter()) {
            let val = value.trim();
            if val.is_empty() {
                continue;
            }
            record.insert(header.clone(), val.to_string());
        }

        res.push(record);
    }

    res
}

/// Parses a line of a CSV file into a vector of values split by the delimiter.
/// Handles quoted values that contain the delimiter.
pub fn parse_csv_line(line: &str, delimiter: char) -> Vec<String> {
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

// Helper function to align to delimiters if using parallel processing
fn align_to_line_delimiter<R: Reader>(reader: &R, mut pos: u64, end: u64, sep: char) -> u64 {
    let sep_u8 = sep as u8;

    while pos < end {
        let len = u64::min(65_536, end - pos);
        let chunk = reader.parse_string(Some(pos), Some(len));
        if let Some(rel) = chunk.as_bytes().iter().position(|&b| b == sep_u8) {
            return pos + rel as u64 + 1; // move past delimiter
        }
        pos += len;
    }
    end
}
