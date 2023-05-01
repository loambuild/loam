#![no_std]
use loam_sdk::{soroban_contract, soroban_sdk};
use loam_sdk_core_riff::{owner::Owner, CoreRiff};
use loam_sdk_ft::{Fungible, Initable};

pub mod ft;

use ft::MyFungibleToken;

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}

impl Fungible for Contract {
    type Impl = MyFungibleToken;
}

impl Initable for Contract {
    type Impl = MyFungibleToken;
}

soroban_contract!();
