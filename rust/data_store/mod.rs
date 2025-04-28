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

#[cfg(feature = "std")]
pub use external_sort::*;
pub use kv::*;
pub use multimap::*;
pub use vector::*;

use crate::geometry::S2CellId;
use core::cmp::Ord;

/// U64 is a type that can be converted to and from u64
pub trait U64: Ord + Copy + Clone + Default + From<u64> + Into<u64> {}
impl U64 for u64 {}
impl U64 for S2CellId {}
impl From<S2CellId> for u64 {
    fn from(val: S2CellId) -> Self {
        val.id
    }
}

#[cfg(test)]
#[coverage(off)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::useless_conversion)]
    fn test_u64_u64() {
        let val: u64 = 12345;
        let converted: u64 = val.into();
        assert_eq!(converted, val);
    }

    #[test]
    fn test_u64_s2cellid() {
        let val: u64 = 12345;
        let cell_id: S2CellId = val.into();
        let converted: u64 = cell_id.into();
        assert_eq!(converted, val);
    }
}
