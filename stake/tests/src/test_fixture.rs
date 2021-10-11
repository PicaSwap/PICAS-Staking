use casper_erc20::Address;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash, ContractPackageHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};

// TODO Connect constants from 'Staking contact' folder
const STAKING_CONTRACT_KEY_NAME: &str = "staking_contract";
const STAKE_TOKEN_HASH_KEY_NAME: &str = "stake_token_hash";
const REWARD_TOKEN_HASH_KEY_NAME: &str = "reward_token_hash";
const REWARD_RATE_KEY_NAME: &str = "reward_rate";
const LAST_UPDATE_KEY_NAME: &str = "last_update_time";
const REWARD_PER_TOKEN_STORED_KEY_NAME: &str = "reward_per_token_stored";
const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";
const BALANCES_KEY_NAME: &str = "balances";
const REWARDS_KEY_NAME: &str = "rewards";
const USER_REWARD_PER_TOKEN_PAID_KEY_NAME: &str = "user_reward_per_token_paid";
const STAKE_ENTRY_POINT_NAME: &str = "stake";
const WITHDRAW_ENTRY_POINT_NAME: &str = "withdraw";
const GET_REWARD_ENTRY_POINT_NAME: &str = "get_reward";
const AMOUNT_KEY_NAME:  &str = "amount";

const CONTRACT_FILE: &str = "staking_contract.wasm";
const CONTRACT_NAME: &str = "stake_wcspr_reward_picas";

const STAKE_CONTRACT_FILE: &str = "wcspr.wasm";
const STAKE_CONTRACT_KEY_NAME: &str = "wcspr_token";

const REWARD_CONTRACT_FILE: &str = "picas_token.wasm";
const REWARD_CONTRACT_KEY_NAME: &str = "picas_token";


fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
    pub reward_contract_hash: ContractHash,
    pub stake_contract_hash: ContractHash,
    pub contract_name: String,
    pub staking_contract_hash: ContractHash,
    pub staking_contract_package_hash: ContractPackageHash,
    pub current_time: u64
}

impl TestFixture {

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(joe.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        // Deploy Stake token
        let session_code = Code::from(STAKE_CONTRACT_FILE);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => "Stake Token",
            consts::SYMBOL_RUNTIME_ARG_NAME => "STAKE",
            consts::DECIMALS_RUNTIME_ARG_NAME => 9 as u8,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => casper_types::U256::from(1000)
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(bob.to_account_hash())
            .with_authorization_keys(&[bob.to_account_hash()])
            .build();
        context.run(session);

        // Deploy Reward token
        let session_code = Code::from(REWARD_CONTRACT_FILE);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => "REWARD Token",
            consts::SYMBOL_RUNTIME_ARG_NAME => "REWARD",
            consts::DECIMALS_RUNTIME_ARG_NAME => 9 as u8,
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => casper_types::U256::from(1000)
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);

        // Deploy Staking Contract
        let reward_contract_hash: ContractHash = context.get_account(ali.to_account_hash()).unwrap().named_keys().get(REWARD_CONTRACT_KEY_NAME).unwrap().normalize().into_hash().unwrap().into();
        let reward_token: Key = Key::from(reward_contract_hash);

        let stake_contract_hash: ContractHash = context.get_account(bob.to_account_hash()).unwrap().named_keys().get(STAKE_CONTRACT_KEY_NAME).unwrap().normalize().into_hash().unwrap().into();
        let stake_token: Key = Key::from(stake_contract_hash);

        let session_code = Code::from(CONTRACT_FILE);
        let session_args = runtime_args! {
            STAKE_TOKEN_HASH_KEY_NAME => stake_token,
            REWARD_TOKEN_HASH_KEY_NAME => reward_token,
            STAKING_CONTRACT_KEY_NAME => CONTRACT_NAME.to_string(),
            REWARD_RATE_KEY_NAME => U256::from(20)
        };
        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();
        context.run(session);

