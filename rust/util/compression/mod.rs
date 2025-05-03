/// Flate decompression (gzip, inflate, inflate-raw)
pub mod fflate;

use crate::parsers::{BufferReader, Reader};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::result::Result;
pub use fflate::*;
#[cfg(feature = "std")]
use flate2::{
    Compression,
    read::{DeflateDecoder, GzDecoder, ZlibDecoder},
    write::{DeflateEncoder, GzEncoder, ZlibEncoder},
};
#[cfg(feature = "std")]
use ruzstd::io::Read;
use s2_tilejson::Encoding;
#[cfg(feature = "std")]
use std::io::Write;
// #[cfg(feature = "std")]
// use ruzstd::decoding::StreamingDecoder;

/// Handles compression errors
#[derive(Debug, PartialEq)]
pub enum CompressError {
    /// Brotli not implemented
    UnimplementedBrotli,
    /// Zstd not implemented
    UnimplementedZstd,
    /// Errors from the FFlate library
    FFlate(FFlateError),
    ///  Zipped Folder has a bad format
    BadZipFormat,
    ///  Zipped Folder has multiple disks
    ZipMultiDiskNotSupported,
    /// Invalid compression method
    InvalidCompressionMethod,
    /// Read error
    ReadError,
    /// Write error
    WriteError,
    /// Other
    Other,
}
impl From<FFlateError> for CompressError {
    fn from(err: FFlateError) -> Self {
        CompressError::FFlate(err)
    }
}

/// Compression formats
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CompressionFormat {
    /// No compression
    #[default]
    None = 1,
    /// Gzip
    Gzip = 2,
    /// Brotli
    Brotli = 3,
    /// Zstd
    Zstd = 4,
    /// Deflate
    Deflate = 5,
    /// Deflate raw
    DeflateRaw = 6,
}
impl From<u8> for CompressionFormat {
    fn from(value: u8) -> Self {
        match value {
            2 => CompressionFormat::Gzip,
            3 => CompressionFormat::Brotli,
            4 => CompressionFormat::Zstd,
            5 => CompressionFormat::Deflate,
            6 => CompressionFormat::DeflateRaw,
            _ => CompressionFormat::None,
        }
    }
}
impl From<&Encoding> for CompressionFormat {
    fn from(encoding: &Encoding) -> Self {
        match encoding {
            Encoding::Gzip => CompressionFormat::Gzip,
            Encoding::Brotli => CompressionFormat::Brotli,
            Encoding::Zstd => CompressionFormat::Zstd,
            _ => CompressionFormat::None,
        }
    }
}
impl From<CompressionFormat> for u8 {
    fn from(compression: CompressionFormat) -> Self {
        match compression {
            CompressionFormat::None => 1,
            CompressionFormat::Gzip => 2,
            CompressionFormat::Brotli => 3,
            CompressionFormat::Zstd => 4,
            CompressionFormat::Deflate => 5,
            CompressionFormat::DeflateRaw => 6,
        }
    }
}
impl From<&str> for CompressionFormat {
    fn from(s: &str) -> Self {
        match s {
            "gzip" => CompressionFormat::Gzip,
            "deflate" => CompressionFormat::Deflate,
            "deflate-raw" => CompressionFormat::DeflateRaw,
            "brotli" => CompressionFormat::Brotli,
            "zstd" => CompressionFormat::Zstd,
            _ => CompressionFormat::None,
        }
    }
}
impl From<CompressionFormat> for String {
    fn from(s: CompressionFormat) -> Self {
        match s {
            CompressionFormat::None => "none".into(),
            CompressionFormat::Gzip => "gzip".into(),
            CompressionFormat::Deflate => "deflate".into(),
            CompressionFormat::DeflateRaw => "deflate-raw".into(),
            CompressionFormat::Brotli => "brotli".into(),
            CompressionFormat::Zstd => "zstd".into(),
        }
    }
}

/// Compresses data using the specified format
#[cfg(feature = "std")]
pub fn compress_data(input: Vec<u8>, format: CompressionFormat) -> Result<Vec<u8>, CompressError> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output = input,
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(&mut output, Compression::default());
            encoder.write_all(&input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(&mut output, Compression::default());
            encoder.write_all(&input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::DeflateRaw => {
            let mut encoder = ZlibEncoder::new(&mut output, Compression::default());
            encoder.write_all(&input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::Brotli => {
            let mut encoder = brotli::CompressorWriter::new(&mut output, 4096, 11, 22);
            encoder.write_all(&input).map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::Zstd => return Err(CompressError::UnimplementedZstd),
    }

    Ok(output)
}

/// Decompress data using the specified format
#[cfg(feature = "std")]
pub fn decompress_data(input: &[u8], format: CompressionFormat) -> Result<Vec<u8>, CompressError> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(input);
            decoder.read_to_end(&mut output).map_err(|_| CompressError::ReadError)?;
        }
        CompressionFormat::Deflate => {
            let mut decoder = DeflateDecoder::new(input);
            decoder.read_to_end(&mut output).map_err(|_| CompressError::ReadError)?;
        }
        CompressionFormat::DeflateRaw => {
            let mut decoder = ZlibDecoder::new(input);
            decoder.read_to_end(&mut output).map_err(|_| CompressError::ReadError)?;
        }
        CompressionFormat::Brotli => {
            let mut decoder = brotli::Decompressor::new(input, 4096);
            _ = decoder.read_to_end(&mut output);
        }
        CompressionFormat::Zstd => return Err(CompressError::UnimplementedZstd),
        // CompressionFormat::Zstd => {
        //     panic!("zstd not implemented");
        //     // let mut decoder = StreamingDecoder::new(input).unwrap();
        //     // decoder.read_to_end(&mut output).unwrap();
        // }
    }

    Ok(output)
}

