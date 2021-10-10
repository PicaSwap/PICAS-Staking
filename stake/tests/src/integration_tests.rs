#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};

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
        let approve_amount = U256::from(1000000);
        let stake_amount = U256::from(500000);
        assert!(approve_amount > stake_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.bob;
        let spender = fixture.staking_contract_hash;
        let recipient = fixture.staking_contract_hash;
        
        let owner_balance_before = fixture
            .stake_balance_of(Key::from(owner))
            .expect("owner should have balance");
        
        fixture.approve_stake_token(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance_stake_token(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        // We stake tokens
        fixture.stake(stake_amount, Sender(owner));

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
