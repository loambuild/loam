// #![no_std]
// // Currently need to import `self` because `contracttype` expects it in the namespace
// use loam_sdk::soroban_sdk::{self, contracttype, Address, IntoKey, Map, String};
// use loam_sdk_core_riffs::{owner::Owner, Ownable};

// pub mod gen;

// #[contracttype]
// #[derive(IntoKey)]
// pub struct Messages(Map<Address, String>);

// //#[loam]
// impl Messages {
//     pub fn get(&self, author: Address) -> Option<String> {
//         self.0.get(author).transpose().unwrap()
//     }

//     pub fn set(&mut self, author: Address, text: String) {
//         author.require_auth();
//         self.0.set(author, text);
//     }
// }

// impl Ownable for Messages {
//     type Impl = Owner;
// }
