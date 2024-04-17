# loam-ft

Contains an example of how to create a Subcontract interface. This example is for fungible tokens.

To find an implementation of the fungible token subcontract see, `examples/soroban/ft/src/ft.rs`. 

Notice that a Core Subcontract must be implemented to use any other Subcontracts, including this fungible tokens. This Core Subcontract, as explained in `path/here`, only requires the admin method to be implemented. 