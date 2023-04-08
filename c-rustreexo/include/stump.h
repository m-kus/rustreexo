#ifndef RUSTREEXO_STUMP
#define RUSTREEXO_STUMP
#include <inttypes.h>
#include <librustreexo/proof.h>

#ifdef __cplusplus
extern "C" {
#endif
/**
 * Stumps are a lightweight representation of the Utreexo state. It has only the roots and
 * the number of leaves. It can verify all proofs, but can't prove. They are useful for clients
 * and wallets.
 * For usage examples, see `examples/stump.c`
 */


/**
 * @brief Modifies a Stump, adding new UTXOs and removing old ones, this function is pure
 * in relation to it's operands, meaning that it doesn't modify the Stump passed in, but
 * returns a new one in `ret`.
 * Callers should make sure that utxos is not empty and all pointers are valid.
 *
 * @param errno The number representing an error while executing this function
 * @param ret The new Stump
 * @param stump The Stump being Updated
 * @param utxos A array of hashes for the new UTXOs
 * @param utxos_len The length of the array of hashes
 * @param del_hashes A set with the hash for UTXOs being removed (STXOs)
 * @param del_hashes_len How many STXOs
 * @param proof A proof for these STXOs, proving that they existed and aren't being double-spent
 *
 * @return 1 on success 0 otherwise
 */
size_t rustreexo_stump_modify(
    size_t *errno,
    rustreexo_stump *out,
    rustreexo_stump stump,
    rustreexo_hash utxos[],
    size_t utxos_len,
    rustreexo_hash del_hashes[],
    size_t del_hashes_len,
    rustreexo_proof proof
) RUSTREEXO_NON_NULL((1)) RUSTREEXO_NON_NULL((2)) RUSTREEXO_NON_NULL((4)) RUSTREEXO_NON_NULL((6));

/**
 * @brief Creates a new empty Stump.
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_create(
    size_t *errno,
    rustreexo_stump *stump
) RUSTREEXO_NON_NULL((1)) RUSTREEXO_NON_NULL((2));
//
/**
 * @brief Debug-prints a Stump. It should be a valid Stump created with the `rustreexo_stump_create`
 * method. Fails if provided Stump is invalid.
 *
 * @param stump The Stump to be printed.
 * @return size_t 1 on success, 0 otherwise.
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_debug_print(
    rustreexo_stump stump
);

/**
 * @brief Frees a Stump. It should be a valid Stump created with the `rustreexo_stump_create`
 *
 * @param errno
 * @param stump
 * @return RUSTREEXO_MUST_USE
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_free(
    size_t *errno,
    rustreexo_stump stump
) RUSTREEXO_NON_NULL((1));

/**
 * @brief Returns the roots of a Stump. It should be a valid Stump created with the `rustreexo_stump_create`
 * method. Fails if provided Stump is invalid. The caller is responsible for freeing the returned
 * array.
 *
 * @param errno: The number representing an error while executing this function if any.
 * @param stump: A valid Stump.
 * @param ret_len: A pointer to a size_t that will be filled with the length of the returned array.
 * @param hash: A pointer to a rustreexo_hash array that will be filled with the roots.
 *              The caller is responsible for freeing it, by calling `rustreexo_stump_roots_free`.
 * @return 1 on success, 0 otherwise.
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_get_roots(
    size_t *errno,
    rustreexo_hash **ret,
    size_t *ret_len,
    rustreexo_stump stump
) RUSTREEXO_NON_NULL((1)) RUSTREEXO_NON_NULL((2)) RUSTREEXO_NON_NULL((3));

/**
 * @brief Frees a rustreexo_hash array returned by `rustreexo_stump_get_roots`.
 *
 * @param errno: The number representing an error while executing this function if any.
 * @param roots: A valid rustreexo_hash array.
 * @return 1 on success, 0 otherwise.
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_roots_free(
    size_t *errno,
    rustreexo_hash *roots
) RUSTREEXO_NON_NULL((1)) RUSTREEXO_NON_NULL((2));

#ifdef __cplusplus
}
#endif
#endif // RUSTREEXO_STUMP