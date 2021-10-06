#![no_main]
#![no_std]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, DECIMALS_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME,
    },
    Address, ERC20, entry_points
};
use casper_types::{CLValue, U256, u8};

const REWARD_RATE_KEY_NAME: &str = "reward_rate";
const LAST_UPDATED_KEY_NAME: &str = "last_updated";

#[no_mangle]
fn call() {
    
    // We need to pass ERC20 contracts of PICAS and WCSPR tokens
    
    // We need to create next named keys:
    
    // "reward_rate" read only
    let rate: u8 = runtime::get_named_arg(REWARD_RATE);

    // "last_update_time" read and write
    // Probably we will be using Block time
    // We set value on initial deploy call from current Block
    let last_update_time: BlockTime;

    // Total supply
    let total_supply: U256;

    // left side of equasion
    // summation of reward rate devided by tottal supply of staked tokens at each given time
    let reward_per_token_stored: U256;

    // We need to create dictionaries:

    // "user_reward_per_token_paid"

    // "rewards"

    // "balances"

}

#[no_mangle]
pub extern "C" fn stake() {
    
    update_reward();
    
    // update total_supply

    // update balance of caller

    // Call WCSPR.transfer()
    // Transfer WCSPR from caller to the stake contract

}

#[no_mangle]
pub extern "C" fn withdraw() {
    
    update_reward();

    // update total_supply

    // update balance of caller

    // Call WCSPR.transfer()
    // Transfer WCSPR to caller from the stake contract

}

#[no_mangle]
pub extern "C" fn get_reward() {
    
    update_reward();

    // get reward_value of the caller stored in "rewards" dictionary

    // set reward_value of the caller in the dictionary to 0

    // Call PICAS.transfer()
    // Transfer PICAS to caller
}

#[no_mangle]
 fn update_reward() {
    
    // update reward_per_token_stored

    // update last_update_time
}

#[no_mangle]
 fn reward_per_token() {
    
    //
}