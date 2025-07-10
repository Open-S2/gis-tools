/// Common functions for all projections
pub mod common;
/// Convert tools
pub mod convert;
/// Geodesic tools
pub mod geodesic;
// /// Generated EPSG codes
// pub mod generated;
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
// pub use generated::*;
pub use geodesic::*;
pub use internal::*;
pub use parse::*;
pub use project::*;
pub use transform::*;

// TODO:
// - [ ] Migrate away from early converting to radians? Maybe instead always be degrees then when
//       pulling the ProjValue we also .to_radians(). The problem is user input is always degrees
