use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc20::constants as consts;
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};
use casper_contract::contract_api::unwrap_or_revert::UnwrapOrRevert;

//#[path = "././stake/src/constants.rs"]
//mod constants;

//use constants as consts;

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
const CONTRACT_KEY_NAME: &str = "stake_wcspr_reward_picas";


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
}

impl TestFixture {
    pub const STAKE_TOKEN: Key = Key::from_formatted_str("hash-d60beb45bdd2002e6e2581467f94196ec9cf4683c25cabe1ffefa4a14d2bb47b").unwrap();
    pub const REWARD_TOKEN: Key = Key::from_formatted_str("hash-d60beb45bdd2002e6e2581467f94196ec9cf4683c25cabe1ffefa4a14d2bb47b").unwrap();
    pub const REWARD_RATE: U256 = U256::from(100000000000);
    pub const STAKING_CONTRACT_KEY_NAME: &'static str = "stake_wcspr_reward_picas";
    //const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 1000;

    //pub fn token_total_supply() -> U256 {
    //  Self::TOKEN_TOTAL_SUPPLY_AS_U64.into()
    //}

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_FILE);
        let session_args = runtime_args! {
            STAKING_CONTRACT_KEY_NAME => TestFixture::STAKING_CONTRACT_KEY_NAME,
            STAKE_TOKEN_HASH_KEY_NAME => TestFixture::STAKE_TOKEN,
            REWARD_TOKEN_HASH_KEY_NAME => TestFixture::REWARD_TOKEN,
            REWARD_RATE_KEY_NAME => TestFixture::REWARD_RATE
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
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

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
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

    pub fn balance(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn reward(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(REWARDS_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn user_reward_per_token_paid(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(USER_REWARD_PER_TOKEN_PAID_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn stake(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            STAKE_ENTRY_POINT_NAME,
            runtime_args! {
                AMOUNT_KEY_NAME => amount
            },
        );
    }

    pub fn withdraw(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            WITHDRAW_ENTRY_POINT_NAME,
            runtime_args! {
                AMOUNT_KEY_NAME => amount
            },
        );
    }

    pub fn get_reward(&mut self, sender: Sender) {
        self.call(
            sender,
            GET_REWARD_ENTRY_POINT_NAME,
            runtime_args! {},
        );
    }
}
