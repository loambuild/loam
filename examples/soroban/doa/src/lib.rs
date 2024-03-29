#![no_std]
// // Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{admin::Admin, CoreRiff};

mod admins;
mod error;

pub use admins::*;

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Admin;
}

impl IsAdmins for Contract {
    type Impl = AdminSet;
}

soroban_contract!();