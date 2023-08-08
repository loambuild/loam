use loam_sdk::soroban_sdk;
use loam_sdk_core_riff::{owner::Owner, CoreRiff};
use loam_sdk_macro::soroban_contract;
struct Contract;
impl CoreRiff for Contract {
    type Impl = Owner;
}
soroban_contract!();
