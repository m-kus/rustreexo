#ifndef RUSTREEXO_LEAF
#define RUSTREEXO_LEAF

typedef struct CHash
{
    char inner[32];
} CHash;

typedef struct rustreexo_bitcoin_outpoint
{
    CHash tx_id;
    uint32_t vout;
} rustreexo_bitcoin_outpoint;

typedef struct rustreexo_bitcoin_tx_out
{

} rustreexo_bitcoin_tx_out;

typedef struct rustreexo_leaf_data
{
    // A commitment to the block creating this utxo
    CHash block_hash;
    // The utxo's outpoint
    rustreexo_bitcoin_outpoint prevout;
    // Header code is a compact commitment to the block height and whether or not this
    // transaction is coinbase. It's defined ass
    uint32_t header_code;
    /// The actual utxo
    rustreexo_bitcoin_tx_out utxo;

} rustreexo_leaf_data;

#endif // RUSTREEXO_LEAF