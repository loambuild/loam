use loam_sdk::soroban_sdk;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};
struct Contract;
impl CoreRiff for Contract {
    type Impl = Owner;
}
macro_wrapper::soroban_contract!();
