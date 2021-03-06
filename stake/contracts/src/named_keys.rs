use casper_types::{U256, URef, contracts::NamedKeys, Key};
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

pub fn default(
    staking_contract_name: String,
    stake_token_hash_key: Key,
    reward_token_hash_key: Key,
    reward_rate: U256
            ) -> NamedKeys {
    
    let mut named_keys = NamedKeys::new();

    // Contract 'Named keys'

    // 0. Name of the Stake contract
    let staking_contract_name_key = {
        let staking_contract_name_uref = storage::new_uref(staking_contract_name).into_read();
        Key::from(staking_contract_name_uref)
    };

    // 1. We need to pass 'Contract Hashes' of ERC20 'Stake'(WCSPR) and 'Reward'(PICAS) tokens, read only
    // We request to pass hash of a erc20 token's contracts as a Key value
    let stake_token_key = {
        let stake_token_uref = storage::new_uref(stake_token_hash_key).into_read();
        Key::from(stake_token_uref)
    };

    let reward_token_key = {
        let reward_token_uref: URef = storage::new_uref(reward_token_hash_key).into_read();
        Key::from(reward_token_uref)
    };

    // 2. "reward_rate", read only
    // [R] in equasion
    let reward_rate_key = {
        let reward_rate_uref = storage::new_uref(reward_rate).into_read();
        Key::from(reward_rate_uref)
    };

    // 3. "last_update_time", read and write
    let last_update_time: U256 = U256::from(u64::from(runtime::get_blocktime()));

    let last_update_time_key = {
        let last_update_time_uref = storage::new_uref(last_update_time).into_read_write();
        Key::from(last_update_time_uref)
    };

    // 4. "reward_per_token_stored", read and write
    // left side of equasion s = sum(t,0,b){ R / L(t) }
    // summation of reward rate devided by tottal supply of staked tokens at each given time
    let reward_per_token_stored: U256 = U256::from(0);
    let reward_per_token_stored_key = {
        let reward_per_token_stored_uref = storage::new_uref(reward_per_token_stored).into_read_write();
        Key::from(reward_per_token_stored_uref)
    };

    // 5. Total supply, read and write
    // Total value of Staked WCSPR tokens in the contract
    let total_supply: U256 = U256::from(0);
    let total_supply_key = {
        let total_supply_uref = storage::new_uref(total_supply).into_read_write();
        Key::from(total_supply_uref)
    };

    named_keys.insert(STAKING_CONTRACT_KEY_NAME.to_string(), staking_contract_name_key);
    named_keys.insert(STAKE_TOKEN_HASH_KEY_NAME.to_string(), stake_token_key);
    named_keys.insert(REWARD_TOKEN_HASH_KEY_NAME.to_string(), reward_token_key);
    named_keys.insert(REWARD_RATE_KEY_NAME.to_string(), reward_rate_key);
    named_keys.insert(LAST_UPDATE_KEY_NAME.to_string(), last_update_time_key);
    named_keys.insert(REWARD_PER_TOKEN_STORED_KEY_NAME.to_string(), reward_per_token_stored_key);
    named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);

    // Contract 'Dictionaries'

    // 1. "user_reward_per_token_paid"
    // Right side of equasion p = sum(t,0,a-1){ R / L(t)}
    let user_reward_per_token_paid_uref = storage::new_dictionary(USER_REWARD_PER_TOKEN_PAID_KEY_NAME).unwrap_or_revert();
    let user_reward_per_token_paid_key = {
        Key::from(user_reward_per_token_paid_uref)
    };

    // 2. "rewards"
    // Stores reward value of users
    // Updated on stake, withdraw or get reward operations
    let rewards_dictionary_uref: URef = storage::new_dictionary(REWARDS_KEY_NAME).unwrap_or_revert();
    let rewards_dictionary_key = {
        Key::from(rewards_dictionary_uref)
    };

    // 3. "balances"
    // Value of the WCSPR tokens staked per user 
    let balances_dictionary_uref: URef = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
    let balances_dictionary_key = {
        Key::from(balances_dictionary_uref)
    };

    named_keys.insert(USER_REWARD_PER_TOKEN_PAID_KEY_NAME.to_string(), user_reward_per_token_paid_key);
    named_keys.insert(REWARDS_KEY_NAME.to_string(), rewards_dictionary_key);
    named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
    
    named_keys
}