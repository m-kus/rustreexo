### Wasm bindings for rustreexo

This is a collection of wasm bindings, meant to use rustreexo with javascript. If you have a Rust project, you can use the [rustreexo crate](https://crates.io/crates/rustreexo) directly. But if your project is in javascript (or anything that interop with wasm) you can use this.

## Building

You need to have `wasm-pack` installed. You can install it with `cargo install wasm-pack`. Then you can build the project with e.g `wasm-pack build --target nodejs`. If no target is specified, it will build for the browser.

The generated code will be in `pkg/`. You can use it in your project by importing it like this:

```javascript
const { Proof, Stump, LeafData } = require('./pkg')
```

## Usage (nodejs)

The api is the exact one from `rustreexo`, but generated with wasm-bindgen. So you can use it like this:

```javascript
const { Proof, Stump, LeafData } = require('./pkg')
const { assert } = require('console')

const Buffer = require('buffer').Buffer

// LeafData is the data that will be committed to in the accumulator. To build one, you can use the constructor. It requires a block_hash, tx_id, outpoint, amount and spk for the UTXO being committed to.
// Hashes and scripts are represented as Uint8Arrays
const tx_id = Buffer.from('ce0195b45b6d7bfd6925d78d797ee92a9393433c6c9dc4399ac15fceca510264', 'hex');
const spk = Buffer.from('0014ac32a27d1e2842d713199ec254a97e75cdedb86a', 'hex');
const block_hash = Buffer.from('000000eb5d56e8e7281a706fecb22d4e7f07a1a5999596b078a27bb0e88ed99a', 'hex');
const outpoint = 0;
const amount = 68273; // satoshis
const leaf = new LeafData(block_hash, tx_id, outpoint, amount, spk);

const leaf_hash = leaf.hash;

// Create a new Stump. Stump is a lighweight representation of the accumulator.
// It only contains the roots and number of insertions, and only weights a few bytes.
const stump = new Stump();

// Add a leaf to the stump. Modify is the main interface to it, you can't add a leaf directly, because the order of the operations is important. If you don't want to add or remove something, just pass an empty list. The last element is a proof for what is being deleted. Since we are not deleting anything, we pass an empty proof.
const update_data = stump.modify([leaf], [], new Proof());

// Now we can generate a proof for the leaf we just added. Rustreexo allows you to generate and update a proof as blocks are generated. This is useful for SPV wallets, that don't have the full accumulator, but can still generate proofs for the UTXOs they own.

// Create and empty proof
const proof = new Proof();

// Update the proof with the data from the stump
proof.update([], [leaf_hash], [], [0], update_data);

// Now we can verify the proof. This will return true if the proof is valid, and false otherwise.
const valid = stump.verify(proof, [leaf_hash]);
assert(valid); // Should be valid
```

## Usage (browser without a bundler)

The usage in the browser is the same as in nodejs, but you need to import a ES6 module instead of a commonjs module. You can do this by adding a script tag to your html file:

```html
<html>
    <head>
        <script type="module">
        import init, { Proof, Stump, LeafData } from './pkg/rustreexo.js';
        init();
        // Now you can use Proof, Stump and LeafData
        </script>
    </head>
    <body>
    </body>
```