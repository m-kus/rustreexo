#ifndef RUSTREEXO_LEAF
#define RUSTREEXO_LEAF
#include <stdint.h>
#include <stddef.h>

typedef struct rustreexo_hash
{
    char inner[32];
} rustreexo_hash;

typedef struct rustreexo_bitcoin_outpoint
{
    rustreexo_hash tx_id;
    uint32_t vout;
} rustreexo_bitcoin_outpoint;

typedef struct rustreexo_bitcoin_tx_out
{
    uint64_t value;
    size_t script_pubkey_len;
    char *script_pubkey;
} rustreexo_bitcoin_tx_out;

typedef struct rustreexo_leaf_data
{
    // A commitment to the block creating this utxo
    rustreexo_hash block_hash;
    // The utxo's outpoint
    rustreexo_bitcoin_outpoint prevout;
    // Header code is a compact commitment to the block height and whether or not this
    // transaction is coinbase. It's defined ass
    uint32_t header_code;
    /// The actual utxo
    rustreexo_bitcoin_tx_out utxo;

} rustreexo_leaf_data;

/**
 * @brief Computes the hash of a given leaf
 *
 * @param hash: Output hash
 * @param leaf_data: The actual leaf
 */
size_t rustreexo_leaf_hash(
    size_t errno,
    rustreexo_hash *hash,
    rustreexo_leaf_data leaf_data);

#endif // RUSTREEXO_LEAF