use loam_sdk::{
    soroban_sdk::{self, contracttype, env, Address, Bytes, Lazy, Map, Vec},
    IntoKey,
};
use loam_subcontract_core::Core;

use crate::{
    subcontract::{IsInitable, IsNonFungible},
    Contract,
};

#[contracttype]
#[derive(IntoKey)]
pub struct MyNonFungibleToken {
    admin: Address,
    name: Bytes,
    total_count: u32,
    owners_to_nft_ids: Map<Address, Vec<u32>>,
    nft_ids_to_owners: Map<u32, Address>,
    nft_ids_to_metadata: Map<u32, Bytes>,
}

impl MyNonFungibleToken {
    #[must_use]
    pub fn new(admin: Address, name: Bytes) -> Self {
        MyNonFungibleToken {
            admin,
            name,
            total_count: 0,
            owners_to_nft_ids: Map::new(env()),
            nft_ids_to_owners: Map::new(env()),
            nft_ids_to_metadata: Map::new(env()),
        }
    }
}

impl Default for MyNonFungibleToken {
    fn default() -> Self {
        MyNonFungibleToken::new(env().current_contract_address(), Bytes::new(env()))
    }
}

impl IsInitable for MyNonFungibleToken {
    fn nft_init(&self, admin: Address, name: Bytes) {
        Contract::admin_get().unwrap().require_auth();
        MyNonFungibleToken::set_lazy(MyNonFungibleToken::new(admin, name));
    }
}

impl IsNonFungible for MyNonFungibleToken {
    // Mint a new NFT with the given owner address and metadata, returning the id
    fn mint(&mut self, owner: Address, metadata: Bytes) -> u32 {
        owner.require_auth();

        let current_count = self.total_count;
        let new_id = current_count + 1;

        //todo: check that the metadata is unique
        self.nft_ids_to_metadata.set(new_id, metadata);
        self.nft_ids_to_owners.set(new_id, owner.clone());

        let mut owner_collection = self
            .owners_to_nft_ids
            .get(owner.clone())
            .unwrap_or_else(|| Vec::new(env()));
        owner_collection.push_back(new_id);

        self.owners_to_nft_ids.set(owner, owner_collection);
        self.total_count = new_id;

        new_id
    }

    // Transfer the NFT from the current owner to the new owner
    fn transfer(&mut self, id: u32, current_owner: Address, new_owner: Address) {
        current_owner.require_auth();
        // ensures that this Address has authorized invocation of the current contract
        // during the on-chain execution the soroban host will perform the needed auth (verify the signatures) and ensure the replay prevention

        // what if current_owner is not the source account?

        let owner_id = self.nft_ids_to_owners.get(id).expect("NFT does not exist");
        assert!(
            owner_id == current_owner,
            "You are not the owner of this NFT"
        );

        // update the nft_ids_to_owners map with the new owner
        self.nft_ids_to_owners.remove(id);
        self.nft_ids_to_owners.set(id, new_owner.clone());

        // remove the NFT id from the current owner's collection
        let mut current_owner_collection = self
            .owners_to_nft_ids
            .get(current_owner.clone())
            .expect("Owner does not have a collection of NFTs");
        current_owner_collection.remove(id);

        if let Some(index) = current_owner_collection
            .iter()
            .position(|nft_id| nft_id == id)
        {
            current_owner_collection.remove(index.try_into().unwrap());
        } else {
            panic!("NFT ID not found in owner's collection");
        }

        self.owners_to_nft_ids
            .set(current_owner, current_owner_collection);

        // Add the NFT id to the new owner's collection
        let mut new_owner_collection = self
            .owners_to_nft_ids
            .get(new_owner.clone())
            .unwrap_or_else(|| Vec::new(env()));
        new_owner_collection.push_back(id);
        self.owners_to_nft_ids.set(new_owner, new_owner_collection);
    }

    // Get the NFT from the contract's storage by id
    fn get_nft(&self, id: u32) -> Option<Bytes> {
        self.nft_ids_to_metadata.get(id)
    }

    // Get the NFT from the contract's storage by owner id
    fn get_owner(&self, id: u32) -> Option<Address> {
        self.nft_ids_to_owners.get(id)
    }

    fn get_total_count(&self) -> u32 {
        self.total_count
    }

    fn get_collection_by_owner(&self, owner: Address) -> Option<Vec<u32>> {
        self.owners_to_nft_ids.get(owner)
    }
}
