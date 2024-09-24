#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};
use loam_subcontract_ft::{Fungible, Initable, Sep41};

pub mod ft;

use ft::MyFungibleToken;

#[derive_contract(
    Core(Admin),
    Sep41(MyFungibleToken),
    Fungible(MyFungibleToken),
    Initable(MyFungibleToken)
)]
pub struct Contract;
