#![allow(unused_variables)]
use loam_sdk::soroban_sdk::BytesN;
pub trait Redeployable: crate::Ownable {
    fn redeploy(hash: BytesN<32>) {
        Self::owner_get().unwrap().require_auth();
        todo!("Currently not implementented")
    }
}
