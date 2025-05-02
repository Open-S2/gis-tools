#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # GIS Tools
//! TODO

/// Data Storage Tools
pub mod data_store {
    pub use data_store::*;
}
/// Data structures
pub mod data_structures {
    pub use data_structures::*;
}
/// Geometry Tools
pub mod geometry {
    pub use geometry::*;
}
/// GIS Core Tools
pub mod parsers {
    pub use parsers::*;
}
/// Projection Tools
pub mod proj {
    pub use proj::*;
}
/// GIS Readers
pub mod readers {
    pub use readers::*;
}
/// Space Tools
pub mod space {
    pub use space::*;
}
/// Generic Geospatial Tools
pub mod tools {
    pub use tools::*;
}
/// Utility Tools
pub mod util {
    pub use util::*;
}
/// GIS Writers
pub mod writers {
    pub use writers::*;
}
