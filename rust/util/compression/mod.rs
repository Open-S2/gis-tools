/// Flate decompression (gzip, inflate, inflate-raw)
pub mod fflate;

pub use fflate::*;

#[cfg(feature = "std")]
use flate2::{
    read::{DeflateDecoder, GzDecoder, ZlibDecoder},
    write::{DeflateEncoder, GzEncoder, ZlibEncoder},
    Compression,
};
// #[cfg(feature = "std")]
// use ruzstd::decoding::StreamingDecoder;
#[cfg(feature = "std")]
use ruzstd::io::Read;
#[cfg(feature = "std")]
use std::io::Write;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use core::result::Result;

use crate::readers::{BufferReader, Reader};

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CompressionFormat {
    /// No compression
    None,
    /// Gzip
    Gzip,
    /// Deflate
    Deflate,
    /// Deflate raw
    DeflateRaw,
    /// Brotli
    Brotli,
    /// Zstd
    Zstd,
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

/// Compresses data using the specified format
#[cfg(feature = "std")]
pub fn compress_data(input: &[u8], format: CompressionFormat) -> Result<Vec<u8>, CompressError> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(&mut output, Compression::default());
            encoder.write_all(input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::Deflate => {
            let mut encoder = DeflateEncoder::new(&mut output, Compression::default());
            encoder.write_all(input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::DeflateRaw => {
            let mut encoder = ZlibEncoder::new(&mut output, Compression::default());
            encoder.write_all(input).map_err(|_| CompressError::WriteError)?;
            encoder.finish().map_err(|_| CompressError::WriteError)?;
        }
        CompressionFormat::Brotli => {
            let mut encoder = brotli::CompressorWriter::new(&mut output, 4096, 11, 22);
            encoder.write_all(input).map_err(|_| CompressError::WriteError)?;
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
            output = decompress_sync(input, None)?;
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
// #[cfg_attr(not(feature = "std"), derive(Debug))]
pub struct ZipItem<'a> {
    /// The file name
    pub filename: String,
    /// The file comment
    pub comment: String,
    /// If the user wants to read the contents of the file, they can use this function and it will unzip it
    // #[cfg_attr(not(feature = "std"), debug(skip))]
    pub read: Box<dyn Fn() -> Result<Vec<u8>, CompressError> + Send + Sync + 'a>,
}

/// Iterates through the items in a zip file
pub fn iter_items(raw: &[u8]) -> Result<Vec<ZipItem<'_>>, CompressError> {
    let mut at: usize = find_end_central_directory(raw)?;
    let mut items = Vec::new();
    let mut reader: BufferReader = raw.into();

    // Read end central directory
    let file_count = reader.uint16_le(Some(10 + at));
    if file_count != reader.uint16_le(Some(8 + at)) {
        return Err(CompressError::ZipMultiDiskNotSupported);
    }
    let central_directory_start = reader.uint32_le(Some(16 + at));
    at = central_directory_start as usize;

    // Read central directory
    for _ in 0..file_count {
        let compression_method = reader.uint16_le(Some(10 + at));
        let filename_length = reader.uint16_le(Some(28 + at));
        let extra_fields_length = reader.uint16_le(Some(30 + at));
        let comment_length = reader.uint16_le(Some(32 + at));
        let compressed_size = reader.uint32_le(Some(20 + at));

        // Find local entry location
        let local_entry_at = reader.uint32_le(Some(42 + at));

        // Read buffers, move at to after entry, and store where we were
        let filename =
            String::from_utf8_lossy(&raw[at + 46..at + 46 + filename_length as usize]).to_string();
        let comment = String::from_utf8_lossy(
            &raw[at + 46 + filename_length as usize + extra_fields_length as usize
                ..at + 46
                    + filename_length as usize
                    + extra_fields_length as usize
                    + comment_length as usize],
        )
        .to_string();

        let next_central_directory_entry = at;

        // >> Start reading entry
        at = local_entry_at as usize;

        // This is the local entry (filename + extra fields) length, which we skip
        let bytes_start = at
            + 30
            + reader.uint16_le(Some(26 + at)) as usize
            + reader.uint16_le(Some(28 + at)) as usize;
        let bytes_end = at + compressed_size as usize + bytes_start;
        let bytes = &raw[bytes_start..bytes_end];

        let read_fn = Box::new(move || {
            if compression_method & 8 > 0 {
                decompress_sync(bytes, None).map_err(|_| CompressError::ReadError)
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn encode_decode_none() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(&expected, CompressionFormat::None).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::None).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_gzip() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(&expected, CompressionFormat::Gzip).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Gzip).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_deflate() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(&expected, CompressionFormat::Deflate).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Deflate).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_deflate_raw() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(&expected, CompressionFormat::DeflateRaw).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::DeflateRaw).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_brotli() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let encoded = compress_data(&expected, CompressionFormat::Brotli).unwrap();
        // decode
        let decoded = decompress_data(&encoded, CompressionFormat::Brotli).unwrap();

        assert_eq!(decoded, expected);
    }

    #[test]
    fn encode_decode_zstd() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/lorem_en_100k.txt");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // encode
        let _encoded = compress_data(&expected, CompressionFormat::Zstd).unwrap_err();
        // assert_eq!(encoded, CompressError::UnimplementedZstd);
        // decode
        let _decoded = decompress_data(&expected, CompressionFormat::Zstd).unwrap_err();

        // assert_eq!(decoded, expected);
    }

    #[test]
    fn test_compression_format() {
        let compression_format = CompressionFormat::from("gzip");
        assert_eq!(compression_format, CompressionFormat::Gzip);

        let compression_format = CompressionFormat::from("deflate");
        assert_eq!(compression_format, CompressionFormat::Deflate);

        let compression_format = CompressionFormat::from("deflate-raw");
        assert_eq!(compression_format, CompressionFormat::DeflateRaw);

        let compression_format = CompressionFormat::from("brotli");
        assert_eq!(compression_format, CompressionFormat::Brotli);

        let compression_format = CompressionFormat::from("zstd");
        assert_eq!(compression_format, CompressionFormat::Zstd);

        let compression_format = CompressionFormat::from("none");
        assert_eq!(compression_format, CompressionFormat::None);
    }

    #[test]
    fn test_compressor_err_from_fflate() {
        let err: CompressError = FFlateError::InvalidBlockType.into();
        assert_eq!(err, CompressError::FFlate(FFlateError::InvalidBlockType));
    }

    #[test]
    fn decode_zip_folder() {
        // get expected
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/util/fixtures/utf.zip");
        let expected: Vec<u8> = fs::read(&path).expect("Failed to read file expected");

        // get files
        let items = iter_items(&expected).unwrap();
        assert_eq!(items.len(), 6);
        // check each string
        // pull out all filenames
        let filenames: Vec<String> = items.iter().map(|item| item.filename.to_string()).collect();
        assert_eq!(
            filenames,
            vec!["utf.cpg", "utf.cpg", "utf.cpg", "utf.cpg", "utf.cpg", "utf.cpg"]
        );
        let first = items.first().unwrap();
        let first_data = (first.read)().unwrap();

        assert_eq!(first_data, vec![85, 84, 70, 45, 56]);
        // convert first_dat to string
        let first_string = String::from_utf8(first_data).unwrap();
        assert_eq!(first_string, "UTF-8");
    }
}
