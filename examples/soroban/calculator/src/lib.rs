#![no_std]
use loam_sdk_core_riff::{admin::Admin, Core};

pub mod error;
pub mod riff;

use error::Error;
use riff::{Calc, Calculator};

#[loam_sdk::derive_contract(Core(Admin), Calc(Calculator))]
pub struct Contract;
