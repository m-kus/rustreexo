from libc.stdlib cimport malloc, free
import numpy as np
from cpython.bytes cimport PyBytes_AsString
from cpython cimport array

cdef extern from "<librustreexo/c-rustreexo.h>":
    cdef struct rustreexo_stump:
        void *_inner;

    ctypedef rustreexo_stump rustreexo_stump

    struct rustreexo_proof:
        void *_inner;
    ctypedef rustreexo_proof rustreexo_proof

    struct rustreexo_hash:
        unsigned char inner[32]

    ctypedef rustreexo_hash rustreexo_hash

    void rustreexo_my_function(unsigned long *data);

    size_t rustreexo_stump_create(
        size_t *errno,
        rustreexo_stump *stump
    )
    size_t rustreexo_stump_debug_print(
        rustreexo_stump stump
    );
    size_t rustreexo_proof_debug_print(
        rustreexo_proof proof
    );

    int rustreexo_stump_modify(
        size_t *errno,
        rustreexo_stump *out,
        rustreexo_stump stump,
        rustreexo_hash *utxos,
        int utxos_len,
        rustreexo_hash *del_hashes,
        int del_hashes_len,
        rustreexo_proof proof);

    int rustreexo_proof_create(
        size_t *errno,
        rustreexo_proof *proof,
        unsigned long *targets,
        size_t n_targets,
        rustreexo_hash *hashes,
        size_t n_hashes);

ctypedef unsigned long u64

cdef class Stump:
    cdef rustreexo_stump s;
    def __init__(self):
        cdef size_t errno = -1
        if rustreexo_stump_create(&errno, &self.s) == 0:
            raise Exception()

    def modify(self, proof: Proof, del_hashes: [rustreexo_hash], new_utxos: [rustreexo_hash]):
        self.__modify__(proof, np.asarray(del_hashes), np.asarray(new_utxos))

    def __modify__(self, proof: Proof, del_hashes: rustreexo_hash[:], new_utxos: rustreexo_hash[:]):
        cdef size_t errno = -1;
        rustreexo_stump_modify(
                &errno,
                &self.s,
                self.s,
                NULL,
                0,
                NULL,
                0,
                proof.p
            );
        return self


cdef class Proof:
    cdef rustreexo_proof p;
    def __init__(self):
        pass
    def new(self, hashes: [bytes], targets):
        cdef size_t errno = -1;

        # Concatenate the bytes into a single bytearray
        ba = bytes().join(hashes)

        # Get a pointer to the unsigned char array
        cdef const unsigned char *_hashes = <unsigned char *> PyBytes_AsString(<object>ba)

        # Create an array of unsigned longs from the list of numbers
        cdef unsigned long[:] arr = array.array('L', targets)

        # Get a pointer to the first element of the array
        cdef unsigned long *ptr = &arr[0]

        rustreexo_proof_create(&errno, &self.p, ptr, len(targets),<rustreexo_hash *> _hashes, len(hashes));
        rustreexo_proof_debug_print(self.p);

cdef class NodeHash:
    cdef rustreexo_hash hash;
    def __init__(self, hash:bytes):
        self.hash.inner = hash