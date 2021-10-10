use casper_erc20::Address;
use casper_types::{U256, URef, contracts::NamedKeys, Key, HashAddr, ContractHash};
use alloc::string::{String, ToString};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert
};

use crate::constants::{
    STAKE_TOKEN_HASH_KEY_NAME, REWARD_TOKEN_HASH_KEY_NAME, REWARD_RATE_KEY_NAME,
    LAST_UPDATE_KEY_NAME, REWARD_PER_TOKEN_STORED_KEY_NAME, TOTAL_SUPPLY_KEY_NAME,
    BALANCES_KEY_NAME, REWARDS_KEY_NAME, USER_REWARD_PER_TOKEN_PAID_KEY_NAME,
    STAKING_CONTRACT_KEY_NAME
};
use crate::helpers::{ dictionary_write, get_immediate_caller_address };

pub fn default(
    staking_contract_name: String,
    stake_token_key: Key,
    reward_token_key: Key,
    reward_rate: U256
            ) -> NamedKeys {
    
    // We need to create next named keys:
    let mut named_keys = NamedKeys::new();

    // 0. Name of the Stake contract

    let staking_contract_name_key = {
        let staking_contract_name_uref = storage::new_uref(staking_contract_name).into_read();
        Key::from(staking_contract_name_uref)
    };

    // 1. We need to pass ERC20 contracts of PICAS and WCSPR tokens

    // We request to pass hash of a erc20 token's contracts
    // as a Key value
    // example: -a "wcspr_contract_hash:key='hash-ca52b10cd3170652275d2ba16bbd6e3a48ac7098a3835feb314da1d0a372bf02'"

    let stake_token_hash_key = {
        let stake_token_uref = storage::new_uref(stake_token_key).into_read();
        Key::from(stake_token_uref)
    };

    let reward_token_hash_key = {
        let reward_token_uref: URef = storage::new_uref(reward_token_key).into_read();
        Key::from(reward_token_uref)
    };

    // 2. "reward_rate"
    // read only
    // [R] in equasion
    let reward_rate_key = {
        let reward_rate_uref = storage::new_uref(reward_rate).into_read();
        Key::from(reward_rate_uref)
    };

    // TODO figure out how to interact with time
    // 3. "last_update_time" read and write
    // Probably we will be using Block time
    // We set value on initial deploy call from current Block
    //let last_update_time: BlockTime = runtime::get_blocktime().into_bytes();
    let last_update_time: U256 = U256::from(u64::from(runtime::get_blocktime()));

    let last_update_time_key = {
        let last_update_time_uref = storage::new_uref(last_update_time).into_read_write();
        Key::from(last_update_time_uref)
    };

    // 4. "reward_per_token_stored"
    // left side of equasion s = sum(t,0,b){ R / L(t) }
    // summation of reward rate devided by tottal supply of staked tokens at each given time
    let reward_per_token_stored: U256 = U256::from(0);
    let reward_per_token_stored_key = {
        let reward_per_token_stored_uref = storage::new_uref(reward_per_token_stored).into_read_write();
        Key::from(reward_per_token_stored_uref)
    };

    // 5. Total supply
    // Total value of Staked WCSPR tokens in the contract
    let total_supply: U256 = U256::from(0);
    let total_supply_key = {
        let total_supply_uref = storage::new_uref(total_supply).into_read_write();
        Key::from(total_supply_uref)
    };

    // We need to create dictionaries:

    // 1. "user_reward_per_token_paid"
    // Right side of equasion p = sum(t,0,a-1){ R / L(t)}
    let user_reward_per_token_paid_uref = storage::new_dictionary(USER_REWARD_PER_TOKEN_PAID_KEY_NAME).unwrap_or_revert();
    let user_reward_per_token_paid_key = {
        runtime::remove_key(USER_REWARD_PER_TOKEN_PAID_KEY_NAME);
        Key::from(user_reward_per_token_paid_uref)
    };

    // 2. "rewards"
    // Stores reward value of users
    // Updated on stake, withdraw or get reward operations
    let rewards_dictionary_uref: URef = storage::new_dictionary(REWARDS_KEY_NAME).unwrap_or_revert();
    let rewards_dictionary_key = {
        
        // Sets up initial balance for the caller - either an account, or a contract.
        //dictionary_write(rewards_dictionary_uref, caller, total_supply);

        runtime::remove_key(REWARDS_KEY_NAME);

        Key::from(rewards_dictionary_uref)
    };

    // 3. "balances"
    // Value of the WCSPR tokens staked per user 
    let balances_dictionary_uref: URef = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
    let balances_dictionary_key = {
        
        // Sets up initial balance for the caller - either an account, or a contract.
        //dictionary_write(balances_dictionary_uref, caller, total_supply);

        runtime::remove_key(BALANCES_KEY_NAME);

        Key::from(balances_dictionary_uref)
    };

    // We need to put the contract on-chain and describe entry_points

    named_keys.insert(STAKING_CONTRACT_KEY_NAME.to_string(), staking_contract_name_key);
    named_keys.insert(STAKE_TOKEN_HASH_KEY_NAME.to_string(), stake_token_hash_key);
    named_keys.insert(REWARD_TOKEN_HASH_KEY_NAME.to_string(), reward_token_hash_key);
    named_keys.insert(REWARD_RATE_KEY_NAME.to_string(), reward_rate_key);
    named_keys.insert(LAST_UPDATE_KEY_NAME.to_string(), last_update_time_key);
    named_keys.insert(REWARD_PER_TOKEN_STORED_KEY_NAME.to_string(), reward_per_token_stored_key);
    named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);

    //Dictionaries

    named_keys.insert(USER_REWARD_PER_TOKEN_PAID_KEY_NAME.to_string(), user_reward_per_token_paid_key);
    named_keys.insert(REWARDS_KEY_NAME.to_string(), rewards_dictionary_key);
    named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
    named_keys
}