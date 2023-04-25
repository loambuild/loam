set dotenv-load

export PATH := './target/bin:' + env_var('PATH')
export CONFIG_DIR := 'target/'
hash := `soroban contract install --wasm ./target/wasm32-unknown-unknown/contracts/example_status_message.wasm --config-dir ./target`

path:
    echo ${PATH}

build:
    cargo build --package 'example*' --profile contracts --target wasm32-unknown-unknown


setup:
    -cargo install --git https://github.com/ahalabs/soroban-tools --rev 351a8b2c17e025acd0770c49141ff29604e4b8ac --root ./target soroban-cli --debug



redeploy: build setup
    soroban config identity generate -d default --config-dir $CONFIG_DIR
    soroban contract deploy --id $DEFAULT_ID --wasm ./target/wasm32-unknown-unknown/contracts/example_base.wasm --config-dir $CONFIG_DIR
    soroban contract invoke --id $DEFAULT_ID --config-dir $CONFIG_DIR -- owner_set --owner default
    soroban contract invoke --id $DEFAULT_ID --config-dir $CONFIG_DIR -- --help
    soroban contract invoke --id $DEFAULT_ID --config-dir $CONFIG_DIR -- redeploy --wasm_hash {{hash}}
    soroban contract invoke --id $DEFAULT_ID --config-dir $CONFIG_DIR -- --help
    
    