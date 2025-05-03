#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # GIS Tools - Tools
//! TODO

extern crate alloc;

/// Fast and robust Delaunay triangulation
pub mod delaunator;
/// Handle travel from one point to another "as the crow flys" using Orthodromic projection
pub mod orthodrome;
/// Create polylabels for a collection of vector polygons or a single vector polygon
pub mod polylabel;

pub use delaunator::*;
pub use orthodrome::*;
pub use polylabel::*;
