// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    riff,
    soroban_sdk::{
        self, contracterror, contracttype, env, panic_with_error, symbol_short, Address, Lazy, Map, Val, Vec
    },
};

use crate::ContractCall;

#[contracttype]
pub struct AdminSet(Map<Address, ()>);

/// Ensure that `AdminSet` is in instance storage
impl Lazy for AdminSet {
    fn get_lazy() -> Option<Self> {
        env().storage().instance().get(&symbol_short!("ADMINSET"))
    }

    fn set_lazy(self) {
        env()
            .storage()
            .instance()
            .set(&symbol_short!("ADMINSET"), &self);
    }
}

impl Default for AdminSet {
    fn default() -> Self {
        Self(Map::new(env()))
    }
}

#[riff]
pub trait IsAdmins {
    /// Initialze the admins
    fn admins_init(&mut self, admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>);
    /// Only the author can set the message
    fn admins_set(
        &mut self,
        admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
        new_member: loam_sdk::soroban_sdk::Address,
    );

    /// Assert that the provided admins are a majority of the current admins and have signed
    fn admins_approve(
        &self,
        admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
    );

    /// Test if the given address is an admin
    fn admins_contains(&self, member: loam_sdk::soroban_sdk::Address) -> bool;

    /// If a the list of provided admins is a majority of the current admins and they have all signed,
    /// then the contract call will be invoked.
    fn admins_invoke(
        &self,
        admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
        contract_call: (
            Option<loam_sdk::soroban_sdk::Address>,
            loam_sdk::soroban_sdk::Symbol,
            loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Val>,
        ),
    ) -> loam_sdk::soroban_sdk::Val;
}

impl IsAdmins for AdminSet {
    fn admins_contains(&self, member: Address) -> bool {
        self.0.contains_key(member)
    }

    fn admins_init(&mut self, admins: Vec<Address>) {
        if !self.0.is_empty() {
            panic_with_error!(env(), Error::AlreadyInitialized);
        }
        if admins.is_empty() {
            panic_with_error!(env(), Error::NonEnoughSignatures);
        }
        for author in admins {
            self.0.set(author, ());
        }
    }

    fn admins_set(&mut self, admins: Vec<Address>, new_member: Address) {
        self.admins_approve(admins);
        self.0.set(new_member, ());
    }

    fn admins_approve(
        &self,
        admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
    ) {
        if self.0.is_empty() {
            panic_with_error!(env(), Error::Uninitialized);
        }
        if admins.len() < self.0.len() / 2 {
            panic_with_error!(env(), Error::NonEnoughSignatures);
        }
        for author in admins {
            if !self.admins_contains(author.clone()) {
                panic_with_error!(env(), Error::AccountIsNotAdmin);
            }
            author.require_auth();
        }
    }

    fn admins_invoke(
        &self,
        admins: Vec<loam_sdk::soroban_sdk::Address>,
        contract_call: (
            Option<loam_sdk::soroban_sdk::Address>,
            loam_sdk::soroban_sdk::Symbol,
            loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Val>,
        ),
    ) -> loam_sdk::soroban_sdk::Val {
        self.admins_approve(admins);
        let contract_call: ContractCall = contract_call.into();
        contract_call.try_invoke::<Val>().unwrap()
    }
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Not enough signatures
    NonEnoughSignatures = 1,
    /// Account is not admin
    AccountIsNotAdmin = 2,
    /// Already initialized
    AlreadyInitialized = 3,
    /// Need at least one initial admin address
    NoInitialAdmin = 4,
    /// Contract Address is incorrect
    IncorrectContractAddress = 5,
    /// Contract method name is incorrect
    IncorrectContractMethodName = 6,
    /// Contract method args are incorrect
    IncorrectArgs = 7,
    /// Invoke failed
    InvokeFailed = 8,
    /// Contract not initialized
    Uninitialized = 9,
}
