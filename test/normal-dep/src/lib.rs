#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

struct Contract;
impl CoreRiff for Contract {
    type Impl = Owner;
}
soroban_contract!();
