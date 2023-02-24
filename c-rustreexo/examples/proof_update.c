/*************************************************************************
 * Written in 2023 by Davidson Souza                                     *
 * To the extent possible under law, the author(s) have dedicated all    *
 * copyright and related and neighboring rights to the software in this  *
 * file to the public domain worldwide. This software is distributed     *
 * without any warranty. For the CC0 Public Domain Dedication, see       *
 * https://creativecommons.org/publicdomain/zero/1.0                     *
 *************************************************************************/

/**
 * Utreexo is a dynamic hash-based accumulator designed to be an alternative to
 * leveldb for storing UTXO sets. As a dynamic accumulator, it supports
 * insertion and deletion of elements. But this comes at the cost of proofs
 * having to be updated from time to time.
 * A proof is only updated when a node that is needed for the proof is modified,
 * for insertions, this becomes exponentially less likely as the tree grows. For
 * deletions, however, this is a constant probability, since the node that is
 * modified is the one that is being deleted. If the deletions updates one of our
 * proof elements, we have to update it.
 *
 * Rustreexo allows you to create and update proofs for a given set of leaves.
 * After updating the accumulator, you can call `rustreexo_proof_update` to
 * update the proof. This function will return a new updated proof. You can also
 * use `rustreexo_proof_update` to create a proof from scratch, by passing a
 * list of leaves and a list of remembered nodes. This will add the element
 * as a target, and update the cached hashes vector.
 *
 * This example shows this simple workflow.
 */

#include <librustreexo/c-rustreexo.h>
#include <openssl/evp.h>
#include <stdio.h>
#include <stdlib.h>

#define ARRAY_SIZE(x) sizeof(x) / sizeof((x)[0])
#define CHECK(x)                                                   \
    if (!x)                                                        \
    {                                                              \
        fprintf(stderr, "%s:%d: %s: %s\n", __FILE__, __LINE__, #x, \
                rustreexo_error_string(errno));                    \
        exit(1);                                                   \
    }
void sha256(rustreexo_hash *out, uint8_t preimage);
int main()
{
    size_t errno = -1;
    rustreexo_hash leaves[1000];
    rustreexo_proof proof = {0};
    rustreexo_hash *del_hashes;
    rustreexo_update_data udata = {0};
    size_t remembers[8] = {0, 10, 22, 200, 1000, 1250, 1500, 9000};
    /* Hash the leaves */
    for (int i = 0; i < ARRAY_SIZE(leaves); i++)
    {
        sha256(&leaves[i], i);
    }
    /* Modify the accumulator */
    rustreexo_stump stump = {0};
    /* Create a Stump and add those leaves into it */
    CHECK(rustreexo_stump_create(&errno, &stump));
    CHECK(rustreexo_proof_create(&errno, &proof, NULL, 0, NULL, 0));
    CHECK(rustreexo_stump_modify(&errno, &stump, &udata, stump, leaves,
                                 ARRAY_SIZE(leaves), NULL, 0, proof));
    /* Update the proof, including the new UTXOs */
    CHECK(rustreexo_proof_update(&errno, &proof, &del_hashes, proof, NULL, 0,
                                 leaves, ARRAY_SIZE(leaves), NULL, 0, remembers,
                                 8, udata));

    /* Verify the proof */
    CHECK(rustreexo_proof_verify(&errno, del_hashes, 8, proof, stump));
    /* Take a subset of the proof */
    rustreexo_proof subproof = {0};
    uint64_t targets[3] = {0, 10, 22};
    CHECK(rustreexo_get_proof_subset(&errno, &subproof, proof, del_hashes, 8, targets, 3, 1000));
    /* Verify the subset */
    CHECK(rustreexo_proof_verify(&errno, (rustreexo_hash[]){del_hashes[0], del_hashes[1], del_hashes[2]}, 3, subproof, stump));

    /* Free the subset */
    CHECK(rustreexo_proof_free(&errno, subproof));
    /* Free the proof */
    CHECK(rustreexo_proof_free(&errno, proof));
    /* Free the stump */
    CHECK(rustreexo_stump_free(&errno, stump));
    CHECK(rustreexo_hashes_free(&errno, del_hashes));
    CHECK(rustreexo_udata_free(&errno, udata));
    return 0;
}

/* Computes the sha256 of a uint8_t */
void sha256(rustreexo_hash *out, uint8_t preimage)
{
    EVP_MD_CTX *mdctx = EVP_MD_CTX_new();
    const EVP_MD *md = EVP_sha256();
    unsigned int md_len;
    EVP_DigestInit_ex(mdctx, md, NULL);
    EVP_DigestUpdate(mdctx, &preimage, 1);
    EVP_DigestFinal_ex(mdctx, out->inner, &md_len);
    EVP_MD_CTX_free(mdctx);
}