"""
A Stump is a compact representation of the forest, it stores only the roots of the trees.
It is useful for lightweight nodes and wallets that do not need to store the entire forest.
"""

from typing import List, Optional, Tuple
from ctypes import c_size_t, c_void_p, c_int, c_char, c_char_p, c_long, POINTER, byref, c_ubyte
from _pytreexo.proof import Proof
from _pytreexo.bindings import librustreexo

class UpdateData:
    def __init__(self, inner: c_void_p):
        self.inner = inner

class Stump:
    """
    A Stump is a compact representation of the forest, it stores only the roots of the trees.
    It is useful for lightweight nodes and wallets that do not need to store the entire forest.
    """

    def __init__(self):
        self.c_stump = c_void_p()
        errno = c_size_t(-1)
        if librustreexo.rustreexo_stump_create(byref(errno), byref(self.c_stump)) != 1:
            raise Exception("Could not create stump")

    def __del__(self):
        errno = c_size_t(-1)
        if librustreexo.rustreexo_stump_free(byref(errno), self.c_stump) != 1:
            raise Exception("Could not free stump")

    def modify(self, utxos: [bytes], del_hashes: [bytes],  proof: Proof) -> UpdateData:
        """
        Modify the stump with a list of utxos and a proof.
        """
        errno = c_size_t(-1)
        update_data = c_void_p()
        hash_type = c_char * 32
        _utxos = (hash_type * len(utxos))(*
                                          [hash_type.from_buffer_copy(utxos[i]) for i in range(len(utxos))])
        _del_hashes = (hash_type * len(del_hashes))(*
                                                    [hash_type.from_buffer_copy(del_hashes[i]) for i in range(len(del_hashes))])

        if librustreexo.rustreexo_stump_modify(byref(errno),  byref(self.c_stump), update_data,  self.c_stump, _utxos, len(utxos), _del_hashes, len(del_hashes), proof.c_proof) != 1:
            raise Exception("Could not modify stump")
        return UpdateData(update_data)

    @property
    def roots(self) -> [bytes]:
        """
        Returns the roots of the trees in the forest
        """
        errno = c_size_t(-1)

        hash_type = c_char * 32
        buf = POINTER(hash_type)()
        buf_len = c_size_t(0)

        librustreexo.rustreexo_stump_get_roots(
            byref(errno), byref(buf), byref(buf_len), self.c_stump)

        return [buf[i].raw for i in range(buf_len.value)]

    def show(self):
        """
        Prints the roots of the trees in the forest
        """
        librustreexo.rustreexo_stump_debug_print(self.c_stump)

    def serialize(self) -> bytes:
        """
        Serializes the stump into a byte array
        """
        buf = POINTER(c_ubyte)()
        buf_len = c_int(0)
        librustreexo.rustreexo_stump_serialize(
            self.c_stump, byref(buf), byref(buf_len), self.c_stump)
        return bytes(buf[:buf_len.value])

    @staticmethod
    def deserialize(serialized: bytes) -> 'Stump':
        """
        Deserializes a stump from a byte array
        """
        c_stump = c_void_p()
        errno = c_size_t(-1)
        if librustreexo.rustreexo_stump_deserialize(byref(errno), byref(c_stump), serialized, len(serialized)) != 1:
            raise Exception("Could not deserialize stump")
        stump = Stump()
        stump.c_stump = c_stump
        return stump

    def __str__(self):
        return str([self.roots[i].hex() for i in range(len(self.roots))])

    def __repr__(self):
        return self.__str__()

    def __eq__(self, other):
        return self.roots == other.roots

    def __ne__(self, other):
        return self.roots != other.roots
