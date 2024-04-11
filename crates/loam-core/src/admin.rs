use loam_sdk::{
    riff,
    soroban_sdk::{self, contracttype, env, symbol_short, Address, BytesN, Lazy, Symbol},
};

#[contracttype(export = false)]
#[derive(Default)]
pub struct Admin(Kind);

fn admin_key() -> Symbol {
    symbol_short!("ADMIN")
}

impl Lazy for Admin {
    fn get_lazy() -> Option<Self> {
        env().storage().instance().get(&admin_key())
    }

    fn set_lazy(self) {
        env().storage().instance().set(&admin_key(), &self);
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

impl IsCore for Admin {
    fn admin_get(&self) -> Option<Address> {
        match &self.0 {
            Kind::Address(address) => Some(address.clone()),
            Kind::None => None,
        }
    }

    fn admin_set(&mut self, new_admin: Address) {
        if let Admin(Kind::Address(admin)) = &self {
            admin.require_auth();
        }
        self.0 = Kind::Address(new_admin);
    }

    fn redeploy(&self, wasm_hash: BytesN<32>) {
        self.admin_get().unwrap().require_auth();
        env().deployer().update_current_contract_wasm(wasm_hash);
    }
}

#[riff]
pub trait IsCore {
    /// Get current admin
    fn admin_get(&self) -> Option<loam_sdk::soroban_sdk::Address>;
    /// Transfer to new admin
    /// Should be called in the same transaction as deploying the contract to ensure that
    /// a different account try to become admin
    fn admin_set(&mut self, new_admin: loam_sdk::soroban_sdk::Address);

    /// Admin can redepoly the contract with given hash.
    fn redeploy(&self, wasm_hash: loam_sdk::soroban_sdk::BytesN<32>);
}
