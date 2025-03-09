use core::marker::PhantomData;

use s2json::{MValue, MValueCompatible, VectorFeature, WMFeature};

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use serde::de::DeserializeOwned;

use crate::{geometry::ConvertFeature, readers::Reader};

use super::{FeatureIterator, ToGisJSON};

const LEFT_BRACE: u8 = 0x7b;
const RIGHT_BRACE: u8 = 0x7d;
const BACKSLASH: u8 = 0x5c;
const STRING: u8 = 0x22;

/// # JSON Reader
///
/// ## Description
/// Parse (Geo|S2)JSON. Can handle millions of features.
/// Implements the {@link FeatureIterator} interface
pub struct JSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: MValueCompatible + DeserializeOwned = MValue,
    D: MValueCompatible + DeserializeOwned = MValue,
> {
    reader: T,
    chunk_size: usize,
    buffer: Vec<u8>,
    offset: usize,
    length: usize,
    pos: usize,
    brace_depth: usize,
    feature: Vec<Vec<u8>>,
    start: Option<usize>,
    end: Option<usize>,
    is_object: bool,
    _phantom: PhantomData<VectorFeature<M, P, D>>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > JSONReader<T, M, P, D>
{
    /// Create a new JSONReader
    pub fn new(reader: T, chunk_size: Option<usize>) -> JSONReader<T, M, P, D> {
        let length = reader.len();
        let mut json_reader = JSONReader {
            reader,
            chunk_size: chunk_size.unwrap_or(65_536),
            buffer: vec![],
            offset: 0,
            length,
            pos: 0,
            brace_depth: 0,
            feature: vec![],
            start: None,
            end: None,
            is_object: true,
            _phantom: PhantomData,
        };

        // buffer the first chunk
        json_reader.buffer =
            json_reader.reader.slice(Some(0), Some(json_reader.chunk_size)).to_vec();
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
    fn set_start_position(&mut self) -> bool {
        let features = "\"features\":".as_bytes();
        let features_size = features.len();

        let mut k = 0;
        while self.pos < self.chunk_size {
            if features[k] == self.buffer[self.pos] {
                k += 1;
                self.pos += 1;
                if k == features_size {
                    return true;
                }
            } else {
                k = 0;
                self.pos += 1;
            }
        }
        // if we made it here, we need to read in the next buffer chunk.
        // If we hit the end of the file, return false
        self.offset += self.chunk_size;
        if self.offset < self.length {
            self.pos = 0;
            if self.offset + self.chunk_size < self.length {
                self.chunk_size = self.length - self.offset;
            }
            self.chunk_size = usize::min(65_536, self.length - self.offset);
            self.buffer = self.reader.slice(Some(self.offset), Some(self.offset + self.chunk_size));
            self.set_start_position()
        } else {
            false
        }
    }

    fn parse_line(&mut self, line: &str) -> Option<VectorFeature<M, P, D>> {
        if line.len() > 1 {
            if let Ok(feature) = line.to_features() {
                match feature {
                    WMFeature::Feature(feature) => {
                        return Some(feature.to_vector(Some(true)));
                    }
                    WMFeature::VectorFeature(vf) => {
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
    pub fn next_feature(&mut self) -> Option<VectorFeature<M, P, D>> {
        // get started
        while self.pos < self.chunk_size {
            if self.buffer[self.pos] == BACKSLASH {
                self.pos += 1;
            } else if self.buffer[self.pos] == STRING {
                self.is_object = !self.is_object;
            } else if self.buffer[self.pos] == LEFT_BRACE && self.is_object {
                if self.brace_depth == 0 {
                    self.start = Some(self.pos);
                }
                self.brace_depth += 1; // first brace is the start of the feature
            } else if self.buffer[self.pos] == RIGHT_BRACE && self.is_object {
                self.brace_depth -= 1; // if this hits zero, we are at the end of the feature
                if self.brace_depth == 0 {
                    self.end = Some(self.pos);
                    break;
                }
            }
            self.pos += 1;
        }

        // what if the last char in current buffer was a BACKSLASH?
        // we need to make sure in the next buffer we account for increment
        let increment_space = self.pos - self.chunk_size;

        if let (Some(start), Some(end)) = (self.start, self.end) {
            self.pos += 1;
            self.feature.push(self.buffer[start..end + 1].to_vec());
            let feature = self.feature.concat();
            // reset variables
            self.feature = vec![];
            self.start = None;
            self.end = None;
            self.brace_depth = 0;
            self.is_object = true;
            // convert feature to a &str and parse it
            let feature_str: String = String::from_utf8_lossy(&feature).to_string();
            self.parse_line(&feature_str)
        } else {
            // if offset isn't at filesize, increment buffer and start again
            if let Some(start) = self.start {
                self.feature.push(self.buffer[start..].to_vec());
                self.start = Some(0);
            }
            self.offset += self.chunk_size;
            if self.offset < self.length {
                self.pos = if increment_space > 0 { increment_space } else { 0 };
                if self.offset + self.chunk_size > self.length {
                    self.chunk_size = self.length - self.offset;
                }
                self.chunk_size = usize::min(65_536, self.length - self.offset);
                self.buffer =
                    self.reader.slice(Some(self.offset), Some(self.offset + self.chunk_size));
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
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > Iterator for JSONReader<T, M, P, D>
{
    type Item = VectorFeature<M, P, D>;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_feature()
    }
}
// Let the library know this struct is compatible as a VectorFeature iterator
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > FeatureIterator<M, P, D> for JSONReader<T, M, P, D>
{
}
