use loam_sdk::soroban_sdk::{self, contracttype, Address, IntoKey, Lazy};

#[contracttype]
#[derive(IntoKey, Default)]
pub struct Owner(Kind);

/// Work around not having `Option` in `contracttype`
#[contracttype]
#[derive(Default)]
pub enum Kind {
    Address(Address),
    #[default]
    None,
}

//#[loam]

impl AnOwnable for Owner {
    fn owner_get(&self) -> Option<Address> {
        match &self.0 {
            Kind::Address(address) => Some(address.clone()),
            Kind::None => None,
        }
    }

    fn owner_set(&mut self, new_owner: Address) {
        self.0 = Kind::Address(new_owner);
    }
}

pub trait AnOwnable {
    fn owner_get(&self) -> Option<Address>;
    fn owner_set(&mut self, new_owner: Address);
}

pub trait Ownable {
    type Impl: Lazy + AnOwnable + Default;
    fn owner_get() -> Option<Address> {
        Self::Impl::get_lazy()?.owner_get()
    }
    fn owner_set(owner: Address) {
        let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
        if let Some(current_owner) = impl_.owner_get() {
            current_owner.require_auth();
        }
        impl_.owner_set(owner);
        Self::Impl::set_lazy(impl_);
    }
}
