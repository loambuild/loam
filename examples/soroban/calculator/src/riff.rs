use loam_sdk::{riff, soroban_sdk::Lazy};

use crate::error::Error;

#[derive(Default)]
pub struct Calculator;

impl Lazy for Calculator {
    fn get_lazy() -> Option<Self> {
        Some(Self)
    }

    fn set_lazy(self) {}
}

#[riff]
pub trait IsCalc {
    /// Add two 32 bit unsigned integers
    fn add_u32(&self, a: u32, b: u32) -> Result<u32, Error>;
}

impl IsCalc for Calculator {
    fn add_u32(&self, a: u32, b: u32) -> Result<u32, Error> {
        a.checked_add(b).ok_or(Error::Overflow)
    }
}
