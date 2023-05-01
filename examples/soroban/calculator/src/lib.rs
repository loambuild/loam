#![no_std]

pub mod error;
pub mod gen;

use error::Error;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}

//#[loam]
impl Contract {
    pub fn add_u32(a: u32, b: u32) -> Result<u32, Error> {
        a.checked_add(b).ok_or(error::Error::Overflow)
    }
}
