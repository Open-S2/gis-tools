#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(f16)]
#![feature(more_float_constants)]
#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
//! # GIS Tools
//! TODO

extern crate alloc;
extern crate pbf;

#[cfg(feature = "std")]
extern crate std;

/// Geometry tools
pub mod geometry;
/// GIS readers
pub mod readers;
/// Space tools
pub mod space;
/// Utility tools
pub mod util;
