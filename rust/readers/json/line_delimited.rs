use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use s2json::{MValue, MValueCompatible, VectorFeature, WMFeature};

use alloc::{string::String, vec, vec::Vec};
use serde::de::DeserializeOwned;

use crate::{geometry::ConvertFeature, readers::Reader};

use super::{FeatureIterator, ToGisJSON};

/// # NewLine Delimited JSON Reader
pub struct NewLineDelimitedJSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: MValueCompatible + DeserializeOwned = MValue,
    D: MValueCompatible + DeserializeOwned = MValue,
> {
    reader: T,
    _phantom: PhantomData<VectorFeature<M, P, D>>,
    seperator: char, // default is '\n'
    cursor: usize,
    offset: usize,
    tmp_chunks: Vec<String>,
    partial_line: String,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > NewLineDelimitedJSONReader<T, M, P, D>
{
    /// Create a Newline-Delimited JSON Reader
    pub fn new(reader: T, seperator: Option<char>) -> NewLineDelimitedJSONReader<T, M, P, D> {
        NewLineDelimitedJSONReader {
            reader,
            _phantom: PhantomData,
            seperator: seperator.unwrap_or('\n'),
            cursor: 0,
            offset: 0,
            tmp_chunks: vec![],
            partial_line: String::new(),
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

    /// Get the next feature
    pub fn next_feature(&mut self) -> Option<VectorFeature<M, P, D>> {
        // 1) check if a line chunk exists
        if !self.tmp_chunks.is_empty() {
            let next = self.tmp_chunks.remove(0);
            return self.parse_line(&next);
        }
        // 2) check if we can build our next chunk
        if self.offset < self.reader.len() {
            let length = usize::min(65_536, self.reader.len() - self.cursor);
            // Prepend any partial line to the new chunk
            let chunk: String = self.partial_line.clone()
                + &self.reader.parse_string(Some(self.offset), Some(length));
            self.partial_line =
                if chunk.ends_with(self.seperator) { self.seperator.into() } else { "".into() };
            // Split the chunk by newlines and yield each complete line
            self.tmp_chunks = chunk
                .split(self.seperator)
                .filter(|line| !line.is_empty())
                .map(String::from)
                .collect();
            let next = self.tmp_chunks.remove(0);
            let res = self.parse_line(&next);
            // Store the remaining partial line for the next iteration
            if let Some(last_chunk) = self.tmp_chunks.last() {
                self.partial_line = last_chunk.clone() + &self.partial_line;
            }
            // Update the cursor and offset
            self.offset += length;
            self.cursor += length;
            return res;
        }

        // 3) Making it here means we have no more data to pull in, but we may have a partial line
        if !self.partial_line.is_empty() {
            let feature = self.parse_line(&self.partial_line.clone());
            self.partial_line = "".into();
            return feature;
        }

        // 4) If we make it here, there was nothing to parse
        None
    }
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > Iterator for NewLineDelimitedJSONReader<T, M, P, D>
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
    > FeatureIterator<M, P, D> for NewLineDelimitedJSONReader<T, M, P, D>
{
}

/// # Text Sequence JSON Reader
pub struct SequenceJSONReader<
    T: Reader,
    M: Clone + DeserializeOwned = (),
    P: MValueCompatible + DeserializeOwned = MValue,
    D: MValueCompatible + DeserializeOwned = MValue,
> {
    newline: NewLineDelimitedJSONReader<T, M, P, D>,
}
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > SequenceJSONReader<T, M, P, D>
{
    /// Create a new SequenceJSONReader
    pub fn new(reader: T) -> SequenceJSONReader<T, M, P, D> {
        SequenceJSONReader { newline: NewLineDelimitedJSONReader::new(reader, Some('␞')) }
    }
}

// Automatically expose NewLineDelimitedJSONReader's methods
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > Deref for SequenceJSONReader<T, M, P, D>
{
    type Target = NewLineDelimitedJSONReader<T, M, P, D>;

    fn deref(&self) -> &Self::Target {
        &self.newline
    }
}
// Automatically expose mutable access to NewLineDelimitedJSONReader's methods via DerefMut
impl<
        T: Reader,
        M: Clone + DeserializeOwned,
        P: MValueCompatible + DeserializeOwned,
        D: MValueCompatible + DeserializeOwned,
    > DerefMut for SequenceJSONReader<T, M, P, D>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.newline
    }
}
