# loam-cli

Build smart contracts authored with Loam SDK, manage smart contract dependencies from a frontend, initialize new loam projects.

Loam CLI comes with three main commands:

* `loam init` - Generates a [Loam frontend](https://github.com/loambuild/template?tab=readme-ov-file) that includes an `environments.toml` file describing the network settings, accounts, and contracts for each environment your team builds against.
* `loam build` - Two build processes in one:
  * Build smart contracts. Essentially, this is a wrapper around `cargo build` that will additionally traverse dependencies and find any with a `package.metadata.loam.contract = true` setting in their Cargo.toml, and ensure that they all get built in the correct order. You can use `loam build` to build non-Loam-SDK projects and it will Just Work.
  * Build frontend clients. If the project contains an `environments.toml` file, `loam build` will match the environment specified by the `LOAM_ENV` environment variable (for `loam build`, the default is `production`) to a predictable starting state. It will turn the contracts you depend on (contract dependencies) into frontend packages (NPM dependencies), getting your frontend app to the point where it is ready to build or run with its own dev server. This is done in as low-intrusive a way as possible (for example, if contracts are already deployed, are they using the correct Wasm hash? Do they need to have their TTL extended? It will update these things, rather than re-deploy every time.)
* `loam dev` - Monitors `contracts/*` and `environments.toml` for changes and re-runs `loam build` as needed. It also defaults to `LOAM_ENV=development`, rather than `production`.

## `loam dev` and `loam build` in Depth

`loam dev` and `loam build` essentially work the same, except that one is for testing and another for production.

### `loam dev`
`loam dev` accomplishes the following when run:
* defaults to development environment
* looks at `environment.toml`, which specifies:
    * if environment uses local or live network
    * binds contracts
    * imports contracts to frontend
* watches `contracts/*` for any changes


### `loam build` Suggestions
We suggest that each frontend have separate contract dependencies, deployed on separate networks.

So, you should build one version of your frontend for mainnet and host it at the root domain, say, `example.com`. Then build a separate version for testnet and host it at a separate domain, maybe `staging.example.com`.