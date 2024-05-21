# Loam-Build

This contains the low level tools **used in loam-cli's build command AND loam-sdk-macro.** Helps build any package in your workspace that is labeled as a loam package. It does this by looking through the workspace to find any crates where the `Cargo.toml` file indicates that this crate is a loam contract. Such crates will contain this snippet in their `Cargo.toml`:

```toml
[package.metadata.loam]  
contract = true

# or, equivalently
package.metadata.loam.contract = true
```

It will find all these dependencies and builds them in the correct order.
