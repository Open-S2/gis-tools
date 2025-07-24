use crate::parsers::{FeatureReader, clean_string};
use alloc::{string::String, vec, vec::Vec};
use s2json::{
    BBox3D, MValue, Properties, VectorFeature, VectorGeometry, VectorLineString,
    VectorMultiLineString, VectorMultiPolygon, VectorPoint,
};

/// WKT Value can be a point or an array of points
#[derive(Debug, Clone, PartialEq)]
pub enum WKTAValue {
    /// A Vector Point
    Point(VectorPoint),
    /// A collection of sub WKT values
    Array(Vec<WKTAValue>),
}
impl WKTAValue {
    /// Get the Vector Point
    pub fn get_point(&mut self) -> Option<&mut VectorPoint> {
        match self {
            WKTAValue::Point(point) => Some(point),
            WKTAValue::Array(arr) => arr.first_mut().and_then(|v| v.get_point()),
        }
    }
    /// Get a vector linestring
    pub fn get_linestring(&mut self) -> Option<VectorLineString> {
        match self {
            WKTAValue::Point(point) => Some(vec![point.clone()]),
            WKTAValue::Array(arr) => {
                arr.iter_mut().map(|v| v.get_point().map(core::mem::take)).collect()
            }
        }
    }
    /// Get a vector multilinestring
    pub fn get_multilinestring(&mut self) -> Option<VectorMultiLineString> {
        match self {
            WKTAValue::Point(point) => Some(vec![vec![point.clone()]]),
            WKTAValue::Array(arr) => arr.iter_mut().map(|v| v.get_linestring()).collect(),
        }
    }
}

/// WKT Array can be an array of points or even nested arrays of points
pub type WKTArray = Vec<WKTAValue>;

/// # WKT Geometry Reader
///
/// ## Description
/// Parse a collection of WKT geometries from a string
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
///
/// The methods you have access to:
/// - [`WKTGeometryReader::new`]: Create a new WKTGeometryReader
///
/// ```rust
/// use gistools::{parsers::FeatureReader, readers::WKTGeometryReader};
///
/// let collection_wkt = r#"POINT(4 6)
/// GEOMETRYCOLLECTION(POINT(1 2), LINESTRING(3 4,5 6))
/// MULTIPOLYGON EMPTY
/// TRIANGLE((0 0 0,0 1 0,1 1 0,0 0 0))"#;
///
/// let reader = WKTGeometryReader::new(collection_wkt.into());
/// let features: Vec<_> = reader.iter().collect();
/// assert_eq!(features.len(), 3);
/// ```
///
/// ## Links
/// - <https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry>
#[derive(Debug)]
pub struct WKTGeometryReader {
    /// The parsed WKT geometries
    pub features: Vec<VectorFeature>,
}
impl WKTGeometryReader {
    /// Create a new WKT Geometry Reader
    pub fn new(data: String) -> Self {
        let mut features = vec![];
        let wkt_strings = split_wkt_geometry(data);
        for wkt_string in wkt_strings {
            let geometry = parse_wkt_geometry(wkt_string);
            if let Some(geometry) = geometry {
                features.push(VectorFeature { geometry, ..Default::default() });
            }
        }
        WKTGeometryReader { features }
    }
}
/// The WKT Iterator tool
#[derive(Debug)]
pub struct WKTIterator<'a> {
    reader: &'a WKTGeometryReader,
    index: usize,
}
impl Iterator for WKTIterator<'_> {
    type Item = VectorFeature;

    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        self.reader.features.get(self.index - 1).cloned()
    }
}
/// A feature reader trait with a callback-based approach
impl FeatureReader<(), Properties, MValue> for WKTGeometryReader {
    type FeatureIterator<'a> = WKTIterator<'a>;

    fn iter(&self) -> Self::FeatureIterator<'_> {
        WKTIterator { reader: self, index: 0 }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}

