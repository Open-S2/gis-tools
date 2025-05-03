#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::collapsible_if)]
//! # GIS Tools - Data Structures
//! TODO

extern crate alloc;

/// Cache System with a max size
pub mod cache;
/// Point Cluster
pub mod point_cluster;
/// Point Grid
pub mod point_grid;
/// Point Index
pub mod point_index;
/// Priority Queue
pub mod priority_queue;
/// Tile Structure
pub mod tile;

pub use cache::*;
pub use point_cluster::*;
pub use point_grid::*;
pub use point_index::*;
pub use priority_queue::*;
pub use tile::*;
