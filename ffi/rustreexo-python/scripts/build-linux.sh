#!/usr/bin/env bash

set -euo pipefail
python --version

echo "Generating rustreexo.py..."
cd ../

cargo run --bin uniffi-bindgen generate src/rustreexo_bindings.udl --language python --out-dir ./rustreexo-python/src --no-format

echo "Generating native binaries..."
rustup default 1.73.0
cargo build --release

echo "Copying linux librustreexo.so..."
cp ../target/release/librustreexo.so ./rustreexo-python/src/librustreexo.so

echo "All done!"
