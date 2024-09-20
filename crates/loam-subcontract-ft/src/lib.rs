#![no_std]
use loam_sdk::{soroban_sdk::Lazy, subcontract};

/// SEP-41: Fungible Token Interface
///
/// This trait defines a standard contract interface for fungible tokens on the Stellar network.
/// It is a subset of the Stellar Asset contract and is compatible with the descriptive and token
/// interfaces defined in CAP-46-6.
///
/// SEP-41 aims to provide a less opinionated interface than the Stellar Asset contract,
/// supporting standard token functionality without the specialized behaviors of Stellar Assets.
/// This allows for greater flexibility and interoperability among different token implementations.
///
/// For full specification, see: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md
#[subcontract]
pub trait IsSep41 {
    /// Returns the allowance for `spender` to transfer from `from`.
    fn allowance(&self, from: loam_sdk::soroban_sdk::Address, spender: loam_sdk::soroban_sdk::Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from `from`.
    fn approve(&mut self, from: loam_sdk::soroban_sdk::Address, spender: loam_sdk::soroban_sdk::Address, amount: i128, live_until_ledger: u32);

    /// Returns the balance of `id`.
    fn balance(&self, id: loam_sdk::soroban_sdk::Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    fn transfer(&mut self, from: loam_sdk::soroban_sdk::Address, to: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    fn transfer_from(&mut self, spender: loam_sdk::soroban_sdk::Address, from: loam_sdk::soroban_sdk::Address, to: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Burn `amount` from `from`.
    fn burn(&mut self, from: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    fn burn_from(&mut self, spender: loam_sdk::soroban_sdk::Address, from: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(&self) -> u32;

    /// Returns the name for this token.
    fn name(&self) -> loam_sdk::soroban_sdk::String;

    /// Returns the symbol for this token.
    fn symbol(&self) -> loam_sdk::soroban_sdk::String;

}

#[subcontract]
pub trait IsFungible: IsSep41 {
    /// Increases the allowance that one address can spend on behalf of another address.
    fn increase_allowance(&mut self, from: loam_sdk::soroban_sdk::Address, spender: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Decreases the allowance that one address can spend on behalf of another address.
    fn decrease_allowance(&mut self, from: loam_sdk::soroban_sdk::Address, spender: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Returns the spendable balance of tokens for a specific address.
    fn spendable_balance(&self, id: loam_sdk::soroban_sdk::Address) -> i128;

    /// Checks if a specific address is authorized.
    fn authorized(&self, id: loam_sdk::soroban_sdk::Address) -> bool;

    /// Sets the authorization status of a specific address.
    fn set_authorized(&mut self, id: loam_sdk::soroban_sdk::Address, authorize: bool);

    /// Mints a specified amount of tokens to a specific address.
    fn mint(&mut self, to: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Retrieves a specified amount of tokens from a specific address (clawback).
    fn clawback(&mut self, from: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Sets a new admin address.
    fn set_admin(&mut self, new_admin: loam_sdk::soroban_sdk::Address);
}

#[subcontract]
pub trait IsInitable {
    /// Initialize ft Subcontract
    fn ft_init(
        &mut self,
        admin: loam_sdk::soroban_sdk::Address,
        name: loam_sdk::soroban_sdk::String,
        symbol: loam_sdk::soroban_sdk::String,
        decimals: u32,
    );
}
