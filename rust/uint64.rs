#![no_std]
// #![deny(missing_docs)]
//! The `uint64` Rust crate...

extern crate alloc;

use alloc::boxed::Box;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Uint64 {
    /// the id contains the face, s, and t components
    pub id: u64,
}
impl Uint64 {
    pub fn new(id: u64) -> Self {
        Self { id }
    }

    pub fn from_low_high(low: u32, high: u32) -> Self {
        Self {
            id: ((high as u64) << 32) + low as u64,
        }
    }
}

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
pub extern "C" fn from_low_high(low: u32, high: u32) -> *mut Uint64 {
    // Create a new Uint64 instance and box it
    let cell = Box::new(Uint64::from_low_high(low, high));
    // Convert the boxed value into a raw pointer and return it
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn add(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;

    let cell = Box::new(Uint64::new(id1.id + id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn sub(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;

    let cell = Box::new(Uint64::new(id1.id - id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn div(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;

    let cell = Box::new(Uint64::new(id1.id / id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn mul(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;

    let cell = Box::new(Uint64::new(id1.id * id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn shift_left(id1: *const Uint64, shift: u32) -> *mut Uint64 {
    let id1 = &*id1;
    let cell = Box::new(Uint64::new(id1.id << shift));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn shift_right(id1: *const Uint64, shift: u32) -> *mut Uint64 {
    let id1 = &*id1;
    let cell = Box::new(Uint64::new(id1.id >> shift));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn bit_and(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;
    let cell = Box::new(Uint64::new(id1.id & id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn bit_or(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;
    let cell = Box::new(Uint64::new(id1.id | id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn bit_xor(id1: *const Uint64, id2: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let id2 = &*id2;
    let cell = Box::new(Uint64::new(id1.id ^ id2.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn bit_not(id1: *const Uint64) -> *mut Uint64 {
    let id1 = &*id1;
    let cell = Box::new(Uint64::new(!id1.id));
    Box::into_raw(cell)
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn low_bits(id1: *const Uint64) -> u32 {
    let id1 = &*id1;
    id1.id as u32
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn high_bits(id1: *const Uint64) -> u32 {
    let id1 = &*id1;
    ((id1.id >> 32) & 0xffffffff) as u32
}

/// # Safety
///
/// This function is not safe, but it's only used in wasm
#[no_mangle]
pub unsafe extern "C" fn compare_uint64(id1: *const Uint64, id2: *const Uint64) -> i32 {
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
pub unsafe extern "C" fn free_uin64(ptr: *mut Uint64) {
    // Safety: Convert the raw pointer back into a Box and drop it to free the memory
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}
