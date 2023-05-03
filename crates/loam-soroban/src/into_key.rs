use soroban_sdk::{Env, IntoVal, RawVal};

pub trait IntoKey {
    type Key: IntoVal<Env, RawVal>;
    fn into_key() -> <Self as IntoKey>::Key;
}
