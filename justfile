set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export CONFIG_DIR := 'target/'
hash := `soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm`

[private]
path:
    just --list

# build contracts
build:
    cargo build --package 'example*' --profile contracts --target wasm32-unknown-unknown

# Setup the project to use a pinned version of the CLI
setup:
    -cargo install --git https://github.com/ahalabs/soroban-tools --rev 351a8b2c17e025acd0770c49141ff29604e4b8ac --root ./target --debug soroban-cli


# Builds contracts. Deploys core riff and then redeploys to status message.
redeploy: build
    soroban config identity generate -d default
    soroban contract deploy --id $DEFAULT_ID --wasm ./target/wasm32-unknown-unknown/contracts/example_core.wasm
    soroban contract invoke --id $DEFAULT_ID -- owner_set --new_owner default
    soroban contract invoke --id $DEFAULT_ID -- --help
    soroban contract invoke --id $DEFAULT_ID -- redeploy --wasm_hash {{hash}}
    soroban contract invoke --id $DEFAULT_ID -- --help
    
    
