#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{admin::Admin, CoreRiff};

struct Contract;
impl CoreRiff for Contract {
    type Impl = Admin;
}
soroban_contract!();
