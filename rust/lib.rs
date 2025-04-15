#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(f16)]
#![feature(more_float_constants)]
#![feature(stmt_expr_attributes)]
#![feature(trivial_bounds)]
#![feature(coverage_attribute)]
#![feature(let_chains)]
#![warn(clippy::print_stdout)]
#![warn(missing_debug_implementations)]
//! # GIS Tools
//! TODO

extern crate alloc;
extern crate pbf;

#[cfg(feature = "std")]
extern crate std;

/// Conversion Tools
pub mod converter;
/// Data Storage Tools
pub mod data_store;
/// Data structures
pub mod data_structures;
/// Geometry Tools
pub mod geometry;
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
