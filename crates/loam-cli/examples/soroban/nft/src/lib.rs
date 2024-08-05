#![no_std]
use loam_sdk::derive_contract;
use loam_subcontract_core::{admin::Admin, Core};

pub mod nft;
pub mod subcontract;

use nft::MyNonFungibleToken;
use subcontract::{Initable, NonFungible};
use crate::subcontract::Metadata;

#[derive_contract(
    Core(Admin),
    NonFungible(MyNonFungibleToken),
    Initable(MyNonFungibleToken)
)]
pub struct Contract;

impl Contract {
    pub(crate) fn require_auth() {
        Contract::admin_get()
            .expect("No admin! Call 'admin_set' first.")
            .require_auth();
    }
}

mod test;
