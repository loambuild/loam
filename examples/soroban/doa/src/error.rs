use loam_sdk::soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Not enough signatures
    NonEnoughSignatures = 1,
    /// Account is not admin
    AccountIsNotAdmin = 2,
    
}
