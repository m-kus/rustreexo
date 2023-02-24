#ifndef RUSTREEXO_STUMP
#define RUSTREEXO_STUMP
#include <inttypes.h>
/**
 * Stumps are a lightweight representation of the Utreexo state. It has only the roots and
 * the number of leaves. It can verify all proofs, but can't prove. They are useful for clients
 * and wallets.
 * For usage examples, see `examples/stump.c`
 */

/**
 * Opaque data structure representing an Stump, the actual internals for this type are
 * only implemented in Rust for the implementation itself. Consumers should hold a pointer
 * to a Stump, and only modify it through the API.
 */
typedef struct Stump Stump;

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
 * @return 0 on success 1 otherwise
 */
size_t rustreexo_stump_modify(
    size_t *errno,
    Stump *stump,
    CHash utxos[],
    size_t utxos_len);

/**
 * @brief Creates a new empty Stump.
 * @return Returns a brand new Stump. This function never fails, and the returned Stump
 * is granted to be valid.
 */
Stump *rustreexo_stump_create();

/**
 * @brief Debug-prints a Stump. It should be a valid Stump created with the `rustreexo_stump_create`
 * method. Fails if provided Stump is invalid.
 *
 * @param stump The Stump to be printed.
 * @return size_t 0 on success, 1 otherwise.
 */
size_t rustreexo_stump_debug_print(Stump *stump);

#endif // RUSTREEXO_STUMP