/// Decompress data using the specified format
#[cfg(not(feature = "std"))]
pub fn decompress_data(input: &[u8], format: CompressionFormat) -> Result<Vec<u8>, CompressError> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip | CompressionFormat::Deflate | CompressionFormat::DeflateRaw => {
            output = decompress_fflate(input, None)?;
        }
        CompressionFormat::Brotli => {
            return Err(CompressError::UnimplementedBrotli);
        }
        CompressionFormat::Zstd => {
            return Err(CompressError::UnimplementedZstd);
        }
    }

    Ok(output)
}

/// Represents a zip item
#[allow(missing_debug_implementations)]
pub struct ZipItem<'a> {
    /// The file name
    pub filename: String,
    /// The file comment
    pub comment: String,
    /// If the user wants to read the contents of the file, they can use this function and it will unzip it
    pub read: Box<dyn Fn() -> Result<Vec<u8>, CompressError> + Send + Sync + 'a>,
}

/// Iterates through the items in a zip file
pub fn iter_zip_folder(raw: &[u8]) -> Result<Vec<ZipItem<'_>>, CompressError> {
    let mut at = find_end_central_directory(raw)? as u64;
    let mut items = Vec::new();
    let reader: BufferReader = raw.into();

    // Read end central directory
    let file_count = reader.uint16_le(Some(10 + at));
    if file_count != reader.uint16_le(Some(8 + at)) {
        return Err(CompressError::ZipMultiDiskNotSupported);
    }
    let central_directory_start = reader.uint32_le(Some(16 + at));
    at = central_directory_start as u64;

    // Read central directory
    for _ in 0..file_count {
        let compression_method = reader.uint16_le(Some(10 + at)) as usize;
        let filename_length = reader.uint16_le(Some(28 + at)) as usize;
        let extra_fields_length = reader.uint16_le(Some(30 + at)) as usize;
        let comment_length = reader.uint16_le(Some(32 + at)) as usize;
        let compressed_size = reader.uint32_le(Some(20 + at)) as usize;

        // Find local entry location
        let local_entry_at = reader.uint32_le(Some(42 + at)) as usize;

        // Read buffers, move at to after entry, and store where we were
        let filename =
            String::from_utf8_lossy(&raw[(at + 46) as usize..(at as usize + 46 + filename_length)])
                .to_string();
        let comment = String::from_utf8_lossy(
            &raw[at as usize + 46 + filename_length + extra_fields_length
                ..at as usize + 46 + filename_length + extra_fields_length + comment_length],
        )
        .to_string();

        let next_central_directory_entry = at;

        // >> Start reading entry
        at = local_entry_at as u64;

        // This is the local entry (filename + extra fields) length, which we skip
        let bytes_start = at as usize
            + 30
            + reader.uint16_le(Some(26 + at)) as usize
            + reader.uint16_le(Some(28 + at)) as usize;
        let bytes_end = at as usize + compressed_size + bytes_start;
        let bytes = &raw[bytes_start..bytes_end];

        let read_fn = Box::new(move || {
            if compression_method & 8 > 0 {
                decompress_fflate(bytes, None).map_err(|_| CompressError::ReadError)
            } else if compression_method > 0 {
                Err(CompressError::InvalidCompressionMethod)
            } else {
                Ok(bytes.to_vec())
            }
        });

        items.push(ZipItem { filename, comment, read: read_fn });

        at = next_central_directory_entry;
    }

    Ok(items)
}

fn find_end_central_directory(raw: &[u8]) -> Result<usize, CompressError> {
    let mut search = raw.len() - 20;
    // let bounds = usize::max(search - 65516, 2); // Sub 2**256 - 20 (max comment length)
    let bounds = if search > 65516 { usize::max(search - 65516, 2) } else { 2 };

    while search > bounds {
        if raw[search..search + 4] == [0x50, 0x4b, 0x05, 0x06] {
            return Ok(search);
        }

        search = raw[..search].iter().rposition(|&x| x == 0x50).unwrap_or(bounds);
    }

    Err(CompressError::BadZipFormat)
}
