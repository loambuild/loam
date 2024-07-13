#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};
use loam_subcontract_ft::{Fungible, Initable};

pub mod ft;

use ft::MyFungibleToken;

#[derive_contract(Core(Admin), Fungible(MyFungibleToken), Initable(MyFungibleToken))]
pub struct Contract;
