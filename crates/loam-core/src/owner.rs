use loam_sdk::{
    riff,
    soroban_sdk::{self, contracttype, env, symbol_short, Address, BytesN, Lazy, Symbol},
};

#[contracttype(export = false)]
#[derive(Default)]
pub struct Owner(Kind);

fn owner_key() -> Symbol {
    symbol_short!("owner")
}

impl Lazy for Owner {
    fn get_lazy() -> Option<Self> {
        env().storage().instance().get(&owner_key())
    }

    fn set_lazy(self) {
        env().storage().instance().set(&owner_key(), &self);
    }
}

/// Work around not having `Option` in `contracttype`
#[contracttype(export = false)]
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
        env().deployer().update_current_contract_wasm(wasm_hash);
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
