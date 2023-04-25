# Loam

A sdk and build tool for writting smart contracts in Rust on Wasm block chains.

Currently the focus will be on Soroban, but the same ideas apply to other VMs.

## Getting setup

Need to install `just`

```bash
cargo install just
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
