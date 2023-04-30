// use loam_sdk::soroban_sdk::{self, contractimpl, set_env, Address, BytesN, Env};
// use loam_sdk_core_riffs::Ownable;

// use crate::{error::Error, Contract};

// pub struct SorobanContract;

// #[contractimpl]
// impl SorobanContract {
//     pub fn add_u32(a: u32, b: u32) -> Result<u32, Error> {
//         Contract::add_u32(a, b)
//     }

//     /*

//     Base riff

//     */
//     pub fn owner_set(env: Env, owner: Address) {
//         set_env(env);
//         Contract::owner_set(owner);
//     }
//     pub fn owner_get(env: Env) -> Option<Address> {
//         set_env(env);
//         Contract::owner_get()
//     }

//     pub fn redeploy(env: Env, wasm_hash: BytesN<32>) {
//         set_env(env);
//         Contract::redeploy(wasm_hash);
//     }
// }
