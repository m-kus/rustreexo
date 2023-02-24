use bitcoin_hashes::{sha256, Hash};
use rustreexo::accumulator::{proof::Proof, stump::Stump};

use crate::{alloc_and_set, get_safe_ty, get_slice, CHash, Error, EXIT_FAILURE, EXIT_SUCCESS};

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

#[no_mangle]
pub extern "C" fn rustreexo_proof_verify(
    errno: *mut Error,
    del_hashes: *mut CHash,
    n_dels: usize,
    proof: *mut Proof,
    stump: *mut Stump,
) -> usize {
    check_ptr!(errno);
    check_ptr!(errno, proof);
    check_ptr!(errno, stump);

    let proof = get_safe_ty(proof);
    let stump = get_safe_ty(stump);
    let hashes = get_slice(del_hashes, n_dels);
    let hashes = hashes
        .iter()
        .map(|hash| sha256::Hash::from_inner(**hash))
        .collect::<Vec<_>>();

    match proof.verify(&hashes, &stump) {
        Ok(valid) => {
            if valid {
                return EXIT_SUCCESS;
            }
            unsafe {
                *errno = Error::InvalidProof;
            }
        }
        Err(_) => unsafe {
            *errno = Error::UtreexoError;
        },
    }
    return EXIT_FAILURE;
}
