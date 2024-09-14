use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Bytes, Lazy, Map},
    IntoKey,
};
use loam_subcontract_core::Core;
use loam_subcontract_ft::{IsFungible, IsInitable};

use crate::Contract;

#[contracttype]
pub struct Txn(Address, Address);

#[contracttype]
#[derive(IntoKey)]
pub struct MyFungibleToken {
    balances: Map<Address, i128>,
    allowances: Map<Txn, i128>,
    authorized: Map<Address, bool>,
    admin: Address,
    name: Bytes,
    symbol: Bytes,
    decimals: u32,
}

impl MyFungibleToken {
    #[must_use]
    pub fn new(admin: Address, name: Bytes, symbol: Bytes, decimals: u32) -> Self {
        MyFungibleToken {
            balances: Map::new(env()),
            allowances: Map::new(env()),
            authorized: Map::new(env()),
            admin,
            name,
            symbol,
            decimals,
        }
    }
}

impl Default for MyFungibleToken {
    fn default() -> Self {
        Self::new(
            env().current_contract_address(),
            Bytes::new(env()),
            Bytes::new(env()),
            0,
        )
    }
}

impl IsInitable for MyFungibleToken {
    fn ft_init(&mut self, admin: Address, name: Bytes, symbol: Bytes, decimals: u32) {
        Contract::admin_get().unwrap().require_auth();
        MyFungibleToken::set_lazy(MyFungibleToken::new(admin, name, symbol, decimals));
    }
}

impl IsFungible for MyFungibleToken {
    fn allowance(&self, from: Address, spender: Address) -> i128 {
        self.allowances.get(Txn(from, spender)).unwrap_or_default()
    }

    fn approve(&mut self, from: Address, spender: Address, amount: i128, live_until_ledger: u32) {
        from.require_auth();
        self.allowances.set(Txn(from, spender), amount);
        env().ledger().set_expiration_ledger(live_until_ledger);
    }

    fn balance(&self, id: Address) -> i128 {
        self.balances.get(id).unwrap_or_default()
    }

    fn transfer(&mut self, from: Address, to: Address, amount: i128) {
        from.require_auth();
        let from_balance = self.balance(from.clone()) - amount;
        let to_balance = self.balance(to.clone()) + amount;
        self.balances.set(from, from_balance);
        self.balances.set(to, to_balance);
    }

    fn transfer_from(&mut self, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        let allowance = self.allowance(from.clone(), spender.clone());
        if allowance >= amount {
            self.transfer(from.clone(), to, amount);
            self.decrease_allowance(from, spender, amount);
        }
    }

    fn burn(&mut self, from: Address, amount: i128) {
        from.require_auth();
        let balance = self.balance(from.clone()) - amount;
        self.balances.set(from, balance);
    }

    fn burn_from(&mut self, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        let allowance = self.allowance(from.clone(), spender.clone());
        if allowance >= amount {
            self.burn(from.clone(), amount);
            self.decrease_allowance(from, spender, amount);
        }
    }

    fn increase_allowance(&mut self, from: Address, spender: Address, amount: i128) {
        from.require_auth();
        let new_allowance = self.allowance(from.clone(), spender.clone()) + amount;
        self.allowances.set(Txn(from, spender), new_allowance);
    }
    fn decrease_allowance(&mut self, from: Address, spender: Address, amount: i128) {
        from.require_auth();
        let new_allowance = self.allowance(from.clone(), spender.clone()) - amount;
        self.allowances.set(Txn(from, spender), new_allowance);
    }

    fn spendable_balance(&self, id: Address) -> i128 {
        self.balance(id)
    }

    fn authorized(&self, id: Address) -> bool {
        self.authorized.get(id).unwrap_or_default()
    }

    fn set_authorized(&mut self, id: Address, authorize: bool) {
        self.admin.require_auth();
        self.authorized.set(id, authorize);
    }

    fn mint(&mut self, to: Address, amount: i128) {
        self.admin.require_auth();
        let balance = self.balance(to.clone()) + amount;
        self.balances.set(to, balance);
    }

    fn clawback(&mut self, from: Address, amount: i128) {
        self.admin.require_auth();
        let balance = self.balance(from.clone()) - amount;
        self.balances.set(from, balance);
    }

    fn decimals(&self) -> u32 {
        self.decimals
    }

    fn name(&self) -> String {
        String::from_slice(env(), &self.name)
    }

    fn symbol(&self) -> String {
        String::from_slice(env(), &self.symbol)
    }
}