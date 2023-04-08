#ifndef RUSTREEXO_PROOF_H
#define RUSTREEXO_PROOF_H
#include <stdlib.h>
#include <inttypes.h>
#include <librustreexo/stump.h>
#include <librustreexo/leaf.h>

/**
 * A proof is a collection of hashes and targets, used to prove that a set of UTXOs
 * actually belongs to a accumulator set. Those proofs are very similar to Merkle proofs
 * used in Bitcoin.
 */

#ifdef __cplusplus
extern "C"
{
#endif // __cplusplus
/**
* @brief Creates a new rustreexo_proof from a set of hashes and targets. Hashes are the hash of each
* node needed to recompute a Merkle path leading to a root. Targets are the node number
* for UTXOs being spent.
*
* @param errno A pointer used to write back error, if any
* @param proof The newly created proof
* @param hashes An array of hashes
* @param n_hashes The hashes array's length
* @param targets  An array of targets
* @param n_targets How many targets there are in targets
*/
size_t rustreexo_proof_create(
    size_t *errno,
    rustreexo_proof *out,
    uint64_t *targets,
    size_t n_targets,
    rustreexo_hash *hashes,
    size_t n_hashes);

/**
 * @brief Verifies if a rustreexo_proof is valid for a given Stump. It will return error if the proof
 * is invalid, with errno == ProofInvalid. Both rustreexo_proof and Stump should be valid structures.
 *
 * @param errno A pointer used to write back error, if any
 * @param stump The accumulator's state we should verify this rustreexo_proof against
 * @param proof An actual proof
 * @return size_t 1 if this proof is valid, 0 otherwise
 */
size_t rustreexo_proof_verify(
    size_t *errno,
    rustreexo_hash *del_hashes,
    size_t n_dels,
    rustreexo_proof proof,
    rustreexo_stump stump);
/**
 * Deserializes a proof
 */
size_t rustreexo_proof_parse(
    size_t *errno,
    rustreexo_proof *parsed_proof,
    const char *proof,
    size_t length);

/**
 * @brief Serializes a proof
 *
 * @param errno
 * @param out
 * @param proof
 * @return size_t
 */
size_t rustreexo_proof_serialize(
    size_t *errno,
    char **out,
    size_t *ser_len,
    rustreexo_proof proof);

/**
 * @brief Frees a proof
 *
 */
size_t rustreexo_proof_free(
    size_t *errno,
    rustreexo_proof proof);

size_t rustreexo_proof_debug_print(
    rustreexo_proof proof
);
#ifdef __cplusplus
}
#endif // __cplusplus

#endif // RUSTREEXO_PROOF_H