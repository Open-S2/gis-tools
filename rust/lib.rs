#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::collapsible_if)]
//! # GIS Tools
//! TODO

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Data Storage Tools
pub mod data_store;
/// Data structures
pub mod data_structures;
/// Geometry Tools
pub mod geometry;
/// GIS Core Tools
pub mod parsers;
/// Projection Tools
pub mod proj;
/// GIS Readers
pub mod readers;
/// Space Tools
pub mod space;
/// Generic Geospatial Tools
pub mod tools;
/// Utility Tools
pub mod util;
/// GIS Writers
pub mod writers;
