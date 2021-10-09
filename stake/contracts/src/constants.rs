//! Constants used by the Stake contract.

pub const STAKING_CONTRACT_KEY_NAME: &str = "staking_contract";

// Named keys

pub const STAKE_TOKEN_HASH_KEY_NAME: &str = "stake_token_hash";
    
pub const REWARD_TOKEN_HASH_KEY_NAME: &str = "reward_token_hash";
    
pub const REWARD_RATE_KEY_NAME: &str = "reward_rate";

pub const LAST_UPDATE_KEY_NAME: &str = "last_update_time";

pub const REWARD_PER_TOKEN_STORED_KEY_NAME: &str = "reward_per_token_stored";

pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";

// Dictionaries

pub const BALANCES_KEY_NAME: &str = "balances";

pub const REWARDS_KEY_NAME: &str = "rewards";

pub const USER_REWARD_PER_TOKEN_PAID_KEY_NAME: &str = "user_reward_per_token_paid";

// Entry points

pub const STAKE_ENTRY_POINT_NAME: &str = "stake";

pub const WITHDRAW_ENTRY_POINT_NAME: &str = "withdraw";

pub const GET_REWARD_ENTRY_POINT_NAME: &str = "get_reward";

// Variable

pub const AMOUNT_KEY_NAME:  &str = "amount";