/// # WKT Geometry Parser
///
/// ## Description
/// Parse individual geometries from a WKT string into a VectorGeometry
///
/// ## Usage
/// ```rust
/// use gistools::readers::parse_wkt_geometry;
/// use s2json::{VectorPoint, VectorGeometry, BBox3D};
///
/// let wkt_str = "POINT Z (5.4321 1.2345 2.3456)";
/// let geo = parse_wkt_geometry(wkt_str.into());
/// let expected = VectorPoint::from_xyz(5.4321, 1.2345, 2.3456);
/// assert_eq!(
///     geo,
///     Some(VectorGeometry::new_point(expected.clone(), Some(BBox3D::from_point(&expected))))
/// );
/// ```
///
/// ## Links
/// - <https://en.wikipedia.org/wiki/Well-known_text_representation_of_geometry>
///
/// ## Parameters
/// - `wkt_str`: WKT Geometry string
///
/// ## Returns
/// A [`VectorGeometry`] if the WKT string is valid
pub fn parse_wkt_geometry(wkt_str: String) -> Option<VectorGeometry> {
    if wkt_str.starts_with("POINT") {
        parse_wkt_point(wkt_str)
    } else if wkt_str.starts_with("MULTIPOINT") {
        parse_wkt_line(wkt_str, LineParseType::MultiPoint)
    } else if wkt_str.starts_with("LINESTRING") {
        parse_wkt_line(wkt_str, LineParseType::LineString)
    } else if wkt_str.starts_with("MULTILINESTRING") {
        parse_wkt_multi_line(wkt_str, MultiLineParseType::MultiLineString)
    } else if wkt_str.starts_with("POLYGON") {
        parse_wkt_multi_line(wkt_str, MultiLineParseType::Polygon)
    } else if wkt_str.starts_with("MULTIPOLYGON") {
        parse_wkt_multi_polygon(wkt_str)
    } else {
        None
    }
}

/// Split a WKT string into individual geometry strings
///
/// Removes EMPTY geometries, flattens GEOMETRYCOLLECTIONs recursively,
/// and returns a vector of individual WKT geometry strings.
///
/// ## Parameters
/// - `input`: WKT string that is a collection of geometries
///
/// ## Returns
/// Array of individual WKT geometries still in string form
pub fn split_wkt_geometry(mut input: String) -> Vec<String> {
    // Remove EMPTY geometries and their preceding type keyword
    let mut words: Vec<&str> = input.split_whitespace().collect();
    let mut i = 0;
    while i < words.len() {
        if words[i].contains("EMPTY") && i > 0 {
            words.drain(i - 1..=i);
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    input = words.join(" ");

    let mut geometries = Vec::new();
    let mut start = 0;
    let mut found = false;
    let mut depth = 0;
    let input_chars: Vec<char> = input.chars().collect();

    for i in 0..input_chars.len() {
        match input_chars[i] {
            '(' => {
                depth += 1;
                found = true;
            }
            ')' => {
                depth -= 1;
                if found && depth == 0 {
                    let end = i + 1;
                    let segment: String =
                        input_chars[start..end].iter().collect::<String>().trim().into();
                    geometries.push(segment);
                    start = end;
                    found = false;
                }
            }
            _ => {}
        }
    }

    let mut i = 0;
    while i < geometries.len() {
        if geometries[i].starts_with("GEOMETRYCOLLECTION") {
            let g = geometries.remove(i);
            let inner = g[g.find('(').unwrap() + 1..g.len() - 1].into();
            let nested = split_wkt_geometry(inner);
            geometries.splice(i..i, nested);
        } else {
            if geometries[i].starts_with(',') {
                geometries[i] = geometries[i].trim_start_matches(',').trim().into();
            }
            i += 1;
        }
    }

    geometries.into_iter().filter(|g| !g.is_empty()).collect()
}

/// Parse a WKT point string to a VectorPoint
///
/// ## Parameters
/// - `wkt_str`: WKT string
///
/// ## Returns
/// A [`VectorPoint`] in a [`VectorGeometry`] if the WKT string is valid
fn parse_wkt_point(wkt_str: String) -> Option<VectorGeometry> {
    if let Some(WKTAValue::Point(point)) = parse_wkt_array(wkt_str).get_mut(0) {
        let bbox = BBox3D::from_point(point);
        Some(VectorGeometry::new_point(core::mem::take(point), Some(bbox)))
    } else {
        None
    }
}

enum LineParseType {
    MultiPoint,
    LineString,
}

/// Parse a WKT array to a LineString or MultiPoint geometry
///
/// ## Parameters
/// - `wkt_str`: WKT string
/// - `type`: 'MultiPoint' or 'LineString'
///
/// ## Returns
/// A [`VectorGeometry`] as either a [`s2json::VectorLineString`] or [`s2json::VectorMultiPoint`]
/// if the WKT string is valid
fn parse_wkt_line(wkt_str: String, r#type: LineParseType) -> Option<VectorGeometry> {
    let mut line = parse_wkt_array(wkt_str);
    let points: VectorLineString =
        line.iter_mut().map(|e| e.get_point().map(core::mem::take).unwrap_or_default()).collect();
    let bbox = BBox3D::from_linestring(&points);
    match r#type {
        LineParseType::MultiPoint => Some(VectorGeometry::new_multipoint(points, Some(bbox))),
        LineParseType::LineString => Some(VectorGeometry::new_linestring(points, Some(bbox))),
    }
}

enum MultiLineParseType {
    MultiLineString,
    Polygon,
}

/// Parse a WKT array to a MultiLineString or Polygon
///
/// ## Parameters
/// - `wkt_str`: WKT string
/// - `type`: 'MultiLineString' or 'Polygon'
///
/// ## Returns
/// A [`VectorGeometry`] as either a [`s2json::VectorMultiLineString`] or [`s2json::VectorPolygon`]
/// if the WKT string is valid
fn parse_wkt_multi_line(wkt_str: String, r#type: MultiLineParseType) -> Option<VectorGeometry> {
    let mut multiline = parse_wkt_array(wkt_str);
    let lines: VectorMultiLineString =
        multiline.iter_mut().map(|e| e.get_linestring().unwrap_or_default()).collect();
    let bbox = BBox3D::from_multi_linestring(&lines);
    match r#type {
        MultiLineParseType::MultiLineString => {
            Some(VectorGeometry::new_multilinestring(lines, Some(bbox)))
        }
        MultiLineParseType::Polygon => Some(VectorGeometry::new_polygon(lines, Some(bbox))),
    }
}

