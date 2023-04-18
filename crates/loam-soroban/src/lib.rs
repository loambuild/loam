#![no_std]
pub use soroban_sdk::*;

pub trait Lazy: Sized {
    fn get_lazy() -> Option<Self>;

    fn set_lazy(self);
}

pub trait IntoKey {
    type Key: IntoVal<Env, RawVal>;
    fn into_key() -> Self::Key;
}

static mut ENV: Option<Env> = None;

pub fn set_env(env: Env) {
    unsafe { ENV = Some(env) };
}

pub fn get_env() -> &'static Env {
    unsafe { ENV.as_ref().unwrap() }
}

impl<T> Lazy for T
where
    T: IntoKey + TryFromVal<Env, RawVal> + IntoVal<Env, RawVal>,
{
    fn get_lazy() -> Option<Self> {
        get_env()
            .storage()
            .get(&Self::into_key())
            .transpose()
            .unwrap()
    }

    fn set_lazy(self) {
        get_env().storage().set(&Self::into_key(), &self)
    }
}

pub use macro_wrapper::IntoKey;
