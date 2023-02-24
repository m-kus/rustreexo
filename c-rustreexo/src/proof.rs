use bitcoin_hashes::{sha256, Hash};
use rustreexo::accumulator::proof::Proof;

use crate::{alloc_and_set, get_slice, CHash, Error, EXIT_SUCCESS};

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
