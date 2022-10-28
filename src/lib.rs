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
    pub language_choosen: String,
}

#[near_bindgen]
impl CourseExample {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        Self { 
            owner_id: env::signer_account_id(), 
            storage_deposits: UnorderedMap::new(StorageKey::StorageDeposits),
            language_choosen: "en".to_string(),
         }
    }
    
    #[payable]
    pub fn deposit_to_storage(&mut self) {
        assert!(env::attached_deposit() > ONE_NEAR, "Deposit must be greater than 1 NEAR");
        let account_id: AccountId = env::signer_account_id();
        let deposit: u128 = env::attached_deposit();
        env::log_str(&format!("You have deposited {} NEAR", (deposit / ONE_NEAR ) ));
        // Check if the account has already deposited
        if self.storage_deposits.get(&account_id).is_some() {
            // If the account has already deposited, add the new deposit to the existing deposit
            let existing_deposit: u128 = self.storage_deposits.get(&account_id).unwrap();
            self.storage_deposits.remove(&account_id);
            self.storage_deposits.insert(&account_id, &(existing_deposit + deposit));
        } else {
            // If the account has not deposited, add the new deposit to the storage deposits
            self.storage_deposits.insert(&account_id, &deposit);
        }
        env::log_str(&format!("Your total deposit is {} NEAR", (self.storage_deposits.get(&account_id).unwrap() / ONE_NEAR ) ));
    }

    pub fn withdraw_storage(&mut self) {
        let account_id: AccountId = env::signer_account_id();
        let deposit: u128 = self.storage_deposits.get(&account_id).unwrap_or(0);
        assert!(deposit > 0, "No storage deposit to withdraw");
        env::log_str(&format!("Withdrew {} NEAR", (deposit / ONE_NEAR)));
        self.storage_deposits.remove(&account_id);
        Promise::new(account_id).transfer(deposit);
    }

    pub fn get_sum_of_deposits(&self) -> U128 {
        let mut sum: u128 = 0;
        for deposit in self.storage_deposits.values() {
            sum += deposit;
        }
        U128(sum)
    }

    pub fn hello_name(&self, name: String) -> String {
        format!("Hello {}, thank you for joining the course!", name)
    }

    pub fn toggle_choosen_language(&mut self) -> String {
        if self.language_choosen == "en".to_string() {
            self.language_choosen = "tr".to_string();
        } else {
            self.language_choosen = "en".to_string();
        }
        format!("You have changed the language to {}", self.language_choosen)
    }

    pub fn hello_name_language(&self, name: String) -> String {
        let mut hello = "Hello".to_string();
        if self.language_choosen == "tr" {
            hello = "selam".to_string();
        }
        format!("{} {}", hello, name)
    }





}