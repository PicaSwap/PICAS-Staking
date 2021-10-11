#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};
    use casper_erc20::{ Address };

    use crate::test_fixture::{Sender, TestFixture};

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
    
    #[test]
    fn should_stake() {
        
        // We approve 'Stake' tokens for 'Staker'
        let approve_amount = U256::from(10);
        let stake_amount = U256::from(5);
        assert!(approve_amount > stake_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.bob;
        let sender = Sender(fixture.bob);
        let spender: Address = Address::from(fixture.staking_contract_package_hash);
        let recipient: Address = Address::from(fixture.staking_contract_package_hash);

        // Send rewards tokens to Staking Contract
        fixture.transfer_reward_token(
            Key::from(recipient),
            U256::from(1000),
            Sender(fixture.ali),
        );

        let contract_balance_reward_token = fixture.reward_token_balance_of(Key::from(recipient)).unwrap();
        assert_eq!(contract_balance_reward_token, U256::from(1000));

        // check that owner has tokens
        let owner_balance_before = fixture.stake_token_balance_of(Key::from(owner)).unwrap();
        assert_eq!(owner_balance_before, U256::from(1000));
        
        //Spender is a ContractPackageHash
        fixture.approve_stake_token(spender, approve_amount, sender);
        assert_eq!(
            fixture.allowance_stake_token(Key::from(fixture.bob), Key::from(spender)),
            Some(approve_amount)
        );
        
        // We stake tokens
         fixture.stake(stake_amount, sender);
        
         // Balance of bob is 'stake_amount' less
        let owner_balance_after_stake = fixture.stake_token_balance_of(Key::from(owner)).unwrap();
        assert_eq!(owner_balance_after_stake, owner_balance_before-stake_amount);

        fixture.add_time(100);

        //fixture.withdrawal
        fixture.withdraw(stake_amount, sender);
        /*
        
        //balance of bob is back to intial amount
        let owner_balance_after_withdrawal = fixture.stake_token_balance_of(Key::from(owner)).unwrap();
        assert_eq!(owner_balance_after_withdrawal, owner_balance_before);

        //balance of bob Reward tokens is > 0
        let owner_balance_after_withdrawal = fixture.reward_token_balance_of(Key::from(owner)).unwrap();
        assert_eq!(owner_balance_after_withdrawal, owner_balance_before);
        */

    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
