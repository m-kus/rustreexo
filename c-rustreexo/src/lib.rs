use bitcoin_hashes::{sha256, Hash};
use rustreexo::accumulator::{proof::Proof, stump::Stump};
use std::{
    alloc::{alloc, Layout},
    ops::Deref,
    vec,
};

/// A raw hash passed from C, this is internally identical to [sha256::Hash] except that we use
/// use a repr(C) to make it compatible to how C stores data. Shouldn't be used anywhere else
#[repr(C)]
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
}

const EXIT_SUCCESS: usize = 1;
const EXIT_FAILURE: usize = 0;

macro_rules! check_ptr {
    ($ptr: ident) => {
        if $ptr.is_null() {
            return EXIT_FAILURE;
        }
    };
    ($errno: ident , $ptr: ident) => {
        if $ptr.is_null() {
            unsafe {
                *$errno = Error::NullPointer;
            }
            return EXIT_FAILURE;
        }
    };
    ($errno: ident,$ptr: ident, $count: ident) => {
        if $ptr.is_null() && $count > 0 {
            unsafe {
                *$errno = Error::NullPointer;
            }
            return EXIT_FAILURE;
        }
    };
}
macro_rules! propagate_error {
    ($errno: ident, $opt: ident) => {
        if let Err(e) = $opt {
            unsafe {
                *$errno = e;
                return EXIT_FAILURE;
            }
        }
    };
}

fn get_safe_ty<T>(thing: *mut T) -> T {
    unsafe { thing.read() }
}
fn get_slice<'a, T>(slice: *mut T, length: usize) -> &'a [T] {
    unsafe { std::slice::from_raw_parts(slice, length) }
}
fn alloc_and_set<T>(dst: *mut *mut T, new_value: T) -> Result<(), Error> {
    let layout = Layout::new::<T>();
    let ptr = unsafe { alloc(layout) as *mut T };
    if ptr.is_null() {
        return Err(Error::AllocationError);
    }
    unsafe {
        *ptr.as_mut().unwrap() = new_value;
        dst.replace(ptr);
    }
    Ok(())
}
#[no_mangle]
pub extern "C" fn rustreexo_stump_modify(
    errno: *mut Error,
    stump: *mut Stump,
    utxos: *mut CHash,
    n_hashes: usize,
) -> usize {
    check_ptr!(errno);
    check_ptr!(errno, stump);
    check_ptr!(errno, utxos, n_hashes);

    // Build a safe vector form a C array
    let utxos = unsafe {
        std::slice::from_raw_parts(utxos, n_hashes)
            .iter()
            .map(|slice| sha256::Hash::from_inner(**slice))
            .collect()
    };

    let r_stump = get_safe_ty(stump);
    let s = r_stump.modify(&utxos, &vec![], &Proof::default());
    if let Ok(s) = s {
        unsafe {
            stump.write(s.0);
        }

        return EXIT_SUCCESS;
    }
    return EXIT_FAILURE;
}

#[no_mangle]
pub extern "C" fn rustreexo_stump_create() -> *mut Stump {
    let layout = Layout::new::<Stump>();
    let ptr = unsafe { alloc(layout) as *mut Stump };

    unsafe {
        *ptr.as_mut().unwrap() = Stump::new();
    }
    ptr
}

#[no_mangle]
pub extern "C" fn rustreexo_stump_debug_print(stump: *mut Stump) -> usize {
    check_ptr!(stump);
    let stump = unsafe { stump.as_ref().unwrap() };
    println!("{stump:?}");
    return EXIT_SUCCESS;
}

#[no_mangle]
pub extern "C" fn rustreexo_proof_create(
    errno: *mut Error,
    ret: *mut *mut Proof,
    targets: *mut u64,
    n_targets: usize,
    hashes: *mut CHash,
    n_hashes: usize,
) -> usize {
    check_ptr!(errno);
    check_ptr!(errno, targets, n_targets);
    check_ptr!(errno, hashes, n_hashes);
    let targets = get_slice(targets, n_targets);
    let hashes = get_slice(hashes, n_hashes);
    let hashes = hashes
        .iter()
        .map(|hash| sha256::Hash::from_inner(**hash))
        .collect::<Vec<_>>();
    let proof = Proof::new(targets.to_vec(), hashes);
    let result = alloc_and_set(ret, proof);
    propagate_error!(errno, result);

    EXIT_SUCCESS
}
