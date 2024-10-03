#![no_std]
// #![deny(missing_docs)]
//! The `s2geometry` Rust crate... TODO

extern crate alloc;

// use geometry::lonlat;
use geometry::s2;

use alloc::boxed::Box;
use s2::cellid::S2CellId;
// use lonlat::LonLat;

#[cfg(target_arch = "wasm32")]
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};

// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

#[cfg(target_arch = "wasm32")]
mod wasm_specific {
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {}
    }
}

#[no_mangle]
pub extern "C" fn from_face_st(face: u8, s: f64, t: f64) -> *mut S2CellId {
    // Create a new S2CellId instance and box it
    let cell = Box::new(S2CellId::from_face_st(face, s, t));
    // Convert the boxed value into a raw pointer and return it
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn low_bits(ptr: *const S2CellId) -> u32 {
    let ptr = &*ptr;
    ptr.low_bits()
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn high_bits(ptr: *const S2CellId) -> u32 {
    let ptr = &*ptr;
    ptr.high_bits()
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn compare_s2_cell_id(id1: *const S2CellId, id2: *const S2CellId) -> i32 {
    let id1 = &*id1;
    let id2 = &*id2;
    if id1 < id2 {
        -1
    } else if id1 == id2 {
        0
    } else {
        1
    }
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn free_s2_cell_id(ptr: *mut S2CellId) {
    // Safety: Convert the raw pointer back into a Box and drop it to free the memory
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}
