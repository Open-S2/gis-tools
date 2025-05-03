#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # GIS Tools - Util
//! TODO

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Compression algorithms
pub mod compression;
/// Javascript Date like object
pub mod date;
/// Image based Processing
pub mod image;
/// Interpolation tools
pub mod interpolation;

pub use compression::*;
pub use date::*;
pub use image::*;
pub use interpolation::*;
