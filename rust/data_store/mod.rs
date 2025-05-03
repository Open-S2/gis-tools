/// Sort files with key-value file pairs that are stored in files. Keys are uint64s.
#[cfg(feature = "std")]
pub mod external_sort;
/// File based reader used by KV, MultiMap, and Vector
#[cfg(feature = "std")]
pub mod file;
/// Key-Value Store (KV)
pub mod kv;
/// MultiMap - Key with multiple values Store
pub mod multimap;
/// Vector Store - Essentially a Vec that enforces an id-value pairing
pub mod vector;

use crate::geometry::S2CellId;
use core::cmp::Ord;
#[cfg(feature = "std")]
pub use external_sort::*;
pub use kv::*;
pub use multimap::*;
pub use vector::*;

/// U64 is a type that can be converted to and from u64
pub trait U64: Ord + Copy + Clone + Default + From<u64> + Into<u64> {}
impl U64 for u64 {}
impl U64 for S2CellId {}
