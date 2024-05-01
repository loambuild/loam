#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_subcontract_core::{admin::Admin, Core};

pub mod error;
pub mod subcontract;

pub use error::Error;
use subcontract::{Calc, Calculator};

pub struct Contract;

impl Core for Contract {
    type Impl = Admin;
}

impl Calc for Contract {
    type Impl = Calculator;
}

soroban_contract!();