/// Parse a WKT array to a MultiPolygon
///
/// ## Parameters
/// - `wkt_str`: WKT string
///
/// ## Returns
/// A [`VectorGeometry`] as a [`s2json::VectorMultiPolygon`] if the WKT string is valid
fn parse_wkt_multi_polygon(wkt_str: String) -> Option<VectorGeometry> {
    let mut multipolygon = parse_wkt_array(wkt_str);
    let polygons: VectorMultiPolygon =
        multipolygon.iter_mut().map(|e| e.get_multilinestring().unwrap_or_default()).collect();
    let bbox = BBox3D::from_multi_polygon(&polygons);
    Some(VectorGeometry::new_multipolygon(polygons, Some(bbox)))
}

/// Parse a WKT array
///
/// ## Parameters
/// - `wkt_str`: WKT string
///
/// ## Returns
/// Collection of points as [`WKTArray`]
pub fn parse_wkt_array(wkt_str: String) -> WKTArray {
    let mut res = Vec::new();
    let _ = _parse_wkt_array(wkt_str, &mut res);
    if let Some(WKTAValue::Array(inner)) = res.first() { inner.clone() } else { res }
}

/// Parse a WKT array.
/// always return the endBracketIndex if we hit it
///
/// ## Parameters
/// - `wkt_str`: WKT string
/// - `res`: collection to store the values
///
/// ## Returns
/// A sliced WKT string with the parsed values
fn _parse_wkt_array(mut wkt_str: String, res: &mut WKTArray) -> String {
    while !wkt_str.is_empty() {
        let comma_index = wkt_str.find(',').unwrap_or(usize::MAX);
        let start_bracket_index = wkt_str.find('(').unwrap_or(usize::MAX);
        let end_bracket_index = wkt_str.find(')').unwrap_or(usize::MAX);

        if comma_index < start_bracket_index.min(end_bracket_index) {
            let key = &wkt_str[..comma_index].trim();
            if !key.is_empty() {
                res.push(WKTAValue::Point(build_point(key)));
            }
            wkt_str = wkt_str[comma_index + 1..].into();
        } else if start_bracket_index < end_bracket_index {
            let mut inner = Vec::new();
            let inner_str = &wkt_str[start_bracket_index + 1..];
            wkt_str = _parse_wkt_array(inner_str.into(), &mut inner);
            res.push(WKTAValue::Array(inner));
        } else {
            if end_bracket_index > 0 {
                let key = &wkt_str[..end_bracket_index].trim();
                if !key.is_empty() {
                    res.push(WKTAValue::Point(build_point(key)));
                }
                wkt_str = wkt_str[end_bracket_index + 1..].into();
            } else {
                wkt_str = wkt_str[1..].into();
            }
            return wkt_str;
        }
    }
    wkt_str
}

/// Build a point from a WKT string
///
/// ## Parameters
/// - `str`: WKT string
///
/// ## Returns
/// A [`VectorPoint`]
fn build_point(input: &str) -> VectorPoint {
    let binding = clean_string(input);
    let parts: Vec<&str> = binding.split_whitespace().collect();

    let x = parts.first().and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
    let y = parts.get(1).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
    let z = parts.get(2).and_then(|v| v.parse::<f64>().ok());

    VectorPoint::new(x, y, z, None)
}
