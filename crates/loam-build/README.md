# Loam-Build

Builds any package in your workspace that is labeled as a loam package. This contains the low level tools **used in loam-cli's build command AND loam-sdk-macro.**

Looks through the workspace to find any crates where the `Cargo.toml` file indicates that this crate is a loam contract. Such crates will contain this snippet in their `Cargo.toml`:

```toml
[package.metadata.loam]  
contract = true

# or, equivalently
package.metadata.loam.contract = true
```

Traverses all members within a workspace by looking at dependencies and builds contracts in the correct order.