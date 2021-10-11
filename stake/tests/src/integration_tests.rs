#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};
    use casper_erc20::{ Address };

    use crate::test_fixture::{Sender, TestFixture};

    /*
    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();

        assert_eq!(fixture.reward_rate(), U256::from(2000));
        assert_eq!(fixture.stake_token_hash(), Key::from(fixture.stake_contract_hash));
        assert_eq!(fixture.reward_token_hash(), Key::from(fixture.reward_contract_hash));
        assert_eq!(fixture.staking_contract_name(), fixture.contract_name);
        assert_eq!(fixture.reward_per_token_stored(), U256::from(0));
        assert_eq!(fixture.total_supply(), U256::from(0));

        //Need to check last_update_time
        // Cant' use runtime:get_blocktime()
        //assert_eq!(fixture.last_update_time(), U256::from(u64::from(runtime::get_blocktime())));
        
    }
    */
    
    
    #[test]
    fn should_stake() {
        
        // We approve 'Stake' tokens for 'Staker'
        let approve_amount = U256::from(10);
        let stake_amount = U256::from(5);
        assert!(approve_amount > stake_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.bob;
        let spender: Address = Address::from(fixture.staking_contract_package_hash);
        let recipient = fixture.staking_contract_package_hash;

        // check that owner has tokens
        let owner_balance_before = fixture.stake_balance_of(Key::from(owner)).unwrap();
        assert_eq!(owner_balance_before, U256::from(1000));
        
        // check spender has no tokens
        //let spender_balance_before = fixture.stake_balance_of(Key::from(spender)).unwrap();
        //assert_eq!(spender_balance_before, U256::from(1000));


        // Check spending directly
        fixture.approve_stake_token(Address::from(fixture.bob), U256::from(1), Sender(fixture.bob));
        assert_eq!(
            fixture.allowance_stake_token(Key::from(Address::from(fixture.bob)), Key::from(Address::from(fixture.bob))),
            Some(U256::from(1))
        );
        fixture.transfer_from(Key::from(fixture.bob), Key::from(fixture.ali), U256::from(5), Sender(fixture.bob));
        // left 5 after transfer
        assert_eq!(
            fixture.allowance_stake_token(Key::from(Address::from(fixture.bob)), Key::from(Address::from(fixture.bob))),
            Some(U256::from(5))
        );

        // Check spending directly via contract
        /*
        fixture.approve_stake_token(spender, U256::from(10), Sender(fixture.bob));
        assert_eq!(
            fixture.allowance_stake_token(Key::from(fixture.bob), Key::from(spender)),
            Some(U256::from(10))
        );
        fixture.transfer_from(Key::from(fixture.bob), Key::from(fixture.ali), U256::from(5), Sender(fixture.bob));

        //Spender is a ContractPackageHash
        fixture.approve_stake_token(spender, U256::from(10), Sender(fixture.bob));
        assert_eq!(
            fixture.allowance_stake_token(Key::from(fixture.bob), Key::from(spender)),
            Some(U256::from(10))
        );
        println!("Spender: {}", Key::from(spender).to_formatted_string().as_str());
        */

        // We stake tokens
        fixture.stake(stake_amount, Sender(fixture.bob));
        println!("spender in contract: {}", fixture.get_debug_msg("debug_msg1").as_str());

        /*
        println!("erc20_contract_key {}", fixture.get_debug_msg("debug_msg1").as_str());
        
        println!("erc20_contract_uref {}", fixture.get_debug_msg("debug_msg2").as_str());
        
        //hash-xxx
        println!("erc20_contract_hash_key {}", fixture.get_debug_msg("debug_msg3").as_str());
        
        //contract-xxx
        println!("erc20_contract_hash {}", fixture.get_debug_msg("debug_msg5").as_str());

        //staker
        println!("staker {}", fixture.get_debug_msg("debug_msg6").as_str());
        //stake_contract
        println!("stake_contract {}", fixture.get_debug_msg("debug_msg7").as_str());

        println!("spender {}", spender);
        println!("owner {}", owner);

        */

        // We don't need to transfer
        /*fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Sender(spender),
        );

        assert_eq!(
            fixture.balance_of(Key::from(owner)),
            Some(owner_balance_before - transfer_amount),
            "should decrease balance of the owner"
        );
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount - transfer_amount),
            "should decrease allowance of the spender"
        );
        assert_eq!(
            fixture.balance_of(Key::from(recipient)),
            Some(transfer_amount),
            "recipient should receive tokens"
        );
        */
        
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
