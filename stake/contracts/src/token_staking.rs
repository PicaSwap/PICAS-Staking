#![no_main]
#![no_std]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

mod helpers;
mod entry_points;
mod named_keys;
pub mod constants;

use crate::helpers::{ set_key, get_key, get_immediate_caller_address, dictionary_read, dictionary_write };

use crate::constants::{
    STAKING_CONTRACT_KEY_NAME, REWARD_TOKEN_HASH_KEY_NAME, STAKE_TOKEN_HASH_KEY_NAME,
    REWARD_RATE_KEY_NAME, TOTAL_SUPPLY_KEY_NAME, AMOUNT_KEY_NAME, BALANCES_KEY_NAME,
    REWARDS_KEY_NAME, USER_REWARD_PER_TOKEN_PAID_KEY_NAME, LAST_UPDATE_KEY_NAME,
    REWARD_PER_TOKEN_STORED_KEY_NAME
};

use alloc::string::String;

use casper_erc20::{ Error, Address::Account, Address,
    constants::{
        TRANSFER_ENTRY_POINT_NAME, TRANSFER_FROM_ENTRY_POINT_NAME, OWNER_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME}
    };

use casper_contract::{contract_api::{runtime, storage}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    contracts::{NamedKeys}, U256, ContractHash, Key,
    URef, RuntimeArgs, runtime_args, account::AccountHash, HashAddr};

#[no_mangle]
fn call() {
    
    let staking_contract_name: String = runtime::get_named_arg(STAKING_CONTRACT_KEY_NAME);
    
    let stake_token_key: Key = runtime::get_named_arg(STAKE_TOKEN_HASH_KEY_NAME);
    let reward_token_key: Key = runtime::get_named_arg(REWARD_TOKEN_HASH_KEY_NAME);

    let reward_rate: U256 = runtime::get_named_arg(REWARD_RATE_KEY_NAME);

    // TODO Check that Reward Token and Stake Token are existing ERC20 contracts

    let named_keys: NamedKeys = named_keys::default(
        staking_contract_name.clone(),
        stake_token_key,
        reward_token_key,
        reward_rate
    );
    
    // We store contract on-chain
    let (contract_hash, _version) =
            storage::new_locked_contract(entry_points::default(), Some(named_keys), None, None);
    
    // ContractHash is saved into 'named keys' of 'Deployer account'
    // Need to be &str
    runtime::put_key(staking_contract_name.as_str(), Key::from(contract_hash));
}

#[no_mangle]
pub extern "C" fn stake() {
    
    // TODO Frontend should call 'allowance' entry point of Stake token ERC20 contract
    // Returns the amount of `owner`'s tokens allowed to be spent by `spender`.
    // Let user to call 'approve' first, before staking

    let amount: U256 = runtime::get_named_arg(AMOUNT_KEY_NAME);

    let staker: Address = get_immediate_caller_address().unwrap_or_revert();
    let balances_key: Key = runtime::get_key(BALANCES_KEY_NAME).unwrap_or_revert();
    let rewards_key: Key = runtime::get_key(REWARDS_KEY_NAME).unwrap_or_revert();
    let balances_uref: URef = balances_key.into_uref().unwrap_or_revert();
    let rewards_uref: URef = rewards_key.into_uref().unwrap_or_revert();
    let stake_contract: AccountHash = runtime::get_caller();
    
    update_reward(staker, balances_uref, rewards_uref);
    
    // update total_supply
    named_key_add(amount, TOTAL_SUPPLY_KEY_NAME);

    // update balance of caller
    dictionary_add(balances_uref, staker, amount);

    // Transfer `amount` of Stake Token from caller to the stake contract
    erc20_transfer_from(
        STAKE_TOKEN_HASH_KEY_NAME,
        Key::from(staker),
        Key::from(stake_contract),
        amount
    );

}

