use loam_sdk::{
    riff,
    soroban_sdk::{self, contracttype, env, Address, BytesN, IntoKey, Lazy},
};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Owner(Kind);

/// Work around not having `Option` in `contracttype`
#[contracttype]
#[derive(Default)]
pub enum Kind {
    Address(Address),
    #[default]
    None,
}

impl IsCoreRiff for Owner {
    fn owner_get(&self) -> Option<Address> {
        match &self.0 {
            Kind::Address(address) => Some(address.clone()),
            Kind::None => None,
        }
    }

    fn owner_set(&mut self, new_owner: Address) {
        if let Kind::Address(owner) = &self.0 {
            owner.require_auth();
        }
        self.0 = Kind::Address(new_owner);
    }

    fn redeploy(&self, wasm_hash: BytesN<32>) {
        self.owner_get().unwrap().require_auth();
        env().update_current_contract_wasm(&wasm_hash);
    }
}

#[riff]
pub trait IsCoreRiff {
    /// Get current owner
    fn owner_get(&self) -> Option<loam_sdk::soroban_sdk::Address>;
    /// Transfer ownership if already set.
    /// Should be called in the same transaction as deploying the contract to ensure that
    /// a different account doesn't claim ownership
    fn owner_set(&mut self, new_owner: loam_sdk::soroban_sdk::Address);

    /// Owner can redepoly the contract with given hash.
    fn redeploy(&self, wasm_hash: loam_sdk::soroban_sdk::BytesN<32>);
}
