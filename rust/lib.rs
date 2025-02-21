#![no_std]
#![deny(missing_docs)]
#![feature(f16)]
#![feature(more_float_constants)]
#![feature(stmt_expr_attributes)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
//! # GIS Tools
//! TODO

extern crate alloc;
extern crate pbf;

/// Geometry tools
pub mod geometry;
/// GIS readers
pub mod readers;
/// Space tools
pub mod space;
