set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export CONFIG_DIR := 'target/'
# hash := `soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm`



[private]
path:
    just --list

loam +args:
    @cargo r -- {{args}}

s +args:
    @stellar {{args}}

stellar +args:
    @stellar {{args}}

build_contract p:
    stellar contract build --profile contracts --package {{p}}

# build contracts
build:
    just loam build --profile contracts --out-dir ./target/loam

# Setup the project to use a pinned version of the CLI
setup:
    -cargo binstall -y --install-path ./target/bin soroban-cli --version 21.0.0

# Build loam-cli test contracts to speed up testing
build-cli-test-contracts:
    cargo run -- build --manifest-path crates/loam-cli/tests/fixtures/soroban-init-boilerplate/Cargo.toml

test: build build-cli-test-contracts
    cargo test

create: build
    rm -rf .soroban
    stellar keys generate default
    just stellar contract deploy --wasm ./target/loam/example_core.wasm --alias core

# # Builds contracts. Deploys core subcontract and then redep

# # Builds contracts. Deploys core subcontract and then redeploys to status message.

redeploy:
    stellar contract invoke -- admin_set --new_admin default
    stellar contract invoke -- --help
    stellar contract invoke -- redeploy --wasm_hash $(just stellar contract install --wasm ./target/loam/example_status_message.wasm)
    stellar contract invoke -- --help
    stellar contract invoke -- --help | grep messages_get || exit 1
