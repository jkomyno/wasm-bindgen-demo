#!/bin/sh

set -euxo pipefail

cargo build -p get-config-wasm --release --target=wasm32-unknown-unknown

echo 'Creating out dir...'
mkdir -p $OUT/src/wasm;

echo 'Generating node module...'
wasm-bindgen \
  --target nodejs \
  --out-dir $OUT/src/wasm \
  target/wasm32-unknown-unknown/release/get_config_wasm.wasm;
