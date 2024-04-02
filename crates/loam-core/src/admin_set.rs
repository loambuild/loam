// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    riff,
    soroban_sdk::{self, contracterror, contracttype, env, panic_with_error, Address, IntoKey, Lazy, Map, Vec},
};

#[contracttype]
#[derive(IntoKey)]
pub struct AdminSet(Map<Address, ()>);

impl Default for AdminSet {
    fn default() -> Self {
        Self(Map::new(env()))
    }
}

#[riff]
pub trait IsAdmins {
    /// Documentation ends up in the contract's metadata and thus the CLI, etc
    fn admins_contains(&self, author: loam_sdk::soroban_sdk::Address) -> bool;

    /// Initialze the admins
    fn admins_init(
       &mut self,
       admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
   );

    /// Only the author can set the message
    fn admins_set(
        &mut self,
        admins: loam_sdk::soroban_sdk::Vec<loam_sdk::soroban_sdk::Address>,
        new_member: loam_sdk::soroban_sdk::Address,
    );

}

impl IsAdmins for AdminSet {
    fn admins_contains(&self, member: Address) -> bool {
        self.0.contains_key(member)
    }

    fn admins_init(&mut self, admins: Vec<Address>) {
        if !self.0.is_empty() {
            panic_with_error!(env(), Error::AlreadyInitialized);
        }
        for author in admins {
            self.0.set(author, ());
        }
    }

    fn admins_set(&mut self, admins: Vec<Address>, new_member: Address) {
        if self.0.is_empty() {
            for author in admins {
                self.0.set(author, ());
            }
            return;
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
        self.0.set(new_member, ());
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
}
