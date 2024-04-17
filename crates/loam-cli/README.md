# loam-cli

Loam CLI helps a user create, develop, and deplopy a loam project with a frontend. [A template repository example using Astro can be seen here](https://github.com/loambuild/template?tab=readme-ov-file#loam-dev).

As seen in that template, you fill out an `environment.toml` file to describe the network settings, accounts, and contracts for each environment your team builds against. This will be used to get environments to a predictable starting state.

Loam CLI comes with three main commands:

* `loam init` - Generates the loam frontend template
* `loam dev` - Turns the contracts you depend on (contract dependencies) into frontend packages (NPM dependencies), getting your app to the point where it is ready to build or run with its own dev server. Also monitors `contracts/*` for changes to the environments.
* `loam build` - Essentially the same as `loam dev` but runs for the production environment identified in `environment.toml` and does not continuously monitor `contracts/*`.

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