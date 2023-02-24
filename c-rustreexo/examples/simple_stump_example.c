#include <stdlib.h>
#include <stdio.h>
#include <memory.h>

#include <c-rustreexo.h>

#define ELEMENTS 5
static char proof_elements[ELEMENTS][32] = {
    {1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    {2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    {3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    {4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0},
    {5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0}};
#define CHECK(exp)                                     \
    if (!exp)                                          \
    {                                                  \
        char *error;                                   \
        printf("%s\n", rustreexo_error_string(errno)); \
        exit(EXIT_FAILURE);                            \
    }
int main()
{
    size_t errno = 0;
    Stump *s = rustreexo_stump_create();
    Proof *p;
    // Creates a new empty proof
    CHECK(rustreexo_proof_create(&errno, &p, NULL, 0, NULL, 0));
    // Prints the Accumulator state
    rustreexo_stump_debug_print(s);
    // Add a few leaves
    CHECK(rustreexo_stump_modify(&errno, s, (CHash *)proof_elements, ELEMENTS, NULL, 0, p));
    // Print the accumulator again, with the new leaves
    rustreexo_stump_debug_print(s);
    return (EXIT_SUCCESS);
}