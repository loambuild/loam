# loam-subcontract-core

Contains the trait and implementation of the Core Subcontract, which contains core functionality needed by every Loam smart contract:

 - `redeploy`: Loam's subcontract pattern is built with upgradeability in mind. Every Loam smart contract gets a `redeploy` method, thanks to `loam-subcontract-core`, that allows it to be updated with new logic.
 - `admin_get` and `admin_set`: you want to make sure only the admin (you, probably, to start out) can call `redeploy` on your contract, to avoid attackers upgrading to a contract definition that kicks you out.

For more information about how to use and author Subcontracts, see the [loam-sdk README](../loam-sdk/README.md).