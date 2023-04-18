extern crate soroban_sdk;
pub use soroban_sdk::*;

use crate::Lazy;

pub trait IntoKey {
    type Key: IntoVal<Env, RawVal>;
    fn into_key() -> Self::Key;
}

static mut ENV: Option<Env> = None;

pub fn set_env(env: Env) {
    unsafe { ENV = Some(env) };
}

pub fn env() -> &'static Env {
    unsafe { ENV.as_ref().unwrap() }
}

impl<T> Lazy for T
where
    T: IntoKey + TryFromVal<Env, RawVal> + IntoVal<Env, RawVal>,
{
    fn get_lazy() -> Option<Self> {
        env().storage().get(&Self::into_key()).transpose().unwrap()
    }

    fn set_lazy(self) {
        env().storage().set(&Self::into_key(), &self)
    }
}

pub use macro_wrapper::IntoKey;
