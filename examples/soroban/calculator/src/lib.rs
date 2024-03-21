#![no_std]
use loam_sdk::soroban_contract;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

pub mod error;
pub mod riff;

use error::Error;
use riff::{Calc, Calculator};

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}

impl Calc for Contract {
    type Impl = Calculator;
}

soroban_contract!();
