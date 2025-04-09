#!/bin/bash
set -e

PATH=./target/bin:$PATH

stellar contract deploy --alias registry \
                        --wasm ./target/loam/loam_registry.wasm \
                        -- \
                        --admin default

registry="stellar contract invoke --id registry --"

$registry --help

for i in $(cargo r build --ls); do
    echo "Publishing $i to registry"
done
