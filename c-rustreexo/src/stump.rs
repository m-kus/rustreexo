use crate::{get_safe_ty, CHash, Error, EXIT_FAILURE, EXIT_SUCCESS};
use bitcoin_hashes::{sha256, Hash};
use rustreexo::accumulator::{proof::Proof, stump::Stump};
use std::alloc::{alloc, Layout};

#[no_mangle]
pub extern "C" fn rustreexo_stump_debug_print(stump: *mut Stump) -> usize {
    check_ptr!(stump);
    let stump = unsafe { stump.as_ref().unwrap() };
    println!("{stump:?}");
    return EXIT_SUCCESS;
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
