# Loam

Build **composable**, **upgradeable**, **secure** Smart Contracts. 

Build frontends that interact easily with (your own or anyone's) Smart Contracts.

Focus on the important parts of your [Soroban](https://soroban.stellar.org/) app. Let Loam handle the tedious bits.

At a high level, Loam is comprised of three main components:

1. Loam SDK - The Software Development Kit (SDK) are for writing smart contracts by creating and assembling **subcontracts**. Think of subcontracts as lego blocks you can snap together into a more complex whole. Currently, the SDK is focused on Stellar/Soroban smart contracts, but the subcontract pattern can be made to work with other blockchains as well.
2. Loam Frontend - Modern frontend tooling paired with declarative environment configurations to help with local, test, and live blockchain networks. The source code for Loam Frontend lives at [loambuild/frontend](https://github.com/loambuild/frontend?tab=readme-ov-file#loam-dev).
3. Loam CLI - Command Line Interface (CLI) for building smart contracts authored with Loam SDK. It's also for easily working with smart contract dependencies in a Loam frontend.

This project is a monorepo containing code primarily for Loam SDK but also for Loam CLI, both of which share build code.

# What's in this monorepo?

This repository contains two main folders, [./crates](./crates) and [./examples](./examples).

## What's in [./crates](./crates)?

Organized hierarchically:

- [Loam SDK](crates/loam-sdk) - Tool for creating subcontracts.
  - [loam-subcontract-core](./crates/loam-subcontract-core) - The most basic and essential subcontract, which manages admin/ownership and redeployability.
  - [loam-sdk-macro](crates/loam-sdk-macro) - Code for the `#[subcontract]` macro to create your own brand new subcontract, if existing subcontracts do not suffice.
- [Loam CLI](crates/loam-cli) - Build smart contracts authored with Loam SDK, manage smart contract dependencies from a frontend, initialize new loam projects
- [loam-build](crates/loam-build) - Used by CLI and SDK to look at dependencies and build contracts in the correct order.
- [loam-soroban-sdk](./crates/loam-soroban-sdk) – This is a wrapper around [soroban-sdk]() that extends it with features needed by Loam SDK.
- [loam-subcontract-ft](./crates/loam-subcontract-ft): like `loam-subcontract-core` above, this contains the source code for a subcontract. This subcontract is mostly in this repository as an example; see below.

## What's in [./examples](./examples)?

A non-exhaustive list; feel free to explore the other examples on your own! (And add to this documentation 😉)

- [Core Subcontract](examples/soroban/core) - Start here; this is an example of the most basic smart contract you can create with Loam SDK. It only implements `loam-subcontract-core`. All the other examples implement multiple subcontracts, but must at least include `loam-subcontract-core`.
- [Fungible Token Subcontract](examples/soroban/ft) - This contains the implementation of a Fungible Token subcontract. Unlike `loam-subcontract-core` which contains its own implementation, [`loam-subcontract-ft`](crates/loam-subcontract-ft) is an _interface-only_ subcontract; a smart contract that uses it must provide its own implementation. This sort of subcontract is still useful to help you ensure that you implemented the interface correctly.
