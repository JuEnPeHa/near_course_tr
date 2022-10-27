use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, BorshStorageKey, AccountId, PanicOnDefault, Promise};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};

const ONE_NEAR: u128 = 1_000_000_000_000_000_000_000_000;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKey {
    StorageDeposits
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CourseExample {
    pub owner_id: AccountId,
    pub storage_deposits: UnorderedMap<AccountId, u128>,
}

#[near_bindgen]
impl CourseExample {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self { owner_id: env::signer_account_id(), storage_deposits: UnorderedMap::new(StorageKey::StorageDeposits) }
    }
    
    #[payable]
    pub fn deposit_to_storage(&mut self) {
        assert!(env::attached_deposit() > ONE_NEAR, "Deposit must be greater than 1 NEAR");
        let account_id: AccountId = env::signer_account_id();
        let deposit: u128 = env::attached_deposit();
        env::log_str(&format!("You have deposited {} NEAR", (deposit / ONE_NEAR ) ));
        self.storage_deposits.insert(&account_id, &deposit);
    }

    pub fn withdraw_storage(&mut self) {
        let account_id: AccountId = env::signer_account_id();
        let deposit: u128 = self.storage_deposits.get(&account_id).unwrap_or(0);
        assert!(deposit > 0, "No storage deposit to withdraw");
        env::log_str(&format!("Withdrew {} NEAR", (deposit / ONE_NEAR)));
        self.storage_deposits.remove(&account_id);
        Promise::new(account_id).transfer(deposit);
    }

    pub fn get_get_sum_of_deposits(&self) -> U128 {
        let mut sum: u128 = 0;
        for deposit in self.storage_deposits.values() {
            sum += deposit;
        }
        U128(sum)
    }
}