#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{admin::Admin, Core};
use loam_sdk_ft::{Fungible, Initable};

pub mod ft;

use ft::MyFungibleToken;

pub struct Contract;

impl Core for Contract {
    type Impl = Admin;
}

impl Fungible for Contract {
    type Impl = MyFungibleToken;
}

impl Initable for Contract {
    type Impl = MyFungibleToken;
}

soroban_contract!();
