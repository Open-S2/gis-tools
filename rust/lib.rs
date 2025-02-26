#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(f16)]
#![feature(more_float_constants)]
#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
#![warn(clippy::print_stdout)]
//! # GIS Tools
//! TODO

extern crate alloc;
extern crate pbf;

#[cfg(feature = "std")]
extern crate std;

/// Data structures
pub mod data_structures;
/// Geometry Tools
pub mod geometry;
/// GIS Readers
pub mod readers;
/// Space Tools
pub mod space;
/// Utility Tools
pub mod util;
