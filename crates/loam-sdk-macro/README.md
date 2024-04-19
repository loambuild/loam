# loam-sdk-macro

This crate only contains macros. Macros generate the code necessary for a user to implement a Subcontract. Ultimately, this means a user only writes a few lines of code to create a Core Subcontract, allowing them to instead focus on other Subcontracts they'd like to create.

These macros implement the key methods of a Core Subcontract, `subcontract`, `into_key`, `lazy`.

`#[subcontract]` is an attribute procedural macro (proc macro) that allows user to get and set an admin. This ability is what creates a Core Subcontract and thus defines the most basic definition of a Subcontract.

`lazy` deserializes keys but only as needed.

`into_key` loads and stores types.

Both `lazy` and `into_key` can be used in your subcontract implementations when appropriate.

See `lib.rs` for the implementations of subcontract, lazy, and into_key.