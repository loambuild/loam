use loam_sdk::{
    soroban_sdk::{self, symbol_short, Lazy, Symbol, Vec},
    subcontract, vec,
};

#[derive(Default, Lazy)]
pub struct Hello;

#[subcontract]
pub trait IsHelloWorld {
    fn hello(&self, to: Symbol) -> Vec<Symbol>;
}

impl IsHelloWorld for Hello {
    fn hello(&self, to: Symbol) -> Vec<Symbol> {
        vec![symbol_short!("Hello"), to]
    }
}
