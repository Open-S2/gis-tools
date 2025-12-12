use crate::{parsers::Reader, util::fetch_url};
use core::cell::RefCell;

/// # Fetch Reader
///
/// ## Description
/// The browser reader that fetches data from a URL.
///
/// Implements the [`Reader`] trait.
///
/// Useful for [`crate::readers::PMTilesReader`] and [`crate::readers::S2TilesReader`]
///
/// ## Usage
/// ```rust,no_run
/// use gistools::{
///     parsers::FetchReader,
///     readers::PMTilesReader,
/// };
/// use s2_tilejson::{Encoding, Metadata, Scheme, SourceType};
///
/// let url = "...".into();
/// let mut reader = PMTilesReader::new(FetchReader::new(url, true), None);
///
/// let metadata = reader.get_metadata();
/// assert_eq!(
///     *metadata,
///     Metadata {
///         s2tilejson: "1.0.0".into(),
///         version: "1.0.0".into(),
///         name: "default".into(),
///         scheme: Scheme::Fzxy,
///         description: "Built with s2maps-cli".into(),
///         r#type: SourceType::Vector,
///         extension: "pbf".into(),
///         encoding: Encoding::None,
///         minzoom: 0,
///         maxzoom: 27,
///         ..Default::default()
///     }
/// );
/// ```
#[derive(Debug, Clone)]
pub struct FetchReader {
    path: String,
    range_requests: bool,
    cursor: RefCell<u64>,
}
impl FetchReader {
    /// Create a new FetchReader
    ///
    /// ## Parameters
    /// - `path`: the location of the PMTiles data
    /// - `range_requests`: FetchReader specific; enable range requests or use urlParam "bytes"
    pub fn new(path: String, range_requests: bool) -> Self {
        Self { path, range_requests, cursor: 0.into() }
    }
}
impl Reader for FetchReader {
    fn len(&self) -> u64 {
        0
    }

    fn uint64(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> u64 {
        0
    }

    fn uint64_be(&self, _byte_offset: Option<u64>) -> u64 {
        0
    }

    fn uint64_le(&self, _byte_offset: Option<u64>) -> u64 {
        0
    }

    fn int64(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> i64 {
        0
    }

    fn int64_be(&self, _byte_offset: Option<u64>) -> i64 {
        0
    }

    fn int64_le(&self, _byte_offset: Option<u64>) -> i64 {
        0
    }

    fn f64(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> f64 {
        0.
    }

    fn f64_be(&self, _byte_offset: Option<u64>) -> f64 {
        0.
    }

    fn f64_le(&self, _byte_offset: Option<u64>) -> f64 {
        0.
    }

    fn uint32(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> u32 {
        0
    }

    fn uint32_be(&self, _byte_offset: Option<u64>) -> u32 {
        0
    }

    fn uint32_le(&self, _byte_offset: Option<u64>) -> u32 {
        0
    }

    fn int32(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> i32 {
        0
    }

    fn int32_be(&self, _byte_offset: Option<u64>) -> i32 {
        0
    }

    fn int32_le(&self, _byte_offset: Option<u64>) -> i32 {
        0
    }

    fn f32(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> f32 {
        0.
    }

    fn f32_be(&self, _byte_offset: Option<u64>) -> f32 {
        0.
    }

    fn f32_le(&self, _byte_offset: Option<u64>) -> f32 {
        0.
    }

    fn uint16(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> u16 {
        0
    }

    fn uint16_be(&self, _byte_offset: Option<u64>) -> u16 {
        0
    }

    fn uint16_le(&self, _byte_offset: Option<u64>) -> u16 {
        0
    }

    fn int16(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> i16 {
        0
    }

    fn int16_be(&self, _byte_offset: Option<u64>) -> i16 {
        0
    }

    fn int16_le(&self, _byte_offset: Option<u64>) -> i16 {
        0
    }

    fn f16(&self, _byte_offset: Option<u64>, _little_endian: Option<bool>) -> f32 {
        0.
    }

    fn f16_be(&self, _byte_offset: Option<u64>) -> f32 {
        0.
    }

    fn f16_le(&self, _byte_offset: Option<u64>) -> f32 {
        0.
    }

    fn uint8(&self, _byte_offset: Option<u64>) -> u8 {
        0
    }

    fn int8(&self, _byte_offset: Option<u64>) -> i8 {
        0
    }

    fn tell(&self) -> u64 {
        *self.cursor.borrow()
    }

    fn seek(&self, pos: u64) {
        *self.cursor.borrow_mut() = pos;
    }

    fn slice(&self, _begin: Option<u64>, _end: Option<u64>) -> Vec<u8> {
        vec![]
    }

    fn seek_slice(&self, _size: usize) -> Vec<u8> {
        vec![]
    }

    fn parse_string(&self, _byte_offset: Option<u64>, _byte_length: Option<u64>) -> String {
        "".into()
    }

    async fn get_slice(&self, byte_offset: u64, byte_length: Option<u64>) -> Vec<u8> {
        if self.range_requests {
            let bytes = format!(
                "{}-{}",
                byte_offset,
                byte_length.map(|l| (l + byte_offset - 1).to_string()).unwrap_or("".to_string())
            );
            fetch_url::<()>(&self.path, &[("Range", &format!("bytes={}", bytes))], None, None)
                .await
                .unwrap()
        } else {
            let bytes = format!(
                "{}-{}",
                byte_offset,
                byte_length.map(|l| (l + byte_offset).to_string()).unwrap_or("".to_string())
            );
            fetch_url::<()>(&format!("{}?bytes={}", self.path, bytes), &[], None, None)
                .await
                .unwrap()
        }
    }
}
