#![no_std]
use loam_sdk::{soroban_sdk::Lazy, subcontract};
/// The `IsFungible` trait defines methods for implementing a fungible token on the Soroban blockchain.
/// Fungible tokens are assets that can be exchanged for one another, like a standard currency.
#[subcontract]
pub trait IsFungible {
    /// Determines the amount of tokens that one address is allowed to spend on behalf of another address.
    fn allowance(
        &self,
        from: loam_sdk::soroban_sdk::Address,
        spender: loam_sdk::soroban_sdk::Address,
    ) -> i128;

    /// Increases the allowance that one address can spend on behalf of another address.
    fn increase_allowance(
        &mut self,
        from: loam_sdk::soroban_sdk::Address,
        spender: loam_sdk::soroban_sdk::Address,
        amount: i128,
    );

    /// Decreases the allowance that one address can spend on behalf of another address.
    fn decrease_allowance(
        &mut self,
        from: loam_sdk::soroban_sdk::Address,
        spender: loam_sdk::soroban_sdk::Address,
        amount: i128,
    );

    /// Returns the balance of tokens held by a specific address.
    fn balance(&self, id: loam_sdk::soroban_sdk::Address) -> i128;

    /// Returns the spendable balance of tokens for a specific address.
    fn spendable_balance(&self, id: loam_sdk::soroban_sdk::Address) -> i128;

    /// Checks if a specific address is authorized.
    fn authorized(&self, id: loam_sdk::soroban_sdk::Address) -> bool;

    /// Transfers tokens from one address to another.
    fn transfer(
        &mut self,
        from: loam_sdk::soroban_sdk::Address,
        to: loam_sdk::soroban_sdk::Address,
        amount: i128,
    );

    /// Transfers tokens from one address to another, with a spender address controlling the transfer.
    fn transfer_from(
        &mut self,
        spender: loam_sdk::soroban_sdk::Address,
        from: loam_sdk::soroban_sdk::Address,
        to: loam_sdk::soroban_sdk::Address,
        amount: i128,
    );

    /// Burns a specified amount of tokens from a specific address.
    fn burn(&mut self, from: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Burns a specified amount of tokens from a specific address, with a spender address controlling the burn.
    fn burn_from(
        &mut self,
        spender: loam_sdk::soroban_sdk::Address,
        from: loam_sdk::soroban_sdk::Address,
        amount: i128,
    );

    /// Sets the authorization status of a specific address.
    fn set_authorized(&mut self, id: loam_sdk::soroban_sdk::Address, authorize: bool);

    /// Mints a specified amount of tokens to a specific address.
    fn mint(&mut self, to: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Retrieves a specified amount of tokens from a specific address (clawback).
    fn clawback(&mut self, from: loam_sdk::soroban_sdk::Address, amount: i128);

    /// Sets a new admin address.
    fn set_admin(&mut self, new_admin: loam_sdk::soroban_sdk::Address);

    /// Returns the number of decimal places the token supports.
    fn decimals(&self) -> u32;

    /// Returns the name of the token as a byte array.
    fn name(&self) -> loam_sdk::soroban_sdk::Bytes;

    /// Returns the symbol of the token as a byte array.
    fn symbol(&self) -> loam_sdk::soroban_sdk::Bytes;
}

#[subcontract]
pub trait IsInitable {
    /// Initialize ft Subcontract
    fn ft_init(
        &self,
        admin: loam_sdk::soroban_sdk::Address,
        name: loam_sdk::soroban_sdk::Bytes,
        symbol: loam_sdk::soroban_sdk::Bytes,
        decimals: u32,
    );
}
