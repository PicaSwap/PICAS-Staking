use alloc::{string::String, vec};

use crate::constants::{
    GET_REWARD_ENTRY_POINT_NAME, WITHDRAW_ENTRY_POINT_NAME,
    STAKE_ENTRY_POINT_NAME, AMOUNT_KEY_NAME
    };

use casper_types::{
    U256, CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter
    };

fn stake() -> EntryPoint {
    EntryPoint::new(
        String::from(STAKE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(AMOUNT_KEY_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn withdraw() -> EntryPoint {
    EntryPoint::new(
        String::from(WITHDRAW_ENTRY_POINT_NAME),
        vec![
            Parameter::new(AMOUNT_KEY_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

fn get_reward() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_REWARD_ENTRY_POINT_NAME),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub(crate) fn default() -> EntryPoints {
    
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(stake());
    entry_points.add_entry_point(withdraw());
    entry_points.add_entry_point(get_reward());

    entry_points
    
}
