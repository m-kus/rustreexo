from _pytreexo.bindings import librustreexo

import ctypes

class Proof():
    # Keep a reference to the C proof object, and only get the python object when needed
    c_proof = ctypes.c_void_p()

    def __init__(self, targets: [int] = [], hashes: [bytes] = [], ptr: ctypes.c_void_p = None):
        """
        Creates a new proof object
        targets: Elements of the tree being proven
        hashes: The hashes needed to prove the targets
        """
        if ptr is not None:
            self.c_proof = ptr
            return

        errno = ctypes.c_size_t(-1)
        targets = (ctypes.c_long * len(targets))(*targets)
        hash_type = ctypes.c_char * 32
        _hashes = (hash_type * len(hashes))(*[hash_type.from_buffer_copy(
            hashes[i]) for i in range(len(hashes))])

        if librustreexo.rustreexo_proof_create(
            ctypes.addressof(errno), ctypes.addressof(
                self.c_proof), targets, len(targets), _hashes, len(hashes)
        ) != 1:
            raise Exception("Could not create proof")

    def __str__(self):
        """
        Returns a string representation of the proof.
        """
        return self.serialize()[0].hex()

    def verify(self, stump) -> bool:
        """
        Verifies the proof against a stump. Returns true if the proof is valid, needs a stump
        and a the hash of the leafs being proven.
        """
        res = ctypes.c_size_t(-1)
        librustreexo.rustreexo_proof_verify(
            ctypes.addressof(res), ctypes.c_void_p(), 0, ctypes.c_void_p(), ctypes.c_void_p())
        return res.value == 0

    def serialize(self):
        """
        Serializes the proof into a byte array
        """
        buf = ctypes.POINTER(ctypes.c_ubyte)()
        buf_len = ctypes.c_int(0)
        librustreexo.rustreexo_proof_serialize(
            self.c_proof, ctypes.byref(buf), ctypes.byref(buf_len), self.c_proof)
        return bytes(buf[:buf_len.value]), buf_len

    def parse(buf: bytes) -> 'Proof':
        """
        Parses a proof from a byte array building a valid proof object
        """
        c_proof = ctypes.c_void_p()
        errno = ctypes.c_size_t(-1)
        ret = librustreexo.rustreexo_proof_parse(
            ctypes.addressof(errno), ctypes.addressof(c_proof), buf, len(buf))
        if ret == 0:
            raise Exception("Could not parse proof")
        return Proof(ptr=c_proof)

    def __del__(self):
        errno = ctypes.c_size_t(-1)
        if librustreexo.rustreexo_proof_free(ctypes.byref(errno), self.c_proof) != 1:
            raise Exception("Could not free proof")

    @property
    def targets(self) -> [int]:
        """
        Returns the targets of the proof
        """
        errno = ctypes.c_size_t(-1)

        buf = ctypes.POINTER(ctypes.c_long)()
        buf_len = ctypes.c_size_t(0)

        librustreexo.rustreexo_proof_get_targets(
            ctypes.byref(errno), ctypes.byref(buf), ctypes.byref(buf_len), self.c_proof)

        return [buf[i] for i in range(buf_len.value)]
    def subset(self, cached_hashes: [bytes], targets: [int], n_leaves: int) -> 'Proof':
        """
        Returns a subset of the proof, only containing the targets specified
        """
        errno = ctypes.c_size_t(-1)
        new_proof = ctypes.c_void_p()
        _targets = (ctypes.c_long * len(targets))(*targets)
        hash_type = ctypes.c_char * 32
        _cached_hashes = (hash_type * len(cached_hashes))(*[hash_type.from_buffer_copy(
            cached_hashes[i]) for i in range(len(cached_hashes))])

        if librustreexo.rustreexo_get_proof_subset(
            ctypes.byref(errno), ctypes.byref(new_proof), self.c_proof, _cached_hashes,
            len(cached_hashes), _targets, len(targets), n_leaves
        ) != 1:
            raise Exception("Could not take a subset of the proof")
        return Proof(ptr=new_proof)

    def update(self, udata: 'UpdateData', cached_hashes: [bytes], add_hashes: [bytes], block_targets: [int], remembers: [int] ):
        """Updates the proof after some changes to the tree."""
        errno = ctypes.c_size_t(-1)
        new_proof = ctypes.c_void_p()
        cached_hash_out = ctypes.c_void_p()

        hash_type = ctypes.c_char * 32
        _cached_hashes = (hash_type * len(cached_hashes))(*[hash_type.from_buffer_copy(
            cached_hashes[i]) for i in range(len(cached_hashes))])
        _add_hashes = (hash_type * len(add_hashes))(*[hash_type.from_buffer_copy(
            add_hashes[i]) for i in range(len(add_hashes))])
        _block_targets = (ctypes.c_long * len(block_targets))(*block_targets)
        _remembers = (ctypes.c_long * len(remembers))(*remembers)
        ret = librustreexo.rustreexo_proof_update(
            ctypes.byref(errno),
            ctypes.byref(new_proof),
            ctypes.byref(cached_hash_out),

            self.c_proof,
            _cached_hashes,
            len(cached_hashes),
            _add_hashes,
            len(add_hashes),
            _block_targets,
            len(block_targets),
            _remembers,
            len(remembers),
            udata.inner)
        if ret == 0:
            raise Exception("Could not update proof {}".format(errno.value))
        self.c_proof = new_proof

    @property
    def hashes(self) -> [bytes]:
        """
        Returns the hashes of the proof
        """
        errno = ctypes.c_size_t(-1)

        buf = ctypes.POINTER(ctypes.c_ubyte)()
        buf_len = ctypes.c_size_t(0)

        librustreexo.rustreexo_proof_get_hashes(
            ctypes.byref(errno), ctypes.byref(buf), ctypes.byref(buf_len), self.c_proof)

        return [bytes(buf[i*32:(i+1)*32]) for i in range(buf_len.value)]
    def from_raw(raw_proof: ctypes.c_void_p):
        """Creates a proof object from a raw pointer"""
        return Proof.c_proof
    def __eq__(self, o):
        if not isinstance(o, Proof):
            return False
        return self.serialize()[0] == o.serialize()[0]

    def __ne__(self, o):
        return not self == o

    def __hash__(self):
        return hash(self.serialize()[0])
