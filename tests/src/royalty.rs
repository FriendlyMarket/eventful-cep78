use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_KEY,
    DEFAULT_PROPOSER_ADDR, DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::{CONTRACT_NAME, NFT_CONTRACT_WASM},
    installer_request_builder::{InstallerRequestBuilder, OwnerReverseLookupMode, OwnershipMode},
    support::{self, call_entry_point_with_ret, get_nft_contract_hash},
};

type Royalty = (u64, Key);

#[test]
fn should_set_token_royalty() {
    let token_id = 0u64;
    let royalty_fee = 500u64; // 5%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        "set_token_royalty",
        runtime_args! {"token_id" => token_id,"royalty" => royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();

    let royalty = support::get_dictionary_value_from_key::<Royalty>(
        &builder,
        nft_contract_key,
        "royalties",
        &token_id.to_string(),
    );

    println!("royalty: {:?}", royalty);
    assert_eq!(royalty, (royalty_fee, Key::from(*DEFAULT_ACCOUNT_KEY)));
}

#[test]
fn should_set_collection_royalty() {
    let collection_royalty_fee = 700u64; // 7%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let installing_account = builder.get_expected_account(*DEFAULT_ACCOUNT_ADDR);
    let nft_contract_key = installing_account
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have key in named keys");

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        "set_collection_royalty",
        runtime_args! {"royalty" => collection_royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();

    let collection_royalty = support::get_stored_value_from_global_state::<Royalty>(
        &builder,
        *nft_contract_key,
        vec!["collection_royalty".to_string()],
    )
    .unwrap();

    println!("collection_royalty: {:?}", collection_royalty);
    assert_eq!(
        collection_royalty,
        (collection_royalty_fee, Key::from(*DEFAULT_ACCOUNT_KEY))
    );
}

#[test]
fn should_get_royalty_info_when_token_royalty_exists() {
    let token_id = 0u64;
    let royalty_fee = 500u64; // 5%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let nft_contract_key: Key = get_nft_contract_hash(&builder).into();

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        "set_token_royalty",
        runtime_args! {"token_id" => token_id,"royalty" => royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();

    let sale_price = 500_000_000_000u64;

    let expected_royalty = (
        U256::from(25_000_000_000u64),
        Key::from(*DEFAULT_ACCOUNT_KEY),
    );

    let actual_royalty: (U256, Key) = call_entry_point_with_ret(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_key,
        runtime_args! {
            "sale_price" => U256::from(sale_price),
            "token_id" => token_id,
        },
        "royalty_info_call.wasm",
        "royalty_info",
    );
    assert_eq!(actual_royalty, expected_royalty);
}

#[test]
fn should_get_collection_royalty_info_when_token_royalty_does_not_exist() {
    let token_id = 0u64;
    let sale_price = 500_000_000_000u64;
    let collection_royalty_fee = 700u64; // 7%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let nft_contract_key: Key = get_nft_contract_hash(&builder).into();

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_hash,
        "set_collection_royalty",
        runtime_args! {"royalty" => collection_royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();

    let expected_royalty = (
        U256::from(35_000_000_000u64),
        Key::from(*DEFAULT_ACCOUNT_KEY),
    );

    let actual_royalty: (U256, Key) = call_entry_point_with_ret(
        &mut builder,
        *DEFAULT_ACCOUNT_ADDR,
        nft_contract_key,
        runtime_args! {
            "sale_price" => U256::from(sale_price),
            "token_id" => token_id,
        },
        "royalty_info_call.wasm",
        "royalty_info",
    );
    assert_eq!(actual_royalty, expected_royalty);
}

#[test]
#[should_panic = "ApiError::User(1)"]
fn should_not_allow_not_installer_to_set_collection_royalty() {
    let collection_royalty_fee = 700u64; // 7%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_PROPOSER_ADDR,
        nft_contract_hash,
        "set_collection_royalty",
        runtime_args! {"royalty" => collection_royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();
}

#[test]
#[should_panic = "ApiError::User(1)"]
fn should_not_allow_not_installer_to_set_token_royalty() {
    let token_id = 1u64; // 7%
    let royalty_fee = 700u64; // 7%
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_builder =
        InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
            .with_total_token_supply(100u64)
            .with_ownership_mode(OwnershipMode::Transferable)
            .with_reporting_mode(OwnerReverseLookupMode::Complete)
            .build();

    builder
        .exec(install_request_builder)
        .expect_success()
        .commit();

    let nft_contract_hash = get_nft_contract_hash(&builder);

    let req = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_PROPOSER_ADDR,
        nft_contract_hash,
        "set_token_royalty",
        runtime_args! {"token_id" => token_id,"royalty" => royalty_fee,"recipient" => Key::from(*DEFAULT_ACCOUNT_KEY)},
    )
    .build();

    builder.exec(req).expect_success().commit();
}
