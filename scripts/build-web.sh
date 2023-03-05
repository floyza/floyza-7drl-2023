#!/usr/bin/env bash
set -eu
cargo build -r --target wasm32-unknown-unknown
~/.cargo/bin/wasm-bindgen ./target/wasm32-unknown-unknown/release/*.wasm --out-dir wasm --no-modules --no-typescript
