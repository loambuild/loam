use loam_sdk::{soroban_sdk::Lazy, subcontract};

#[subcontract]
pub trait IsNonFungible {
    /// Mint a new NFT with the given owner and metadata
    fn mint(
        &mut self,
        owner: loam_sdk::soroban_sdk::Address,
        metadata: loam_sdk::soroban_sdk::Bytes,
    ) -> u128;

    /// Transfer the NFT with the given ID from current_owner to new_owner
    fn transfer(
        &mut self,
        id: u128,
        current_owner: loam_sdk::soroban_sdk::Address,
        new_owner: loam_sdk::soroban_sdk::Address,
    );

    /// Get the NFT with the given ID
    fn get_nft(&self, id: u128) -> Option<loam_sdk::soroban_sdk::Bytes>;

    /// Find the owner of the NFT with the given ID
    fn get_owner(&self, id: u128) -> Option<loam_sdk::soroban_sdk::Address>;

    /// Get the total count of NFTs
    fn get_total_count(&self) -> u128;

    /// Get all of the NFTs owned by the given address
    fn get_collection_by_owner(
        &self,
        owner: loam_sdk::soroban_sdk::Address,
    ) -> loam_sdk::soroban_sdk::Vec<u128>;
}

#[subcontract]
pub trait IsInitable {
    /// Initialize the NFT contract with the given admin and name
    fn nft_init(&self, name: loam_sdk::soroban_sdk::Bytes);
}
