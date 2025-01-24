// #![no_std]
#![deny(missing_docs)]
#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
//! # GIS Tools
//! TODO

extern crate alloc;
extern crate pbf;

/// Geometry tools
pub mod geometry;

pub use geometry::*;
