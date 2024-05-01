#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_subcontract_core::{admin::Admin, Core};

struct Contract;
impl Core for Contract {
    type Impl = Admin;
}
soroban_contract!();
