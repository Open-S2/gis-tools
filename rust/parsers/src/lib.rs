#![no_std]
#![deny(missing_docs)]
#![feature(coverage_attribute)]
#![feature(f16)]
//! # GIS Tools - Core
//! Components that are shared between modules
//! TODO

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Generic reading tools used across modules
pub mod read;
/// WKT Parsing of various formats
pub mod wkt;
/// Generic writing tools used across modules
pub mod write;
/// XML Parser
pub mod xml;

pub use read::*;
pub use wkt::*;
pub use write::*;
pub use xml::*;
