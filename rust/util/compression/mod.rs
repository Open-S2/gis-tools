/// Flate decompression (gzip, inflate, inflate-raw)
pub mod fflate;

pub use fflate::*;

#[cfg(feature = "std")]
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
#[cfg(feature = "std")]
use ruzstd::decoding::StreamingDecoder;
#[cfg(feature = "std")]
use ruzstd::io::Read;
#[cfg(feature = "std")]
use std::io::Write;

use alloc::vec::Vec;

/// Compression formats
#[derive(Debug, Clone, Copy)]
pub enum CompressionFormat {
    /// No compression
    None,
    /// Gzip
    Gzip,
    /// Brotli
    Brotli,
    /// Zstd
    Zstd,
}

/// Compresses data using the specified format
#[cfg(feature = "std")]
pub fn compress_data(input: &[u8], format: CompressionFormat) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip => {
            let mut encoder = GzEncoder::new(&mut output, Compression::default());
            encoder.write_all(input)?;
            encoder.finish()?;
        }
        CompressionFormat::Brotli => {
            let mut encoder = brotli::CompressorWriter::new(&mut output, 4096, 11, 22);
            encoder.write_all(input)?;
        }
        CompressionFormat::Zstd => {
            panic!("zstd not implemented");
        }
    }

    Ok(output)
}

/// Decompress data using the specified format
#[cfg(feature = "std")]
pub fn decompress_data(input: &[u8], format: CompressionFormat) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip => {
            let mut decoder = GzDecoder::new(input);
            decoder.read_to_end(&mut output)?;
        }
        CompressionFormat::Brotli => {
            let mut decoder = brotli::Decompressor::new(input, 4096);
            _ = decoder.read_to_end(&mut output);
        }
        CompressionFormat::Zstd => {
            let mut decoder = StreamingDecoder::new(input).unwrap();
            decoder.read_to_end(&mut output).unwrap();
        }
    }

    Ok(output)
}

/// Decompress data using the specified format
#[cfg(not(feature = "std"))]
pub fn decompress_data(input: &[u8], format: CompressionFormat) -> std::io::Result<Vec<u8>> {
    let mut output = Vec::new();

    match format {
        CompressionFormat::None => output.extend_from_slice(input),
        CompressionFormat::Gzip => {
            output = decompress_sync(input, None);
        }
        CompressionFormat::Brotli => {
            panic!("brotli not implemented");
        }
        CompressionFormat::Zstd => {
            panic!("zstd not implemented");
        }
    }

    Ok(output)
}