#[no_mangle]
pub extern "C" fn withdraw() {
    
    let amount: U256 = runtime::get_named_arg(AMOUNT_KEY_NAME);

    let staker = get_immediate_caller_address().unwrap_or_revert();
    let balances_uref = get_key(BALANCES_KEY_NAME).unwrap_or_revert();
    let rewards_uref = get_key(REWARDS_KEY_NAME).unwrap_or_revert();

    update_reward(staker, balances_uref, rewards_uref);

    // update total_supply
    named_key_sub(amount, TOTAL_SUPPLY_KEY_NAME);

    // update balance of caller
    dictionary_sub(balances_uref, staker, amount);

    // Transfer `amount` of Stake Token from the stake contract to caller
    erc20_transfer(
        STAKE_TOKEN_HASH_KEY_NAME,
        staker,
        amount
    );

    // TODO Brainstorm: Send Reward to caller on Withdrawal?
    //get_reward();

}

#[no_mangle]
pub extern "C" fn get_reward() {
    
    let staker = get_immediate_caller_address().unwrap_or_revert();
    let balances_uref = get_key(BALANCES_KEY_NAME).unwrap_or_revert();
    let rewards_uref = get_key(REWARDS_KEY_NAME).unwrap_or_revert();

    update_reward(staker, balances_uref, rewards_uref);

    // get reward_value of the caller stored in "rewards" dictionary
    let staker_reward: U256 = dictionary_read(rewards_uref, staker);
    
    // set reward_value of the caller in the dictionary to 0
    dictionary_write(rewards_uref, staker, U256::from(0));

    // Transfer `amount` of Reward Token to caller
    erc20_transfer(
        REWARD_TOKEN_HASH_KEY_NAME,
        staker,
        staker_reward
    );

}

#[no_mangle]
 fn update_reward(
    staker: Address,
    balances_uref: URef,
    rewards_uref: URef
 ) {
    
    let current_block_time: U256 = U256::from(u64::from(runtime::get_blocktime()));
    let user_reward_per_token_paid_key: Key = runtime::get_key(USER_REWARD_PER_TOKEN_PAID_KEY_NAME).unwrap_or_revert();
    let user_reward_per_token_paid_uref: URef = user_reward_per_token_paid_key.into_uref().unwrap_or_revert();
    let user_reward_per_token_paid: U256 = dictionary_read(user_reward_per_token_paid_uref, staker);
    
    // update reward_per_token_stored
    let reward_per_token_stored: U256 = reward_per_token(current_block_time);
    
    // update last_update_time
    set_key(LAST_UPDATE_KEY_NAME, current_block_time);
    //let last_update_time: BlockTime = runtime::get_blocktime().into_bytes();

    // update reward amount of the staker
    dictionary_add(
        rewards_uref,
        staker,
        earned(staker, balances_uref, user_reward_per_token_paid)
    );
    
    // update "user_reward_per_token_paid" dictionary
    dictionary_write(user_reward_per_token_paid_uref, staker, reward_per_token_stored);
}

#[no_mangle]
/// Computes the running sum of 'R' over 'total supply' of 'token stake'
fn reward_per_token(current_block_time: U256) -> U256 {
    
    let total_supply: U256 = get_key(TOTAL_SUPPLY_KEY_NAME).unwrap_or_revert();
    let reward_per_token_stored: U256 = get_key(REWARD_PER_TOKEN_STORED_KEY_NAME).unwrap_or_revert();

    if total_supply.is_zero() {
        // TODO implement Error for runtime::revert()
        // TODO Brainstorm: Return 0 or current reward_per_token_stored
        reward_per_token_stored
    } else {
        
        let reward_rate: U256 = get_key(REWARD_RATE_KEY_NAME).unwrap_or_revert();
        let last_update_time: U256 = get_key(LAST_UPDATE_KEY_NAME).unwrap_or_revert();
        
        let new_value: U256 = {
            reward_per_token_stored
                // TODO rework time operations
                .checked_add(reward_rate * ( current_block_time - last_update_time ) / total_supply)
                .ok_or(Error::Overflow).unwrap_or_revert()
        };
    
        set_key(REWARD_PER_TOKEN_STORED_KEY_NAME, new_value);

        new_value

    }
}

