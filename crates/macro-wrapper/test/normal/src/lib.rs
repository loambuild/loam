use loam_sdk::soroban_sdk;
use loam_sdk_core_riffs::{owner::Owner, Ownable};
struct Contract;
impl Ownable for Contract {
    type Impl = Owner;
}
macro_wrapper::soroban_contract!();
