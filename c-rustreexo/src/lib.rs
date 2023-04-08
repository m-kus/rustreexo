use bitcoin_hashes::{sha256, Hash};
use std::ops::Deref;

/// A raw hash passed from C, this is internally identical to [sha256::Hash] except that we use
/// use a repr(C) to make it compatible to how C stores data. Shouldn't be used anywhere else
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CHash([u8; 32]);

impl Deref for CHash {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = [u8; 32];
}
impl Into<sha256::Hash> for CHash {
    fn into(self) -> sha256::Hash {
        sha256::Hash::from_inner(*self)
    }
}

#[repr(C)]
pub enum Error {
    None = 0,
    NullPointer = 1,
    InvalidSlice = 2,
    UtreexoError = 3,
    AllocationError = 4,
    InvalidProof = 5,
}

pub const EXIT_SUCCESS: usize = 1;
pub const EXIT_FAILURE: usize = 0;

macro_rules! check_ptr {
    ($ptr: ident) => {
        if $ptr.is_null() {
            return crate::EXIT_FAILURE;
        }
    };
    ($errno: ident , $ptr: ident) => {
        if $ptr.is_null() {
            unsafe {
                *$errno = Error::NullPointer;
            }
            return crate::EXIT_FAILURE;
        }
    };
    ($errno: ident,$ptr: ident, $count: ident) => {
        if $ptr.is_null() && $count > 0 {
            unsafe {
                *$errno = Error::NullPointer;
            }
            return crate::EXIT_FAILURE;
        }
    };
}

fn get_safe_ty<T>(thing: *mut T) -> &'static T {
    unsafe { thing.as_ref().unwrap() }
}
fn get_owned<T>(thing: *mut T) -> Box<T> {
    unsafe { Box::from_raw(thing) }
}
fn get_slice<'a, T>(slice: *mut T, length: usize) -> &'a [T] {
    if length == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(slice, length) }
}
fn get_slice_const<'a, T>(slice: *const T, length: usize) -> &'a [T] {
    if length == 0 {
        return &[];
    }
    unsafe { std::slice::from_raw_parts(slice, length) }
}

pub mod misc;
pub mod proof;
pub mod stump;
