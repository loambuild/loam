#![no_std]
// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk_core_riffs::{owner::Owner, Ownable, Redeployable};

pub mod gen;

pub struct Contract;

impl Ownable for Contract {
    type Impl = Owner;
}

impl Redeployable for Contract {}
