
// The MIT License (MIT)

// Copyright (c) 2023 Davidson Souza

//  Permission is hereby granted, free of charge, to any person obtaining a
//  copy of this software and associated documentation files (the "Software"),
//  to deal in the Software without restriction, including without limitation
//  the rights to use, copy, modify, merge, publish, distribute, sublicense,
//  and/or sell copies of the Software, and to permit persons to whom the
//  Software is furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in
//  all copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
//  OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//  DEALINGS IN THE SOFTWARE.

/**
 * This is a C/C++ biding from the Rustreexo library (https://github.com/mit-dci/rustreexo).
 * It implements utreexo's logic in Rust, and exposes a relatively sane interface for consumers.
 * We adopted an API inspired by libsecp256k1, with the difference that we return a numeric
 * error for each possible error in our lib. Functions only returns 0/1 or void, all returns
 * are made using output pointers. This gives more flexibility to what can we do on the implementation side,
 * especially return more than one data in a easy way. Some key considerations about it:
 *
 * 1 - Output pointers always come first. If the function has one or more outputs, they will
 *     be made using a user provided pointer. No allocation is required for this pointer,
 *     the only invariant is that they are valid pointers.
 *
 * 2 - Arrays always have a length argument immediately after it in the argument list
 */
#ifndef RUSTREEXO_H
#define RUSTREEXO_H
#ifdef __cplusplus
extern "C" {
#endif
#include <inttypes.h>


/**
 * @brief Numeric error values used for telling what went wrong in implementation side.
 *  To obtain a human-meaningful string for each error, see `rustreexo_error_string`
 */
enum rustreexo_error
{
    None = 0,
    NullPointer = 1,
    InvalidSlice = 2,
    UtreexoError = 3,
    InvalidProof = 4,
} rustreexo_error;

/**
 * Opaque data structure representing an rustreexo_stump, the actual internals for this type are
 * only implemented in Rust for the implementation itself. Consumers should hold a pointer
 * to a rustreexo_stump, and only modify it through the API.
 */
typedef struct {
    void *_inner;
} rustreexo_stump;

typedef struct {
    void *_inner;
} rustreexo_proof;
// todo: Make this portable
#define RUSTREEXO_MUST_USE  __attribute_warn_unused_result__
#define RUSTREEXO_NON_NULL(arg)  __nonnull(arg)

#include "leaf.h"
#include "stump.h"

/**
 * @brief Returns a human meaningful string indicating what error happened during a function
 * execution.
 *
 * @param error_string A return string, allocating beforehand is not needed
 * @param errno The error number returned by a function
 */
static inline const char *rustreexo_error_string(size_t errno)
{
    switch (errno)
    {
    case None:
        return "None";
    case NullPointer:
        return "A null pointer was passed in";
    case InvalidSlice:
        return "An invalid slice was passed in";
    case UtreexoError:
        return "Some utreexo related error happened";
    case InvalidProof:
        return "The proof is invalid";
    default:
        printf("Invalid error number: %d\n", errno);
        return "Invalid error number";
    }
}
#ifdef __cplusplus
}
#endif
#endif // RUSTREEXO_H