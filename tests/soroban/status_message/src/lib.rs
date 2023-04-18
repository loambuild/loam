#![no_std]
// Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::soroban_sdk::{self, contracttype, Address, Map, String};

pub mod gen;

#[contracttype]
// #[derive(IntoSorobanKey)]
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
