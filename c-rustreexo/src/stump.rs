use crate::{get_safe_ty, get_slice, CHash, Error, EXIT_FAILURE, EXIT_SUCCESS};
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
    utxos_len: usize,
    del_hashes: *mut CHash,
    del_hashes_len: usize,
    proof: *mut Proof,
) -> usize {
    check_ptr!(errno);
    check_ptr!(errno, stump);
    check_ptr!(errno, utxos, utxos_len);
    check_ptr!(errno, del_hashes, del_hashes_len);
    check_ptr!(errno, proof);

    // Build a safe vector form a C array
    let utxos = get_slice(utxos, utxos_len);
    let utxos = utxos
        .iter()
        .map(|slice| sha256::Hash::from_inner(**slice))
        .collect();

    let del_hashes = get_slice(del_hashes, del_hashes_len);
    let del_hashes = del_hashes
        .iter()
        .map(|hash| sha256::Hash::from_inner(**hash))
        .collect::<Vec<_>>();

    let proof = get_safe_ty(proof);
    let r_stump = get_safe_ty(stump);
    let s = r_stump.modify(&utxos, &del_hashes, &proof);
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
