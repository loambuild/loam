# Loam

Build **composable**, **upgradeable**, **secure** Smart Contracts. 

Build frontends that interact easily with (your own or anyone's) Smart Contracts.

Focus on the important parts of your [Soroban](https://soroban.stellar.org/) app. Let Loam handle the tedious bits.

Loam is comprised of three main components:
  1. Loam SDK - Create Smart Contracts for Soroban using smaller, more flexible building blocks called "Subcontracts".
  2. Loam Frontend - Modern frontend tooling paired with a declarative environment configurations to help with local, test, and live blockchain networks.
  3. Loam CLI - Build and deploy Loam Frontend to work with Smart Contracts.

This project is a mono repo containing code primarily for the Loam SDK but also for the Loam CLI, both of which share build code.

This shared build code was created because Smart contracts built with Loam often cannot be built correctly with a standard `cargo build`. They may have complex subcontract interdependencies that need to be resolved in the correct order. So, `loam-build` guarantees that subcontracts get compiled in the correct order. 

This readme primarily serves to direct you to more specific readme's about the SDK and CLI and examples.

# What is Loam SDK and what is Loam CLI?

The Software Development Kit (SDK) and build tool is for writing smart contracts by creating and assembling "subcontracts". Think of subcontracts as lego blocks you can snap together into a more complex whole. Currently, the SDK is focused on Stellar/Soroban smart contracts, but the subcontract pattern can be made to work with other blockchains as well.

The Command Line Interface (CLI) is for building smart contracts authored with Loam SDK. It's also for easily working with smart contract dependencies in a Loam frontend.


# Loam SDK and Loam CLI in Depth
- [Loam SDK](crates/loam-sdk/README.md) - Tool for creating subcontracts.
  - [loam-subcontract-core](./crates/loam-subcontract-core) - The most basic form of a subcontract, creating an admin/ownsership trait.
  - [loam-sdk-macro](crates/loam-sdk-macro/README.md) - Code for the `#[subcontract]` macro to create your own brand new subcontract, if existing subcontracts do not suffice.
- [Loam CLI](crates/loam-cli/README.md) - Build smart contracts authored with Loam SDK, manage smart contract dependencies from a frontend, initialize new loam projects
- [Loam Build](crates/loam-build/README.md) - Used by CLI and SDK to look at dependencies and build contracts in the correct order.

# Examples of Loam SDK Created Subcontracts
- [Core Subcontract](examples/soroban/core) - This is required for the creation of all other subcontracts and can be seen in the other examples within [`examples/`](examples)
- [Fungible Tokens Subcontract](examples/soroban/ft) - This contains the implementation of a Fungible Token Subcontract interface. Find the interface inside of [`crates/loam-subcontract-ft`](crates/loam-subcontract-ft)
- [Loam Frontend Repository](https://github.com/loambuild/frontend?tab=readme-ov-file#loam-dev) - A template of how Loam CLI commands work with Loam Frontend.