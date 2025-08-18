use super::ToGisJSON;
use crate::{
    geometry::ConvertFeature,
    parsers::{FeatureReader, Reader},
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
///
/// ## Description
///
/// Parse (Geo|S2)JSON from a file that is in a newline-delimited format
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
/// ```rust
/// use gistools::{parsers::{FileReader, FeatureReader}, readers::NewLineDelimitedJSONReader};
/// use s2json::{Properties, MValue};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path = path.join("tests/readers/json/fixtures/points.geojsonld");
///
/// let reader: NewLineDelimitedJSONReader<_> = NewLineDelimitedJSONReader::new(FileReader::from(path), None);
/// let features: Vec<_> = reader.iter().collect();
/// assert_eq!(features.len(), 3);
/// ```
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

    /// Reset to the beginning
    pub fn reset(&self) {
        let mut parser = self.parser.borrow_mut();
        parser.offset = 0;
        parser.tmp_chunks.clear();
        parser.partial_line.clear();
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
        if line.len() > 1
            && let Ok(feature) = line.to_features()
        {
            match feature {
                Features::Feature(feature) => {
                    return Some(feature.to_vector(Some(true)));
                }
                Features::VectorFeature(vf) => {
                    return Some(vf);
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
#[derive(Debug)]
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
        self.reset();
        NewLineDelimitedJSONIterator { reader: self }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}

/// # Text Sequence JSON Reader
///
/// ## Description
///
/// Parse GeoJSON from a file that is in the `geojson-text-sequences` format.
///
/// Implements the [`FeatureReader`] trait
///
/// ## Usage
/// ```rust
/// use gistools::{parsers::{FileReader, FeatureReader}, readers::SequenceJSONReader};
/// use s2json::{Properties, MValue};
/// use std::path::PathBuf;
///
/// let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
/// path = path.join("tests/readers/json/fixtures/features.geojsonseq");
///
/// let reader: SequenceJSONReader<_> = SequenceJSONReader::new(FileReader::from(path));
/// let features: Vec<_> = reader.iter().collect();
/// assert_eq!(features.len(), 3);
/// ```
///
/// ## Links
/// - <https://datatracker.ietf.org/doc/html/rfc7464>
/// - <https://datatracker.ietf.org/doc/html/rfc8142>
/// - <https://github.com/geojson/geojson-text-sequences?tab=readme-ov-file>
#[derive(Debug)]
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

    /// Reset to the beginning
    pub fn reset(&self) {
        let mut parser = self.newline.parser.borrow_mut();
        parser.offset = 0;
        parser.tmp_chunks.clear();
        parser.partial_line.clear();
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
#[derive(Debug)]
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
        self.reset();
        SequenceJSONIterator { reader: self }
    }

    #[cfg(feature = "std")]
    fn par_iter(&self, _pool_size: usize, _thread_id: usize) -> Self::FeatureIterator<'_> {
        self.iter()
    }
}
