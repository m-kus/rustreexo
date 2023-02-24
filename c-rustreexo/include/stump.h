#ifndef RUSTREEXO_STUMP
#define RUSTREEXO_STUMP
#include <inttypes.h>
#include <proof.h>
#ifdef __GNUC__
#define RUSTREEXO_MUST_USE  __attribute_warn_unused_result__
#endif
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
    Stump *stump,
    CHash utxos[],
    size_t utxos_len,
    CHash del_hashes[],
    size_t del_hashes_len,
    Proof *proof);

/**
 * @brief Creates a new empty Stump.
 * @return Returns a brand new Stump. This function never fails, and the returned Stump
 * is granted to be valid.
 */
RUSTREEXO_MUST_USE Stump *rustreexo_stump_create();

/**
 * @brief Debug-prints a Stump. It should be a valid Stump created with the `rustreexo_stump_create`
 * method. Fails if provided Stump is invalid.
 *
 * @param stump The Stump to be printed.
 * @return size_t 1 on success, 0 otherwise.
 */
RUSTREEXO_MUST_USE size_t rustreexo_stump_debug_print(Stump *stump);
#ifdef __cplusplus
}
#endif
#endif // RUSTREEXO_STUMP