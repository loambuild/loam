#![no_std]
// // Currently need to import `self` because `contracttype` expects it in the namespace
use loam_sdk::{
    riff, soroban_contract,
    soroban_sdk::{self, contracttype, get_env, Address, IntoKey, Lazy, Map, String},
};
use loam_sdk_core_riff::{owner::Owner, CoreRiff};

#[contracttype]
#[derive(IntoKey)]
pub struct Messages(Map<Address, String>);

impl Default for Messages {
    fn default() -> Self {
        Self(Map::new(get_env()))
    }
}

#[riff]
pub trait IsPostable {
    /// Documentation ends up in the contract's metadata and thus the CLI, etc
    fn messages_get(&self, author: Address) -> Option<String>;

    /// Only the author can set the message
    fn messages_set(&mut self, author: Address, text: String);
}

impl IsPostable for Messages {
    fn messages_get(&self, author: Address) -> Option<String> {
        self.0.get(author).transpose().unwrap()
    }

    fn messages_set(&mut self, author: Address, text: String) {
        author.require_auth();
        self.0.set(author, text);
    }
}

pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}

impl Postable for Contract {
    type Impl = Messages;
}

soroban_contract!();
