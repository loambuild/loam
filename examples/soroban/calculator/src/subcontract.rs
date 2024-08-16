use loam_sdk::{
    soroban_sdk::{self, Lazy},
    subcontract, vec,
};

use crate::error::Error;

#[derive(Lazy, Default)]
pub struct Calculator;

#[subcontract]
pub trait IsCalc {
    /// Add two 32 bit unsigned integers
    #[allow(clippy::missing_errors_doc)]
    fn add_u32(&self, a: u32, b: u32) -> Result<u32, Error>;

    /// Puts two into into a vector
    fn two_array(&self, a: u32, b: u32) -> loam_sdk::soroban_sdk::Vec<u32>;
}

impl IsCalc for Calculator {
    fn add_u32(&self, a: u32, b: u32) -> Result<u32, Error> {
        a.checked_add(b).ok_or(Error::Overflow)
    }

    fn two_array(&self, a: u32, b: u32) -> soroban_sdk::Vec<u32> {
        vec![a, b]
    }
}
