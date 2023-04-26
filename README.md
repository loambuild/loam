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
#[loam]
impl Messages {
    pub fn get(&self, author: Address) -> Option<String> {
        self.0.get(author).transpose().unwrap()
    }

    pub fn set(&mut self, author: Address, text: String) {
        author.require_auth();
        self.0.set(author, text);
    }
}
```

Generates to

```rust
#[contractimpl]
impl SorobanContract {
    pub fn messages_get(env: Env, author: Address) -> Option<String> {
        set_env(env);
        let this = Messages::get_lazy()?;
        this.get(author)
    }

    pub fn messages_set(env: Env, author: Address, text: String) {
        set_env(env);
        let mut this = Messages::get_lazy().unwrap_or_default();
        this.set(author, text);
        Messages::set_lazy(this);
    }
}
```


## Core Riffs

The minimum logic needed for a contract is for it to be redeployable; a contract should be able to be redeployed to a contract which can be redeployed.

To be redeployed requires ownership since it would be bad for anyone to be able to redeploy the contract. Thus the two Riffs needed are `Ownable` and `Redeployable`

### `Ownable`
```rust

/// The trait that the instance riff must implement
pub trait AnOwnable {
    fn owner_get(&self) -> Option<Address>;
    fn owner_set(&mut self, new_owner: Address);
}

/// Contract level interface
pub trait Ownable {
    type Impl: Lazy + AnOwnable + Default;
    // The associated type must be Lazy, to be loaded and set
    // Must be `AnOwnable` so that the instance implements the needed methods
    // Must implement default so that when setting the owner it can be constructed if not set

    fn owner_get() -> Option<Address> {
        Self::Impl::get_lazy()?.owner_get()
    }
    fn owner_set(owner: Address) {
        let mut impl_ = Self::Impl::get_lazy().unwrap_or_default();
        if let Some(current_owner) = impl_.owner_get() {
            current_owner.require_auth();
        }
        impl_.owner_set(owner);
        Self::Impl::set_lazy(impl_);
    }
}

```

### `Redeployable`

The previous riff is stateful, however, the `Redeployable` is functional, though it depends on the contract being `Ownable`.

```rust
pub trait Redeployable: crate::Ownable {
    fn redeploy(wasm_hash: BytesN<32>) {
        /// Since `Self` is `Ownable` we can call `owner_get`
        Self::owner_get().unwrap().require_auth();
        get_env().update_current_contract_wasm(&wasm_hash);
    }
}
```


## Using the core riffs

In the examples is the `base` contract, which just implements the base riffs. 


`lib.rs`:
```rust
pub struct Contract;

impl Ownable for Contract {
    type Impl = Owner;
}

impl Redeployable for Contract {}
```

`gen.rs`
```rust
use crate::Contract;

pub struct SorobanContract;

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
}

```

Since the two traits have default methods we only need to specify what the associated type is for `Ownable`. The reason this is not the default is so that it's possible to use a different implementation `AnOwner`.

Notice that the generated code calls `Contract::redeploy`, etc.  This ensures that the `Contract` type is redeployable, but also allows for extension since `Contract` could overwrite the default methods.
