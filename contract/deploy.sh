#!/bin/sh

./build.sh

echo ">> Deploying contract"

local_near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/stickyhabits.wasm