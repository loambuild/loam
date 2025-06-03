use loam_sdk::{
    soroban_sdk::{self, contracttype, env, symbol_short, Address, BytesN, Lazy, Symbol},
    subcontract,
};

#[contracttype(export = false)]
pub struct Admin(Address);

impl Default for Admin {
    fn default() -> Self {
        // Admin should always be initialized in the constructor
        unreachable!()
    }
}

fn admin_key() -> Symbol {
    symbol_short!("ADMIN")
}

impl Lazy for Admin {
    fn get_lazy() -> Option<Self> {
        env().storage().instance().get(&admin_key()).map(Admin)
    }

    fn set_lazy(self) {
        env().storage().instance().set(&admin_key(), &self.0);
    }
}

impl IsCore for Admin {
    fn admin_get(&self) -> Option<Address> {
        Some(self.0.clone())
    }

    fn admin_set(&mut self, new_admin: Address) {
        self.0.require_auth();
        self.0 = new_admin;
    }

    fn upgrade(&self, wasm_hash: BytesN<32>) {
        self.0.require_auth();
        env().deployer().update_current_contract_wasm(wasm_hash);
    }

    fn __constructor(admin: Address) {
        Self::set_lazy(Self(admin));
    }
}

#[subcontract]
pub trait IsCore {
    /// Get current admin
    fn admin_get(&self) -> Option<loam_sdk::soroban_sdk::Address>;
    /// Transfer to new admin
    /// Should be called in the same transaction as deploying the contract to ensure that
    /// a different account try to become admin
    fn admin_set(&mut self, new_admin: loam_sdk::soroban_sdk::Address);

    /// Admin can upgrade the contract with given hash.
    fn upgrade(&self, wasm_hash: loam_sdk::soroban_sdk::BytesN<32>);

    /// Constructor to set the admin
    fn __constructor(admin: loam_sdk::soroban_sdk::Address);
}
