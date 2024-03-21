#![no_std]
// // Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

mod status_message;
pub use status_message::*;

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}

impl Postable for Contract {
    type Impl = StatusMessage;
}

soroban_contract!();
