// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    riff,
    soroban_sdk::{self, contracttype, env, panic_with_error, Address, IntoKey, Lazy, Map, Vec},
};

use crate::error::Error;

#[contracttype]
#[derive(IntoKey)]
pub struct AdminSet(Map<Address, ()>);

impl Default for AdminSet {
    fn default() -> Self {
        Self::new()
    }
}

impl AdminSet {
    pub fn new() -> Self {
        Self(Map::new(env()))
    }

    pub fn contains(&self, member: Address) -> bool {
        self.0.contains_key(member)
    }

    pub fn add_member(&mut self, author: Address) {
        self.0.set(author, ());
    }

    pub fn len(&self) -> u32 {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn vec(&self) -> Vec<Address> {
        self.0.keys()
    }
}

#[riff]
pub trait IsAdmins {
    /// Return the value associated with the given admin if any
    fn admins_contains(&self, member: loam_sdk::soroban_sdk::Address) -> bool;
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

    fn admins_set(&mut self, admins: Vec<Address>, new_member: Address) {
        if self.is_empty() {
            for author in admins {
                self.add_member(author)
            }
            return;
        }
        if admins.len() < self.len() / 2 {
            panic_with_error!(env(), Error::NonEnoughSignatures);
        }
        for author in admins {
            if !self.admins_contains(author.clone()) {
                panic_with_error!(env(), Error::AccountIsNotAdmin);
            }
            author.require_auth()
        }
        self.add_member(new_member);
    }
}
