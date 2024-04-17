# Loam

Loam is a set of tools:
  1. Loam SDK - Create Smart Contracts for Soroban using smaller, more flexible building blocks called "Subcontracts".
  2. Loam Frontend - Modern frontend tooling paired with a declarative environment configurations to help with local, test, and live blockchain networks.
  3. Loam CLI - Build and deploy Loam Frontend to work with Smart Contracts.

This project is a mono repo containing code primarily for the Loam SDK but also for the Loam CLI, both of which share build code created because Smart contracts built with Loam often cannot be built correctly with a standard cargo build. They may have complex subcontract interdependencies that need to be resolved in the correct order. So, `loam-build` guarantees that subcontracts get compiled in the correct order. 

This readme primarily serves to direct you to more specific readme's.

# What is Loam SDK and what is Loam CLI?

The Software Development Kit (SDK) and build tool is for writing smart contracts, using the concept  in Rust on Wasm blockchains. Currently, the focus is on the Soroban VM, but the same ideas apply to other VMs.

The Command Line Interface (CLI) is for creating, developing, and deploying a loam project with a frontend using a file that defines network settings, accounts, and contracts.


# Loam SDK and Loam CLI in Depth
- [Loam SDK](crates/loam-sdk/README.md)
  - [loam-subcontract-core](./crates/loam-subcontract-core) - The most basic form of a subcontract, creating an admin/ownsership trait.
  - [loam-sdk-macro](crates/loam-sdk-macro/README.md)
- [Loam CLI](crates/loam-cli/README.md)
- [Loam Build](crates/loam-build/README.md)

# Examples of Loam SDK Created Subcontracts
- [Core Subcontract](examples/soroban/core) - This is required for the creation of all other subcontracts and can be seen in the other examples within [`examples/`](examples)
- [Fungible Tokens Subcontract](examples/soroban/ft) - This contains the implementation of a Fungible Token Subcontract interface. Find the interface inside of [`crates/loam-subcontract-ft`](crates/loam-subcontract-ft)