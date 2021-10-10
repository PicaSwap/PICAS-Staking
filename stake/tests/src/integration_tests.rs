#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};

    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();

        assert_eq!(fixture.stake_token_hash(), Key::from(fixture.stake_contract_hash));
        assert_eq!(fixture.reward_token_hash(), Key::from(fixture.reward_contract_hash));
        //assert_eq!(fixture.reward_rate(), TestFixture::REWARD_RATE);

        /*
        assert_eq!(fixture.staking_contract_name(), TestFixture::STAKING_CONTRACT_KEY_NAME);
        assert_eq!(fixture.reward_per_token_stored(), U256::from(0));
        assert_eq!(fixture.total_supply(), U256::from(0));
        // Cant' use runtime:get_blocktime()
        //assert_eq!(fixture.last_update_time(), U256::from(u64::from(runtime::get_blocktime())));
        //assert_eq!(fixture.balance(Key::from(fixture.ali)),Some(TestFixture::token_total_supply()));
        */
    }
    
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
