#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![feature(coverage_attribute)]
//! # GIS Tools - Library
//! TODO

extern crate alloc;

/// Common functions for all projections
pub mod common;
/// Convert tools
pub mod convert;
/// Geodesic tools
pub mod geodesic;
/// Projection internal tooling
pub mod internal;
/// Projection Parsing tools
pub mod parse;
/// Projection tools
pub mod project;
/// Transformation tools
pub mod transform;

pub use common::*;
pub use convert::*;
pub use internal::*;
pub use parse::*;
// pub use pipelines::*;
pub use project::*;
pub use transform::*;

/// A Projection Transform Definition
/// Temporary placeholder
#[derive(Debug)]
pub struct ProjectionTransformDefinition {}

/// Conversion trait for modifying a Point
pub trait CoordinateStep {
    /// Create a new Converter
    fn new(proj: &mut Proj) -> Self;
    /// forward conversion
    fn forward<P: TransformCoordinates>(&self, proj: &Proj, point: &mut P);
    /// inverse conversion
    fn inverse<P: TransformCoordinates>(&self, proj: &Proj, point: &mut P);
}
