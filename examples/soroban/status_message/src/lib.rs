#![no_std]
// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::soroban_sdk::{self, contracttype, Address, IntoKey, Map, String};

pub mod gen;

#[contracttype]
#[derive(IntoKey)]
pub struct Messages(Map<Address, String>);

//#[loam]
impl Messages {
    pub fn get(&self, author: Address) -> Option<String> {
        self.0.get(author).transpose().unwrap()
    }

    pub fn set(&mut self, author: Address, text: String) {
        author.require_auth();
        self.0.set(author, text);
    }
}

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Owner(OwnerKind);

#[contracttype]
#[derive(Default)]
pub enum OwnerKind {
    Address(Address),
    #[default]
    None,
}

//#[loam]
impl Owner {
    pub fn get(&self) -> Option<Address> {
        match &self.0 {
            OwnerKind::Address(address) => Some(address.clone()),
            OwnerKind::None => None,
        }
    }

    pub fn set(&mut self, owner: Address) {
        if let OwnerKind::Address(current_owner) = &self.0 {
            current_owner.require_auth()
        };
        self.0 = OwnerKind::Address(owner);
    }
}
