#![allow(unused_parens)]
#![allow(dead_code)]

use core::convert::TryInto;

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self},
        storage::{self},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::FromBytes, CLTyped, ContractHash, ContractPackageHash, Key, URef};

use crate::{
    constants::{CEP78_PREFIX, COLLECTION_NAME},
    error::NFTCoreError,
    modalities::TokenIdentifier,
    utils,
};

pub enum CEP78Event {
    Init {
        collection_name: String,
        collection_symbol: String,
        total_token_supply: u64,
        allow_minting: bool,
        minting_mode: u8,
        ownership_mode: u8,
        nft_kind: u8,
        holder_mode: u8,
        whitelist_mode: u8,
        contract_whitelist: Vec<ContractHash>,
        json_schema: String,
        nft_metadata_kind: u8,
        identifier_mode: u8,
        metadata_mutability: u8,
        burn_mode: u8,
        reporting_mode: u8,
    },
    SetVariables {
        allow_minting: bool,
        contract_whitelist: Vec<ContractHash>,
    },
    Approve {
        owner: Key,
        spender: Key,
        token_id: TokenIdentifier,
    },
    Transfer {
        from: Key,
        to: Key,
        token_id: TokenIdentifier,
        from_balance: u64,
        to_balance: u64,
    },
    Mint {
        to: Key,
        token_id: TokenIdentifier,
        balance: u64,
    },
    Burn {
        from: Key,
        token_id: TokenIdentifier,
        balance: u64,
    },
    UpdateMetadata {
        token_id: TokenIdentifier,
        updated_token_metadata: String,
    },
}

pub fn get_key<T: FromBytes + CLTyped>(name: &str) -> Option<T> {
    match runtime::get_key(name) {
        None => None,
        Some(value) => {
            let key = value.try_into().unwrap_or_revert();
            let value = storage::read(key).unwrap_or_revert().unwrap_or_revert();
            Some(value)
        }
    }
}

pub fn contract_package_hash() -> ContractPackageHash {
    let collection_name = utils::get_stored_value_with_user_errors::<String>(
        COLLECTION_NAME,
        NFTCoreError::MissingCollectionName,
        NFTCoreError::InvalidCollectionName,
    );

    ContractPackageHash::from_formatted_str(
        &storage::read::<String>(
            runtime::get_key(&format!("{}{}", CEP78_PREFIX, collection_name))
                .unwrap()
                .into_uref()
                .unwrap(),
        )
        .unwrap()
        .unwrap(),
    )
    .unwrap()
}

pub fn emit_cep78(cep78_event: &CEP78Event) {
    let mut events = Vec::new();
    let package = contract_package_hash();
    match cep78_event {
        CEP78Event::Init {
            collection_name,
            collection_symbol,
            total_token_supply,
            allow_minting,
            minting_mode,
            ownership_mode,
            nft_kind,
            holder_mode,
            whitelist_mode,
            contract_whitelist,
            json_schema,
            nft_metadata_kind,
            identifier_mode,
            metadata_mutability,
            burn_mode,
            reporting_mode,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_init".to_string());
            event.insert("collection_name", collection_name.to_string());
            event.insert("collection_symbol", collection_symbol.to_string());
            event.insert("total_token_supply", total_token_supply.to_string());
            event.insert("allow_minting", allow_minting.to_string());
            event.insert("minting_mode", minting_mode.to_string());
            event.insert("ownership_mode", ownership_mode.to_string());
            event.insert("nft_kind", nft_kind.to_string());
            event.insert("holder_mode", holder_mode.to_string());
            event.insert("whitelist_mode", whitelist_mode.to_string());
            event.insert("contract_whitelist", format!("{:?}", contract_whitelist));
            event.insert("json_schema", json_schema.to_string());
            event.insert("nft_metadata_kind", nft_metadata_kind.to_string());
            event.insert("identifier_mode", identifier_mode.to_string());
            event.insert("metadata_mutability", metadata_mutability.to_string());
            event.insert("burn_mode", burn_mode.to_string());
            event.insert("reporting_mode", reporting_mode.to_string());

            events.push(event);
        }
        CEP78Event::Approve {
            owner,
            spender,
            token_id,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_approve".to_string());
            event.insert("owner", owner.to_string());
            event.insert("spender", spender.to_string());
            match token_id {
                TokenIdentifier::Index(index) => {
                    event.insert("token_id", index.to_string());
                }
                TokenIdentifier::Hash(hash) => {
                    event.insert("token_id", hash.to_string());
                }
            }
            events.push(event);
        }
        CEP78Event::Transfer {
            from,
            to,
            token_id,
            from_balance,
            to_balance,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_transfer".to_string());
            event.insert("from", from.to_string());
            event.insert("to", to.to_string());
            event.insert("from_balance", from_balance.to_string());
            event.insert("to_balance", to_balance.to_string());
            match token_id {
                TokenIdentifier::Index(index) => {
                    event.insert("token_id", index.to_string());
                }
                TokenIdentifier::Hash(hash) => {
                    event.insert("token_id", hash.to_string());
                }
            }
            events.push(event);
        }
        CEP78Event::Mint {
            to,
            token_id,
            balance,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_mint".to_string());
            event.insert("to", to.to_string());
            event.insert("balance", balance.to_string());
            match token_id {
                TokenIdentifier::Index(index) => {
                    event.insert("token_id", index.to_string());
                }
                TokenIdentifier::Hash(hash) => {
                    event.insert("token_id", hash.to_string());
                }
            }
            events.push(event);
        }
        CEP78Event::Burn {
            from,
            token_id,
            balance,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_burn".to_string());
            event.insert("from", from.to_string());
            event.insert("balance", balance.to_string());
            match token_id {
                TokenIdentifier::Index(index) => {
                    event.insert("token_id", index.to_string());
                }
                TokenIdentifier::Hash(hash) => {
                    event.insert("token_id", hash.to_string());
                }
            }
            events.push(event);
        }

        CEP78Event::SetVariables {
            allow_minting,
            contract_whitelist,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_set_variables".to_string());
            event.insert("allow_minting", allow_minting.to_string());
            event.insert("contract_whitelist", format!("{:?}", contract_whitelist));

            events.push(event);
        }
        CEP78Event::UpdateMetadata {
            token_id,
            updated_token_metadata,
        } => {
            let mut event = BTreeMap::new();
            event.insert("contract_package_hash", package.to_string());
            event.insert("event_type", "cep78_update_metadata".to_string());
            event.insert("updated_token_metadata", updated_token_metadata.to_string());
            match token_id {
                TokenIdentifier::Index(index) => {
                    event.insert("token_id", index.to_string());
                }
                TokenIdentifier::Hash(hash) => {
                    event.insert("token_id", hash.to_string());
                }
            }
            events.push(event);
        }
    };
    for event in events {
        let _: URef = casper_contract::contract_api::storage::new_uref(event);
    }
}
