
import ctypes

from ctypes import (
    byref, c_byte, c_int, c_uint, c_char_p, c_size_t, c_void_p, create_string_buffer,
    CFUNCTYPE, POINTER, cast, create_string_buffer, Structure, c_ubyte
)
from ctypes import util

def LibNotFound(BaseException):
    pass


def InvalidLib(BaseException):
    pass

def find_lib() -> str:
    try:
        return util.find_library("rustreexo")
    except BaseException as e:
        print("Error while opening librustreexo: {}".format(e))
        exit(1)

def load_library():
    librustreexo_path = find_lib()
    
    librustreexo = ctypes.cdll.LoadLibrary(librustreexo_path)
    
    if not librustreexo:
        raise Exception("Failed reading librustreexo.so")

    stump = ctypes.c_void_p
    proof = ctypes.c_void_p
    hashes = ctypes.c_void_p
    targets = POINTER(ctypes.c_long)
    udata = ctypes.c_void_p
    try:
        # proof
        librustreexo.rustreexo_proof_parse.argtypes = [
            c_void_p, c_void_p, c_char_p, c_size_t]
        librustreexo.rustreexo_proof_parse.restype = c_size_t

        librustreexo.rustreexo_proof_verify.argtypes = [
            c_void_p, c_void_p, c_int, c_void_p, c_void_p]
        librustreexo.rustreexo_proof_verify.restype = c_size_t

        librustreexo.rustreexo_proof_create.argtypes = [
            c_void_p, c_void_p, c_void_p, c_size_t, c_void_p, c_size_t]
        librustreexo.rustreexo_proof_create.restype = c_size_t

        librustreexo.rustreexo_proof_serialize.argtypes = [
            c_void_p, POINTER(POINTER(c_ubyte)), POINTER(c_int), c_void_p]
        librustreexo.rustreexo_proof_serialize.restype = c_size_t

        librustreexo.rustreexo_proof_free.argtypes = [
            POINTER(c_size_t), c_void_p]
        librustreexo.rustreexo_proof_free.restype = c_size_t

        # stump
        librustreexo.rustreexo_stump_create.argtypes = [
            POINTER(c_size_t), c_void_p]
        librustreexo.rustreexo_stump_create.restype = c_size_t

        librustreexo.rustreexo_stump_modify.argtypes = [
            POINTER(c_size_t), POINTER(stump), POINTER(udata), stump, hashes, c_size_t, hashes, c_size_t, proof]
        librustreexo.rustreexo_stump_modify.restype = c_size_t

        librustreexo.rustreexo_stump_free.argtypes = [
            POINTER(c_size_t), stump]
        librustreexo.rustreexo_stump_free.restype = c_size_t

        librustreexo.rustreexo_stump_debug_print.argtypes = [
            stump]
        librustreexo.rustreexo_stump_debug_print.restype = c_size_t

        librustreexo.rustreexo_stump_get_roots.argtypes = [
            POINTER(c_size_t), c_void_p, POINTER(c_size_t), stump]
        librustreexo.rustreexo_stump_get_roots.restype = c_size_t

        librustreexo.rustreexo_stump_roots_free.argtypes = [
            POINTER(c_size_t), POINTER(c_ubyte)]
        librustreexo.rustreexo_stump_roots_free.restype = c_size_t

        librustreexo.rustreexo_proof_update.argtypes = [
            POINTER(c_size_t), POINTER(proof), POINTER(hashes),
            proof, hashes, c_size_t, hashes, c_size_t, targets,
            c_size_t, targets, c_size_t, c_void_p]
        librustreexo.rustreexo_proof_update.restype = c_size_t

        librustreexo.rustreexo_get_proof_subset.argtypes = [
            POINTER(c_size_t), POINTER(proof), proof, hashes, c_size_t, targets, c_size_t, c_size_t]
        librustreexo.rustreexo_get_proof_subset.restype = c_size_t
    except BaseException as e:
        print("Error while reading librustreexo: {}".format(e))
        exit(0)
    return librustreexo


librustreexo = load_library()
