#![cfg(test)]

use super::{
    SorobanContract__ as SorobanContract, SorobanContract__Client as SorobanContractClient,
};
use loam_sdk::soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    Address, Bytes, Env, IntoVal,
};

extern crate std;

mod contract {
    use loam_sdk::soroban_sdk;

    soroban_sdk::contractimport!(file = "../../../../../target/loam/example_nft.wasm");
}

fn init() -> (Env, SorobanContractClient<'static>, Address) {
    let env = Env::default();
    let contract_id = env.register_contract(None, SorobanContract);
    let client = SorobanContractClient::new(&env, &contract_id);
    (env, client, contract_id)
}

#[test]
fn test_nft() {
    let (env, client, contract_id) = &init();

    let admin = Address::generate(&env);

    // test admin_get and admin_set
    match client.admin_get() {
        Some(_) => assert!(false, "Admin already set"),
        None => assert!(true),
    }
    client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "admin_set",
                args: (&admin,).into_val(env),
                sub_invokes: &[],
            },
        }])
        .admin_set(&admin);

    match client.admin_get() {
        Some(a) => assert_eq!(a, admin),
        None => assert!(false, "No admin set"),
    }

    // test nft_init
    let name = Bytes::from_slice(env, "nftexample".as_bytes());
    client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "nft_init",
                args: (name.clone(),).into_val(env),
                sub_invokes: &[],
            },
        }])
        .nft_init(&name);

    // test initial state
    assert_eq!(client.get_total_count(), 0);
    assert!(client.get_nft(&1).is_none());
    assert!(client.get_owner(&1).is_none());
    assert_eq!(client.get_collection_by_owner(&admin).len(), 0);

    // test mint & getter fns
    let owner_1 = Address::generate(&env);
    let metadata = Bytes::from_slice(env, "metadata".as_bytes());
    let nft_id = client
        .mock_auths(&[MockAuth {
            address: &admin,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "mint",
                args: (owner_1.clone(), metadata.clone()).into_val(env),
                sub_invokes: &[],
            },
        }])
        .mint(&owner_1, &metadata);
    assert_eq!(nft_id, 1);
    assert_eq!(client.get_nft(&nft_id), Some(metadata));
    assert_eq!(client.get_owner(&nft_id), Some(owner_1.clone()));
    assert_eq!(client.get_total_count(), 1);
    assert_eq!(client.get_collection_by_owner(&owner_1).len(), 1);

    // test transfer
    let owner_2 = Address::generate(&env);
    client
        .mock_auths(&[MockAuth {
            address: &owner_1,
            invoke: &MockAuthInvoke {
                contract: &contract_id,
                fn_name: "transfer",
                args: (nft_id.clone(), owner_1.clone(), owner_2.clone()).into_val(env),
                sub_invokes: &[],
            },
        }])
        .transfer(&nft_id, &owner_1, &owner_2);
    assert_eq!(client.get_owner(&nft_id), Some(owner_2.clone()));
    assert_eq!(client.get_total_count(), 1);
    assert_eq!(client.get_collection_by_owner(&owner_1).len(), 0);
    assert_eq!(client.get_collection_by_owner(&owner_2).len(), 1);
}
