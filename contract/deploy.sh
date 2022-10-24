#!/bin/sh

./build.sh

echo ">> Deploying contract"

source ~/near/init-local-near-env.sh
local_near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/stickyhabits.wasm