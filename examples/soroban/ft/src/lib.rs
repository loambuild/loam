#![no_std]
use loam_sdk::{derive_contract, soroban_contract};
use loam_sdk_core_riff::{admin::Admin, Core};
use loam_sdk_ft::{Fungible, Initable};

pub mod ft;

use ft::MyFungibleToken;

#[derive_contract(Core(Admin), Fungible(MyFungibleToken), Initable(MyFungibleToken))]
pub struct Contract;
