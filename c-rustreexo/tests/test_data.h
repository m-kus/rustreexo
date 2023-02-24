/**
 * The same as test_data, but with strings instead of CHashes. This is used to parse the
 * test vectors from the json file.
 */
typedef struct
{
    uint64_t *leaf_preimages;
    size_t preimage_count;
    uint64_t *target_values;
    char *proofhashes[64];
    char *expected_roots[64];
    size_t proofhashes_len;
    size_t expected_roots_len;
    size_t target_value_len;
} test_data_input;

static const test_data_input test1 = {
    .expected_roots = {
        "b151a956139bb821d4effa34ea95c17560e0135d1e4661fc23cedc3af49dac42",
        "9c053db406c1a077112189469a3aca0573d3481bef09fa3d2eda3304d7d44be8"},
    .leaf_preimages = (uint64_t[]){0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11},
    .expected_roots_len = 2,
    .proofhashes_len = 4,
    .preimage_count = 12,
    .proofhashes = {"4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a", "2b4c342f5433ebe591a1da77e013d1b72475562d48578dca8b84bac6651c3cb9", "9576f4ade6e9bc3a6458b506ce3e4e890df29cb14cb5d3d887672aef55647a2b", "c413035120e8c9b0ca3e40c93d06fe60a0d056866138300bb1f1dd172b4923c3"},
    .target_values = (uint64_t[]){0, 4, 5, 6, 7, 8},
    .target_value_len = 6};
