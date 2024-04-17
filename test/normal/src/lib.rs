#![no_std]
use loam_sdk_core_riff::{admin::Admin, Core};

#[loam_sdk::derive_contract(Core(Admin))]
pub struct Contract;
