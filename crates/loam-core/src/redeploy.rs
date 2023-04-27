#![allow(unused_variables)]
use loam_sdk::soroban_sdk::{get_env, BytesN};

pub trait Redeployable: crate::Ownable {
    fn redeploy(wasm_hash: BytesN<32>) {
        Self::owner_get().unwrap().require_auth();
        get_env().update_current_contract_wasm(&wasm_hash);
    }
}
