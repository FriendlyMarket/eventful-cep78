#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use alloc::string::String;

use casper_contract::contract_api::{runtime, storage};
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

const ENTRY_POINT_ROYALTY_INFO: &str = "royalty_info";
const ARG_NFT_CONTRACT_HASH: &str = "nft_contract_hash";
const ARG_TOKEN_ID: &str = "token_id";
const ARG_SALE_PRICE: &str = "sale_price";
const ARG_KEY_NAME: &str = "key_name";

#[no_mangle]
pub extern "C" fn call() {
    let nft_contract_hash: ContractHash = runtime::get_named_arg::<Key>(ARG_NFT_CONTRACT_HASH)
        .into_hash()
        .map(|hash| ContractHash::new(hash))
        .unwrap();
    let sale_price: U256 = runtime::get_named_arg(ARG_SALE_PRICE);
    let token_id: u64 = runtime::get_named_arg(ARG_TOKEN_ID);
    let key_name: String = runtime::get_named_arg(ARG_KEY_NAME);

    let royalty_info = runtime::call_contract::<(U256, Key)>(
        nft_contract_hash,
        ENTRY_POINT_ROYALTY_INFO,
        runtime_args! {
            ARG_SALE_PRICE => sale_price,
            ARG_TOKEN_ID => token_id,
        },
    );
    runtime::put_key(&key_name, storage::new_uref(royalty_info).into());
}
