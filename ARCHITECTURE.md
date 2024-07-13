Loam Ecosystem
==============

Loam aims to provide an end-to-end development experience, from writing and composing smart contracts, to publishing binaries, deploying contracts, tracking contract expiration to automatically extend lifetimes, and finally creating a frontend to match your contracts. 


`loam-sdk`
==========

This sdk wraps around `soroban-sdk`, providing a re-export `loam_sdk::soroban_sdk`. However, it makes several improvements most notable adding subcontract support. This adds some quality of life improvements such as automatic loading and saving of contract state, the edition of a `env()` function removing the need to always pass `env` around.

Subcontracts are created using macros provided by the sdk, `#[subcontract]` above a trait and `#[derive_contract(..)]` above a `Contract` struct.

It also provides a `contract_import` macro that makes importing another contract as easy as a normal rust crate, instead of needing to find the path to a wasm binary.


`loam-cli`
==========

This CLI provides the following commands:

- `build` will find the loam contracts in your rust workspace and build them in the correct order if one depends on another. Optionally it will also build the TS Bindings for the contracts needed for the frontend
- `dev` watches the changes in your code to trigger `build` 
- `init` let's you start a project given some example contracts
- `update-env` `stellar-cli` supports using a `.env` file to set environment variables such as network, account, etc. This command makes it easy to update the file

Coming soon:
- `publish` lets you publish a contract's binary with a version to a package manager contract allowing it to be deployed
- `deploy` deploys a new contract using a pubilshed binary and claims unique name to a contract registry contract. Optionally you can invoke the initization function on the contract the same way you can use `stellar contract invoke` making it very simple to initialize your contract in a single transaction
- `install` create a local contract alias from the registry


Smart Contracts
===============

- `Package Manager` - Normally Wasm binaries uploaded to the network are referenced with hashes. This contract allows for human readable names, versions, and repo information to make it easy to inspect published binaries.
- `Contract Registry` - Deploying a contract requires a Wasm hash and returns a unique Contract Id which isn't human readable. With integration with the `Package Manager`, this contract allows you to register a name for your contract, deploy it, and initialize it all in one transaction.
- `Expiration Tracker` - Contract's and Wasm binaries can both expire. This contract allows tracking and prepaying so that you don't have to worry about manually extending lifetimes.

```mermaid
graph TD
    SDK[loam-sdk]
    CLI[loam-cli]

    subgraph "loam-cli Commands"
        CLI --> C1[build]
        CLI --> C2[dev]
        CLI --> C3[init]
        CLI --> C4[update-env]
        CLI --> C5[publish]
        CLI --> C6[deploy]
        CLI --> C7[install]
    end

    subgraph SmartContracts[Smart Contracts]
        SC1[Package Manager]
        SC2[Contract Registry]
        SC3[Expiration Tracker]

        SC2 -.->|Uses for deploy/redeploy| SC1
        SC1 -.->|Provides Wasm info| SC3
        SC2 -.->|Provides Contract id info| SC3
    end

    %% SDK Interactions
    SDK -.->|Used for development| SmartContracts
    SDK -.->|Allows importing deployed contracts| C7

    %% CLI Command Interactions
    C1 -.->SDK

    C5 -.->|Interacts with| SC1
    C6 -.->|Register name and initialize| SC2
    C7 -.->|Retrieves contract id| SC2
    ```