#[no_mangle]
/// Amount of Rewads tokens user can claim so far
fn earned(
    staker: Address,
    balances_uref: URef,
    user_reward_per_token_paid: U256
) -> U256 {
    
    let balance: U256 = dictionary_read(balances_uref, staker);
    let reward_per_token_stored: U256 = get_key(REWARD_PER_TOKEN_STORED_KEY_NAME).unwrap_or_revert();

    balance * ( reward_per_token_stored - user_reward_per_token_paid )
    
}

fn dictionary_add(
    dictionary_uref: URef,
    staker: Address,
    amount: U256,
) -> Result<(), Error> {
    if amount.is_zero() {
        return Ok(());
    }

    let new_staker_balance = {
        let staker_balance = dictionary_read(dictionary_uref, staker);
        staker_balance
            .checked_add(amount)
            .ok_or(Error::Overflow)?
    };

    dictionary_write(dictionary_uref, staker, new_staker_balance);

    Ok(())
}

fn dictionary_sub(
    dictionary_uref: URef,
    staker: Address,
    amount: U256,
) -> Result<(), Error> {
    if amount.is_zero() {
        return Ok(());
    }

    let new_staker_balance = {
        let staker_balance = dictionary_read(dictionary_uref, staker);
        staker_balance
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance)?
    };

    dictionary_write(dictionary_uref, staker, new_staker_balance);

    Ok(())
}

fn named_key_add(amount: U256, key_name: &str) {
    
    let new_value: U256 = {
        let current_value: U256 = get_key(key_name).unwrap_or_revert();
        current_value
            .checked_add(amount)
            .ok_or(Error::Overflow).unwrap_or_revert()
    };

    set_key(key_name, new_value);

}

fn named_key_sub(amount: U256, key_name: &str) {
    
    let new_value: U256 = {
        let current_value: U256 = get_key(key_name).unwrap_or_revert();
        current_value
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance).unwrap_or_revert()
    };

    set_key(key_name, new_value);

}

fn erc20_transfer_from(
    erc20_hash_key_name: &str,
    staker: Key,
    stake_contract: Key,
    amount: U256
) {
    let erc20_contract_key: Key = runtime::get_key(erc20_hash_key_name).unwrap_or_revert();
    let erc20_contract_uref: URef = erc20_contract_key.into_uref().unwrap_or_revert();
    
    let erc20_contract_hash_key: Key =  storage::read(erc20_contract_uref).unwrap_or_revert().unwrap_or_revert();

    let erc20_contract_hash_addr: HashAddr  = erc20_contract_hash_key.into_hash().unwrap_or_revert();
    let erc20_contract_hash: ContractHash = ContractHash::new(erc20_contract_hash_addr);
    set_key("debug_msg1", erc20_contract_key.to_formatted_string());
    //let tv = format!("{}", r)
    set_key("debug_msg2", erc20_contract_uref.to_formatted_string());
    //let tv = format!("{}", r)
    set_key("debug_msg3", erc20_contract_hash_key.to_formatted_string());
    //let tv = format!("{}", erc20_contract_hash_addr)
    //set_key("debug_msg4", erc20_contract_hash_addr.as_contract_package_hash().to_formatted_string());
    set_key("debug_msg5", erc20_contract_hash.to_formatted_string());

    runtime::call_contract(erc20_contract_hash, SYMBOL_RUNTIME_ARG_NAME, runtime_args!{
    })
    /*
    runtime::call_contract(erc20_contract_hash, TRANSFER_FROM_ENTRY_POINT_NAME, runtime_args!{
        OWNER_RUNTIME_ARG_NAME => staker,
        RECIPIENT_RUNTIME_ARG_NAME => stake_contract,
        AMOUNT_RUNTIME_ARG_NAME => amount
    })
    */
}

fn erc20_transfer(
    erc20_hash_key_name: &str,
    staker: Address,
    amount: U256
) {
    let erc20_contract_hash: ContractHash = get_key(erc20_hash_key_name).unwrap_or_revert();
    runtime::call_contract(erc20_contract_hash, TRANSFER_ENTRY_POINT_NAME, runtime_args!{
        RECIPIENT_RUNTIME_ARG_NAME => staker,
        AMOUNT_RUNTIME_ARG_NAME => amount
    })
}