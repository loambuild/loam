use loam_sdk::soroban_sdk::{self, contracttype, Address, IntoKey, Lazy};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Owner(OwnerKind);

/// Work around not having `Option` in `contracttype`
#[contracttype]
#[derive(Default)]
pub enum OwnerKind {
    Address(Address),
    #[default]
    None,
}

//#[loam]

pub trait AnOwnable {
    fn owner_get(&self) -> Option<Address>;
    fn owner_set(&mut self, new_owner: Address);
}

impl AnOwnable for Owner {
    fn owner_get(&self) -> Option<Address> {
        match &self.0 {
            OwnerKind::Address(address) => Some(address.clone()),
            OwnerKind::None => None,
        }
    }

    fn owner_set(&mut self, new_owner: Address) {
        if let OwnerKind::Address(current_owner) = &self.0 {
            current_owner.require_auth()
        };
        self.0 = OwnerKind::Address(new_owner);
    }
}

pub trait Ownable {
    type Impl: Lazy + AnOwnable;
    fn owner_get() -> Option<Address> {
        Self::Impl::get_lazy()?.owner_get()
    }
    fn owner_set(owner: Address) {
        let mut impl_ = Self::Impl::get_lazy().unwrap();
        impl_.owner_set(owner);
        Self::Impl::set_lazy(impl_)
    }
}
