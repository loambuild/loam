#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

pub mod nft;
pub mod subcontract;

use nft::MyNonFungibleToken;
use subcontract::{Initable, NonFungible};

#[derive_contract(
    Core(Admin),
    NonFungible(MyNonFungibleToken),
    Initable(MyNonFungibleToken)
)]
pub struct Contract;
