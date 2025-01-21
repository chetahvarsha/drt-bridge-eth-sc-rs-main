#!/bin/sh

# cleans all wasm targets

set -e
SMART_CONTRACT_JSONS=$(find . -name "numbat.json")
for smart_contract_json in $SMART_CONTRACT_JSONS
do
    smart_contract_folder=$(dirname $smart_contract_json)
    echo ""
    (set -x; drtpy --verbose contract clean $smart_contract_folder)
done

# not wasm, but worth cleaning from time to time

cargo clean
cd numbat-wasm-node
cargo clean
cd ..
cd numbat-wasm-output
cargo clean
cd ..
