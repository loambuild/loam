use soroban_sdk::{Env, IntoVal, Val};

pub trait IntoKey {
    type Key: IntoVal<Env, Val>;
    fn into_key() -> <Self as IntoKey>::Key;
}
