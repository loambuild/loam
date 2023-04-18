use loam_sdk::{
    soroban_sdk::{self, contractimpl, set_env, Address, Env, IntoSorobanKey, String, Symbol},
    Lazy,
};

use crate::Messages;

impl IntoSorobanKey for Messages {
    type Key = Symbol;
    fn into_key() -> Symbol {
        Symbol::short("messages")
    }
}

pub struct SorobanContract;

#[contractimpl]
impl SorobanContract {
    pub fn messages_get(env: Env, author: Address) -> Option<String> {
        set_env(env);
        Messages::get_lazy().unwrap().get(author)
    }

    pub fn message_set(env: Env, author: Address, text: String) {
        set_env(env);
        let mut this = Messages::get_lazy().unwrap();
        this.set(author, text);
        Messages::set_lazy(this);
    }
}
