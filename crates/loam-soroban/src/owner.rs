// use soroban_sdk::Address;

// use crate::{contracttype, IntoKey};

// #[contracttype]
// #[derive(IntoKey, Default)]
// pub struct Owner(OwnerKind);

// #[contracttype]
// #[derive(Default)]
// pub enum OwnerKind {
//     Address(Address),
//     #[default]
//     None,
// }

// //#[loam]
// impl Owner {
//     pub fn get(&self) -> Option<Address> {
//         match &self.0 {
//             OwnerKind::Address(address) => Some(address.clone()),
//             OwnerKind::None => None,
//         }
//     }

//     pub fn set(&mut self, owner: Address) {
//         if let OwnerKind::Address(current_owner) = &self.0 {
//             current_owner.require_auth()
//         };
//         self.0 = OwnerKind::Address(owner);
//     }
// }

// trait Ownable {
//     fn get() -> Option<Address>;
//     fn set(owner: Address);
// }

// impl Ownable for Owner {}
