# Loam-Build

Builds any package in your workspace that is labeled as a loam package. This contains the low level tools **used in loam-cli's build command AND loam-sdk-macro.**

Anything in `test/` is there to test loam build.

Looks through the workspace to find any crates where the `Cargo.toml` file indicates that a contract exists.

Indicating a contract exists is accomplished with this snippet:
```toml
[package.metadata.loam]  
contract = true
```

Traverses all members within a workspace by looking at dependencies and builds contracts in the correct order.