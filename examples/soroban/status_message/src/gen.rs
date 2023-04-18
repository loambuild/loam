use loam_sdk::soroban_sdk::{
    self, contractimpl, get_env, set_env, Address, Env, Lazy, Map, String,
};

use crate::Messages;

pub struct SorobanContract;

impl Default for Messages {
    fn default() -> Self {
        Self(Map::new(get_env()))
    }
}

#[contractimpl]
impl SorobanContract {
    pub fn messages_get(env: Env, author: Address) -> Option<String> {
        set_env(env);
        let this = Messages::get_lazy().unwrap();
        this.get(author)
    }

    pub fn message_set(env: Env, author: Address, text: String) {
        set_env(env);
        let mut this = Messages::get_lazy().unwrap_or_default();
        this.set(author, text);
        Messages::set_lazy(this);
    }
}