        let contract_name = String::from(CONTRACT_NAME);
        let contract_hash_key_name = String::from(contract_name.clone());
        let contract_package_hash_key_name = String::from(contract_name.clone() + "_package_hash");

        let staking_contract_hash: ContractHash = context.get_account(ali.to_account_hash()).unwrap().named_keys().get(&contract_hash_key_name).unwrap().normalize().into_hash().unwrap().into();
        let staking_contract_package_hash: ContractPackageHash = context.get_account(ali.to_account_hash()).unwrap().named_keys().get(&contract_package_hash_key_name).unwrap().normalize().into_hash().unwrap().into();
        
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
            stake_contract_hash: stake_contract_hash,
            reward_contract_hash: reward_contract_hash,
            contract_name: CONTRACT_NAME.to_string(),
            staking_contract_hash: staking_contract_hash,
            staking_contract_package_hash: staking_contract_package_hash,
            current_time: 0 as u64
        }
    }

    pub fn add_time(&mut self, step: u64) {
        self.current_time += step;
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    pub fn staking_contract_name(&self) -> String {
        self.query_contract(STAKING_CONTRACT_KEY_NAME)
            .unwrap()
    }

    pub fn stake_token_hash(&self) -> Key {
        self.query_contract(STAKE_TOKEN_HASH_KEY_NAME)
            .unwrap()
    }

    pub fn reward_token_hash(&self) -> Key {
        self.query_contract(REWARD_TOKEN_HASH_KEY_NAME)
            .unwrap()
    }

    pub fn reward_rate(&self) -> U256 {
        self.query_contract(REWARD_RATE_KEY_NAME)
            .unwrap()
    }

    pub fn last_update_time(&self) -> U256 {
        self.query_contract(LAST_UPDATE_KEY_NAME)
            .unwrap()
    }

    pub fn reward_per_token_stored(&self) -> U256 {
        self.query_contract(REWARD_PER_TOKEN_STORED_KEY_NAME)
            .unwrap()
    }

    pub fn total_supply(&self) -> U256 {
        self.query_contract(TOTAL_SUPPLY_KEY_NAME)
            .unwrap()
    }

    fn call(&mut self, sender: Sender, contract_hash: ContractHash, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(contract_hash.value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .with_block_time(self.current_time)
            .build();
        self.context.run(session);
    }

    pub fn stake(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            self.staking_contract_hash,
            STAKE_ENTRY_POINT_NAME,
            runtime_args! {
                AMOUNT_KEY_NAME => amount
            },
        );
    }

    pub fn stake_token_balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.stake_contract_hash.value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn reward_token_balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.reward_contract_hash.value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn approve_stake_token(&mut self, spender: Address, amount: U256, sender: Sender) {
        self.call(
            sender,
            self.stake_contract_hash,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn allowance_stake_token(&self, owner: Key, spender: Key) -> Option<U256> {
        let mut preimage = Vec::new();
        preimage.append(&mut owner.to_bytes().unwrap());
        preimage.append(&mut spender.to_bytes().unwrap());
        let key_bytes = blake2b256(&preimage);
        let allowance_item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.stake_contract_hash.value());

        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::ALLOWANCES_KEY_NAME.to_string()),
                allowance_item_key,
            )
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }
    
    pub fn transfer_reward_token(&mut self, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            self.reward_contract_hash,
            consts::TRANSFER_ENTRY_POINT_NAME,
            runtime_args! {
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn withdraw(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            self.staking_contract_hash,
            WITHDRAW_ENTRY_POINT_NAME,
            runtime_args! {
                AMOUNT_KEY_NAME => amount
            },
        );
    }

    pub fn get_reward(&mut self, sender: Sender) {
        self.call(
            sender,
            self.staking_contract_hash,
            GET_REWARD_ENTRY_POINT_NAME,
            runtime_args! {},
        );
    }

    /*
    pub fn get_debug_msg(&self, msg: &str) -> String {
        self.query_contract(msg)
            .unwrap()
    }
    */

}