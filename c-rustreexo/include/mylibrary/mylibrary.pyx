cdef extern from "c-rustreexo.h":
    cdef struct rustreexo_stump:
        pass
    cdef struct rustreexo_chash:
        pass

    rustreexo_stump *rustreexo_stump_create();
    int rustreexo_stump_modify(
        size_t *errno,
        rustreexo_stump *stump,
        rustreexo_chash *utxos,
        int utxos_len,
        void *del_hashes,
        int del_hashes_len,
        void *proof);
    int rustreexo_proof_create(size_t *errno,
        void **proof,
        char *hashes[32],
        int n_hashes,
        unsigned long *targets,
        int n_targets);

cdef class Stump:
    cdef rustreexo_stump *s;
    def __init__(self):
        self.s = rustreexo_stump_create()
    def modify(self, proof: Proof):
        cdef size_t errno = -1;
        cdef size_t *perrno = &errno;
        rustreexo_stump_modify(perrno, self.s, NULL, 0, NULL, 0, proof.p)

cdef class Proof:
    cdef void *p;
    def __init__(self):
        cdef size_t errno = -1;
        rustreexo_proof_create(&errno, &self.p, NULL, 0, NULL, 0);