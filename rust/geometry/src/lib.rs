#![no_std]
// #![deny(missing_docs)]
//! The `s2geometry` Rust crate... TODO

pub mod planets;
pub mod s1;
pub mod lonlat;
pub mod s2;

// TODO:
// * s2point index (sorts as an array in-place for memory, otherwise use another approach for disk)
// * s2shape index (sorts as you add regardless of storage type)
// * To S2_JSON
// * to db
// * * 