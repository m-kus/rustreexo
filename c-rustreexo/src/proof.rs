use std::io::Cursor;

use rustreexo::accumulator::{node_hash::NodeHash, proof::Proof, stump::Stump};

use crate::{get_safe_ty, get_slice, get_slice_const, CHash, Error, EXIT_FAILURE, EXIT_SUCCESS};

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
        .map(|hash| NodeHash::from(**hash))
        .collect::<Vec<_>>();
    let proof = Proof::new(targets.to_vec(), hashes);
    unsafe {
        ret.write(Box::into_raw(Box::new(proof)));
    }

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
        .map(|hash| NodeHash::from(**hash))
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
// Serialization uses the following schema:
// <n_targets><targets><n_hashes><hashes>
// <n_targets>: u32,
// <targets>: [u64]
// <n_hashes>: u32,
// <hashes>: [CHash]
#[no_mangle]
pub extern "C" fn rustreexo_proof_parse(
    errno: *mut Error,
    parsed_proof: *mut *mut Proof,
    proof: *const u8,
    length: usize,
) -> usize {
    check_ptr!(errno);
    check_ptr!(errno, parsed_proof);
    check_ptr!(errno, proof);
    let proof = get_slice_const(proof, length);
    let data = Cursor::new(proof);

    let proof = Proof::deserialize(data);
    if let Ok(proof) = proof {
        unsafe { parsed_proof.write(Box::into_raw(Box::new(proof))) };
        return EXIT_SUCCESS;
    }
    EXIT_FAILURE
}
#[no_mangle]
pub extern "C" fn rustreexo_proof_serialize(
    errno: *mut Error,
    out: *mut *mut u8,
    ser_len: *mut usize,
    proof: *mut Proof,
) -> usize {
    check_ptr!(errno);
    check_ptr!(out);
    check_ptr!(proof);

    let proof = get_safe_ty(proof);
    let serialized = proof.serialize();
    if let Ok(proof) = serialized {
        unsafe {
            ser_len.write(proof.len());
        }
        let mut proof = std::mem::ManuallyDrop::new(proof);
        unsafe { out.write(proof.as_mut_ptr()) };
        return EXIT_SUCCESS;
    }
    EXIT_FAILURE
}
#[no_mangle]
pub extern "C" fn rustreexo_proof_free(errno: *mut Error, proof: *mut Proof) -> usize {
    check_ptr!(errno, proof);
    unsafe {
        let _ = Box::from_raw(proof);
    }
    EXIT_SUCCESS
}
