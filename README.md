# Loam

A sdk and build tool for writting smart contracts in Rust on Wasm block chains.

Currently the focus will be on Soroban, but the same ideas apply to other VMs.

## Getting setup

Need to install `just`

```bash
cargo install just
```

Then setup:

```bash
just setup
```

To see redeploy in action use

```bash
just redeploy
```

## Contract Riff

A contract riff (or mixin) is a type that implements the `IntoKey` trait which is used to lazyily load and store the type.

```rust
#[contracttype]
#[derive(IntoKey)]
pub struct Messages(Map<Address, String>);
```

This will genereate 

```rust
impl IntoKey for Messages {
    type Key = IntoVal<Env, RawVal>;
    fn into_key() -> Self::Key {
      String::from_slice("messages")
    }
```

Next since a `contracttype` implements the needed traits the following blanket implementation for the `Lazy` trait can implement:

```rust
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
```

Lastly the `Message` Riff can have external API:

```rust
#[riff]
pub trait IsPostable {
    /// Documentation ends up in the contract's metadata and thus the CLI, etc
    fn messages_get(&self, author: Address) -> Option<String>;

    /// Only the author can set the message
    fn messages_set(&mut self, author: Address, text: String);
}
```

```rust
impl IsPostable for Message {
    fn messages_get(&self, author: Address) -> Option<String> {
        self.0.get(author).transpose().unwrap()
    }

    fn messages_set(&mut self, author: Address, text: String) {
        author.require_auth();
        self.0.set(author, text);
    }
}
```


## Core Riffs

The minimum logic needed for a contract is for it to be redeployable; a contract should be able to be redeployed to a contract which can be redeployed.

To be redeployed requires ownership since it would be bad for account to be able to redeploy the contract.

### `CoreRiff`
```rust

/// The trait that the instance riff must implement
pub trait IsCoreRiff {
    fn owner_get(&self) -> Option<Address>;
    /// 
    fn owner_set(&mut self, new_owner: Address);
    /// Only the owne can redeploy the contract
    fn redeploy(&mut self, wasm_hash: BytesN<32>);
}
```

## Using the core riffs

In the examples is the `loam-sdk-core-riff` contract, which just implements the base riffs. 


`lib.rs`:
```rust
pub struct Contract;

impl CoreRiff for Contract {
    type Impl = Owner;
}


soroban_contract!();
```
generates:

```rust
struct SorobanContract;

#[contractimpl]
impl SorobanContract {
     pub fn owner_set(env: Env, owner: Address) {
        set_env(env);
        Contract::owner_set(owner);
    }
    pub fn owner_get(env: Env) -> Option<Address> {
        set_env(env);
        Contract::owner_get()
    }
    pub fn redeploy(env: Env, wasm_hash: BytesN<32>) {
        set_env(env);
        Contract::redeploy(wasm_hash);
    }
    // Riff methods would be inserted here.
    // Contract must implement all Riffs and is the proxy for the contract calls.
    // This is because the Riffs have default implementations which call the associated type
}

```

Since the two traits have default methods we only need to specify what the associated type is for `CoreRiff`. The reason this is not the default is so that it's possible to use a different implementation of `IsCoreRiff`.

Notice that the generated code calls `Contract::redeploy`, etc.  This ensures that the `Contract` type is redeployable, but also allows for extension since `Contract` could overwrite the default methods.
