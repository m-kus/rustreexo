
# Rustreexo ffi bindings

This crate provides ffi bindings for the [rustreexo](https://github.com/mit-dci/rustreexo) library. This project uses the Uniffi framework to generate the ffi bindings, therefore the bindings are available for the following languages:

 - Kotlin
 - Swift
 - Python
 - Ruby

## Building

To build the ffi bindings, you need to have the Rust compiler installed. You can install it using [rustup](https://rustup.rs/). The minimum supported version is 1.51.0.

Before building the bindings, you should build the rustreexo library. You can do this by running the following command in the rustreexo directory:

```bash
cargo build --release
```

this will build the rustreexo library in the `target/release` directory. Now you can generate the bindings to your desired language. For example, to generate the bindings for Kotlin, run the following command:

```bash
cargo run --bin uniffi-bindgen generate src/rustreexo_bindings.udl --language kotlin --out-dir ./kotin
```

replace `kotlin` with the desired language. This will generate the bindings in the `kotlin` directory, you can change this to your desired directory. Then you should copy
the `librustreexo.so` file from the `target/release` directory to the directory where you generated the bindings. For example, if you generated the bindings in the `kotlin` directory, you should copy the `librustreexo.so` file to the `kotlin` directory. And that's it, you can now use the bindings in your project.

## Examples

You can seed more examples under the specific dir of the language. For example, for Kotlin, you can find the examples under the `kotlin/examples` directory. But here's a simple example of how to use the bindings in python:

```python
from rustreexo import Stump, Proof

# Create a new stump
stump = Stump()
proof = Proof()

# Add a leaf to the stump
stump.modify([0] * 32, [], proof)
```

## License

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for more info.