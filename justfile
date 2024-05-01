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
    @soroban {{args}}

soroban +args:
    @soroban {{args}}

build_contract p:
    soroban contract build --profile contracts --package {{p}}

# build contracts
build:
    just loam build --profile contracts --out-dir ./target/loam

# Setup the project to use a pinned version of the CLI
setup:
    -cargo binstall -y --install-path ./target/bin soroban-cli --version 20.3.1

test: build
    cargo test

create: build
    rm -rf .soroban
    soroban keys generate default
    just soroban contract deploy --wasm ./target/loam/example_core.wasm | just loam update-env --name SOROBAN_CONTRACT_ID

# # Builds contracts. Deploys core subcontract and then redep

# # Builds contracts. Deploys core subcontract and then redeploys to status message.

redeploy:
    soroban contract invoke -- admin_set --new_admin default
    soroban contract invoke -- --help
    soroban contract invoke -- redeploy --wasm_hash $(soroban contract install --wasm ./target/loam/example_status_message.wasm)
    soroban contract invoke -- --help | grep messages_get || exit 1
